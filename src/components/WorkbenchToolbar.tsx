export default function WorkbenchToolbar() {
  return (
    <header className="flex h-9 shrink-0 items-center gap-4 border-b border-border px-3 text-xs text-fg-muted">
      <span className="font-medium text-fg">No workspace</span>
      <span className="h-3 w-px bg-border" aria-hidden />
      <span>Working copy idle</span>
    </header>
  );
}
