import { Tabs } from "@ark-ui/react/tabs";

import cn from "@/lib/cn";

const tabs = [
  { value: "files", label: "Files" },
  { value: "history", label: "History" },
  { value: "changes", label: "Changes" },
  { value: "compare", label: "Compare" },
] as const;

type TabValue = (typeof tabs)[number]["value"];

const emptyCopy: Record<TabValue, string> = {
  files: "No file selected",
  history: "No revision selected",
  changes: "Working copy is clean",
  compare: "Select two revisions to compare",
};

export default function AppTabs() {
  return (
    <Tabs.Root className="flex min-h-0 flex-1 flex-col text-fg" defaultValue="files">
      <Tabs.List className="flex shrink-0 gap-1 border-b border-border px-2">
        {tabs.map(({ value, label }) => (
          <Tabs.Trigger
            key={value}
            className={cn(
              "flex h-11 min-h-11 cursor-pointer items-center border-b-2 border-transparent px-3 text-xs font-medium text-fg-muted focus-visible:outline-2 focus-visible:outline-offset-[-2px] focus-visible:outline-focus-ring data-disabled:cursor-not-allowed data-disabled:opacity-50 data-selected:border-accent data-selected:text-fg",
            )}
            value={value}
          >
            {label}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
      {tabs.map(({ value }) => (
        <Tabs.Content
          key={value}
          className="min-h-0 flex-1 overflow-auto p-4 outline-none focus-visible:outline-2 focus-visible:outline-offset-[-2px] focus-visible:outline-focus-ring"
          value={value}
        >
          <p className={cn("m-0 text-sm leading-relaxed text-fg-muted")}>{emptyCopy[value]}</p>
        </Tabs.Content>
      ))}
    </Tabs.Root>
  );
}
