import { invoke } from "@tauri-apps/api/core";
import { attachConsole, error } from "@tauri-apps/plugin-log";

function reportCrash(source: string, message: string, stack?: string) {
  void error(`[${source}] ${message}`);
  void invoke("log_crash", {
    source,
    message,
    stack: stack ?? null,
  });
}

export function initCrashLogger() {
  if (import.meta.env.DEV) {
    void attachConsole();
  }

  window.addEventListener("error", (event) => {
    const stack = event.error instanceof Error ? event.error.stack : undefined;
    reportCrash("window", event.message || "unknown error", stack);
  });

  window.addEventListener("unhandledrejection", (event) => {
    const { reason } = event;
    const message =
      reason instanceof Error
        ? reason.message
        : typeof reason === "string"
          ? reason
          : "unhandled rejection";
    const stack = reason instanceof Error ? reason.stack : undefined;
    reportCrash("unhandledrejection", message, stack);
  });
}
