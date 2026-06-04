import { Splitter } from "@ark-ui/react/splitter";
import { useEffect, useState } from "react";

import cn from "@/lib/cn";
import useMediaNarrow from "@/lib/use-media-narrow";
import useOpenRepoShortcut from "@/lib/use-open-repo-shortcut";
import useWorkspace from "@/lib/use-workspace";

import AppTabs from "@/components/AppTabs";
import PaneHeader from "@/components/PaneHeader";
import ReviewSidebar from "@/components/ReviewSidebar";
import WorkbenchToolbar from "@/components/WorkbenchToolbar";

export default function WorkbenchLayout() {
  const narrow = useMediaNarrow();
  const { recents, activeRepo, openError, pickAndOpen, openRecent } = useWorkspace();
  const [outerSize, setOuterSize] = useState<number[]>([18, 82]);
  const [innerSize, setInnerSize] = useState<number[]>([28, 72]);

  useOpenRepoShortcut(pickAndOpen);

  useEffect(() => {
    if (narrow) {
      setOuterSize([100]);
      setInnerSize([100]);
      return;
    }
    setOuterSize([18, 82]);
    setInnerSize([28, 72]);
  }, [narrow]);

  return (
    <>
      <h1 className="sr-only">Juguitsu</h1>
      <Splitter.Root
        className="flex min-h-0 w-full flex-1"
        panels={
          narrow
            ? [{ id: "workspace", minSize: 100 }]
            : [
                { id: "review", minSize: 12, collapsible: true, collapsedSize: 0 },
                { id: "workspace", minSize: 40 },
              ]
        }
        size={outerSize}
        onResize={({ size }) => setOuterSize(size)}
      >
        {!narrow ? (
          <>
            <Splitter.Panel
              className={cn("flex min-h-0 min-w-0 flex-col overflow-hidden bg-bg")}
              id="review"
            >
              <ReviewSidebar
                activeRepo={activeRepo}
                openError={openError}
                recents={recents}
                onOpen={pickAndOpen}
                onSelectRecent={openRecent}
              />
            </Splitter.Panel>
            <Splitter.ResizeTrigger
              className={cn(
                "focus-kbd flex w-2 shrink-0 cursor-col-resize items-stretch justify-center border-0 bg-transparent p-0 motion-reduce:transition-none",
              )}
              id="review:workspace"
              aria-label="Resize review pane"
            >
              <Splitter.ResizeTriggerIndicator
                className={cn(
                  "w-px shrink-0 bg-border transition-colors hover:bg-accent/40 focus-visible:bg-fg-muted/35 data-dragging:bg-accent motion-reduce:transition-none",
                )}
              />
            </Splitter.ResizeTrigger>
          </>
        ) : null}
        <Splitter.Panel
          className={cn("flex min-h-0 min-w-0 flex-col overflow-hidden bg-bg")}
          id="workspace"
        >
          <main className="flex h-full min-h-0 flex-col">
            <WorkbenchToolbar
              narrow={narrow}
              recents={recents}
              repo={activeRepo}
              onOpen={pickAndOpen}
              onSelectRecent={openRecent}
            />
            <Splitter.Root
              className="flex min-h-0 flex-1"
              panels={
                narrow
                  ? [{ id: "main", minSize: 100 }]
                  : [
                      { id: "tree", minSize: 16, collapsible: true, collapsedSize: 0 },
                      { id: "main", minSize: 28 },
                    ]
              }
              size={innerSize}
              onResize={({ size }) => setInnerSize(size)}
            >
              {!narrow ? (
                <>
                  <Splitter.Panel
                    className={cn("flex min-h-0 min-w-0 flex-col overflow-hidden bg-bg")}
                    id="tree"
                  >
                    <PaneHeader>Files</PaneHeader>
                  </Splitter.Panel>
                  <Splitter.ResizeTrigger
                    className={cn(
                      "focus-kbd flex w-2 shrink-0 cursor-col-resize items-stretch justify-center border-0 bg-transparent p-0 motion-reduce:transition-none",
                    )}
                    id="tree:main"
                    aria-label="Resize file tree pane"
                  >
                    <Splitter.ResizeTriggerIndicator
                      className={cn(
                        "w-px shrink-0 bg-border transition-colors hover:bg-accent/40 focus-visible:bg-fg-muted/35 data-dragging:bg-accent motion-reduce:transition-none",
                      )}
                    />
                  </Splitter.ResizeTrigger>
                </>
              ) : null}
              <Splitter.Panel
                className={cn("flex min-h-0 min-w-0 flex-col overflow-hidden bg-bg")}
                id="main"
              >
                <AppTabs />
              </Splitter.Panel>
            </Splitter.Root>
          </main>
        </Splitter.Panel>
      </Splitter.Root>
    </>
  );
}
