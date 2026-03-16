Prepare a new release for ApexDisk version `$ARGUMENTS`.

Walk through each step to prepare the release:

1. **Bump version** — Update the version string to `$ARGUMENTS` in all three files (they must match exactly):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`

2. **Update RELEASES.md** — Add a new `## v$ARGUMENTS` section at the top of `RELEASES.md`. Summarize the changes since the last release by reviewing the git log since the last version tag.

3. **Verify consistency** — Confirm all three version files and RELEASES.md heading match `$ARGUMENTS`.

4. **Commit** — Stage and commit with the message `Release v$ARGUMENTS`.

Do NOT push or trigger the GitHub Actions workflow — just prepare the commit locally.
