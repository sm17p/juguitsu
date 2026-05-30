---
name: sm17p-design
description: Design and review UI for juguitsu — a Tauri desktop local-first jj/git workbench. Use before changing layout, navigation, diff/file views, motion, keyboard flows, or visible React/Ark UI.
version: 0.2.0
user-invocable: true
argument-hint: "[task or area]"
---

# juguitsu Design Skill

Use this skill before touching visible UI in `juguitsu`.

## Read First

1. `AGENTS.md` for local-first architecture, perceived speed, and keyboard-first expectations.
2. `README.md` for stack and dev commands.
3. [jjui docs](https://idursun.github.io/jjui/) when designing history, diff, bookmark, or git-remote flows — match mental models, not the terminal chrome.

## Product Context

**juguitsu** is a desktop **local GitHub**: browse repos, history, and changes on disk with **jj** (primary) and git remotes, without a hosted account.

The interface should feel like a **fast native workbench** for power users: dense, keyboard-friendly, and instant on local data. It is not a marketing site, not a generic SaaS dashboard, and not a web clone of github.com purple chrome.

Wrong direction: centered hero, empty states with spinners, card grids, decorative gradients, or UI that blocks on `jj` before showing anything.

## Architecture

```
Tauri (Rust)  →  jj / git / filesystem
       ↓ invoke
React + Ark UI  →  shell, menus, tabs, dialogs
       ↓
@pierre/trees/react  →  file tree, git status, search
@pierre/diffs/react  →  diffs, review, compare
```

| Layer | Owns |
|-------|------|
| **Rust (`src-tauri/`)** | Run `jj`, parse output, cache repo metadata, open folders, errors |
| **React shell** | Layout, routing between views, command palette, repo picker |
| **Ark UI** | Tabs, menus, selects, dialogs, tooltips, switches — headless + project CSS |
| **Pierre Trees** | File explorer, drag/drop, git badges, virtualization |
| **Pierre Diffs** | Unified/split diff, line selection, annotations (review later) |

Do not reimplement tree or diff rendering in raw React when Pierre components apply.

## Shell Layout

Default target: **three-pane workbench** (desktop-first; collapse thoughtfully on narrow widths).

```text
┌──────────────┬─────────────────────────────────────────────┐
│ Repos        │  [bookmark ▾]  ·  status pills              │
│ (recents)    ├──────────────┬──────────────────────────────┤
│              │  File tree   │  Main: file · history · diff │
│  [Open…]     │  (Trees)     │  (Tabs + Pierre)             │
└──────────────┴──────────────┴──────────────────────────────┘
```

Primary nav tabs (Ark `Tabs` or sidebar):

- **Files** — tree + file viewer
- **History** — revision log (jj change ids, descriptions)
- **Changes** — working copy diff (daily driver)
- **Compare** — two revisions (later)

Use **bookmarks** in UI copy where jj uses bookmarks; avoid “branch” unless showing git-exported refs.

## Local-First UI Rules

From `AGENTS.md`, adapted for desktop + jj:

- **Disk and cache are the source of truth for the UI.** After opening a repo, show the last-known tree and log immediately; refresh in background.
- **No blocking spinners** on navigation. Prefer stale-while-revalidate: keep previous content visible while new `jj` results arrive.
- **Network is only for remotes.** Fetch/push are explicit actions (toolbar or git menu), not implicit on every view load.
- **Optimistic local edits** (describe, bookmark move): update UI when the Rust command succeeds; show inline error + rollback on failure — not a full-page error.
- **Keyboard-first.** Every primary action needs a shortcut or command-palette entry; document shortcuts in UI (jjui-style).
- **Command palette** (`⌘K` / `Ctrl+K`): search repos, files, revisions, and jj actions against **local** data first.

Do not port Linear-specific tactics (IndexedDB, service workers, modulepreload waterfalls) unless juguitsu gains a web target.

## Stack Rules

- **React 19 + Vite** for UI; **TypeScript** strict, no type assertions.
- **Ark UI** (`@ark-ui/react/*`) for interactive primitives; style with Tailwind utilities and Ark `data-*` variants.
- **Pierre** React entrypoints for trees and diffs; use `WorkerPoolContextProvider` / `Virtualizer` per Pierre docs when lists or diffs are large.
- **Tauri** for all jj/git/filesystem IO — do not shell out from the frontend.
- Prefer **verb-based** names for methods that do work; nouns for pure accessors.
- No comments in code; readable names instead.
- Match existing file layout: `src/components/`, `src/styles/`.
- Do not add a second component library (MUI, Chakra, shadcn) without an explicit decision.

## Pre-Build Protocol

Before changing UI, answer:

1. Which **view** (Files, History, Changes, Compare) and **pane** does this touch?
2. Is data **local** (instant) or **remote** (fetch/push) — and does the UI reflect that?
3. What is the **generic GitHub/web clone** version of this feature?
4. What is the **smallest** change that fits juguitsu’s workbench density and jj vocabulary?

Only write code after this is clear. **Plan and ask for review** before implementing non-trivial UI.

## Visual Direction

- **Neutral workbench chrome** — light/dark via Tailwind theme tokens (`src/index.css` `@theme`); system theme as default.
- **Density over whitespace** — readable at 13–14px body; mono for change ids, paths, hashes.
- **GitHub-familiar layout**, not GitHub brand colors — one accent, gray surfaces, clear borders.
- **Status over decoration** — git/jj badges (Trees), ahead/behind pills, dirty count; no hero illustrations.
- **One accent interaction** per surface — e.g. selected tab, focused row; avoid competing glows.

## Motion Rules

- Respect `prefers-reduced-motion`.
- Animate only **`transform`** and **`opacity`** for layout motion; use **color/background** transitions ≤150ms for hover/focus.
- Never animate `width`, `height`, `margin`, `padding`, `top`, `left`.
- Appear fast, dismiss slightly slower (~150ms fade-out).
- No idle looping animation in the main workbench.

## Accessibility Rules

- Every icon-only control needs an accessible name.
- Preserve visible `:focus-visible` styles (Ark-friendly).
- Do not rely on color alone for git status — use letters/badges (Trees git status).
- Tree and diff surfaces must be keyboard-operable (Pierre + Ark both document keyboard patterns).
- Desktop: support **Reduce motion** and sufficient contrast in both themes.

## jj / jjui Alignment

When adding VCS UI, prefer parity with **jjui** flows and underlying **jj** commands:

| User job | jj | juguitsu surface |
|----------|-----|------------------|
| Browse history | `jj log` | History tab |
| Commit detail | `jj show` | Revision panel |
| File diff | `jj diff` | Pierre Diffs |
| Working changes | `jj status`, `jj diff` | Changes tab |
| Bookmarks | `jj bookmark list` | Top bar selector |
| Remotes | `jj git fetch/push` | Git menu / toolbar |
| File history | revset `files(path)` | File context action |

See project notes / agent transcripts for the full jjui → juguitsu command map.

## Agent Context Rules

When changing product scope, nav model, stack choices, or design direction, update in the same change when relevant:

- `AGENTS.md` (architecture principles)
- `README.md` (stack one-liner)
- This skill if the rules above change

## Review Checklist

Before finalizing:

- Does it feel like a **local jj workbench**, not a website or Electron demo?
- Does navigation work **without waiting** on jj when cached data exists?
- Are **Ark** and **Pierre** used at the right layer (not duplicated)?
- Are shortcuts, focus, and git status accessible without color alone?
- Does **`pnpm build`** pass?
- For desktop flows, smoke-test with **`pnpm tauri dev`** when the change touches shell or Tauri commands.
