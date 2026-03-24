Prepare a new release for ApexDisk version `$ARGUMENTS` (semver **without** the `v` prefix, e.g. `0.0.11` — the changelog heading is `## v0.0.11` and the git tag becomes `v0.0.11`).

Walk through each step to prepare the release:

1. **Bump version** — Update the version string to `$ARGUMENTS` in all three files (they must match exactly):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`

2. **Update RELEASES.md** — Add a new `## v$ARGUMENTS` section at the **end** of the file (after all existing `## v…` blocks). The GitHub **Release** workflow uses the **last** `## vX.Y.Z` line as the canonical version—if you insert at the top, CI would read the wrong version. Summarize changes since the last tag (e.g. `git log` since the previous `v*` tag).

3. **Verify consistency** — Confirm all three version files and RELEASES.md heading match `$ARGUMENTS`.

4. **Commit** — Stage and commit with the message `Release v$ARGUMENTS`.

Do NOT push or trigger the GitHub Actions workflow — just prepare the commit locally.
