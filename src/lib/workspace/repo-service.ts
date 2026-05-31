import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { Context, Data, Effect, Layer } from "effect";

import type { RepoSummary } from "./repo-summary";

export class RepoCommandError extends Data.TaggedError("RepoCommandError")<{
  message: string;
}> {}

const toRepoCommandError = (error: unknown) =>
  new RepoCommandError({
    message:
      typeof error === "string"
        ? error
        : error instanceof Error
          ? error.message
          : "repo command failed",
  });

const invokeCommand = <A>(command: string, args?: Record<string, unknown>) =>
  Effect.tryPromise({
    try: () => (args === undefined ? invoke<A>(command) : invoke<A>(command, args)),
    catch: toRepoCommandError,
  });

const pickFolder = Effect.tryPromise({
  try: () => open({ directory: true, multiple: false }),
  catch: toRepoCommandError,
});

export class RepoService extends Context.Tag("@juguitsu/RepoService")<
  RepoService,
  {
    readonly listRecents: Effect.Effect<readonly RepoSummary[], RepoCommandError>;
    readonly openAt: (path: string) => Effect.Effect<RepoSummary, RepoCommandError>;
    readonly pickAndOpen: Effect.Effect<RepoSummary | null, RepoCommandError>;
  }
>() {}

export const RepoServiceLive = Layer.succeed(RepoService, {
  listRecents: invokeCommand<RepoSummary[]>("list_recent_repos"),
  openAt: (path) => invokeCommand<RepoSummary>("open_repo_at", { path }),
  pickAndOpen: Effect.gen(function* () {
    const selected = yield* pickFolder;
    if (selected == null) return null;

    const path = typeof selected === "string" ? selected : selected[0];
    if (path == null) return null;

    return yield* invokeCommand<RepoSummary>("open_repo_at", { path });
  }),
});
