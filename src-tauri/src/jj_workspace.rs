use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjWorkspaceError {
    #[error("not a jj repository: {0}")]
    NotJjRepo(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
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

pub fn inspect_jj_workspace(picked: &Path) -> Result<JjWorkspaceInspect, JjWorkspaceError> {
    if !picked.is_dir() {
        return Err(JjWorkspaceError::NotJjRepo(picked.display().to_string()));
    }

    let workspace_root = find_workspace_root(picked)
        .ok_or_else(|| JjWorkspaceError::NotJjRepo(picked.display().to_string()))?;
    let opened_path = if workspace_root == picked {
        None
    } else {
        Some(canonicalize_path(picked))
    };

    let jj_dir = workspace_root.join(".jj");
    let (repo_path, repo_link_kind) = resolve_repo_dir(&jj_dir)?;
    let repo_path_main = match repo_link_kind {
        RepoLinkKind::Pointer => Some(repo_path.clone()),
        RepoLinkKind::Inline => None,
    };

    let commit_store = read_type_file(&repo_path.join("store").join("type"))
        .unwrap_or_else(|| "unknown".into());
    let op_store = read_type_file(&repo_path.join("op_store").join("type"))
        .unwrap_or_else(|| "unknown".into());
    let op_heads = read_type_file(&repo_path.join("op_heads").join("type"))
        .unwrap_or_else(|| "unknown".into());
    let working_copy_store = read_type_file(&jj_dir.join("working_copy").join("type"))
        .unwrap_or_else(|| "unknown".into());

    let workspace_name = read_workspace_name(&jj_dir.join("working_copy").join("checkout"));
    let (colocated, git_repo_layout) = detect_git_layout(&workspace_root, &repo_path);
    let sibling_workspaces = read_sibling_workspaces(&repo_path, &workspace_root);

    Ok(JjWorkspaceInspect {
        workspace_root,
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

fn find_workspace_root(mut path: &Path) -> Option<PathBuf> {
    loop {
        if path.join(".jj").is_dir() {
            return Some(canonicalize_path(path));
        }
        path = path.parent()?;
    }
}

fn canonicalize_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_owned())
}

fn resolve_repo_dir(jj_dir: &Path) -> Result<(PathBuf, RepoLinkKind), JjWorkspaceError> {
    let repo_entry = jj_dir.join("repo");
    if repo_entry.is_file() {
        let bytes = fs::read(&repo_entry)?;
        let relative = path_from_os_bytes(&bytes)
            .ok_or_else(|| JjWorkspaceError::NotJjRepo(jj_dir.display().to_string()))?;
        let target = jj_dir.join(relative);
        if !target.is_dir() {
            return Err(JjWorkspaceError::NotJjRepo(target.display().to_string()));
        }
        return Ok((canonicalize_path(&target), RepoLinkKind::Pointer));
    }
    if repo_entry.is_dir() {
        return Ok((canonicalize_path(&repo_entry), RepoLinkKind::Inline));
    }
    Err(JjWorkspaceError::NotJjRepo(
        jj_dir.display().to_string(),
    ))
}

fn path_from_os_bytes(bytes: &[u8]) -> Option<PathBuf> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        Some(PathBuf::from(OsStr::from_bytes(bytes)))
    }
    #[cfg(not(unix))]
    {
        let text = std::str::from_utf8(bytes).ok()?;
        Some(PathBuf::from(text.trim()))
    }
}

fn read_type_file(path: &Path) -> Option<String> {
    let raw = fs::read_to_string(path).ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn read_workspace_name(checkout_path: &Path) -> String {
    let bytes = match fs::read(checkout_path) {
        Ok(value) => value,
        Err(_) => return "default".into(),
    };
    for field in [3u32, 2u32] {
        if let Some(name) = read_proto_string_field(&bytes, field) {
            if !name.is_empty() {
                return name;
            }
        }
    }
    "default".into()
}

fn read_proto_string_field(data: &[u8], field_number: u32) -> Option<String> {
    let key = u8::try_from((field_number << 3) | 2).ok()?;
    let mut offset = 0usize;
    while offset < data.len() {
        if data[offset] != key {
            let skipped = skip_proto_field(&data[offset..])?;
            offset += skipped;
            continue;
        }
        offset += 1;
        let (length, consumed) = read_varint(&data[offset..])?;
        offset += consumed;
        let end = offset + length as usize;
        if end > data.len() {
            return None;
        }
        let text = std::str::from_utf8(&data[offset..end]).ok()?;
        return Some(text.to_owned());
    }
    None
}

fn skip_proto_field(data: &[u8]) -> Option<usize> {
    if data.is_empty() {
        return None;
    }
    let tag = data[0];
    let wire_type = tag & 0x07;
    let mut offset = 1usize;
    match wire_type {
        0 => {
            let (_, consumed) = read_varint(&data[offset..])?;
            offset += consumed;
        }
        1 => offset += 8,
        2 => {
            let (length, consumed) = read_varint(&data[offset..])?;
            offset += consumed + length as usize;
        }
        5 => offset += 4,
        _ => return None,
    }
    Some(offset)
}

fn read_varint(data: &[u8]) -> Option<(u64, usize)> {
    let mut result = 0u64;
    let mut shift = 0u32;
    for (index, byte) in data.iter().enumerate() {
        let value = (byte & 0x7f) as u64;
        result |= value << shift;
        if byte & 0x80 == 0 {
            return Some((result, index + 1));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
    None
}

fn detect_git_layout(workspace_root: &Path, repo_path: &Path) -> (bool, GitRepoLayout) {
    let store_path = repo_path.join("store");
    if workspace_root.join(".git").exists() {
        return (true, GitRepoLayout::Colocated);
    }
    if store_path.join("git").is_dir() {
        return (false, GitRepoLayout::Hidden);
    }
    let git_target = store_path.join("git_target");
    if git_target.is_file() {
        if let Ok(bytes) = fs::read(&git_target) {
            if let Some(relative) = path_from_os_bytes(&bytes) {
                let resolved = canonicalize_path(&store_path.join(relative));
                if resolved.exists() {
                    if resolved == canonicalize_path(&workspace_root.join(".git")) {
                        return (false, GitRepoLayout::Colocated);
                    }
                    return (false, GitRepoLayout::External);
                }
            }
        }
    }
    (false, GitRepoLayout::None)
}

fn read_sibling_workspaces(repo_path: &Path, workspace_root: &Path) -> Vec<SiblingWorkspace> {
    let index_path = repo_path.join("workspace_store").join("index");
    let bytes = match fs::read(&index_path) {
        Ok(value) => value,
        Err(_) => return Vec::new(),
    };
    let entries = parse_workspace_store_index(&bytes);
    entries
        .into_iter()
        .filter_map(|(name, relative)| {
            let absolute = canonicalize_path(&repo_path.join(&relative));
            if absolute == workspace_root {
                return None;
            }
            Some(SiblingWorkspace {
                name,
                path: absolute.to_string_lossy().into_owned(),
            })
        })
        .collect()
}

fn parse_workspace_store_index(data: &[u8]) -> Vec<(String, PathBuf)> {
    let mut entries = Vec::new();
    let mut offset = 0usize;
    while offset < data.len() {
        if data[offset] != 0x0a {
            let skipped = match skip_proto_field(&data[offset..]) {
                Some(value) => value,
                None => break,
            };
            offset += skipped;
            continue;
        }
        offset += 1;
        let (length, consumed) = match read_varint(&data[offset..]) {
            Some(value) => value,
            None => break,
        };
        offset += consumed;
        let end = offset + length as usize;
        if end > data.len() {
            break;
        }
        if let Some((name, relative)) = parse_workspace_entry(&data[offset..end]) {
            entries.push((name, relative));
        }
        offset = end;
    }
    entries
}

fn parse_workspace_entry(data: &[u8]) -> Option<(String, PathBuf)> {
    let name = read_proto_string_field(data, 1)?;
    let relative_bytes = read_proto_bytes_field(data, 2)?;
    let relative = path_from_os_bytes(&relative_bytes)?;
    Some((name, relative))
}

fn read_proto_bytes_field(data: &[u8], field_number: u32) -> Option<Vec<u8>> {
    let key = u8::try_from((field_number << 3) | 2).ok()?;
    let mut offset = 0usize;
    while offset < data.len() {
        if data[offset] != key {
            let skipped = skip_proto_field(&data[offset..])?;
            offset += skipped;
            continue;
        }
        offset += 1;
        let (length, consumed) = read_varint(&data[offset..])?;
        offset += consumed;
        let end = offset + length as usize;
        if end > data.len() {
            return None;
        }
        return Some(data[offset..end].to_vec());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_workspace_name_from_checkout() {
        let bytes = [
            0x12, 0x40, 0x6c, 0xb1, 0xe1, 0x4d, 0x08, 0x01, 0x46, 0x0d, 0x14, 0x2f, 0x5d, 0x1b,
            0xd2, 0x70, 0xcc, 0xc6, 0x51, 0x1c, 0xdb, 0xd4, 0xbe, 0xc3, 0x25, 0x3b, 0x00, 0xfe,
            0x9a, 0xb0, 0x29, 0x79, 0x64, 0x36, 0x5b, 0x86, 0x38, 0x39, 0x9f, 0x1e, 0x25, 0x2e,
            0xb4, 0x80, 0x18, 0x0a, 0x62, 0x13, 0xfb, 0xfb, 0xcc, 0x7e, 0x05, 0x40, 0x1c, 0x3d,
            0x4e, 0x9f, 0xce, 0x07, 0xd6, 0xc4, 0xa6, 0x49, 0x4a, 0xe3, 0x1a, 0x07, 0x64, 0x65,
            0x66, 0x61, 0x75, 0x6c, 0x74,
        ];
        assert_eq!(
            read_proto_string_field(&bytes, 3).as_deref(),
            Some("default")
        );
    }

    #[test]
    fn inspects_fixture_workspace_when_present() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        if !root.join(".jj").is_dir() {
            return;
        }
        let inspect = inspect_jj_workspace(&root).expect("fixture workspace");
        assert_eq!(inspect.commit_store, "git");
        assert_eq!(inspect.workspace_name, "default");
        assert_eq!(inspect.git_repo_layout, GitRepoLayout::Colocated);
        assert!(inspect.colocated);
    }

    #[test]
    fn parses_workspace_store_index() {
        let data = [
            0x0a, 0x11, 0x0a, 0x07, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x12, 0x06, 0x2e,
            0x2e, 0x2f, 0x2e, 0x2e, 0x2f,
        ];
        let entries = parse_workspace_store_index(&data);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, "default");
        assert_eq!(entries[0].1, PathBuf::from("../.."));
    }
}
