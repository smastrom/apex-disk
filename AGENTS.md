# AGENTS.md

## Project

ApexDisk — macOS-only Tauri 2 desktop app (Rust backend + Vue 3 frontend) for disk usage analysis and cleanup. Ships as a universal binary (Intel + Apple Silicon). Minimum macOS 10.15.

## Stack

| Layer                      | Tech                                                                                                       |
| -------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Backend                    | Rust, Tauri 2, objc2 (Foundation/AppKit bindings)                                                          |
| Frontend                   | Vue 3 (`<script setup lang="ts">`), TypeScript, Vite                                                       |
| Styling                    | Scoped CSS with CSS nesting, lightningcss (Safari 13 target)                                               |
| Testing                    | Rust: `cargo test` (src-tauri/tests/), E2E: WebdriverIO (e2e/)                                             |
| Formatting (TS/Vue/JS/CSS) | **Oxfmt** (Prettier-style formatter); **Oxlint** `--fix` adjunct for import order + statement padding only |
| Linting                    | Core language tooling only — **no ESLint**; see below                                                      |
| Package manager            | pnpm                                                                                                       |

### Linting (no general-purpose JS linter)

We do **not** use ESLint, Oxlint categories, or any other style/correctness linter for TS/Vue/JS. The only linters are **core language checkers**:

| Language         | Tool                                       | Command          |
| ---------------- | ------------------------------------------ | ---------------- |
| TypeScript / Vue | `vue-tsc`                                  | `pnpm typecheck` |
| Rust             | `rustc` (via `cargo test` / `cargo check`) | `pnpm test:unit` |

Oxlint is **not** a linter in this repo — only a formatter adjunct (import order + statement padding). Do not add ESLint or enable Oxlint rule categories.

### TS / Vue / JS formatting pipeline

Do not hand-sort imports or hand-place statement blank lines. Two tools, fixed roles:

1. **Oxfmt** (`.oxfmtrc.jsonc`) — general formatting (indentation, quotes, wrapping, CSS, JSON, etc.). Same role as Prettier.
2. **Oxlint** (`.oxlintrc.json`, `oxlint-plugins.mjs`) — formatter adjunct only. Categories are off; JS plugin enables just:
   - `stylistic/sort-imports`
   - `stylistic/padding-line-between-statements`

On staged `.js` / `.ts` / `.vue` and on editor save: **oxlint `--fix` first**, then **oxfmt**. Details: `reference/code-style.md`.

## Agent-facing reference (`reference/`)

`reference/` holds deep specs that agents read on demand. The
`reference-loader` rule in `.claude/rules/` maps operations to the right
file; the `pre-commit-protocol` rule guarantees a docs sweep before any
commit.

| File                          | Covers                                                                                      |
| ----------------------------- | ------------------------------------------------------------------------------------------- |
| `reference/architecture.md`   | Frontend/backend split, what each side owns, boundary conventions, build/testing.           |
| `reference/scanning.md`       | Scan + trash flow, `FolderInfo`, progress events, cancellation, selection model.            |
| `reference/tauri-commands.md` | IPC channels, command surface, `lib.rs` registration, settings store, locale + menu.        |
| `reference/translations.md`   | Per-component YAML, 10 languages, `useTranslations()`, CJK folding rules.                   |
| `reference/themes.md`         | CSS variables, `data-theme` switching, 9 themes, adding a new theme.                        |
| `reference/code-style.md`     | Oxfmt + Oxlint adjunct, Vue/CSS/TS conventions, file naming, license headers, comments.     |
| `reference/testing.md`        | Suites, when to run, Rust integration patterns, E2E + `e2e` cargo feature, what not to add. |
| `reference/compatibility.md`  | macOS / Safari / architecture targets, progressive enhancement matrix, oxfmt fallbacks.     |
| `reference/logging.md`        | `[apex:…]` diagnostic scheme, Vue categories, Rust trace channels, `APEX_DISK_DEBUG`.       |
| `reference/releases.md`       | How to cut stable and Beta builds, version fields, changelog conventions, workflows.        |
| `reference/updates.md`        | In-app updater (auto/manual), endpoint, signing, dialogs.                                   |
| `reference/voice.md`          | Tone and prose rules for user-facing docs (README, RELEASES, marketing, comments, copy).    |

Root-level `RELEASES.md`, `RELEASES_BETA.md`, `LICENSE.md`,
`CODE_OF_CONDUCT.md`, `SECURITY.md`, `README.md` stay at the repo root —
they are user-/CI-facing, not agent instructions.

## Outcome-facing content (`marketing/`)

`marketing/` holds content describing the product to users — FAQs, feature
descriptions, marketing copy. Unlike `reference/` (how the code works), this
is about what the user gets. Agents update it only when outcomes change
(new feature shipped, behavior reframed, FAQ becomes stale); they do not
edit it for implementation reasons. The `/sync` sweep still covers it.

## Slash commands (`.claude/commands/`)

| Command                | Purpose                                                                                         |
| ---------------------- | ----------------------------------------------------------------------------------------------- |
| `/sync`                | Group uncommitted work into logical commits, sweep `.md` + `.coderabbit.yaml`, run tests, push. |
| `/force-sync`          | Reconcile `.md` + `.coderabbit.yaml` against commits that bypassed `/sync`, then commit drift.  |
| `/compatibility-check` | Full compatibility verification against macOS 10.15 / Safari 13 / MSRV.                         |
| `/release`             | Prepare a stable release — auto path: bump versions, generate notes from git log.               |
| `/release-from-notes`  | Prepare a stable release — curated path: verify hand-written notes exist, bump only.            |
| `/beta-notes`          | Add a dated section to `RELEASES_BETA.md` for the Beta workflow's pre-release body.             |

