import { Tabs } from "@ark-ui/react/tabs";
import styles from "./app-tabs.module.css";

const tabs = [
  { value: "files", label: "Files", body: <FilesPanel /> },
  { value: "history", label: "History", body: <Placeholder text="Revision log coming soon." /> },
  { value: "changes", label: "Changes", body: <Placeholder text="Working copy diff coming soon." /> },
  { value: "compare", label: "Compare", body: <Placeholder text="Compare revisions coming soon." /> },
] as const;

function FilesPanel() {
  return (
    <>
      <h1 className="title">juguitsu</h1>
      <p className="tagline">Local jj workbench</p>
      <p className="hint">Open a repo to browse files.</p>
    </>
  );
}

function Placeholder({ text }: { text: string }) {
  return <p className="hint">{text}</p>;
}

export default function AppTabs() {
  return (
    <Tabs.Root className={styles.root} defaultValue="files">
      <Tabs.List className={styles.list}>
        {tabs.map(({ value, label }) => (
          <Tabs.Trigger key={value} className={styles.trigger} value={value}>
            {label}
          </Tabs.Trigger>
        ))}
        <Tabs.Indicator className={styles.indicator} />
      </Tabs.List>
      {tabs.map(({ value, body }) => (
        <Tabs.Content key={value} className={styles.content} value={value}>
          {body}
        </Tabs.Content>
      ))}
    </Tabs.Root>
  );
}
