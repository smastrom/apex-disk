Create logical commits for all uncommitted work since the latest commit, then push to the remote.

1. **Survey** — `git status`, `git diff`, `git diff --staged`, and `git log --oneline -10` (for the commit-message style). Read every modified and untracked file enough to understand what conceptually changed — not just which files moved.

2. **Group** — split the work into logically-cohesive commits: one intent per commit. Don't mix unrelated changes. If a single file spans multiple intents, stage it in pieces (edit the file down to one intent, commit, restore the rest).

3. **Docs sweep** — for each group, open **every** file under `docs/` plus `AGENTS.md` and decide whether any of them no longer match the change (per the AGENTS.md pre-commit rule). Update any that don't — stage the doc edits alongside the commit whose change they describe, not in a separate commit.

4. **Commit** — one commit per group. Follow the repo convention: imperative action-title subject line (≤70 chars), blank line, then a concise bulleted or prose body only when the *why* isn't obvious from the subject. Include the agent `Co-Authored-By` trailer. Never skip hooks.

5. **Push** — `git push`. If the branch has no upstream, `git push -u origin <branch>`.

Do **not** bump version fields, edit `RELEASES.md` or `RELEASES_BETA.md`, or trigger the Release/Beta workflows — those belong to `/release` and `/beta-notes`.

If `$ARGUMENTS` is provided, treat it as extra context for how to group or describe the commits (e.g. "group all scan-related changes into one commit", "note the fix is for issue #42").
