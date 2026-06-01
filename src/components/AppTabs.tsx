import { Tabs } from "@ark-ui/react/tabs";

import cn from "@/lib/cn";

const tabs = [
  { value: "files", label: "Files" },
  { value: "history", label: "History" },
  { value: "changes", label: "Changes" },
  { value: "compare", label: "Compare" },
] as const;

type TabValue = (typeof tabs)[number]["value"];

const emptyCopy: Record<TabValue, { text: string; tone?: "success" }> = {
  files: { text: "No file selected" },
  history: { text: "No revision selected" },
  changes: { text: "Clean working copy", tone: "success" },
  compare: { text: "Pick two revisions" },
};

export default function AppTabs() {
  return (
    <Tabs.Root className="flex min-h-0 flex-1 flex-col text-fg" defaultValue="files">
      <Tabs.List className="flex shrink-0 border-b border-border px-1">
        {tabs.map(({ value, label }) => (
          <Tabs.Trigger
            key={value}
            className={cn(
              "focus-kbd-tab flex h-8 min-h-8 cursor-pointer items-center border-b-2 border-transparent px-2 text-xs font-medium text-fg-muted transition-colors duration-150 hover:text-fg data-disabled:cursor-not-allowed data-disabled:opacity-50 data-selected:bg-accent-subtle data-selected:text-accent-fg motion-reduce:transition-none",
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
          className="focus-kbd-pane min-h-0 flex-1 overflow-auto p-1.5 outline-none"
          value={value}
        >
          <p
            className={cn(
              "m-0 text-xs",
              emptyCopy[value].tone === "success" ? "text-success" : "text-fg-muted",
            )}
            role="status"
          >
            {emptyCopy[value].text}
          </p>
        </Tabs.Content>
      ))}
    </Tabs.Root>
  );
}
