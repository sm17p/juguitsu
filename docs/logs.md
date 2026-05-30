# Logs

## Locations

juguitsu writes logs under the Tauri app log directory for identifier `me.sm17p.juguitsu`.

| Platform | Directory                           |
| -------- | ----------------------------------- |
| macOS    | `~/Library/Logs/me.sm17p.juguitsu/` |
| Linux    | `~/.config/me.sm17p.juguitsu/logs/` |
| Windows  | `%APPDATA%\me.sm17p.juguitsu\logs\` |

## Files

| File           | Contents                                                                          |
| -------------- | --------------------------------------------------------------------------------- |
| `juguitsu.log` | Operational logs from Rust (`tracing`) and the webview (`@tauri-apps/plugin-log`) |
| `crashes.log`  | Rust panics and uncaught frontend errors (append-only)                            |

For Rust panic backtraces in `crashes.log`, run with `RUST_BACKTRACE=1`.

## Architecture

```text
Rust                          Frontend
────                          ────────
tracing::info!                @tauri-apps/plugin-log (info/warn/error)
       │                              │
       │ (tracing "log" feature)      │
       ▼                              ▼
              tauri-plugin-log
                     │
         ┌───────────┼───────────┐
         ▼           ▼           ▼
   juguitsu.log   stdout     webview console
   (release)      (dev)         (dev)

Panic / crash paths
───────────────────
std::panic ──► crash_log hook ──► crashes.log (sync append, jiff timestamp)
                    │
                    └──► color-eyre stderr report (dev)

window error / unhandledrejection ──► log_crash IPC ──► crashes.log
React ErrorBoundary ─────────────────► log_crash IPC ──► crashes.log
                     └──► @tauri-apps/plugin-log ──────► juguitsu.log
```

### Rust

| Piece                     | Role                                                                         |
| ------------------------- | ---------------------------------------------------------------------------- |
| `tracing` + `log` feature | App logging API; emits to the `log` facade when no tracing subscriber is set |
| `tauri-plugin-log`        | File/stdout/webview sink; timestamps via `jiff` in the formatter             |
| `color-eyre`              | Installed in `main.rs`; pretty panic reports to stderr in dev                |
| `crash_log`               | Resolves `app_log_dir`, chained panic hook, `log_crash` command              |

The panic hook writes `crashes.log` first, then delegates to color-eyre. Log subscribers may not flush on panic, so crash entries use direct file append.

### Frontend

| Piece               | Role                                                                                 |
| ------------------- | ------------------------------------------------------------------------------------ |
| `initCrashLogger()` | Registers global error handlers; attaches webview console in dev                     |
| `ErrorBoundary`     | Catches React render errors → `log_crash` + plugin `error()`                         |
| `log_crash`         | Tauri command that appends to `crashes.log` with source, message, and optional stack |
