import {
  Result,
  useAtomMount,
  useAtomSet,
  useAtomSubscribe,
  useAtomValue,
} from "@effect-atom/atom-react";
import { useCallback } from "react";

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
  const recents = Result.isSuccess(recentsResult) ? recentsResult.value : [];

  const activeRepo = useAtomValue(activeRepoAtom);
  const openError = useAtomValue(openErrorAtom);

  const runPickAndOpen = useAtomSet(pickAndOpenAtom, { mode: "promiseExit" });
  const runOpenRecent = useAtomSet(openRecentAtom, { mode: "promiseExit" });

  const pickAndOpen = useCallback(() => runPickAndOpen(undefined), [runPickAndOpen]);

  const openRecent = useCallback((path: string) => runOpenRecent(path), [runOpenRecent]);

  return { recents, activeRepo, openError, pickAndOpen, openRecent };
}
