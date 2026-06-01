import IconFolderPlus from "~icons/tabler/folder-plus";

import cn from "@/lib/cn";
import openShortcutLabel from "@/lib/open-shortcut-label";

type Props = {
  className?: string;
  onOpen: () => void;
};

export default function OpenRepoButton({ className, onOpen }: Props) {
  const shortcut = openShortcutLabel();

  return (
    <button
      type="button"
      aria-label={`Open repository (${shortcut})`}
      className={cn(
        "focus-kbd group flex shrink-0 cursor-pointer items-center gap-1.5 rounded-md text-xs font-medium text-fg transition-colors duration-150 hover:bg-accent-subtle hover:text-accent-fg motion-reduce:transition-none",
        "h-8 min-h-8 px-1.5",
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