## Testing

Full suite matrix, commands, and conventions live in `reference/testing.md`. Key always-on rules:

1. `/sync` and `/force-sync` run the relevant suites before pushing. Never push red code, never bypass with `--no-verify` / `--force`. If a suite fails, stop and surface — fix forward in a follow-up commit.
2. Tests use temp dirs, never the real user home.
3. Do not add tests unless asked.

The protocol is enforced from the repo by a `PreToolUse` hook in
`.claude/hooks/pre-commit-gate.sh` (wired via `.claude/settings.json`),
which refuses agent-initiated `git commit` / `git push` outside of `/sync`
or `/force-sync`. Details in `.claude/rules/pre-commit-protocol.md`.

## `.claude/` layout

| Path                          | Status                | Purpose                                                |
| ----------------------------- | --------------------- | ------------------------------------------------------ |
| `.claude/commands/*.md`       | committed             | Slash commands (`/sync`, `/release`, …).               |
| `.claude/rules/*.md`          | committed             | Always-loaded routing + protocol rules.                |
| `.claude/hooks/*.sh`          | committed             | Hook scripts referenced by `settings.json`.            |
| `.claude/settings.json`       | committed             | Shared hooks + repo-relevant permissions.              |
| `.claude/settings.local.json` | gitignored            | Personal overrides (paths, one-off allowlist entries). |
| `.claude/.sync-active`        | gitignored, ephemeral | Marker created by `/sync` to open the pre-commit gate. |

## Key directories

```
src/
  components/        # Vue components (ui/ for presentational)
  assets/css/        # theme.css, global.css, reset.css, classes.css, animations.css, rtl.css
  assets/translations/  # Per-component translation files + index.ts
  lib/               # Composables (use-*), utilities, constants
  stores/            # App settings store
  types/             # Shared TypeScript types

src-tauri/
  src/               # Rust backend modules
  tests/             # Rust integration tests
  tauri.conf.json    # Tauri config (bundle targets, min macOS version)

reference/           # Agent-facing reference (see table above)
.claude/
  rules/             # Always-loaded routing + protocol rules
  commands/          # Slash commands
```

## Voice for user-facing docs

Canonical example: `[marketing/faq.md](marketing/faq.md)`. Every user-facing
surface (`README.md`, `RELEASES.md`, `RELEASES_BETA.md`, anything under
`marketing/`, in-app strings, release-note bullets, error copy) should sound
like that file. Full guide and worked examples in
`[reference/voice.md](reference/voice.md)`.

Quick rules:

1. **Second person** ("you", "your Mac"), never "the user". Applies to
   `README.md`, `marketing/`, and in-app strings. Does **not** apply to code
   comments (they address the next maintainer) or `RELEASES*.md` (technical
   changelogs use past-tense action verbs: "Fixed…", "Added…", "Improved…").
2. **Plain English, no marketing jargon.** Avoid "leverage", "unleash",
   "powerful", "seamless", "blazing-fast", "simply", "just".
3. **Honest. State limits plainly.** No overselling.
4. **Short paragraphs.** One to three sentences. Contractions are fine.
5. **Action-first verbs in instructions.** "Download…", "Drag…", "Grant…".

## Prose style: em and en dashes

**Em dashes (`—`) are allowed as label separators**, where the dash sits
between a short label and its description. This applies broadly: bullets,
list items, table headings, CSS / Rust / TS section-header comments,
file-title comments, complexity annotations — any `[label] — [description]`
construct. They are **never** used as parenthetical interrupts in running
prose.

Allowed:

- `**Fast** — scans hundreds of thousands of files in seconds.`
- `**New Features** — user-visible additions.`
- `/* Theme: macOS Dark — Apple's system dark appearance */`
- `/// Silent update check — returns the available version string or null.`

Not allowed (interrupt in a flowing sentence):

- ~~"ApexDisk is built with Rust — a fast systems language — and runs natively."~~
  → "ApexDisk is built with Rust, a fast systems language, and runs natively."
- ~~"Scanning is fast — even on huge folders."~~
  → "Scanning is fast, even on huge folders."
- ~~"We only ever replace the whole array — never mutate in place."~~
  → "We only ever replace the whole array, never mutate in place."

The test: if the words around the dash form a flowing sentence and the dash
is interrupting it to add a side note, it's an interrupt and must go.

**Log strings:** even when they look like `[category] — [data]`, remove the
dash. The existing convention is `"Scan: complete 5 folders"`, not
`"Scan: complete — 5 folders"`.

**En dashes (`–`) are not used anywhere in this project.** Use an em dash (when
the label-separator rule allows it) or a hyphen.

**Hyphens (`-`)** in compound words (`agent-facing`, `user-visible`,
`open-source`) are normal and unrelated.

**Scope:** the prose rules apply to every file outside `.claude/` and
`reference/`: `README.md`, `RELEASES.md`, `RELEASES_BETA.md`, `SECURITY.md`,
`CODE_OF_CONDUCT.md`, `LICENSE.md`, `marketing/*.md`, code comments (`.ts`,
`.tsx`, `.vue`, `.rs`, `.sh`, CSS), commit messages, PR descriptions, and any
user-visible string. Files under `.claude/` and `reference/` are
agent-facing and exempt.

## What not to do

- Do not install npm packages unless asked.
- Do not create tests or docs unless asked.
- Do not add platform-specific code for Windows/Linux.
- Do not propose launching dev servers (it's always running).
- Do not use provide/inject for settings.
- Do not use Options API or plain `<script>`.
- Do not add narration comments that only restate the next line (for example `// Load system info` before `await loadSystemInfo()`).
- Do not use em dashes as parenthetical interrupts in user-facing prose, and
  do not use en dashes anywhere (see Prose style above).
