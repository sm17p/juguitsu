use std::{
    fs,
    path::{Path, PathBuf},
};

use jj_lib::{
    config::{ConfigGetError, StackedConfig},
    git_backend::GitBackend,
    repo::{read_store_type, RepoLoaderError, StoreFactories},
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
    pub workspace_name: String,
    pub workspace_root: String,
}

#[derive(Clone, Debug)]
pub struct JjWorkspaceInspect {
    pub workspace_root: PathBuf,
    pub cwd_path: Option<PathBuf>,
    pub workspace_name: String,
    pub repo_path: PathBuf,
    pub repo_link_kind: RepoLinkKind,
    pub pointer_repo_path: Option<PathBuf>,
    pub commit_store: String,
    pub op_store: String,
    pub op_heads: String,
    pub working_copy_store: String,
    pub colocated: bool,
    pub git_repo_layout: GitRepoLayout,
    pub sibling_workspaces: Vec<SiblingWorkspace>,
}

pub fn is_valid_workspace_root(path: &Path) -> bool {
    find_workspace_root(path)
        .and_then(|workspace_root| create_workspace_loader(&workspace_root).ok())
        .is_some()
}

pub async fn inspect_jj_workspace(start_path: &Path) -> Result<JjWorkspaceInspect, JjWorkspaceError> {
    if !start_path.is_dir() {
        return Err(JjWorkspaceError::NotJjRepo(start_path.display().to_string()));
    }

    let discovered_workspace_root = find_workspace_root(start_path)
        .ok_or_else(|| JjWorkspaceError::NotJjRepo(start_path.display().to_string()))?;
    let workspace_root = canonicalize_path(&discovered_workspace_root);
    let canonical_cwd_path = canonicalize_path(start_path);

    let workspace_loader = create_workspace_loader(&discovered_workspace_root)?;
    let loaded_workspace = workspace_loader
        .load(
            &UserSettings::from_config(StackedConfig::with_defaults())?,
            &StoreFactories::default(),
            &default_working_copy_factories(),
        )
        .map_err(JjWorkspaceError::Load)?;
    let repo_path = loaded_workspace.repo_path().to_path_buf();
    let jj_repo = loaded_workspace.repo_loader().load_at_head().await?;
    let repo_store = loaded_workspace.repo_loader().store();

    let inline_repo_path = workspace_root.join(".jj").join("repo");
    let (repo_link_kind, pointer_repo_path) = if paths_equal(&inline_repo_path, &repo_path) {
        (RepoLinkKind::Inline, None)
    } else {
        (
            RepoLinkKind::Pointer,
            Some(repo_path.to_path_buf()),
        )
    };

    let (colocated, git_repo_layout) = match repo_store.backend_impl::<GitBackend>() {
        None => (false, GitRepoLayout::None),
        Some(git_backend) => match git_backend.git_workdir() {
            Some(git_workdir) if paths_equal(git_workdir, &workspace_root) => {
                (true, GitRepoLayout::Colocated)
            }
            Some(_) => (false, GitRepoLayout::External),
            None => {
                let hidden_git_path = repo_path.join("store").join("git");
                if paths_equal(git_backend.git_repo_path(), &hidden_git_path) {
                    (false, GitRepoLayout::Hidden)
                } else {
                    (false, GitRepoLayout::External)
                }
            }
        },
    };

    let jj_workspace_store = SimpleWorkspaceStore::load(loaded_workspace.repo_path())?;
    let active_workspace_name = loaded_workspace.workspace_name();
    let mut sibling_workspaces = Vec::new();
    for workspace_name in jj_repo.view().wc_commit_ids().keys() {
        if workspace_name.as_str() == active_workspace_name.as_str() {
            continue;
        }
        let Some(relative_workspace_path) =
            jj_workspace_store.get_workspace_path(workspace_name.as_ref())?
        else {
            continue;
        };
        let sibling_workspace_root =
            canonicalize_path(&loaded_workspace.repo_path().join(relative_workspace_path));
        if sibling_workspace_root == workspace_root {
            continue;
        }
        sibling_workspaces.push(SiblingWorkspace {
            workspace_name: workspace_name.as_str().to_string(),
            workspace_root: sibling_workspace_root.to_string_lossy().into_owned(),
        });
    }

    Ok(JjWorkspaceInspect {
        cwd_path: if workspace_root == canonical_cwd_path {
            None
        } else {
            Some(canonical_cwd_path)
        },
        commit_store: repo_store.backend().name().to_owned(),
        op_store: read_store_type_or_unknown(
            "operation",
            repo_path.join("op_store").join("type"),
        ),
        op_heads: read_store_type_or_unknown(
            "operation heads",
            repo_path.join("op_heads").join("type"),
        ),
        working_copy_store: workspace_loader
            .get_working_copy_type()
            .unwrap_or_else(|_| "unknown".into()),
        workspace_name: loaded_workspace.workspace_name().as_str().to_owned(),
        workspace_root,
        repo_path,
        repo_link_kind,
        pointer_repo_path,
        colocated,
        git_repo_layout,
        sibling_workspaces,
    })
}

fn create_workspace_loader(
    workspace_root: &Path,
) -> Result<Box<dyn WorkspaceLoader>, JjWorkspaceError> {
    DefaultWorkspaceLoaderFactory
        .create(workspace_root)
        .map_err(|error| match error {
            WorkspaceLoadError::NoWorkspaceHere(path) => {
                JjWorkspaceError::NotJjRepo(path.display().to_string())
            }
            other => JjWorkspaceError::Load(other),
        })
}

fn find_workspace_root(path: &Path) -> Option<PathBuf> {
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

fn read_store_type_or_unknown(store: &'static str, path: PathBuf) -> String {
    read_store_type(store, path)
        .map(|value| value.trim().to_owned())
        .unwrap_or_else(|_| "unknown".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn inspects_fixture_workspace_when_present() {
        let fixture_workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        if !fixture_workspace_root.join(".jj").is_dir() {
            return;
        }
        let workspace_inspect = inspect_jj_workspace(&fixture_workspace_root)
            .await
            .expect("fixture workspace");
        assert_eq!(workspace_inspect.commit_store, "git");
        assert_eq!(workspace_inspect.workspace_name, "default");
        assert_eq!(workspace_inspect.git_repo_layout, GitRepoLayout::Colocated);
        assert!(workspace_inspect.colocated);
    }
}
