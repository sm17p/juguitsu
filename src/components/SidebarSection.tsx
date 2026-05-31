import type { ComponentType, ReactNode, SVGProps } from "react";

import cn from "@/lib/cn";

import PaneHeader from "./PaneHeader";

type IconProps = SVGProps<SVGSVGElement>;

type Props = {
  title: string;
  emptyLabel: string;
  icon?: ComponentType<IconProps>;
  iconTone?: "bookmark" | "query" | "muted";
  children?: ReactNode;
};

const iconToneClass = {
  bookmark: "text-bookmark",
  query: "text-query",
  muted: "text-fg-muted",
} as const;

export default function SidebarSection({
  title,
  emptyLabel,
  icon: Icon,
  iconTone = "muted",
  children,
}: Props) {
  const empty = children == null;

  return (
    <section className="flex flex-col gap-0.5">
      <div className="flex items-center gap-1 px-1.5 pb-0.5">
        {Icon ? (
          <Icon aria-hidden className={cn("size-3 shrink-0", iconToneClass[iconTone])} />
        ) : null}
        <PaneHeader className="px-0 pt-0 pb-0">{title}</PaneHeader>
      </div>
      {empty ? (
        <p className="m-0 px-1.5 py-1 text-xs text-fg-muted" role="status">
          {emptyLabel}
        </p>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-px p-0 pl-1">{children}</ul>
      )}
    </section>
  );
}

type RowProps = {
  label: string;
  selected?: boolean;
  onSelect: () => void;
};

export function SidebarRow({ label, selected = false, onSelect }: RowProps) {
  return (
    <li>
      <button
        type="button"
        aria-current={selected ? "true" : undefined}
        className={cn(
          "flex h-8 min-h-8 w-full cursor-pointer items-center rounded px-1.5 text-left text-xs transition-colors duration-150 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring motion-reduce:transition-none",
          selected
            ? "bg-accent-subtle font-medium text-fg"
            : "text-fg-muted hover:bg-bg-subtle hover:text-fg",
        )}
        onClick={onSelect}
      >
        <span className="truncate">{label}</span>
      </button>
    </li>
  );
}
