import IconFolderPlus from "~icons/tabler/folder-plus";

import cn from "@/lib/cn";
import openShortcutLabel from "@/lib/open-shortcut-label";

type Props = {
  className?: string;
  compact?: boolean;
  onOpen: () => void;
};

export default function OpenRepoButton({ className, compact = false, onOpen }: Props) {
  const shortcut = openShortcutLabel();

  return (
    <button
      type="button"
      aria-label={`Open repository (${shortcut})`}
      className={cn(
        "group flex shrink-0 cursor-pointer items-center gap-2 rounded-md text-xs font-medium text-fg transition-colors duration-150 hover:bg-accent-subtle hover:text-accent-fg focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring motion-reduce:transition-none",
        compact ? "h-8 min-h-8" : "h-11 min-h-11 px-3",
        className,
      )}
      onClick={onOpen}
    >
      <IconFolderPlus
        aria-hidden
        className="size-3.5 shrink-0 text-accent transition-colors duration-150 group-hover:text-accent-fg motion-reduce:transition-none"
      />
      <span>Open…</span>
      <span className="font-mono text-[0.625rem] text-fg-muted">{shortcut}</span>
    </button>
  );
}
