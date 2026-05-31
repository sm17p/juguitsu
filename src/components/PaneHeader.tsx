import cn from "@/lib/cn";

type Props = {
  children: string;
  className?: string;
};

export default function PaneHeader({ children, className }: Props) {
  return (
    <h2
      className={cn(
        "shrink-0 px-1.5 pt-1.5 pb-0.5 text-[0.6875rem] font-semibold tracking-wider text-fg-muted uppercase",
        className,
      )}
    >
      {children}
    </h2>
  );
}
