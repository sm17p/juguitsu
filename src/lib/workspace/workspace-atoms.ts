import { Atom, Result } from "@effect-atom/atom-react";
import { Effect } from "effect";

import { RepoService, RepoServiceLive } from "./repo-service";
import type { RepoSummary } from "./repo-summary";

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
  const mostRecent = recentsResult.value.reduce((best, repo) =>
    repo.last_opened_at > best.last_opened_at ? repo : best,
  );
  get.set(activeRepoAtom, mostRecent);
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
    const list = yield* repos.listRecents;
    get.set(recentsAtom, Result.success(list));
    return opened;
  }),
  { reactivityKeys: ["recents"] },
);

export const openRecentAtom = runtimeAtom.fn(
  Effect.fn(function* (path: string, get) {
    get.set(openErrorAtom, null);
    const repos = yield* RepoService;
    const opened = yield* repos.openAt(path).pipe(
      Effect.catchTag("RepoCommandError", (error) => {
        get.set(openErrorAtom, error.message);
        return Effect.fail(error);
      }),
    );
    get.set(activeRepoAtom, opened);
    const list = yield* repos.listRecents;
    get.set(recentsAtom, Result.success(list));
    return opened;
  }),
  { reactivityKeys: ["recents"] },
);
