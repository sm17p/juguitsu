use std::{
    fs,
    path::{Path, PathBuf},
};

use jj_lib::{
    config::{ConfigGetError, StackedConfig},
    git_backend::GitBackend,
    repo::{read_store_type, Repo, RepoLoaderError, StoreFactories},
    settings::UserSettings,
    workspace::{
        default_working_copy_factories, DefaultWorkspaceLoaderFactory, WorkspaceLoadError,
        WorkspaceLoader, WorkspaceLoaderFactory,
    },
    workspace_store::{SimpleWorkspaceStore, WorkspaceStore, WorkspaceStoreError},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjWorkspaceError {
    #[error("not a jj repository: {0}")]
    NotJjRepo(String),
    #[error(transparent)]
    Load(#[from] WorkspaceLoadError),
    #[error(transparent)]
    Config(#[from] ConfigGetError),
    #[error(transparent)]
    Repo(#[from] RepoLoaderError),
    #[error(transparent)]
    WorkspaceStore(#[from] WorkspaceStoreError),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepoEngine {
    Jj,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepoLinkKind {
    Inline,
    Pointer,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GitRepoLayout {
    Colocated,
    Hidden,
    External,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SiblingWorkspace {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct JjWorkspaceInspect {
    pub workspace_root: PathBuf,
    pub opened_path: Option<PathBuf>,
    pub workspace_name: String,
    pub repo_path: PathBuf,
    pub repo_link_kind: RepoLinkKind,
    pub repo_path_main: Option<PathBuf>,
    pub commit_store: String,
    pub op_store: String,
    pub op_heads: String,
    pub working_copy_store: String,
    pub colocated: bool,
    pub git_repo_layout: GitRepoLayout,
    pub sibling_workspaces: Vec<SiblingWorkspace>,
}

pub fn is_valid_workspace_root(path: &Path) -> bool {
    workspace_root_for(path)
        .and_then(|root| create_loader(&root).ok())
        .is_some()
}

pub async fn inspect_jj_workspace(picked: &Path) -> Result<JjWorkspaceInspect, JjWorkspaceError> {
    if !picked.is_dir() {
        return Err(JjWorkspaceError::NotJjRepo(picked.display().to_string()));
    }

    let workspace_root = workspace_root_for(picked)
        .ok_or_else(|| JjWorkspaceError::NotJjRepo(picked.display().to_string()))?;
    let canonical_root = canonicalize_path(&workspace_root);
    let canonical_picked = canonicalize_path(picked);
    let opened_path = if canonical_root == canonical_picked {
        None
    } else {
        Some(canonical_picked)
    };

    let loader = create_loader(&workspace_root)?;
    let working_copy_store = loader
        .get_working_copy_type()
        .unwrap_or_else(|_| "unknown".into());
    let workspace = load_workspace(loader.as_ref())?;
    let repo_path = workspace.repo_path().to_path_buf();
    let repo = workspace.repo_loader().load_at_head().await?;
    let store = workspace.repo_loader().store();

    let (repo_link_kind, repo_path_main) = repo_link_kind(&canonical_root, &repo_path);
    let commit_store = store.backend().name().to_owned();
    let op_store = read_store_type_or_unknown("operation", repo_path.join("op_store").join("type"));
    let op_heads =
        read_store_type_or_unknown("operation heads", repo_path.join("op_heads").join("type"));
    let workspace_name = workspace.workspace_name().as_str().to_owned();
    let (colocated, git_repo_layout) = git_layout_from_store(&canonical_root, &repo_path, store);
    let sibling_workspaces = sibling_workspaces(&workspace, repo.as_ref(), &canonical_root)?;

    Ok(JjWorkspaceInspect {
        workspace_root: canonical_root,
        opened_path,
        workspace_name,
        repo_path,
        repo_link_kind,
        repo_path_main,
        commit_store,
        op_store,
        op_heads,
        working_copy_store,
        colocated,
        git_repo_layout,
        sibling_workspaces,
    })
}

fn create_loader(workspace_root: &Path) -> Result<Box<dyn WorkspaceLoader>, JjWorkspaceError> {
    DefaultWorkspaceLoaderFactory
        .create(workspace_root)
        .map_err(|error| match error {
            WorkspaceLoadError::NoWorkspaceHere(path) => {
                JjWorkspaceError::NotJjRepo(path.display().to_string())
            }
            other => JjWorkspaceError::Load(other),
        })
}

fn load_workspace(
    loader: &dyn WorkspaceLoader,
) -> Result<jj_lib::workspace::Workspace, JjWorkspaceError> {
    let config = StackedConfig::with_defaults();
    let settings = UserSettings::from_config(config)?;
    loader
        .load(
            &settings,
            &StoreFactories::default(),
            &default_working_copy_factories(),
        )
        .map_err(JjWorkspaceError::Load)
}

fn workspace_root_for(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|ancestor| ancestor.join(".jj").is_dir())
        .map(canonicalize_path)
}

fn canonicalize_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_owned())
}

fn paths_equal(left: &Path, right: &Path) -> bool {
    canonicalize_path(left) == canonicalize_path(right)
}

fn repo_link_kind(
    workspace_root: &Path,
    resolved_repo_path: &Path,
) -> (RepoLinkKind, Option<PathBuf>) {
    let inline_candidate = workspace_root.join(".jj").join("repo");
    if paths_equal(&inline_candidate, resolved_repo_path) {
        (RepoLinkKind::Inline, None)
    } else {
        (RepoLinkKind::Pointer, Some(resolved_repo_path.to_path_buf()))
    }
}

fn read_store_type_or_unknown(store: &'static str, path: PathBuf) -> String {
    read_store_type(store, path)
        .map(|value| value.trim().to_owned())
        .unwrap_or_else(|_| "unknown".into())
}

fn git_layout_from_store(
    workspace_root: &Path,
    repo_path: &Path,
    store: &jj_lib::store::Store,
) -> (bool, GitRepoLayout) {
    let Some(git) = store.backend_impl::<GitBackend>() else {
        return (false, GitRepoLayout::None);
    };

    match git.git_workdir() {
        Some(workdir) if paths_equal(workdir, workspace_root) => (true, GitRepoLayout::Colocated),
        Some(_) => (false, GitRepoLayout::External),
        None => {
            let hidden_git = repo_path.join("store").join("git");
            if paths_equal(git.git_repo_path(), &hidden_git) {
                (false, GitRepoLayout::Hidden)
            } else {
                (false, GitRepoLayout::External)
            }
        }
    }
}

fn sibling_workspaces(
    workspace: &jj_lib::workspace::Workspace,
    repo: &dyn Repo,
    workspace_root: &Path,
) -> Result<Vec<SiblingWorkspace>, JjWorkspaceError> {
    let workspace_store = SimpleWorkspaceStore::load(workspace.repo_path())?;
    let current = workspace.workspace_name();
    let mut siblings = Vec::new();

    for name in repo.view().wc_commit_ids().keys() {
        if name.as_str() == current.as_str() {
            continue;
        }
        let Some(relative) = workspace_store.get_workspace_path(name.as_ref())? else {
            continue;
        };
        let absolute = canonicalize_path(&workspace.repo_path().join(relative));
        if absolute == workspace_root {
            continue;
        }
        siblings.push(SiblingWorkspace {
            name: name.as_str().to_string(),
            path: absolute.to_string_lossy().into_owned(),
        });
    }

    Ok(siblings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn inspects_fixture_workspace_when_present() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        if !root.join(".jj").is_dir() {
            return;
        }
        let inspect = inspect_jj_workspace(&root)
            .await
            .expect("fixture workspace");
        assert_eq!(inspect.commit_store, "git");
        assert_eq!(inspect.workspace_name, "default");
        assert_eq!(inspect.git_repo_layout, GitRepoLayout::Colocated);
        assert!(inspect.colocated);
    }
}
