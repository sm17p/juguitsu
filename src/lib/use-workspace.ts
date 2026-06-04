import {
  Result,
  useAtomMount,
  useAtomSet,
  useAtomSubscribe,
  useAtomValue,
} from "@effect-atom/atom-react";

import {
  activeRepoAtom,
  hydrateActiveAtom,
  openErrorAtom,
  openRecentAtom,
  pickAndOpenAtom,
  recentsAtom,
} from "@/lib/workspace/workspace-atoms";

export type { RepoSummary } from "@/lib/workspace/repo-summary";

export default function useWorkspace() {
  useAtomMount(recentsAtom);

  const hydrateActive = useAtomSet(hydrateActiveAtom);

  useAtomSubscribe(
    recentsAtom,
    (result) => {
      if (Result.isSuccess(result)) {
        hydrateActive(undefined);
      }
    },
    { immediate: true },
  );

  const recentsResult = useAtomValue(recentsAtom);
  const activeRepo = useAtomValue(activeRepoAtom);
  const openError = useAtomValue(openErrorAtom);
  const runPickAndOpen = useAtomSet(pickAndOpenAtom, { mode: "promiseExit" });
  const runOpenRecent = useAtomSet(openRecentAtom, { mode: "promiseExit" });

  return {
    recents: Result.isSuccess(recentsResult) ? recentsResult.value : [],
    activeRepo,
    openError,
    pickAndOpen: () => void runPickAndOpen(undefined),
    openRecent: (workspaceRoot: string) => void runOpenRecent(workspaceRoot),
  };
}
