# In-App Update System

ApexDisk uses [tauri-plugin-updater](https://v2.tauri.app/plugin/updater/) for signed, in-app updates distributed via GitHub Releases.

## How It Works

Updates can be **automatic** or **manual**, controlled by the "Automatic Updates" toggle in Settings. Automatic updates are **disabled by default** (opt-in). The setting takes effect on next app start.

### Automatic Updates (opt-in)

When enabled, the app follows the pattern used by modern desktop apps (Claude, VS Code, Slack):

1. The app silently checks for updates on startup
2. If an update is found, it is **downloaded automatically in the background** — no dialogs
3. Once downloaded, the UI shows a **"Restart to Update"** button in Settings, and the menu item changes to **"Restart to Update (vX.Y.Z)"**
4. The user restarts at their convenience to apply the update

### Manual Updates (default)

When automatic updates are disabled:

1. No background checks or downloads happen on startup
2. The user clicks **"Check for Updates"** (in Settings or menu bar) to trigger a native dialog
3. The dialog checks for updates, downloads if found, and prompts to restart — all in one flow
4. The menu item always shows **"Check for Updates…"** (never changes to "Restart to Update")

### Endpoint

```
https://github.com/smastrom/apex-disk/releases/latest/download/latest.json
```

GitHub's `/releases/latest/` URL always resolves to the most recent **non-pre-release**. This is how stable users are protected from beta builds.

### Update Entry Points

Both the menu item and the Settings button **always trigger the same native dialog flow**: check → download → "Restart now?" prompt. The only difference is that when automatic updates are ON and an update is already staged, the menu item restarts immediately.

| Trigger | Auto-updates ON | Auto-updates OFF |
|---|---|---|
| **App start** | Silent check → auto-download → "Restart to Update" in Settings + menu | Nothing |
| **Menu bar** click (update staged) | Restarts immediately | Native dialog flow |
| **Menu bar** click (no update staged) | Native dialog flow (menu text updates during check/download) | Native dialog flow (menu text unchanged) |
| **Settings** button click | Native dialog flow | Native dialog flow |

### Update Flow Diagram

```
App start (auto-updates ON only)
  └─ check_for_updates_silent → version available?
       ├─ No  → UI shows "Updated ✓"
       └─ Yes → download_update (background, no dialogs)
            ├─ Success → UI: "Restart to Update" button + menu item text changes
            └─ Error   → logged to console, UI unchanged

Button / Menu click (both modes)
  └─ check_for_updates_dialog → native dialog flow
       ├─ Check fails → "No Updates Available" dialog (error logged)
       ├─ No update   → "No Updates Available" dialog
       └─ Update found → download → "Restart now?" dialog
```

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
6. Users running older versions will see the update on next app start (if auto-updates ON) or when checking manually

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

### Dev mode behavior

The silent auto-check (`checkSilently`) is **skipped during `pnpm tauri dev`** — the `import.meta.env.DEV` guard prevents background update checks. However, the **manual check button** (`onCheckForUpdates`) works in dev mode and will trigger the native dialog flow (which will fail with a logged error since no release endpoint is available).

### Testing the real update flow

To test the full update experience (check → download → "Restart to Update"):

1. Release version `N` via GitHub Actions
2. Download the `.dmg` from the release page and install it to `/Applications`
3. Release version `N+1` via GitHub Actions
4. Open the installed version `N` — it will detect `N+1`, auto-download, and show "Restart to Update"


## File Overview

| File | Role |
|---|---|
| `src-tauri/src/updater.rs` | Rust module: silent check, native dialog flow, menu text updates, `autoUpdates` setting reader |
| `src/lib/use-app-update.ts` | Vue composable: reactive state (checking → downloading → ready), auto-check on start (auto-updates only), manual check via native dialog |
| `src/components/SettingsView.vue` | UI: update description + action button, auto-updates toggle |
| `src-tauri/src/menu_translations.rs` | Menu label translations including "Checking for Updates…", "Downloading Update…", "Restart to Update" |
| `src-tauri/tauri.conf.json` | Updater config: endpoint URL, public key, `createUpdaterArtifacts` |
| `src-tauri/Entitlements.plist` | macOS entitlement: `com.apple.security.network.client` for downloads |
| `.github/workflows/release.yml` | CI: builds, signs artifacts, generates `latest.json`, uploads to release |
