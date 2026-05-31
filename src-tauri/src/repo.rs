use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use thiserror::Error;

const RECENTS_SCHEMA_VERSION: u32 = 1;
const RECENTS_FILE: &str = "recents.json";
const RECENTS_TMP: &str = "recents.json.tmp";
const MAX_RECENTS: usize = 20;

#[derive(Error, Debug)]
enum RepoError {
    #[error("app data dir unavailable: {0}")]
    DataDir(String),
    #[error("failed to read recents: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse recents: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("not a jj repository: {0}")]
    NotJjRepo(String),
    #[error("unsupported recents schema version: {0}")]
    UnsupportedSchema(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepoSummary {
    pub path: String,
    pub name: String,
    pub last_opened_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct RecentsStore {
    schema_version: u32,
    recents: Vec<RepoSummary>,
}

impl RecentsStore {
    fn empty() -> Self {
        Self {
            schema_version: RECENTS_SCHEMA_VERSION,
            recents: Vec::new(),
        }
    }
}

fn resolve_data_dir(app: &AppHandle) -> Result<PathBuf, RepoError> {
    app.path()
        .app_data_dir()
        .map_err(|error| RepoError::DataDir(error.to_string()))
}

fn recents_path(data_dir: &Path) -> PathBuf {
    data_dir.join(RECENTS_FILE)
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn is_jj_repo(path: &Path) -> bool {
    path.join(".jj").is_dir()
}

fn repo_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repo")
        .to_string()
}

fn migrate_recents_store(store: RecentsStore) -> Result<RecentsStore, RepoError> {
    match store.schema_version {
        RECENTS_SCHEMA_VERSION => Ok(store),
        version => Err(RepoError::UnsupportedSchema(version)),
    }
}

fn prune_recents(recents: Vec<RepoSummary>) -> Vec<RepoSummary> {
    recents
        .into_iter()
        .filter(|recent| {
            let path = Path::new(&recent.path);
            path.is_dir() && is_jj_repo(path)
        })
        .collect()
}

fn enforce_max_recents(recents: &mut Vec<RepoSummary>) {
    while recents.len() > MAX_RECENTS {
        let oldest_idx = recents
            .iter()
            .enumerate()
            .min_by_key(|(_, recent)| recent.last_opened_at)
            .map(|(index, _)| index)
            .unwrap_or(0);
        recents.remove(oldest_idx);
    }
}

fn upsert_recent(recents: &mut Vec<RepoSummary>, summary: RepoSummary) {
    if let Some(existing) = recents
        .iter_mut()
        .find(|recent| recent.path == summary.path)
    {
        existing.name = summary.name;
        existing.last_opened_at = summary.last_opened_at;
        return;
    }
    recents.push(summary);
    enforce_max_recents(recents);
}

fn load_recents(app: &AppHandle) -> Result<RecentsStore, RepoError> {
    let data_dir = resolve_data_dir(app)?;
    let path = recents_path(&data_dir);

    if !path.exists() {
        return Ok(RecentsStore::empty());
    }

    let raw = fs::read_to_string(&path)?;
    let parsed: RecentsStore = match serde_json::from_str(&raw) {
        Ok(store) => store,
        Err(error) => {
            tracing::error!(error = %error, "failed to parse recents");
            return Ok(RecentsStore::empty());
        }
    };

    let store = match migrate_recents_store(parsed) {
        Ok(store) => store,
        Err(error) => {
            tracing::error!(error = %error, "unsupported recents schema");
            return Ok(RecentsStore::empty());
        }
    };

    let original_len = store.recents.len();
    let mut recents = prune_recents(store.recents);
    enforce_max_recents(&mut recents);

    if recents.len() != original_len {
        save_recents_store(
            app,
            &RecentsStore {
                schema_version: RECENTS_SCHEMA_VERSION,
                recents: recents.clone(),
            },
        )?;
    }

    Ok(RecentsStore {
        schema_version: RECENTS_SCHEMA_VERSION,
        recents,
    })
}

fn save_recents_store(app: &AppHandle, store: &RecentsStore) -> Result<(), RepoError> {
    let data_dir = resolve_data_dir(app)?;
    fs::create_dir_all(&data_dir)?;

    let path = recents_path(&data_dir);
    let tmp_path = data_dir.join(RECENTS_TMP);
    let json = serde_json::to_string_pretty(store)?;

    {
        let mut file = File::create(&tmp_path)?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?;
    }

    fs::rename(tmp_path, path)?;
    Ok(())
}

fn open_repo_at_path(app: &AppHandle, path: PathBuf) -> Result<RepoSummary, RepoError> {
    if !path.is_dir() || !is_jj_repo(&path) {
        return Err(RepoError::NotJjRepo(path.display().to_string()));
    }

    let summary = RepoSummary {
        path: path.to_string_lossy().into_owned(),
        name: repo_name(&path),
        last_opened_at: now_unix(),
    };

    let mut store = load_recents(app).unwrap_or_else(|_| RecentsStore::empty());
    upsert_recent(&mut store.recents, summary.clone());
    save_recents_store(app, &store)?;

    tracing::info!(path = %summary.path, "opened repo");
    Ok(summary)
}

#[tauri::command]
pub fn list_recent_repos(app: AppHandle) -> Result<Vec<RepoSummary>, String> {
    load_recents(&app)
        .map(|store| store.recents)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_repo_at(app: AppHandle, path: String) -> Result<RepoSummary, String> {
    open_repo_at_path(&app, PathBuf::from(path)).map_err(|error| {
        tracing::error!(error = %error, "failed to open repo");
        error.to_string()
    })
}
