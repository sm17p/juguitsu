import { Menu } from "@ark-ui/react/menu";
import IconChevronDown from "~icons/tabler/chevron-down";

import cn from "@/lib/cn";
import type { RepoSummary } from "@/lib/use-workspace";

type Props = {
  recents: readonly RepoSummary[];
  activePath: string | null;
  onSelectRecent: (path: string) => void;
};

const itemClass =
  "focus-kbd flex h-8 min-h-8 cursor-pointer items-center rounded px-1.5 text-left text-xs outline-none transition-colors duration-150 data-highlighted:bg-bg-subtle motion-reduce:transition-none";

export default function NarrowRecentsMenu({ recents, activePath, onSelectRecent }: Props) {
  return (
    <Menu.Root>
      <Menu.Trigger
        className={cn(
          "focus-kbd flex h-8 min-h-8 shrink-0 cursor-pointer items-center gap-1.5 rounded-md px-1.5 text-xs text-fg-muted transition-colors duration-150 hover:bg-bg-subtle hover:text-fg data-[state=open]:bg-bg-subtle data-[state=open]:text-fg motion-reduce:transition-none",
        )}
      >
        Repos
        <Menu.Indicator className="text-fg-muted">
          <IconChevronDown aria-hidden className="size-3.5" />
        </Menu.Indicator>
      </Menu.Trigger>
      <Menu.Positioner>
        <Menu.Content
          className={cn(
            "z-50 flex max-h-72 min-w-48 flex-col gap-px overflow-auto rounded-md border border-border bg-bg p-0.5 shadow-lg outline-none",
          )}
        >
          {recents.length === 0 ? (
            <p className="m-0 px-1.5 py-0.5 text-xs text-fg-muted" role="status">
              No repositories
            </p>
          ) : (
            recents.map((recent) => {
              const active = recent.path === activePath;
              return (
                <Menu.Item
                  key={recent.path}
                  className={cn(
                    itemClass,
                    active ? "bg-accent-subtle font-medium text-fg" : "text-fg-muted",
                  )}
                  value={recent.path}
                  onSelect={() => onSelectRecent(recent.path)}
                >
                  <span className="truncate">{recent.name}</span>
                </Menu.Item>
              );
            })
          )}
        </Menu.Content>
      </Menu.Positioner>
    </Menu.Root>
  );
}
