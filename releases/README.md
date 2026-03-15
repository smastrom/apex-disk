# Release System

This folder contains the release infrastructure for ApexDisk.

## How It Works

A single GitHub Action (`.github/workflows/release.yml`) handles the entire release process. It is triggered manually from the Actions tab. The workflow builds a universal macOS binary signed with a Developer ID certificate, notarizes it, packages it as a `.dmg`, and publishes it to a GitHub Release.

The app checks for updates by fetching the latest release tag from the GitHub API and comparing it against the current version. If a newer version is available, the user is invited to visit the releases page — no automatic downloads or installs.

## macOS Codesigning

Tauri v2 does **not** load `.env` files automatically. When no `APPLE_SIGNING_IDENTITY` is set, it skips `codesign` entirely, leaving the binary with only the linker's default adhoc signature. This causes macOS TCC to use a wrong, unstable identifier — breaking Full Disk Access and per-folder permission grants.

### Local builds

`pnpm tauri:build` sources `.env` via `scripts/load-env.sh`, which validates that all required variables are set before invoking the build. The `.env` file must contain:

```
APPLE_SIGNING_IDENTITY="Developer ID Application: Name (TEAMID)"
APPLE_ID="your@email.com"
APPLE_PASSWORD="app-specific-password"
APPLE_TEAM_ID="TEAMID"
```

If any variable is missing or the `.env` file doesn't exist, the script exits with an error.

To build **without** signing (ad-hoc), use `pnpm tauri:build:unsigned`.

### CI builds

The GitHub Actions workflow imports the Developer ID certificate from a base64-encoded secret into a temporary keychain, extracts the signing identity automatically, and passes it to Tauri. The following repository secrets must be configured:

| Secret                       | Description                                      |
| ---------------------------- | ------------------------------------------------ |
| `APPLE_CERTIFICATE`          | Base64-encoded `.p12` Developer ID certificate   |
| `APPLE_CERTIFICATE_PASSWORD` | Password for the `.p12` file                     |
| `KEYCHAIN_PASSWORD`          | Arbitrary password for the temporary CI keychain |
| `APPLE_ID`                   | Apple ID email for notarization                  |
| `APPLE_PASSWORD`             | App-specific password for notarization           |
| `APPLE_TEAM_ID`              | Apple Developer Team ID                          |

To export the certificate as base64:

```sh
base64 -i YourCertificate.p12 | pbcopy
```

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

1. Go to https://github.com/smastrom/apex-disk/actions
2. Select the **Release** workflow
3. Click **Run workflow**

The workflow will:

1. Validate that RELEASES.md, package.json, Cargo.toml, and tauri.conf.json all have the same version
2. Check that the tag doesn't already exist
3. Run tests (`pnpm test`)
4. Build, sign and notarize a universal macOS binary (Intel + Apple Silicon)
5. Create a git tag (e.g., `v0.2.0`)
6. Create a GitHub Release with the `.dmg` attached and release notes from RELEASES.md

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

## Files

| File                            | Purpose                                                         |
| ------------------------------- | --------------------------------------------------------------- |
| `RELEASES.md`                   | Changelog — add entries here for each release                   |
| `README.md`                     | This file                                                       |
| `.github/workflows/release.yml` | GitHub Action — validates, builds, tags, and publishes releases |
| `scripts/load-env.sh`           | Loads and validates `.env` vars for local signed builds         |
