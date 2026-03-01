# Release System

This folder contains the release infrastructure for Mac User Lens.

## How It Works

A single GitHub Action (`.github/workflows/release.yml`) handles the entire release process. It is triggered manually from the Actions tab. The workflow builds a signed universal macOS binary, produces updater artifacts (`.app.tar.gz` and `.sig`), generates a `latest.json` for the in-app updater, and publishes everything to a GitHub Release.

## Updater Signing (one-time setup)

The in-app updater requires **signed** artifacts. Do this once before your first updater-enabled release.

1. Generate signing keys:

   ```sh
   pnpm tauri signer generate -w ~/.tauri/mac-user-lens.key
   ```

2. Copy the **public key** contents into `src-tauri/tauri.conf.json` → `plugins.updater.pubkey` (single-line, `\n`-escaped). The public key is safe to commit.

3. Add the **private key** as a GitHub repo secret: **Settings → Secrets → Actions** → `TAURI_SIGNING_PRIVATE_KEY`.

## Creating a Release

### Step 1: Implement your changes

Write your code, fix bugs, add features — whatever the release includes.

### Step 2: Bump the version

Update the version string in **all three files**:

| File                        | Field       |
| --------------------------- | ----------- |
| `package.json`              | `"version"` |
| `src-tauri/Cargo.toml`      | `version`   |
| `src-tauri/tauri.conf.json` | `"version"` |

All three must match exactly (e.g., `0.2.0`).

### Step 3: Update RELEASES.md

Add a new section **at the top** of `releases/RELEASES.md`:

```markdown
## v0.2.0

- Added dark mode support
- Fixed crash on startup with macOS 12

## v0.0.8

Initial release.
```

The heading must be `## vX.Y.Z` where `X.Y.Z` matches the version you set in Step 2.

### Step 4: Commit and push

```sh
git add -A
git commit -m "Release v0.2.0"
git push
```

### Step 5: Trigger the release

1. Go to https://github.com/smastrom/mac-user-lens/actions
2. Select the **Release** workflow
3. Click **Run workflow**

The workflow will:

1. Validate that RELEASES.md, package.json, Cargo.toml, and tauri.conf.json all have the same version
2. Check that the tag doesn't already exist
3. Run tests (`pnpm test`)
4. Build a signed universal macOS binary (Intel + Apple Silicon) and updater artifacts (`.app.tar.gz`, `.sig`) using `TAURI_SIGNING_PRIVATE_KEY`
5. Generate `latest.json` for the in-app updater
6. Create a git tag (e.g., `v0.2.0`)
7. Create a GitHub Release with the `.dmg`, `.app.tar.gz`, `.sig`, and `latest.json` attached, and release notes from RELEASES.md

## Handling Failures

If the workflow fails after the tag was created:

1. Delete the tag remotely:
   ```sh
   git push origin :refs/tags/v0.2.0
   ```
2. Fix the issue
3. Commit and push the fix
4. Trigger the workflow again

If it fails before tagging (validation, tests, or build), just fix the issue, push, and re-trigger.

If the build step fails with an error about the updater signature or missing key, ensure the repo secret `TAURI_SIGNING_PRIVATE_KEY` is set (see **Updater Signing** above).

## Files

| File                            | Purpose                                                         |
| ------------------------------- | --------------------------------------------------------------- |
| `RELEASES.md`                   | Changelog — add entries here for each release                   |
| `README.md`                     | This file                                                       |
| `.github/workflows/release.yml` | GitHub Action — validates, builds, tags, and publishes releases |
