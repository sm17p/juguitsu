import cn from "@/lib/cn";

type Props = {
  children: string;
  className?: string;
};

export default function PaneHeader({ children, className }: Props) {
  return (
    <h2
      className={cn(
        "shrink-0 px-1 py-0.5 text-[0.6875rem] leading-none font-semibold tracking-wide text-fg-muted uppercase",
        className,
      )}
    >
      {children}
    </h2>
  );
}
