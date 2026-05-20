Create logical commits for all uncommitted work since the latest commit, then push to the remote.

0. **Open the gate** — `mkdir -p .claude && touch .claude/.sync-active`. The pre-commit hook in `.claude/hooks/pre-commit-gate.sh` blocks `git commit` and `git push` unless this marker is present, so `/sync` must set it before its own git steps and clear it at the end (step 9). The marker is gitignored.

1. **License headers** — run `pnpm headers` before anything else. It adds the SPDX + copyright header to any new source file (`.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`) and is idempotent on files that already have one. If it modified anything, those changes join whichever commit covers the files they touched.

2. **Survey** — `git status`, `git diff`, `git diff --staged`, and `git log --oneline -10` (for the commit-message style). Read every modified and untracked file enough to understand what conceptually changed — not just which files moved.

3. **Group** — split the work into logically-cohesive commits: one intent per commit. Don't mix unrelated changes. If a single file spans multiple intents, stage it in pieces (edit the file down to one intent, commit, restore the rest).

4. **Docs + tests + file-top comment sweep** — for each group:
   - **Repo-wide `.md` sweep.** Open **every** `.md` file in the repo and decide whether any no longer match the change. This includes `reference/*.md`, `marketing/*.md`, `AGENTS.md`, `CLAUDE.md`, `README.md`, `.claude/rules/*.md`, and `.claude/commands/*.md`. `marketing/` is outcome-facing — only touch it when user-visible outcomes change, not for internal refactors. Skip `RELEASES.md` / `RELEASES_BETA.md` (owned by `/release` and `/beta-notes`) and the immutable surface (`LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`) unless the change directly affects them.
   - **`.coderabbit.yaml` sweep.** Re-read `.coderabbit.yaml`'s `path_instructions` and verify they still match current conventions (Composition API rule, scoped styles + design tokens, tauri-plugin-store schema, translation key coverage, e2e fixture style, capability surface, workflow expectations). CodeRabbit auto-reviews every PR against these, so drift here turns into noisy or incorrect review comments. Update keys whose intent changed; do not edit `path_filters`, `auto_review`, `tools`, or the chat / knowledge_base blocks unless the change directly touches them.
   - **Test sweep.** For every group that changes runtime behavior (Vue, Rust, scanning, trash flow, settings, IPC, translations, themes, fixture shape), open the test files that exercise that surface and verify their setup, assertions, and helper calls still describe the new behavior. Agents routinely change a feature and forget the tests. Check:
      - **`e2e/specs/*.spec.ts`** — selectors (`data-testid`), fixture row names (e.g. `MyData`, `Projects`), expected counts, copy strings, and flow steps (Review → trash list → confirm). If the UI was renamed, restructured, or had testids changed, the spec is stale.
      - **`e2e/helpers/*.ts` and anything under `e2e/fixtures/`** — helpers encode the current selector contract; fixtures encode the current scan shape. Both must move with the code.
      - **`src-tauri/tests/*.rs`** — Rust integration tests around `FolderInfo`, scan walker, trash, settings store. Verify command names, payload shapes, and emitted-event names still match.
      - Because `/sync` skips `pnpm test:e2e` (it rebuilds the debug binary and is too slow), this sweep is the only chance to catch stale e2e expectations before CI does. If a test edit is non-obvious, surface it to the user instead of guessing.
   - **File-top comment sweep.** For every modified `.vue` and `.rs` file in the group, re-read its leading comment block and verify it still matches the current implementation. Agents routinely forget this. Check:
      - **`.vue`** — the `<!-- ComponentName ... -->` block after the SPDX header. Verify **Purpose**, **Props**, and **Example** still describe the component as it now is (props renamed, emits added, behavior shifted, etc.).
      - **`.rs`** — the `//!` module doc comment at the top of the file. Verify it still describes the module's responsibility and any externally-visible behavior (channel names, command names, emitted events).
      - **`.ts`** — only if the file has a JSDoc/`/** */` header (e.g. `src/lib/log.ts`); follow the same rule.
   - Update anything that drifted. Stage the doc/test/comment edits alongside the commit whose change they describe, not in a separate commit.

5. **Typecheck before commit** — committed code must be type-clean. Before staging each group that touches TypeScript or Vue (`*.ts`, `*.tsx`, `*.vue`), run `pnpm typecheck` (= `vue-tsc --noEmit`) on the working tree and fix every error. No `// @ts-ignore` or `as any` to silence the checker — diagnose and resolve. Skip this step only for groups that touch no `.ts` / `.tsx` / `.vue` files (e.g. pure docs, pure Rust, pure CSS).

6. **Commit** — one commit per group. Follow the repo convention: imperative action-title subject line (≤70 chars), blank line, then a concise bulleted or prose body only when the _why_ isn't obvious from the subject. Include the agent `Co-Authored-By` trailer. Never skip hooks.

7. **Verify** — before pushing, run the relevant test suite on `HEAD` so red code never lands on `origin`. Run only the suites that match what changed across the commits in this sync (tests of pure-docs commits would just burn time):
   - Always: `pnpm headers:check`, `pnpm fmt:check`, and `pnpm oxlint`.
   - If any commit touched TypeScript or Vue (`src/**/*.ts`, `*.vue`): `pnpm typecheck` again on `HEAD` (belt-and-suspenders — catches drift introduced by hook reformatting between groups).
   - If any commit touched Rust (`src-tauri/**`): `pnpm test:unit`.
   - **Do not run `pnpm test:e2e`.** It rebuilds the debug Tauri binary and takes minutes; CI runs it on every push. The step-4 test sweep is what catches stale e2e expectations locally — surface the skip in the end-of-sync summary, and offer to run e2e on demand (e.g. before a release).
   - Skip the whole step if every commit in this sync is docs-only (changes confined to `reference/`, root `*.md`, `.claude/`, or comments).

   If anything fails, stop and surface the failures. Do **not** push. The user fixes forward (a follow-up commit) or asks for a revert — never bypass with `--no-verify` / `--force`.

8. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`.

9. **Close the gate** — `rm -f .claude/.sync-active`. Run this on success **and** on any early-exit (failed gate, user abort, push refusal). The marker exists only for the duration of the `/sync` run.

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`.

If `$ARGUMENTS` is provided, treat it as extra context for how to group or describe the commits (e.g. "group all scan-related changes into one commit", "note the fix is for issue #42").
