export type RepoEngine = "jj";

export type RepoLinkKind = "inline" | "pointer";

export type GitRepoLayout = "colocated" | "hidden" | "external" | "none";

export type SiblingWorkspace = {
  name: string;
  path: string;
};

export type RepoSummary = {
  path: string;
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
  opened_path?: string;
  repo_path_main?: string;
  sibling_workspaces?: SiblingWorkspace[];
};
