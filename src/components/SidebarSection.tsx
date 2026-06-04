import type { ComponentType, ReactNode, SVGProps } from "react";

import cn from "@/lib/cn";

import PaneHeader from "@/components/PaneHeader";

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

const headerRowClass = "flex items-center gap-1.5 px-1 py-0.5";
const sidebarIconClass = "size-3 shrink-0";
const emptyCopyClass = "m-0 py-0 pl-5 pr-1 text-xs text-fg-muted";

export default function SidebarSection({
  title,
  emptyLabel,
  icon: Icon,
  iconTone = "muted",
  children,
}: Props) {
  return (
    <section className="flex flex-col gap-0">
      <div className={headerRowClass}>
        {Icon ? (
          <Icon aria-hidden className={cn(sidebarIconClass, iconToneClass[iconTone])} />
        ) : null}
        <PaneHeader className="px-0 pt-0 pb-0 leading-none">{title}</PaneHeader>
      </div>
      {children == null ? (
        <p className={emptyCopyClass} role="status">
          {emptyLabel}
        </p>
      ) : (
        <ul className="m-0 flex list-none flex-col gap-px p-0">{children}</ul>
      )}
    </section>
  );
}
