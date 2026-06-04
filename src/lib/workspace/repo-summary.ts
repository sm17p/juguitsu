export type RepoEngine = "jj";

export type RepoLinkKind = "inline" | "pointer";

export type GitRepoLayout = "colocated" | "hidden" | "external" | "none";

export type SiblingWorkspace = {
  workspace_name: string;
  workspace_root: string;
};

export type RepoSummary = {
  workspace_root: string;
  name: string;
  last_opened_at: number;
  engine: RepoEngine;
  workspace_name: string;
  repo_path: string;
  repo_link_kind: RepoLinkKind;
  commit_store: string;
  op_store: string;
  op_heads: string;
  working_copy_store: string;
  colocated: boolean;
  git_repo_layout: GitRepoLayout;
  cwd_path?: string;
  pointer_repo_path?: string;
  sibling_workspaces?: SiblingWorkspace[];
};
