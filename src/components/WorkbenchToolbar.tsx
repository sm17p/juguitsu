import cn from "@/lib/cn";
import type { RepoSummary } from "@/lib/use-workspace";

import OpenRepoButton from "./OpenRepoButton";

type Props = {
  repo: RepoSummary | null;
  narrow: boolean;
  onOpen: () => void;
};

export default function WorkbenchToolbar({ repo, narrow, onOpen }: Props) {
  const label = repo?.name ?? "No workspace";

  return (
    <header className="flex h-11 min-h-11 shrink-0 items-center gap-2 border-b border-border px-3 text-xs">
      <span
        className={cn("min-w-0 truncate", repo ? "font-medium text-accent-fg" : "text-fg-muted")}
      >
        {label}
      </span>
      {narrow ? <OpenRepoButton className="ml-auto shrink-0" onOpen={onOpen} /> : null}
    </header>
  );
}
