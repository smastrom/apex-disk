Reconcile every `.md` file in the repo against recent commits that bypassed the `/sync` docs sweep, then commit any drift.

Use this when source changes were committed directly (not through `/sync`), so the working tree is clean but docs may no longer match the code. Unlike `/sync`, there are no staged or unstaged edits to group — the recovery work is to detect stale docs and bring them back in line.

0. **Open the gate** — `mkdir -p .claude && touch .claude/.sync-active`. The pre-commit hook in `.claude/hooks/pre-commit-gate.sh` blocks `git commit` and `git push` unless this marker is present. Clear it at the end (step 8). The marker is gitignored.

1. **License headers** — run `pnpm headers` first. It adds the SPDX + copyright header to any new source file (`.ts`, `.tsx`, `.vue`, `.rs`, `.sh` under `src/`, `src-tauri/src/`, `e2e/`, `tests/`, `scripts/`) and is idempotent on files that already have one. If it modified anything, stage those changes into the docs-drift commit (or a separate commit if the touched files are unrelated to the docs work).

2. **Find the window to re-check** — determine the range of commits that need re-verifying:
   - `git log -1 --format=%H -- reference/ marketing/ AGENTS.md CLAUDE.md README.md .claude/rules/ .claude/commands/ .coderabbit.yaml` gives the last commit that touched any tracked doc. Use `<that-sha>..HEAD` as the default window.
   - If that produces nothing (or the range is empty), fall back to the last 10 commits: `HEAD~10..HEAD`.
   - If `$ARGUMENTS` is provided, use it as the range or ref (e.g. `HEAD~5..HEAD`, a SHA, or a count).
   - Print the chosen range and list its commits with `git log --oneline <range>` so it's clear what's being reconciled.

3. **Survey the changes in the window** — `git diff <range>` and read the touched source files at their current state. Focus on behavior, flags, file paths, commands, and public APIs — the kinds of things docs reference. Ignore pure refactors that don't change externally-observable behavior.

4. **Docs + file-top comment sweep** —
   - **Repo-wide `.md` sweep.** Open **every** `.md` file in the repo. This includes `reference/*.md`, `marketing/*.md`, `AGENTS.md`, `CLAUDE.md`, `README.md`, `.claude/rules/*.md`, and `.claude/commands/*.md`. `marketing/` is outcome-facing — only touch it when user-visible outcomes changed in the window, not for internal refactors. Skip `RELEASES.md` / `RELEASES_BETA.md` (owned by `/release` and `/beta-notes`) and the immutable surface (`LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`) unless the change directly affects them. For each, decide whether it still matches the current source. Fix anything stale: outdated paths, renamed symbols, removed flags, changed commands, wrong version numbers, obsolete instructions. Don't rewrite prose that's still accurate.
   - **`.coderabbit.yaml` sweep.** Re-read its `path_instructions` against the source touched in the window and update any keys whose intent drifted. Don't touch `path_filters`, `auto_review`, `tools`, or the chat / knowledge_base blocks unless the window directly affects them.
   - **File-top comment sweep.** For every `.vue` and `.rs` file touched in the window's commits, re-read its leading comment block against the current implementation. Check:
      - **`.vue`** — the `<!-- ComponentName ... -->` block (Purpose / Props / Example).
      - **`.rs`** — the `//!` module doc at the top of the file.
      - **`.ts`** — only if the file has a JSDoc/`/** */` header (e.g. `src/lib/log.ts`).
        Fix any drift in the same recovery commit.

5. **Commit** — if any docs changed, stage and commit them as one commit (or split if the drift covers unrelated areas). Subject in the imperative, ≤70 chars; body only when the _why_ isn't obvious; include the agent `Co-Authored-By` trailer. Never skip hooks. If no docs changed, skip this step (don't create an empty commit) but still run step 6.

6. **Verify** — the whole point of `/force-sync` is to recover from commits that bypassed `/sync`'s checks, so always run the test suite on `HEAD` (even if no docs drifted — the source commits in the window may themselves be broken):
   - Always: `pnpm headers:check` and `pnpm fmt:check`.
   - If any commit in the window touched TypeScript or Vue (`src/**/*.ts`, `*.vue`): `pnpm typecheck` (= `vue-tsc --noEmit`).
   - If any commit in the window touched Rust (`src-tauri/**`): `pnpm test:unit`.
   - If any commit in the window touched frontend (`src/**`), Rust (`src-tauri/**`), or e2e (`e2e/**`): `pnpm test:e2e`.
   - If every commit in the window is docs-only, the suite can be skipped.

   If anything fails, stop and surface the failures. Do **not** push. Failures predate this `/force-sync` run, so fix forward (a new commit) — do not amend or revert the source commits behind your back, and never bypass with `--no-verify` / `--force`.

7. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`. Skip if step 5 produced no commit and `HEAD` is already at `origin`.

8. **Close the gate** — `rm -f .claude/.sync-active`. Run this on success **and** on any early-exit (failed gate, user abort, push refusal).

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`. Do **not** rewrite or amend the source commits that caused the drift; only add follow-up doc commits.
