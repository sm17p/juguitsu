import type { RepoSummary } from "@/lib/use-workspace";

import RepoAccordion from "./RepoAccordion";

type Props = {
  recents: readonly RepoSummary[];
  activeRepo: RepoSummary | null;
  openError: string | null;
  onOpen: () => void;
  onSelectRecent: (path: string) => void;
};

export default function ReviewSidebar({
  recents,
  activeRepo,
  openError,
  onOpen,
  onSelectRecent,
}: Props) {
  return (
    <nav aria-label="Review navigation" className="flex h-full min-h-0 flex-col px-1.5 py-1">
      <RepoAccordion
        activePath={activeRepo?.path ?? null}
        recents={recents}
        onOpen={onOpen}
        onSelectRecent={onSelectRecent}
      />
      {openError ? (
        <p
          className="m-0 shrink-0 rounded-md bg-error-subtle px-1.5 py-1 text-[0.6875rem] text-fg-error"
          role="alert"
        >
          {openError}
        </p>
      ) : null}
    </nav>
  );
}
