import { Accordion } from "@ark-ui/react/accordion";
import { useEffect, useState } from "react";
import IconChevronDown from "~icons/tabler/chevron-down";
import IconFolder from "~icons/tabler/folder";

import cn from "@/lib/cn";
import type { RepoSummary } from "@/lib/use-workspace";

import OpenRepoButton from "@/components/OpenRepoButton";
import PaneHeader from "@/components/PaneHeader";
import RepoPanel from "@/components/RepoPanel";

type Props = {
  recents: readonly RepoSummary[];
  activeWorkspaceRoot: string | null;
  onOpen: () => void;
  onSelectRecent: (workspaceRoot: string) => void;
};

const triggerClass =
  "focus-kbd flex h-8 min-h-8 w-full cursor-pointer items-center gap-1.5 rounded px-1 text-left text-xs transition-colors duration-150 motion-reduce:transition-none";

export default function RepoAccordion({
  recents,
  activeWorkspaceRoot,
  onOpen,
  onSelectRecent,
}: Props) {
  const [expanded, setExpanded] = useState<string[]>(() =>
    activeWorkspaceRoot != null ? [activeWorkspaceRoot] : [],
  );

  useEffect(() => {
    if (activeWorkspaceRoot == null) return;
    setExpanded((current) =>
      current.includes(activeWorkspaceRoot) ? current : [...current, activeWorkspaceRoot],
    );
  }, [activeWorkspaceRoot]);

  return (
    <section aria-label="Repositories" className="flex min-h-0 flex-1 flex-col gap-0">
      <div className="flex items-center px-1 py-0.5">
        <PaneHeader className="px-0 pt-0 pb-0 leading-none">Repos</PaneHeader>
      </div>
      <div className="min-h-0 flex-1 overflow-auto">
        {recents.length === 0 ? (
          <p className="m-0 px-1 py-0 text-xs text-fg-muted" role="status">
            No repositories
          </p>
        ) : (
          <Accordion.Root
            className="flex flex-col gap-px"
            collapsible
            multiple
            value={expanded}
            onValueChange={({ value }) => setExpanded(value)}
          >
            {recents.map((recent) => {
              const active = recent.workspace_root === activeWorkspaceRoot;
              return (
                <Accordion.Item
                  key={recent.workspace_root}
                  className="flex flex-col"
                  value={recent.workspace_root}
                >
                  <Accordion.ItemTrigger
                    className={cn(
                      triggerClass,
                      active
                        ? "bg-accent-subtle font-medium text-fg"
                        : "text-fg-muted hover:bg-bg-subtle hover:text-fg",
                    )}
                    onClick={() => onSelectRecent(recent.workspace_root)}
                  >
                    <IconFolder
                      aria-hidden
                      className={cn("size-3.5 shrink-0", active ? "text-accent" : "text-fg-muted")}
                    />
                    <span className="min-w-0 flex-1 truncate">{recent.name}</span>
                    <Accordion.ItemIndicator className="shrink-0 text-fg-muted transition-transform duration-150 data-[state=open]:rotate-180 motion-reduce:transition-none">
                      <IconChevronDown aria-hidden className="size-3.5" />
                    </Accordion.ItemIndicator>
                  </Accordion.ItemTrigger>
                  <Accordion.ItemContent className="overflow-hidden">
                    <RepoPanel />
                  </Accordion.ItemContent>
                </Accordion.Item>
              );
            })}
          </Accordion.Root>
        )}
      </div>
      <OpenRepoButton className="h-8 min-h-8 w-full shrink-0 justify-center px-1" onOpen={onOpen} />
    </section>
  );
}
