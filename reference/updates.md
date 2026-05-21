# Updates

In-app update behavior for **stable** builds (the [tauri-plugin-updater](https://v2.tauri.app/plugin/updater/) channel backed by GitHub Releases). For how stable and Beta builds are **cut** — release workflow, version fields, changelog files, Beta channel — see `releases.md`.

## In-App Updates (Stable Releases)

Two independent toggles in Settings control update behavior:

| Setting                         | Persisted key        | What it does                                                                                                    |
| ------------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------- |
| Automatically check for updates | `autoCheckUpdates`   | On app start, silently asks the GitHub endpoint whether a newer version exists. **No download.**                |
| Automatically install updates   | `autoInstallUpdates` | When a check finds an update, silently downloads + stages it so the next launch picks it up. Requires checking. |

**Cascade rule** — enabling install forces checking on; disabling checking forces install off. The store enforces this in `setAutoCheckUpdates` / `setAutoInstallUpdates` (in `src/stores/app-settings.ts`) so callers don't need to coordinate. The UI also grays out the install toggle when checking is off.

**Defaults on fresh install** — both **OFF** (opt-in).

### State matrix

What the user sees in each combination, including the silent-check outcome and what the manual "update" button / menu item does.

| Auto-check | Auto-install | App start                                                         | Settings row when update found                                                               | Manual button label                                                         | Menu item                                                   |
| ---------- | ------------ | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ----------------------------------------------------------- |
| OFF        | OFF          | Nothing                                                           | (never reached automatically — user clicks button to trigger)                                | "Check for Updates" → native dialog flow                                    | "Check for Updates…" → native dialog flow                   |
| ON         | OFF          | Silent check, **no download**                                     | Description: _"Update vX.Y.Z available"_. **No "Install and relaunch"** — nothing is staged. | "Update to vX.Y.Z" → native dialog flow (check + download + restart prompt) | "Update to vX.Y.Z" → same dialog flow                       |
| ON         | ON           | Silent check + silent background download                         | Description: _"Update vX.Y.Z ready"_. Accent button.                                         | "Install and Relaunch" → calls `restart_app` directly                       | "Restart to Update (vX.Y.Z)" → calls `restart_app` directly |
| OFF        | ON           | (impossible — cascade rule forces this state to OFF/OFF or ON/ON) |

**"Install and Relaunch" is only visible when an update is already staged on disk.** That requires either `autoInstallUpdates: true` (silent staging) or a completed manual dialog flow. With check-ON + install-OFF, the button instead reads "Update to vX.Y.Z" and clicking it starts the download flow on demand.

### Update Entry Points

The **Settings button** and **menu item** both read the same state and act on it:

- **Update already staged** (any path) → restart immediately (no redundant dialog).
- **Update not staged** → run the native dialog flow (check → download → "Restart now?").
- The Settings button additionally holds a spinner state via `isChecking` while the dialog flow runs (the Rust side has no progress events, so check + download are visually one phase). The menu item updates its text directly during the flow (Checking → Downloading → Restart to Update) because the menu IS the entrypoint in that path.

### Endpoint

```
https://github.com/smastrom/apex-disk/releases/latest/download/latest.json
```

GitHub's `/releases/latest/` URL always resolves to the most recent **non-pre-release**. This is how stable users are protected from pre-release builds (both semver pre-releases and Beta-channel builds).

### Update Flow Diagram

```
App start
  └─ autoCheckUpdates ON?
       ├─ No  → do nothing
       └─ Yes → check_for_updates_silent → version available?
            ├─ No  → UI shows "Up to date"
            └─ Yes → autoInstallUpdates ON?
                 ├─ No  → set_update_menu_available(v)
                 │        UI: "Update to vX.Y.Z" button, menu: "Update to vX.Y.Z"
                 └─ Yes → download_update (background, no dialogs)
                      ├─ Success → UI: "Install and Relaunch" + menu: "Restart to Update (vX.Y.Z)"
                      └─ Error   → logged to console, UI/menu unchanged

Button / Menu click
  └─ Update already staged (ready_version is Some)?
       ├─ Yes → restart_app
       └─ No  → check_for_updates_dialog (native dialog flow)
            ├─ Check fails → "No Updates Available" dialog (error logged)
            ├─ No update   → "No Updates Available" dialog
            └─ Update found → download_and_install → "Restart now?" dialog
```

## Update artifact shape (per-architecture)

The Release workflow produces **two** `.tar.gz` update bundles — one for Apple Silicon (`aarch64`), one for Intel (`x86_64`) — each with its own `.tar.gz.sig` signature. `latest.json` lists both under `platforms.darwin-aarch64` and `platforms.darwin-x86_64`. The Tauri updater plugin reads the entry that matches the running machine and downloads only that slice, so an Apple Silicon Mac never pulls the Intel binary and vice versa.

Existing users running an older universal-binary install transition to a per-arch binary automatically on their next update — the plugin doesn't care that the previous install was universal; it just replaces it with the correct slice.

The universal DMG is install-only — no universal `.tar.gz` is uploaded. Universal-binary installs that haven't updated yet still receive updates correctly (the plugin uses the running arch, not the installed binary's arch).

## Signing Keys

Updates are signed with a keypair generated via `pnpm tauri signer generate`. The **public key** is embedded in `tauri.conf.json` under `plugins.updater.pubkey`. The **private key** and its **password** are stored as GitHub secrets:

- `TAURI_SIGNING_PRIVATE_KEY` — the base64-encoded private key
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` — the password for the private key

The **Release** workflow uses these to sign each per-arch `.tar.gz` update artifact. The app verifies the signature against the embedded public key before applying any update.

## Why stable users don't receive pre-releases

The updater endpoint `https://github.com/.../releases/latest/download/latest.json` is a redirect. GitHub resolves `latest` to the most recent release that is **not** marked as a pre-release. So:

- `v0.9.0` (stable) → `latest.json` is served from here
- `v0.10.0-beta.1` (pre-release) → has its own `latest.json`, but nobody fetches it because `/releases/latest/` still points to `v0.9.0`

Each release has a direct URL too: `/releases/download/v0.10.0-beta.1/latest.json` — but the app never uses that URL. This also keeps Beta-channel builds (tag `beta-<run_id>`) off the updater: they're GitHub pre-releases and don't ship `latest.json` anyway.

Beta and dev builds also short-circuit `check_for_updates_silent` / `check_for_updates_dialog` / `check_for_updates_from_menu` by bundle identifier so a release-mode Beta DMG never advertises stable updates even if it inherits the stable updater endpoint from `tauri.conf.json`. The runtime check compares `app.config().identifier` against the hardcoded stable identifier in `updater.rs`.

## Local Development

### Dev mode behavior

The silent check (`checkSilently`) is **skipped during `pnpm tauri dev`** — the `import.meta.env.DEV` guard prevents background update checks regardless of `autoCheckUpdates`. The **manual check button** (`onCheckForUpdates`) works in dev mode and will trigger the native dialog flow (which will fail with a logged error since no release endpoint is available).

### Testing the real update flow

To test the full update experience (check → download → "Restart to Update"):

1. Release version `N` via GitHub Actions
2. Download the `.dmg` from the release page and install it to `/Applications`
3. Release version `N+1` via GitHub Actions
4. Open the installed version `N` — it will detect `N+1`, auto-download, and show "Restart to Update"

## File Overview

| File                                 | Role                                                                                                                                                       |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src-tauri/src/updater.rs`           | Rust module: silent check, native dialog flow, menu text updates, `autoCheckUpdates` / `autoInstallUpdates` setting readers                                |
| `src-tauri/src/store.rs`             | Persisted settings for `autoCheckUpdates` / `autoInstallUpdates`                                                                                           |
| `src/lib/use-app-update.ts`          | Vue composable: reactive state (checking → downloading → ready), orchestrates silent check, conditional silent download, and manual dialog/restart routing |
| `src/components/SettingsView.vue`    | UI: update description + action button, both auto-check and auto-install toggles (cascade + gray-out)                                                      |
| `src-tauri/src/menu_translations.rs` | Menu label translations including "Checking for Updates…", "Downloading Update…", "Update to", "Restart to Update"                                         |
| `src-tauri/tauri.conf.json`          | Updater config: endpoint URL, public key, `createUpdaterArtifacts`                                                                                         |
| `src-tauri/tauri.beta.conf.json`     | Merge config for the Beta-channel DMG: bundle id / product name, `createUpdaterArtifacts: false`                                                           |
| `src-tauri/Entitlements.plist`       | macOS entitlements (minimal): `com.apple.security.cs.allow-jit` for WKWebView, `com.apple.security.network.client` for downloads                           |
| `.github/workflows/release.yml`      | CI: builds, signs artifacts, generates `latest.json`, uploads to release                                                                                   |
| `.github/workflows/beta.yml`         | CI: `workflow_dispatch` only — unit tests, Beta DMG, **pre-release** (`beta-<run_id>`) + artifact                                                          |
| `../RELEASES_BETA.md`                | Optional Beta QA notes; pre-release body + artifact; first `##` section in job summary; not used for semver                                                |
