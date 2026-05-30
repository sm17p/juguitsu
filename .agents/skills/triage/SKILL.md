---
name: triage
description: Triages GitHub issues for sm17p.me with code inspection, browser accessibility checks when UI is involved, and a posted issue comment summarizing verdict, evidence, and planned fix. Use when the user shares an issue URL, asks to triage a bug, or wants investigation before implementation.
version: 0.1.0
user-invocable: true
argument-hint: "[issue URL or number]"
---

# Issue Triage

Use this skill when a GitHub issue needs investigation before code changes. The deliverable is a **posted triage comment on the issue** plus a short plan the user can approve.

## Read first

1. `AGENTS.md` for commands, project layout, and safe edit boundaries.
2. The issue body, labels, and linked files or line hints.
3. `DESIGN.md` or `sm17p-design` when the issue touches visible UI.

## Workflow

```
Issue → Read code → Reproduce / inspect (browser if UI) → Verdict → Post triage comment → Plan fix
```

### 1. Load the issue

```bash
gh issue view <number> --repo sm17p/sm17p.me --json title,body,labels,comments
```

Extract: category, location, impact, suggested approach, severity label if any.

### 2. Inspect the code

- Open files named in the issue. Read surrounding markup, ARIA, and state handling.
- Search for related patterns (`aria-*`, `hidden`, popover/disclosure usage) in `apps/www`.
- Decide whether behavior is **app-authored** (missing attributes, wrong DOM) or **framework/library** (only if the framework strips or blocks correct markup).

Default: assume **app-level** until proven otherwise.

### 3. Verify in the browser (UI / a11y issues)

When the issue involves visible UI or assistive technology:

1. Run `moon run www:dev` (or confirm dev server on `http://localhost:4321/`).
2. Use browser snapshot and CDP where helpful:
   - DOM: `aria-*`, `id`, `hidden`, `aria-hidden`
   - AX tree: `expanded`, `controls`, whether collapsed content is still exposed
3. Toggle the control (click, keyboard) and re-check collapsed vs expanded.

Record **before** state in notes. Do not claim a fix works until checked after patch.

### 4. Write the verdict

Pick one primary label:

| Verdict | When |
|---------|------|
| **Bug / spec gap** | Spec or WCAG expectation not met in our markup |
| **Enhancement** | Works but could be better |
| **Won't fix / by design** | Intentional; document why |
| **Not reproducible** | Cannot confirm; say what was tried |
| **Upstream / dependency** | Library or browser bug with evidence |

For a11y: separate what the **issue asks for** (e.g. `aria-controls`) from **follow-ups** (e.g. `aria-hidden` when only `scale(0)` hides content).

### 5. Post the triage comment

Post only after investigation. Use `gh issue comment`:

```bash
gh issue comment <number> --repo sm17p/sm17p.me --body "$(cat <<'EOF'
## Triage

**Verdict:** <one line>

### Evidence
- <DOM / AX / code finding>
- <repro steps or "not reproducible" detail>

### Root cause
<One short paragraph. Name the file and what is missing or wrong.>

### Planned fix
1. <concrete step>
2. <verification step>

### Follow-up (optional)
- <out of scope for the issue, if any>
EOF
)"
```

Keep the comment scannable: tables for before/after checks are fine; avoid long prose.

### 6. Plan and implement

- Wait for user approval unless they asked to implement in the same turn.
- Scope the PR to the issue; do not bundle unrelated lockfile or dependency churn.
- Use `jj` for status (`jj status`, `jj diff`). Commit with `Fixes #<n>` when appropriate.
- For PR writeups and review comments, use the `pr` and `pr-range-comments` skills.

## sm17p.me defaults

| Area | Path / command |
|------|----------------|
| Site UI | `apps/www` (Astro + Svelte) |
| Geo API | `apps/g1` |
| Dev server | `moon run www:dev` → `http://localhost:4321/` |
| Design rules | `DESIGN.md`, `sm17p-design` skill |

## Anti-patterns

- Calling a missing `aria-*` attribute a framework bug without checking the component source.
- Skipping browser verification for footer, header, menu, or credits interactions.
- Posting triage with no evidence or without reading the cited file.
- Expanding scope in the same PR (e.g. lockfile churn) unless the user asks.

## Quick checklist

- [ ] Issue read and location confirmed in repo
- [ ] Code inspected; verdict chosen
- [ ] Browser evidence collected (if UI/a11y)
- [ ] Triage comment posted on GitHub
- [ ] Plan shared; implementation scoped to issue
