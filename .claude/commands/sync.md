Create logical commits for all uncommitted work since the latest commit, then push to the remote.

1. **License headers** — run `pnpm headers` before anything else. It adds the SPDX + copyright header to any new source file (`.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`) and is idempotent on files that already have one. If it modified anything, those changes join whichever commit covers the files they touched.

2. **Survey** — `git status`, `git diff`, `git diff --staged`, and `git log --oneline -10` (for the commit-message style). Read every modified and untracked file enough to understand what conceptually changed — not just which files moved.

3. **Group** — split the work into logically-cohesive commits: one intent per commit. Don't mix unrelated changes. If a single file spans multiple intents, stage it in pieces (edit the file down to one intent, commit, restore the rest).

4. **Docs + file-top comment sweep** — for each group:

   - **Repo-wide `.md` sweep.** Open **every** `.md` file in the repo and decide whether any no longer match the change. This includes `reference/*.md`, `marketing/*.md`, `AGENTS.md`, `CLAUDE.md`, `README.md`, `.claude/rules/*.md`, and `.claude/commands/*.md`. `marketing/` is outcome-facing — only touch it when user-visible outcomes change, not for internal refactors. Skip `RELEASES.md` / `RELEASES_BETA.md` (owned by `/release` and `/beta-notes`) and the immutable surface (`LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`) unless the change directly affects them.
   - **File-top comment sweep.** For every modified `.vue` and `.rs` file in the group, re-read its leading comment block and verify it still matches the current implementation. Agents routinely forget this. Check:
     - **`.vue`** — the `<!-- ComponentName ... -->` block after the SPDX header. Verify **Purpose**, **Props**, and **Example** still describe the component as it now is (props renamed, emits added, behavior shifted, etc.).
     - **`.rs`** — the `//!` module doc comment at the top of the file. Verify it still describes the module's responsibility and any externally-visible behavior (channel names, command names, emitted events).
     - **`.ts`** — only if the file has a JSDoc/`/** */` header (e.g. `src/lib/log.ts`); follow the same rule.
   - Update anything that drifted. Stage the doc/comment edits alongside the commit whose change they describe, not in a separate commit.

5. **Typecheck before commit** — committed code must be type-clean. Before staging each group that touches TypeScript or Vue (`*.ts`, `*.tsx`, `*.vue`), run `pnpm typecheck` (= `vue-tsc --noEmit`) on the working tree and fix every error. No `// @ts-ignore` or `as any` to silence the checker — diagnose and resolve. Skip this step only for groups that touch no `.ts` / `.tsx` / `.vue` files (e.g. pure docs, pure Rust, pure CSS).

6. **Commit** — one commit per group. Follow the repo convention: imperative action-title subject line (≤70 chars), blank line, then a concise bulleted or prose body only when the _why_ isn't obvious from the subject. Include the agent `Co-Authored-By` trailer. Never skip hooks.

7. **Verify** — before pushing, run the relevant test suite on `HEAD` so red code never lands on `origin`. Run only the suites that match what changed across the commits in this sync (tests of pure-docs commits would just burn time):
   - Always: `pnpm headers:check` and `pnpm fmt:check`.
   - If any commit touched TypeScript or Vue (`src/**/*.ts`, `*.vue`): `pnpm typecheck` again on `HEAD` (belt-and-suspenders — catches drift introduced by hook reformatting between groups).
   - If any commit touched Rust (`src-tauri/**`): `pnpm test:unit`.
   - If any commit touched frontend (`src/**`), Rust (`src-tauri/**`), or e2e (`e2e/**`): `pnpm test:e2e`.
   - Skip the whole step if every commit in this sync is docs-only (changes confined to `reference/`, root `*.md`, `.claude/`, or comments).

   If anything fails, stop and surface the failures. Do **not** push. The user fixes forward (a follow-up commit) or asks for a revert — never bypass with `--no-verify` / `--force`.

8. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`.

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`.

If `$ARGUMENTS` is provided, treat it as extra context for how to group or describe the commits (e.g. "group all scan-related changes into one commit", "note the fix is for issue #42").
