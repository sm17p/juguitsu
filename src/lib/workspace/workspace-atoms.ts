import { Atom, Result } from "@effect-atom/atom-react";
import { Effect } from "effect";

import { RepoService, RepoServiceLive } from "@/lib/workspace/repo-service";
import type { RepoSummary } from "@/lib/workspace/repo-summary";

const runtimeAtom = Atom.runtime(RepoServiceLive);

export const recentsAtom = runtimeAtom
  .atom(
    Effect.gen(function* () {
      const repos = yield* RepoService;
      return yield* repos.listRecents;
    }),
  )
  .pipe(Atom.withReactivity(["recents"]), Atom.keepAlive);

export const activeRepoAtom = Atom.make<RepoSummary | null>(null).pipe(Atom.keepAlive);

export const openErrorAtom = Atom.make<string | null>(null).pipe(Atom.keepAlive);

export const hydrateActiveAtom = Atom.fnSync((_: void, get) => {
  const recentsResult = get(recentsAtom);
  if (!Result.isSuccess(recentsResult)) return;
  const active = get(activeRepoAtom);
  if (active != null || recentsResult.value.length === 0) return;
  get.set(
    activeRepoAtom,
    recentsResult.value.reduce((latest, workspaceSummary) =>
      workspaceSummary.last_opened_at > latest.last_opened_at ? workspaceSummary : latest,
    ),
  );
});

export const pickAndOpenAtom = runtimeAtom.fn(
  Effect.fn(function* (_: void, get) {
    get.set(openErrorAtom, null);
    const repos = yield* RepoService;
    const opened = yield* repos.pickAndOpen.pipe(
      Effect.catchTag("RepoCommandError", (error) => {
        get.set(openErrorAtom, error.message);
        return Effect.fail(error);
      }),
    );
    if (opened == null) return null;
    get.set(activeRepoAtom, opened);
    get.set(recentsAtom, Result.success(yield* repos.listRecents));
    return opened;
  }),
  { reactivityKeys: ["recents"] },
);

export const openRecentAtom = runtimeAtom.fn(
  Effect.fn(function* (workspaceRoot: string, get) {
    get.set(openErrorAtom, null);
    const repos = yield* RepoService;
    const opened = yield* repos.openAt(workspaceRoot).pipe(
      Effect.catchTag("RepoCommandError", (error) => {
        get.set(openErrorAtom, error.message);
        return Effect.fail(error);
      }),
    );
    get.set(activeRepoAtom, opened);
    get.set(recentsAtom, Result.success(yield* repos.listRecents));
    return opened;
  }),
  { reactivityKeys: ["recents"] },
);
