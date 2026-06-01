import cn from "@/lib/cn";
import type { RepoSummary } from "@/lib/use-workspace";

import NarrowRecentsMenu from "@/components/NarrowRecentsMenu";
import OpenRepoButton from "@/components/OpenRepoButton";

type Props = {
  repo: RepoSummary | null;
  recents: readonly RepoSummary[];
  narrow: boolean;
  onOpen: () => void;
  onSelectRecent: (path: string) => void;
};

export default function WorkbenchToolbar({ repo, recents, narrow, onOpen, onSelectRecent }: Props) {
  const label = repo?.name ?? "No workspace";

  return (
    <header className="flex h-8 min-h-8 shrink-0 items-center gap-1 border-b border-border px-2 text-xs">
      {narrow ? (
        <NarrowRecentsMenu
          activePath={repo?.path ?? null}
          recents={recents}
          onSelectRecent={onSelectRecent}
        />
      ) : null}
      <span
        className={cn("min-w-0 truncate", repo ? "font-medium text-accent-fg" : "text-fg-muted")}
      >
        {label}
      </span>
      {narrow ? <OpenRepoButton className="ml-auto shrink-0" onOpen={onOpen} /> : null}
    </header>
  );
}
