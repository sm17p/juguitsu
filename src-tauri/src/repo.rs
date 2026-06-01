use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use thiserror::Error;

use crate::jj_workspace::{
    self, GitRepoLayout, JjWorkspaceInspect, RepoEngine, RepoLinkKind, SiblingWorkspace,
};

const RECENTS_SCHEMA_VERSION: u32 = 2;
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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepoSummary {
    pub path: String,
    pub name: String,
    pub last_opened_at: u64,
    pub engine: RepoEngine,
    pub workspace_name: String,
    pub repo_path: String,
    pub repo_link_kind: RepoLinkKind,
    pub commit_store: String,
    pub op_store: String,
    pub op_heads: String,
    pub working_copy_store: String,
    pub colocated: bool,
    pub git_repo_layout: GitRepoLayout,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_path_main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sibling_workspaces: Option<Vec<SiblingWorkspace>>,
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

fn repo_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("repo")
        .to_string()
}

fn summary_from_inspect(inspect: JjWorkspaceInspect, last_opened_at: u64) -> RepoSummary {
    let path = inspect.workspace_root.to_string_lossy().into_owned();
    let name = repo_name(&inspect.workspace_root);
    let opened_path = inspect
        .opened_path
        .map(|value| value.to_string_lossy().into_owned());
    let repo_path = inspect.repo_path.to_string_lossy().into_owned();
    let repo_path_main = inspect
        .repo_path_main
        .map(|value| value.to_string_lossy().into_owned());
    let sibling_workspaces = if inspect.sibling_workspaces.is_empty() {
        None
    } else {
        Some(inspect.sibling_workspaces)
    };

    RepoSummary {
        path,
        name,
        last_opened_at,
        engine: RepoEngine::Jj,
        workspace_name: inspect.workspace_name,
        repo_path,
        repo_link_kind: inspect.repo_link_kind,
        commit_store: inspect.commit_store,
        op_store: inspect.op_store,
        op_heads: inspect.op_heads,
        working_copy_store: inspect.working_copy_store,
        colocated: inspect.colocated,
        git_repo_layout: inspect.git_repo_layout,
        opened_path,
        repo_path_main,
        sibling_workspaces,
    }
}

fn workspace_root_valid(path: &Path) -> bool {
    path.is_dir() && path.join(".jj").is_dir()
}

fn prune_recents(recents: Vec<RepoSummary>) -> Vec<RepoSummary> {
    recents
        .into_iter()
        .filter(|recent| workspace_root_valid(Path::new(&recent.path)))
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
        *existing = summary;
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
    let store = match serde_json::from_str::<RecentsStore>(&raw) {
        Ok(store) if store.schema_version == RECENTS_SCHEMA_VERSION => store,
        Ok(store) => {
            tracing::error!(version = store.schema_version, "unsupported recents schema");
            return Ok(RecentsStore::empty());
        }
        Err(error) => {
            tracing::error!(error = %error, "failed to parse recents");
            return Ok(RecentsStore::empty());
        }
    };

    let before_prune = store.recents.clone();
    let mut recents = prune_recents(store.recents);
    enforce_max_recents(&mut recents);

    let needs_save = recents != before_prune;

    if needs_save {
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
    let inspect = jj_workspace::inspect_jj_workspace(&path).map_err(|error| match error {
        jj_workspace::JjWorkspaceError::NotJjRepo(message) => RepoError::NotJjRepo(message),
        jj_workspace::JjWorkspaceError::Io(io_error) => RepoError::Read(io_error),
    })?;
    let summary = summary_from_inspect(inspect, now_unix());

    let mut store = load_recents(app).unwrap_or_else(|_| RecentsStore::empty());
    upsert_recent(&mut store.recents, summary.clone());
    save_recents_store(app, &store)?;

    tracing::info!(
        path = %summary.path,
        workspace_name = %summary.workspace_name,
        commit_store = %summary.commit_store,
        git_repo_layout = ?summary.git_repo_layout,
        "opened repo"
    );
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
