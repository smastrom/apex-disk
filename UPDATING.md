# In-App Update System

ApexDisk uses [tauri-plugin-updater](https://v2.tauri.app/plugin/updater/) for signed, in-app updates distributed via GitHub Releases.

## How It Works

1. The app checks a JSON endpoint on GitHub for the latest version
2. If a newer version exists, the user is prompted via a native macOS dialog
3. The update is downloaded, verified (signature check), and installed
4. The app restarts with the new version

### Endpoint

```
https://github.com/smastrom/apex-disk/releases/latest/download/latest.json
```

GitHub's `/releases/latest/` URL always resolves to the most recent **non-pre-release**. This is how stable users are protected from beta builds.

### Update Entry Points

| Trigger | What happens |
|---|---|
| **App start** | Silent check — updates the SettingsView status (no dialogs) |
| **Menu bar** → "Check for Updates…" | Full native dialog flow: check → confirm → download → restart |
| **Settings** → "Check for Updates" button | Same native dialog flow as the menu item |

## Signing Keys

Updates are signed with a keypair generated via `pnpm tauri signer generate`. The **public key** is embedded in `tauri.conf.json` under `plugins.updater.pubkey`. The **private key** and its **password** are stored as GitHub secrets:

- `TAURI_SIGNING_PRIVATE_KEY` — the base64-encoded private key
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — the password for the private key

The CI build uses these to sign the `.tar.gz` update artifact. The app verifies the signature against the embedded public key before applying any update.

## Release Workflow

### Stable Release

1. Update the version in `package.json`, `tauri.conf.json`, `Cargo.toml`, and add a heading in `RELEASES.md`
2. Commit and push to `main`
3. Go to **Actions → Release → Run workflow**
4. Leave "Mark as pre-release (beta)" **unchecked**
5. CI builds, signs, notarizes, creates a GitHub Release with:
   - `.dmg` — the installer for new users
   - `.tar.gz` + `.tar.gz.sig` — the signed update bundle
   - `latest.json` — the update manifest pointing to the `.tar.gz`
6. Users running older versions will see the update on next app start or when checking manually

### Beta / Pre-release

1. Update the version to something like `0.10.0-beta.1` in all the same files
2. Commit and push
3. Go to **Actions → Release → Run workflow**
4. **Check** "Mark as pre-release (beta)"
5. CI builds the same artifacts and creates a GitHub **pre-release**
6. **Stable users are not affected** — the `/releases/latest/` URL still points to the last stable release, so `latest.json` from the beta is not served to them
7. To test the beta, download the `.dmg` directly from the GitHub pre-release page

### Why stable users don't receive betas

GitHub's URL `https://github.com/.../releases/latest/download/latest.json` is a redirect. GitHub resolves `latest` to the most recent release that is **not** marked as a pre-release. So:

- `v0.9.0` (stable) → `latest.json` is served from here
- `v0.10.0-beta.1` (pre-release) → has its own `latest.json`, but nobody fetches it because `/releases/latest/` still points to `v0.9.0`

Each release has a direct URL too: `/releases/download/v0.10.0-beta.1/latest.json` — but the app never uses that URL.

## Local Development

### Testing the SettingsView UI

In `src/lib/use-app-update.ts`, change the `DEV_MOCK_VERSION` constant:

```ts
// Simulate "update available" — shows version 1.2.0 as available
const DEV_MOCK_VERSION: string | null = '1.2.0'

// Simulate "up to date" — no update available
const DEV_MOCK_VERSION: string | null = null
```

This mock is only active during `pnpm tauri dev` (when `import.meta.env.DEV` is true). In production builds, the real updater endpoint is used.

### Testing the full update flow locally

To test the actual download/install flow without GitHub:

1. Build a `.tar.gz` with `pnpm tauri build`
2. Serve a `latest.json` from a local HTTP server (e.g. `npx serve .`) pointing to the local `.tar.gz`
3. Override the endpoint in `tauri.dev.conf.json`:

```json
{
   "plugins": {
      "updater": {
         "endpoints": ["http://localhost:3000/latest.json"]
      }
   }
}
```

4. Run `pnpm tauri dev` — the app will check your local server

## File Overview

| File | Role |
|---|---|
| `src-tauri/src/updater.rs` | Rust module: silent check command, native dialog flow, menu handler |
| `src/lib/use-app-update.ts` | Vue composable: reactive state, dev mock, auto-check on start |
| `src/components/SettingsView.vue` | UI: inline status + "Check for Updates" button |
| `src-tauri/tauri.conf.json` | Updater config: endpoint URL, public key, `createUpdaterArtifacts` |
| `src-tauri/Entitlements.plist` | macOS entitlement: `com.apple.security.network.client` for downloads |
| `.github/workflows/release.yml` | CI: builds, signs artifacts, generates `latest.json`, uploads to release |
