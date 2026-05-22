Prepare a stable release for ApexDisk version `$ARGUMENTS` (semver **without** the `v` prefix, e.g. `0.0.11`) using the release notes already in `RELEASES.md`.

Use this command when you wrote the `## v$ARGUMENTS` block by hand. If you want the agent to generate the notes from `git log` instead, run `/release $ARGUMENTS`.

0. **Open the gate** — `mkdir -p .claude && touch .claude/.sync-active`. The pre-commit hook in `.claude/hooks/pre-commit-gate.sh` blocks `git commit` and `git push` unless this marker is present. Clear it at the end (step 5). The marker is gitignored.

1. **Verify the notes are in place** — read `RELEASES.md` and confirm the **first** `## vX.Y.Z` heading is exactly `## v$ARGUMENTS`. The Release workflow reads the first `## v` heading (`grep '^## v'` then `head -1`) as the canonical version, so anything else on top would ship the wrong build. If the first heading is missing, points at a different version, or the file doesn't have one at all, **stop and surface** (close the gate — step 5): do not generate notes, do not bump versions, do not commit. Tell the user to either prepend the section by hand or run `/release $ARGUMENTS` for the auto-generated path.

2. **Bump version** — update the version string to `$ARGUMENTS` in all three files (they must match exactly):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`

3. **Verify consistency** — confirm all three version files and the `RELEASES.md` heading match `$ARGUMENTS`.

4. **Commit** — stage and commit with the message `Release v$ARGUMENTS`.

5. **Close the gate** — `rm -f .claude/.sync-active`. Run this on success **and** on any early-exit (reject in step 1, user abort).

Do NOT push or trigger the GitHub Actions workflow — just prepare the commit locally. Do NOT edit the existing notes in this command; if they need changes, do that as a separate commit **before** running this one.
