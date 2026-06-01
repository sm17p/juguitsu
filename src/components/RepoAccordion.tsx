import { Accordion } from "@ark-ui/react/accordion";
import { useEffect, useState } from "react";
import IconChevronDown from "~icons/tabler/chevron-down";
import IconFolder from "~icons/tabler/folder";

import cn from "@/lib/cn";
import type { RepoSummary } from "@/lib/use-workspace";

import OpenRepoButton from "./OpenRepoButton";
import PaneHeader from "./PaneHeader";
import RepoPanel from "./RepoPanel";

type Props = {
  recents: readonly RepoSummary[];
  activePath: string | null;
  onOpen: () => void;
  onSelectRecent: (path: string) => void;
};

const triggerClass =
  "focus-kbd flex h-8 min-h-8 w-full cursor-pointer items-center gap-1.5 rounded px-1 text-left text-xs transition-colors duration-150 motion-reduce:transition-none";

export default function RepoAccordion({ recents, activePath, onOpen, onSelectRecent }: Props) {
  const [expanded, setExpanded] = useState<string[]>(() =>
    activePath != null ? [activePath] : [],
  );

  useEffect(() => {
    if (activePath == null) return;
    setExpanded((current) => (current.includes(activePath) ? current : [...current, activePath]));
  }, [activePath]);

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
              const active = recent.path === activePath;
              return (
                <Accordion.Item key={recent.path} className="flex flex-col" value={recent.path}>
                  <Accordion.ItemTrigger
                    className={cn(
                      triggerClass,
                      active
                        ? "bg-accent-subtle font-medium text-fg"
                        : "text-fg-muted hover:bg-bg-subtle hover:text-fg",
                    )}
                    onClick={() => onSelectRecent(recent.path)}
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
