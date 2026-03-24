# Updates

Stable **in-app updates** use [tauri-plugin-updater](https://v2.tauri.app/plugin/updater/) and GitHub Releases. **Nightly** builds are triggered **only** via **Actions → Nightly → Run workflow** (no push trigger); each run publishes a **pre-release** on GitHub (tag `nightly-<run_id>`) with the DMG plus `RELEASES_NIGHTLY.md`, and still uploads a workflow artifact. No `latest.json` / nightly updater channel.

## In-App Updates (Stable Releases)

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

The **Settings** update button always runs the native dialog flow (check → download → "Restart now?"). The **menu** matches that when no update is staged; when automatic updates are **on** and an update is **already staged**, the menu item **restarts the app immediately** instead of opening the dialog.

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

The **Release** workflow uses these to sign the `.tar.gz` update artifact. The app verifies the signature against the embedded public key before applying any update.

## Release Workflow

The **Release** workflow reads the version from **`RELEASES.md`** (the last `## vX.Y.Z` heading) and fails if `package.json`, `tauri.conf.json`, and `Cargo.toml` do not match.

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

## `RELEASES.md` vs `RELEASES_NIGHTLY.md`

| | `RELEASES.md` | `RELEASES_NIGHTLY.md` |
| --- | --- | --- |
| **Used by CI for version?** | **Yes.** The **Release** workflow takes the version from the **last** `## vX.Y.Z` heading and checks it matches `package.json`, `tauri.conf.json`, and `Cargo.toml`. | **No.** Nightly does not change semver; it tags pre-releases as `nightly-<run_id>` only. |
| **Used by CI at all?** | Drives tag, release notes body, and build. | Attached to each **Nightly** pre-release, included in the workflow artifact, and the **first** `## …` section is echoed into the job summary. |
| **Purpose** | Canonical shipping history + semver for the updater and installers. | Optional tester-facing notes (what to smoke-test, known issues) for a given `main` snapshot. |

So **`RELEASES_NIGHTLY.md` is not a duplicate of `RELEASES.md`**: it does not replace version bookkeeping because nightly builds **reuse the repo’s current semver** (see below). If you tried to drive nightly versions from a second file, you would either duplicate `RELEASES.md` or conflict with it.

## Version nomenclature (stable vs nightly)

**Stable and pre-releases (Release workflow)**  
One **semantic version** (e.g. `0.0.10`, or `0.10.0-beta.1`) lives in `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and the newest `## v…` block in **`RELEASES.md`**. That number is what appears in the app, the Git tag (`v0.0.10`), and `latest.json`.

**Nightly**  
The **ApexDisk Nightly** build uses the **same** version fields as whatever is on the **branch you select** when you run the workflow manually. The Nightly workflow does **not** bump or validate them against a second doc.

Implications:

- **Several nightlies in a row can show the same version** (e.g. three builds while `main` is still `0.0.10`). That is expected: nightly is a **snapshot of the branch**, not a new release line.
- **Tell builds apart** using the GitHub **run** (artifact name includes the run id), **commit SHA**, or the date you downloaded the artifact—not the About-box semver alone.
- **Optional team policy:** only bump `RELEASES.md` / semver when cutting a real **Release**; nightlies between releases keep reporting the last shipped version until you merge a version bump. Alternatively bump version on `main` right after each release so nightlies show the *upcoming* version early—still one source of truth, no `RELEASES_NIGHTLY` version.

The project does **not** currently append build metadata (e.g. `0.0.10+nightly.abc1234`) in CI; adding that would require changing the three version fields (or build scripts) on every nightly run.

## Nightly Builds (QA Only)

Nightly runs **only when you start it manually** (no `push` trigger). Each run creates a **GitHub pre-release** (tag `nightly-<run_id>`) with the signed DMG and `RELEASES_NIGHTLY.md`, so testers can install from **Releases** (filter or scroll to pre-releases). The workflow also uploads **`ApexDisk-Nightly-<run id>`** as an artifact. There is **no** nightly updater channel.

1. Optionally edit **`RELEASES_NIGHTLY.md`** (add a new `## YYYY-MM-DD` section at the top with tester notes — it becomes the top of the pre-release body).
2. Go to **Actions → Nightly → Run workflow**, choose the **branch** to build (e.g. `main`, `nightly`, or `dev`).
3. When the job finishes: open the **pre-release** on the repo’s Releases page, or download the artifact. Install the DMG like any app. The job summary still shows the first `##` section from `RELEASES_NIGHTLY.md`.

**Config** (`src-tauri/tauri.nightly.conf.json`): **ApexDisk Nightly** / `com.smastrom.apex-disk.nightly` so it does not replace the store build, and `bundle.createUpdaterArtifacts` is **false** (no `.tar.gz` / signatures for the updater).

**Local build:** `pnpm tauri:build:nightly` (same signing env as a normal release build).

The updater plugin and stable endpoint are still in the binary; the project does **not** publish `latest.json` or artifacts for nightly in-app updates. Treat nightly like dev for update expectations: focus on the DMG, not on seamless upgrades between nightlies.

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
| `src-tauri/tauri.nightly.conf.json` | Merge config for nightly DMG: bundle id / product name, `createUpdaterArtifacts: false` |
| `src-tauri/Entitlements.plist` | macOS entitlement: `com.apple.security.network.client` for downloads |
| `.github/workflows/release.yml` | CI: builds, signs artifacts, generates `latest.json`, uploads to release |
| `.github/workflows/nightly.yml` | CI: `workflow_dispatch` only — unit tests, nightly DMG, **pre-release** (`nightly-<run_id>`) + artifact |
| `RELEASES_NIGHTLY.md` | Optional nightly QA notes; pre-release body + artifact; first `##` section in job summary; not used for semver |
