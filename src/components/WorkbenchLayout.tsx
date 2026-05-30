import { Splitter } from "@ark-ui/react/splitter";
import AppTabs from "./AppTabs";
import styles from "./workbench-layout.module.css";

const panels = [
  { id: "repos", minSize: 12 },
  { id: "tree", minSize: 15 },
  { id: "main", minSize: 30 },
] as const;

function ReposPane() {
  return (
    <div className={styles.pane}>
      <h2 className={styles.paneHeader}>Repos</h2>
      <div className={styles.paneBody}>
        <p className={styles.hint}>Recents coming soon.</p>
        <button type="button" className={styles.openAction}>
          Open…
        </button>
      </div>
    </div>
  );
}

function TreePane() {
  return (
    <div className={styles.pane}>
      <h2 className={styles.paneHeader}>Files</h2>
      <div className={styles.paneBody}>
        <p className={styles.hint}>Open a repo to browse the tree.</p>
      </div>
    </div>
  );
}

function ResizeHandle({ id }: { id: `${string}:${string}` }) {
  return (
    <Splitter.ResizeTrigger className={styles.resizeTrigger} id={id} aria-label="Resize pane">
      <Splitter.ResizeTriggerIndicator className={styles.resizeTriggerIndicator} />
    </Splitter.ResizeTrigger>
  );
}

export default function WorkbenchLayout() {
  return (
    <Splitter.Root
      className={styles.root}
      panels={[...panels]}
      defaultSize={[18, 22, 60]}
    >
      <Splitter.Panel className={styles.panel} id="repos">
        <ReposPane />
      </Splitter.Panel>
      <ResizeHandle id="repos:tree" />
      <Splitter.Panel className={styles.panel} id="tree">
        <TreePane />
      </Splitter.Panel>
      <ResizeHandle id="tree:main" />
      <Splitter.Panel className={styles.panel} id="main">
        <AppTabs />
      </Splitter.Panel>
    </Splitter.Root>
  );
}
