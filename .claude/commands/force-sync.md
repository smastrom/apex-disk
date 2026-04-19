Reconcile `docs/` and `AGENTS.md` against recent commits that bypassed the `/sync` docs sweep, then commit any drift.

Use this when source changes were committed directly (not through `/sync`), so the working tree is clean but docs may no longer match the code. Unlike `/sync`, there are no staged or unstaged edits to group — the recovery work is to detect stale docs and bring them back in line.

1. **Find the window to re-check** — determine the range of commits that need re-verifying:
   - `git log -1 --format=%H -- docs/ AGENTS.md` gives the last commit that touched docs. Use `<that-sha>..HEAD` as the default window.
   - If that produces nothing (or the range is empty), fall back to the last 10 commits: `HEAD~10..HEAD`.
   - If `$ARGUMENTS` is provided, use it as the range or ref (e.g. `HEAD~5..HEAD`, a SHA, or a count).
   - Print the chosen range and list its commits with `git log --oneline <range>` so it's clear what's being reconciled.

2. **Survey the changes in the window** — `git diff <range>` and read the touched source files at their current state. Focus on behavior, flags, file paths, commands, and public APIs — the kinds of things docs reference. Ignore pure refactors that don't change externally-observable behavior.

3. **Docs sweep** — open **every** file under `docs/` plus `AGENTS.md`. For each, decide whether it still matches the current source. Fix anything stale: outdated paths, renamed symbols, removed flags, changed commands, wrong version numbers, obsolete instructions. Don't rewrite prose that's still accurate.

4. **Commit** — if any docs changed, stage and commit them as one commit (or split if the drift covers unrelated areas). Subject in the imperative, ≤70 chars; body only when the *why* isn't obvious; include the agent `Co-Authored-By` trailer. Never skip hooks. If no docs changed, say so and stop — don't create an empty commit.

5. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`.

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`. Do **not** rewrite or amend the source commits that caused the drift; only add follow-up doc commits.
