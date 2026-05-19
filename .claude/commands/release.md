Prepare a new release for ApexDisk version `$ARGUMENTS` (semver **without** the `v` prefix, e.g. `0.0.11` — the changelog heading is `## v0.0.11` and the git tag becomes `v0.0.11`).

Use this command when **you haven't written the release notes yet** — the agent generates them from `git log`. If you've already prepended the `## v$ARGUMENTS` block to `RELEASES.md` by hand, run `/release-from-notes $ARGUMENTS` instead.

Walk through each step to prepare the release:

0. **Reject if notes are already there** — read `RELEASES.md` and look at the first `## vX.Y.Z` heading. If it equals `## v$ARGUMENTS`, **stop immediately**: do not overwrite or duplicate the existing notes. Tell the user to run `/release-from-notes $ARGUMENTS` instead. (If the first heading is a _different_ version, that's fine — it's the previous release, and the new `## v$ARGUMENTS` section will go above it.)

1. **Bump version** — Update the version string to `$ARGUMENTS` in all three files (they must match exactly):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`

2. **Update RELEASES.md** — Add a new `## v$ARGUMENTS` section at the **top** of the changelog (newest-first), directly below the `---` horizontal rule and above the previous entry. The GitHub **Release** workflow uses the **first** `## vX.Y.Z` line as the canonical version—if you append at the bottom, CI would read the wrong version. Summarize changes since the last tag (e.g. `git log` since the previous `v*` tag), **grouped under these `###` subheadings in this order**:
   - **New Features** — user-visible additions.
   - **Improvements** — enhancements to existing behavior, UX polish, perf wins, refactors with observable effects.
   - **Bug Fixes** — defect fixes.
   - **Chores** — internal housekeeping with no user-visible effect (deps bumps, CI, docs, dead-code removal, test-only changes).

   Omit any group that has no entries; do **not** leave an empty `###` heading. Each entry is a single bullet. Classify by the dominant effect of the commit, not the commit-message prefix.

   **Sort within every group from most to least significant.** Lead with the changes that have the biggest user-visible footprint — a new default theme, a core scan behavior change, a settings split that reshapes a flow — and trail with small touches like footer labels, copy tweaks, single-toggle additions. Use UI prominence and breadth of impact as the heuristic: would a returning user notice this in the first minute (top) or only on a settings deep-dive (bottom)?

   **Frontend / Backend split for Improvements and Bug Fixes — only when the group has more than 4 entries.** At 4 or fewer, list them flat under the `###` heading sorted by significance. At 5 or more, split into `#### Frontend` and `#### Backend` sub-blocks (omit either if empty), and apply the significance sort inside each sub-block. Frontend covers UI, themes, translations, components, settings surface; backend covers Rust, scanning, IPC, updater, build. New Features and Chores stay flat regardless of count.

3. **Verify consistency** — Confirm all three version files and RELEASES.md heading match `$ARGUMENTS`.

4. **Commit** — Stage and commit with the message `Release v$ARGUMENTS`.

Do NOT push or trigger the GitHub Actions workflow — just prepare the commit locally.
