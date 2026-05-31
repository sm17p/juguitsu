import { Splitter } from "@ark-ui/react/splitter";
import { useEffect, useState } from "react";

import cn from "@/lib/cn";
import useMediaNarrow from "@/lib/use-media-narrow";

import AppTabs from "./AppTabs";
import WorkbenchToolbar from "./WorkbenchToolbar";

export default function WorkbenchLayout() {
  const narrow = useMediaNarrow();
  const [outerSize, setOuterSize] = useState<number[]>([18, 82]);
  const [innerSize, setInnerSize] = useState<number[]>([28, 72]);

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
                { id: "repos", minSize: 12, collapsible: true, collapsedSize: 0 },
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
              id="repos"
            >
              <nav aria-label="Repositories" className="flex h-full min-h-0 flex-col gap-2 p-2">
                <h2
                  className={cn(
                    "shrink-0 px-2 pt-2 text-[0.6875rem] font-semibold tracking-wider text-fg-muted uppercase",
                  )}
                >
                  Repos
                </h2>
                <div className="min-h-0 flex-1" />
                <button
                  type="button"
                  className={cn(
                    "flex h-11 min-h-11 shrink-0 cursor-pointer items-center justify-center rounded-md border border-border bg-bg-subtle px-3 text-xs font-medium text-fg hover:border-accent hover:text-accent-fg focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus-ring",
                  )}
                >
                  Open…
                </button>
              </nav>
            </Splitter.Panel>
            <Splitter.ResizeTrigger
              className={cn(
                "flex w-3 shrink-0 cursor-col-resize items-stretch justify-center border-0 bg-transparent p-0 focus-visible:outline-2 focus-visible:outline-offset-[-1px] focus-visible:outline-focus-ring",
              )}
              id="repos:workspace"
              aria-label="Resize repos pane"
            >
              <Splitter.ResizeTriggerIndicator
                className={cn(
                  "w-px shrink-0 bg-border transition-colors hover:bg-accent/40 focus-visible:bg-accent data-dragging:bg-accent",
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
            <WorkbenchToolbar />
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
                    <div className="flex h-full min-h-0 flex-col">
                      <h2
                        className={cn(
                          "shrink-0 px-2 pt-2 text-[0.6875rem] font-semibold tracking-wider text-fg-muted uppercase",
                        )}
                      >
                        Files
                      </h2>
                      <div className="min-h-0 flex-1" />
                    </div>
                  </Splitter.Panel>
                  <Splitter.ResizeTrigger
                    className={cn(
                      "flex w-3 shrink-0 cursor-col-resize items-stretch justify-center border-0 bg-transparent p-0 focus-visible:outline-2 focus-visible:outline-offset-[-1px] focus-visible:outline-focus-ring",
                    )}
                    id="tree:main"
                    aria-label="Resize file tree pane"
                  >
                    <Splitter.ResizeTriggerIndicator
                      className={cn(
                        "w-px shrink-0 bg-border transition-colors hover:bg-accent/40 focus-visible:bg-accent data-dragging:bg-accent",
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
