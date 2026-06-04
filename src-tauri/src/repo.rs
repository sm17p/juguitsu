use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use thiserror::Error;

use crate::jj_workspace::{self, GitRepoLayout, RepoEngine, RepoLinkKind, SiblingWorkspace};

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
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepoSummary {
    pub workspace_root: String,
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
    pub cwd_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer_repo_path: Option<String>,
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

fn enforce_max_recents(recents: &mut Vec<RepoSummary>) {
    while recents.len() > MAX_RECENTS {
        recents.remove(
            recents
                .iter()
                .enumerate()
                .min_by_key(|(_, workspace_summary)| workspace_summary.last_opened_at)
                .map(|(index, _)| index)
                .unwrap_or(0),
        );
    }
}

fn load_recents(app: &AppHandle) -> Result<RecentsStore, RepoError> {
    let recents_file_path = recents_path(&resolve_data_dir(app)?);

    if !recents_file_path.exists() {
        return Ok(RecentsStore::empty());
    }

    let recents_store = match serde_json::from_str::<RecentsStore>(&fs::read_to_string(
        &recents_file_path,
    )?) {
        Ok(recents_store) if recents_store.schema_version == RECENTS_SCHEMA_VERSION => {
            recents_store
        }
        Ok(recents_store) => {
            tracing::error!(
                version = recents_store.schema_version,
                "unsupported recents schema"
            );
            return Ok(RecentsStore::empty());
        }
        Err(error) => {
            tracing::error!(error = %error, "failed to parse recents");
            return Ok(RecentsStore::empty());
        }
    };

    let recents_before_prune = recents_store.recents.clone();
    let mut workspace_summaries = recents_store
        .recents
        .into_iter()
        .filter(|workspace_summary| {
            jj_workspace::is_valid_workspace_root(Path::new(&workspace_summary.workspace_root))
        })
        .collect();
    enforce_max_recents(&mut workspace_summaries);

    if workspace_summaries != recents_before_prune {
        save_recents_store(
            app,
            &RecentsStore {
                schema_version: RECENTS_SCHEMA_VERSION,
                recents: workspace_summaries.clone(),
            },
        )?;
    }

    Ok(RecentsStore {
        schema_version: RECENTS_SCHEMA_VERSION,
        recents: workspace_summaries,
    })
}

fn save_recents_store(app: &AppHandle, recents_store: &RecentsStore) -> Result<(), RepoError> {
    let data_dir = resolve_data_dir(app)?;
    fs::create_dir_all(&data_dir)?;

    let tmp_path = data_dir.join(RECENTS_TMP);

    {
        let mut file = File::create(&tmp_path)?;
        file.write_all(serde_json::to_string_pretty(recents_store)?.as_bytes())?;
        file.sync_all()?;
    }

    fs::rename(tmp_path, recents_path(&data_dir))?;
    Ok(())
}

#[tauri::command]
pub fn list_recent_repos(app: AppHandle) -> Result<Vec<RepoSummary>, String> {
    load_recents(&app)
        .map(|recents_store| recents_store.recents)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn open_repo_at(app: AppHandle, path: String) -> Result<RepoSummary, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let workspace_inspect = futures::executor::block_on(jj_workspace::inspect_jj_workspace(
            Path::new(&path),
        ))
        .map_err(|error| match error {
            jj_workspace::JjWorkspaceError::NotJjRepo(message) => RepoError::NotJjRepo(message),
            other => RepoError::NotJjRepo(other.to_string()),
        })?;
        let workspace_summary = RepoSummary {
            workspace_root: workspace_inspect
                .workspace_root
                .to_string_lossy()
                .into_owned(),
            name: workspace_inspect
                .workspace_root
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("repo")
                .to_string(),
            last_opened_at: jiff::Timestamp::now()
                .as_second()
                .try_into()
                .unwrap_or(0),
            cwd_path: workspace_inspect
                .cwd_path
                .map(|cwd_path| cwd_path.to_string_lossy().into_owned()),
            repo_path: workspace_inspect.repo_path.to_string_lossy().into_owned(),
            pointer_repo_path: workspace_inspect
                .pointer_repo_path
                .map(|pointer_repo_path| pointer_repo_path.to_string_lossy().into_owned()),
            sibling_workspaces: if workspace_inspect.sibling_workspaces.is_empty() {
                None
            } else {
                Some(workspace_inspect.sibling_workspaces)
            },
            engine: RepoEngine::Jj,
            workspace_name: workspace_inspect.workspace_name,
            repo_link_kind: workspace_inspect.repo_link_kind,
            commit_store: workspace_inspect.commit_store,
            op_store: workspace_inspect.op_store,
            op_heads: workspace_inspect.op_heads,
            working_copy_store: workspace_inspect.working_copy_store,
            colocated: workspace_inspect.colocated,
            git_repo_layout: workspace_inspect.git_repo_layout,
        };

        let mut recents_store = load_recents(&app).unwrap_or_else(|_| RecentsStore::empty());
        if let Some(existing) = recents_store.recents.iter_mut().find(|existing| {
            existing.workspace_root == workspace_summary.workspace_root
        }) {
            *existing = workspace_summary.clone();
        } else {
            recents_store.recents.push(workspace_summary.clone());
            enforce_max_recents(&mut recents_store.recents);
        }
        save_recents_store(&app, &recents_store)?;

        tracing::info!(
            workspace_root = %workspace_summary.workspace_root,
            workspace_name = %workspace_summary.workspace_name,
            commit_store = %workspace_summary.commit_store,
            git_repo_layout = ?workspace_summary.git_repo_layout,
            "opened workspace"
        );
        Ok(workspace_summary)
    })
    .await
    .map_err(|join_error| join_error.to_string())?
    .map_err(|error: RepoError| {
        tracing::error!(error = %error, "failed to open workspace");
        error.to_string()
    })
}
