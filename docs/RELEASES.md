# Releases

How to cut **stable** and **Beta** builds of ApexDisk. For the in-app update behavior (auto vs manual, signing, dialogs), see `UPDATES.md`.

There are two changelog files at the repo root, one per channel:

| | `../RELEASES.md` | `../RELEASES_BETA.md` |
| --- | --- | --- |
| **Contents** | One `## vX.Y.Z` section per shipped stable version, newest-first. | One `## YYYY-MM-DD` section per Beta run you want to annotate, newest-first. |
| **Used by CI for version?** | **Yes** — the Release workflow reads the **first** `## vX.Y.Z` heading (`grep '^## v' | head -1`) and fails if `package.json`, `tauri.conf.json`, and `Cargo.toml` don't match. | **No** — the Beta workflow does not touch semver; it tags pre-releases as `beta-<run_id>`. |
| **Used by CI at all?** | Drives the Git tag, GitHub-Release body, and build. | The **first** `## …` section becomes the pre-release body and the job summary; the full file is attached as an artifact. |
| **Purpose** | Canonical shipping history + semver for the updater and installers. | Optional tester-facing notes (what to smoke-test, known issues) for a given branch snapshot. |

So `../RELEASES_BETA.md` is **not** a duplicate of `../RELEASES.md`: it does not replace version bookkeeping because Beta builds reuse the repo's current semver (see [Version nomenclature](#version-nomenclature) below).

## Stable release

Use the `/release` slash command or follow these steps by hand:

1. Pick a semver (no `v` prefix), e.g. `0.0.13`. Update it in all three files (they must match exactly):
   - `package.json` → `"version"`
   - `src-tauri/Cargo.toml` → `version`
   - `src-tauri/tauri.conf.json` → `"version"`
2. **Prepend** a new `## v0.0.13` section at the top of `../RELEASES.md` (directly under the `---` rule). The Release workflow reads the **first** `## vX.Y.Z` heading, so appending at the bottom would make CI read the wrong version. Group the bullets under `###` subheadings — see [Authoring conventions](#authoring-conventions).
3. Commit and push to `main`.
4. Go to **Actions → Release → Run workflow**, leave "Mark as pre-release" **unchecked**.
5. CI builds, signs, notarizes, tags `v0.0.13`, and creates a GitHub Release with:
   - `.dmg` — the installer for new users
   - `.tar.gz` + `.tar.gz.sig` — the signed update bundle
   - `latest.json` — the update manifest pointing to the `.tar.gz`

Users on older stable versions pick it up on next app start (auto-updates ON) or when they check manually.

### Semver pre-release (e.g. `-beta.1`, `-rc.1`)

A **Release-workflow** run whose GitHub release is flagged as a pre-release, so stable users don't auto-update to it. Distinct from the [Beta channel](#beta-channel) below.

1. Set the version to something like `0.10.0-beta.1` in all three files + `../RELEASES.md`.
2. Commit and push.
3. **Actions → Release → Run workflow**, **check** "Mark as pre-release".
4. CI produces the same artifacts and creates a GitHub **pre-release**.

Stable users are unaffected: the updater fetches `https://github.com/.../releases/latest/download/latest.json`, and `/releases/latest/` resolves to the newest non-pre-release. To test the build, download the `.dmg` directly from the pre-release page.

## Beta channel

The Beta channel is **QA-only**: dispatch-triggered, no updater, sits beside the stable app.

1. (Optional) Add a dated section to `../RELEASES_BETA.md` — use the `/beta-notes` slash command or add a `## YYYY-MM-DD` section at the top with tester notes. The **first** such section becomes the pre-release body.
2. Go to **Actions → Beta → Run workflow** and pick the **branch** to build (e.g. `main`, a feature branch).
3. When the job finishes, open the **pre-release** on the Releases page (tag `beta-<run_id>`) and download the DMG, or grab the `ApexDisk-Beta-<run_id>` artifact.

Config (`src-tauri/tauri.beta.conf.json`): product name **ApexDisk Beta**, bundle id `com.smastrom.apex-disk.beta`, and `bundle.createUpdaterArtifacts: false`. This lets Beta install side-by-side with stable and keeps it out of the updater channel.

**Local build:** `pnpm tauri:build:beta` (same signing env as a stable build).

## Version nomenclature

**Stable + semver pre-releases** — one semver (e.g. `0.0.13`, `0.10.0-beta.1`) lives in `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, and the newest `## v…` block in `../RELEASES.md`. That number appears in the app, the Git tag (`v0.0.13`), and `latest.json`.

**Beta channel** — the DMG reports whatever semver is on the selected branch at dispatch time. The Beta workflow does not bump or validate the version.

Implications:

- Several Beta builds can share the same version (e.g. three builds while `main` is still `0.0.13`). Beta is a branch **snapshot**, not a new release line.
- Tell builds apart by **run id** (in the artifact name), **commit SHA**, or download date — not the About-box semver.
- Optional policy: bump `../RELEASES.md` / semver only when cutting a real Release, or bump right after each release so Beta builds report the *upcoming* version early. Either way, still one source of truth — no `RELEASES_BETA` version.

The project does **not** embed build metadata (e.g. `0.0.13+beta.abc1234`) — adding that would require mutating the three version fields on every Beta run.

## Authoring conventions

Both files are **newest-first**. Always prepend new sections at the top, directly below the `---` rule.

- `../RELEASES.md`: `## vX.Y.Z` heading summarizing changes since the previous tag (`git log <prev-tag>..HEAD`), with bullets grouped under these `###` subheadings in this order:
  - **New Features** — user-visible additions.
  - **Improvements** — enhancements to existing behavior, UX polish, perf wins, refactors with observable effects.
  - **Bug Fixes** — defect fixes.
  - **Chores** — internal housekeeping with no user-visible effect (deps bumps, CI, docs, dead-code removal, test-only changes).

  Omit any group that has no entries (no empty `###` headings). One bullet per change. Classify by the dominant effect of the commit, not the commit-message prefix. Older entries pre-date this convention and are not retroactively rewritten.
- `../RELEASES_BETA.md`: `## YYYY-MM-DD` (UTC) heading, a one-line summary, then bullets covering UI flows that changed, updater/menu/store touches, and macOS-version-sensitive behavior.

Never edit older sections to retcon history.
