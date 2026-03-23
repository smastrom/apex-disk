# In-App Update System

ApexDisk uses [tauri-plugin-updater](https://v2.tauri.app/plugin/updater/) for signed, in-app updates distributed via GitHub Releases.

## How It Works

The update experience follows the pattern used by modern desktop apps (Claude, VS Code, Slack):

1. The app silently checks for updates on startup
2. If an update is found, it is **downloaded automatically in the background** — no dialogs
3. Once downloaded, the UI shows a **"Restart to Update"** button in Settings, and the menu item changes to **"Restart to Update (vX.Y.Z)"**
4. The user restarts at their convenience to apply the update

### Endpoint

```
https://github.com/smastrom/apex-disk/releases/latest/download/latest.json
```

GitHub's `/releases/latest/` URL always resolves to the most recent **non-pre-release**. This is how stable users are protected from beta builds.

### Update Entry Points

| Trigger | What happens |
|---|---|
| **App start** | Silent check → auto-download → "Restart to Update" appears in Settings + menu (no dialogs) |
| **Menu bar** → "Restart to Update (vX.Y.Z)" | Restarts the app to apply the staged update |
| **Menu bar** → "Check for Updates…" (no update staged) | Checks → if found, downloads silently → prompts Restart / Later via native dialog. If up to date, shows native "No Updates" dialog |
| **Settings** → "Restart to Update" button | Restarts the app to apply the staged update |
| **Settings** → "Check for Updates" button (no update staged) | Checks → if found, downloads silently → UI updates to show "Restart to Update" |

### Update Flow Diagram

```
App start
  └─ check_for_updates_silent → version available?
       ├─ No  → UI shows "Updated ✓"
       └─ Yes → download_update (background, no dialogs)
            ├─ Success → UI: "Restart to Update" button + menu item text changes
            └─ Error   → logged to console, UI unchanged
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

### Private repositories

While the repository is private, GitHub requires authentication to access release assets. Add a `GITHUB_TOKEN` to your `.env` file:

```env
GITHUB_TOKEN=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

The Rust updater reads this at runtime and attaches it as an `Authorization` header to all update requests. Once the repository is public, remove the token — it's no longer needed.

To generate a token: **GitHub → Settings → Developer settings → Personal access tokens → Fine-grained tokens** with `Contents: Read-only` permission on the repository.

### Testing the update flow against GitHub releases

To test the real update flow during development, your local version must be **older** than the latest GitHub release:

1. Ensure `GITHUB_TOKEN` is set in `.env` (see above)
2. Temporarily lower the version in `src-tauri/Cargo.toml` (e.g. `version = "0.0.1"`)
3. Run `pnpm tauri dev`
4. The app will detect the newer GitHub release, download it, and show "Restart to Update"

Remember to revert `Cargo.toml` after testing.

### Testing the full update flow locally (without GitHub)

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
| `src-tauri/src/updater.rs` | Rust module: silent check, silent download, restart command, menu-initiated flow with native dialogs |
| `src/lib/use-app-update.ts` | Vue composable: reactive state (checking → downloading → ready), auto-check + auto-download on start, dev mock |
| `src/components/SettingsView.vue` | UI: inline status + "Check for Updates" / "Restart to Update" button |
| `src-tauri/src/menu_translations.rs` | Menu label translations including "Restart to Update" |
| `src-tauri/tauri.conf.json` | Updater config: endpoint URL, public key, `createUpdaterArtifacts` |
| `src-tauri/Entitlements.plist` | macOS entitlement: `com.apple.security.network.client` for downloads |
| `.github/workflows/release.yml` | CI: builds, signs artifacts, generates `latest.json`, uploads to release |
