Create logical commits for all uncommitted work since the latest commit, then push to the remote.

0. **Open the gate** — `mkdir -p .claude && touch .claude/.sync-active`. The pre-commit hook in `.claude/hooks/pre-commit-gate.sh` blocks `git commit` and `git push` unless this marker is present, so `/sync` must set it before its own git steps and clear it at the end (step 9). The marker is gitignored.

1. **License headers** — run `pnpm headers` before anything else. It adds the SPDX + copyright header to any new source file (`.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`) and is idempotent on files that already have one. If it modified anything, those changes join whichever commit covers the files they touched.

2. **Survey** — `git status`, `git diff`, `git diff --staged`, and `git log --oneline -10` (for the commit-message style). Read every modified and untracked file enough to understand what conceptually changed — not just which files moved.

3. **Group** — split the work into logically-cohesive commits: one intent per commit. Don't mix unrelated changes. If a single file spans multiple intents, stage it in pieces (edit the file down to one intent, commit, restore the rest).

4. **Docs + tests + file-top comment sweep** — for each group:
   - **Scoped doc sweep.** Walk the **Changed paths → re-verify** table in [`reference/index.md`](../reference/index.md). Update any listed doc that drifted with the change.
   - **Repo-wide `.md` skim.** Skim every other `.md` in the repo for eligibility (including `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/commands/`, `.cursor/rules/`, `.cursor/commands/`). Skip `RELEASES.md` / `RELEASES_BETA.md` (owned by `/release` and `/beta-notes`) and `LICENSE.md` / `CODE_OF_CONDUCT.md` / `SECURITY.md` unless the change directly affects them.
   - **`.coderabbit.yaml` sweep.** Re-read `path_instructions` and verify they still match current conventions. Update keys whose intent changed; do not edit `path_filters`, `auto_review`, `tools`, or chat / knowledge_base blocks unless the change directly touches them.
   - **Test sweep.** For every group that changes runtime behavior, verify tests still describe the new behavior. See [`reference/e2e.md`](../reference/e2e.md) for the selector/fixture contract. Check:
      - **`e2e/specs/*.spec.ts`** — `sel` selectors, fixture row names (`MyData`, `Projects`, …), counts, copy, flow steps.
      - **`e2e/helpers/*.ts`** — selector registry and helpers move with the UI.
      - **`src-tauri/tests/*.rs`** — command names, payload shapes, event names.
      - `/sync` skips `pnpm test:e2e` locally (CI runs it). Step 4 is the read sweep; surface the skip in the end-of-sync summary.
   - **File-top comment sweep.** For every modified `.vue` and `.rs` file in the group, re-read its leading comment block and verify it still matches the current implementation. Agents routinely forget this. Check:
      - **`.vue`** — the `<!-- ComponentName ... -->` block after the SPDX header. Verify **Purpose**, **Props**, and **Example** still describe the component as it now is (props renamed, emits added, behavior shifted, etc.).
      - **`.rs`** — the `//!` module doc comment at the top of the file. Verify it still describes the module's responsibility and any externally-visible behavior (channel names, command names, emitted events).
      - **`.ts`** — only if the file has a JSDoc/`/** */` header (e.g. `src/lib/log.ts`); follow the same rule.
   - Update anything that drifted. Stage the doc/test/comment edits alongside the commit whose change they describe, not in a separate commit.

5. **Typecheck before commit** — committed code must be type-clean. Before staging each group that touches TypeScript or Vue (`*.ts`, `*.tsx`, `*.vue`), run `pnpm typecheck` (= `vue-tsc --noEmit`) on the working tree and fix every error. No `// @ts-ignore` or `as any` to silence the checker — diagnose and resolve. Skip this step only for groups that touch no `.ts` / `.tsx` / `.vue` files (e.g. pure docs, pure Rust, pure CSS).

6. **Commit** — one commit per group. Follow the repo convention: imperative action-title subject line (≤70 chars), blank line, then a concise bulleted or prose body only when the _why_ isn't obvious from the subject. Include the agent `Co-Authored-By` trailer. Never skip hooks.

7. **Verify** — before pushing, run checks on `HEAD` that lint-staged does not cover. Lint-staged already ran oxlint/oxfmt/rustfmt on staged files at each commit; step 7 catches whole-tree drift and missing headers/types.

   Run only what matches the commits in this sync:
   - When not docs-only: `pnpm headers:check`, `pnpm fmt:check`, and `pnpm oxlint` (whole-tree belt-and-suspenders).
   - If any commit touched TypeScript or Vue (`src/**/*.ts`, `*.vue`): `pnpm typecheck` on `HEAD`.
   - If any commit touched Rust (`src-tauri/**`): `pnpm test:unit`.
   - **Do not run `pnpm test:e2e`.** CI runs it. Step 4 test sweep + [`reference/e2e.md`](../reference/e2e.md) cover stale expectations locally.
   - Skip the whole step if every commit is docs-only (changes confined to `reference/`, root `*.md`, `.claude/`, or comments).

   If anything fails, stop and surface the failures. Do **not** push. The user fixes forward (a follow-up commit) or asks for a revert — never bypass with `--no-verify` / `--force`.

8. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`.

9. **Close the gate** — `rm -f .claude/.sync-active`. Run this on success **and** on any early-exit (failed gate, user abort, push refusal). The marker exists only for the duration of the `/sync` run.

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`.

If `$ARGUMENTS` is provided, treat it as extra context for how to group or describe the commits (e.g. "group all scan-related changes into one commit", "note the fix is for issue #42").
