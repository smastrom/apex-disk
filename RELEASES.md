# Releases

Changelog for **stable** builds shipped via the GitHub **Release** workflow (DMG + signed updater bundle + `latest.json`). For Beta QA DMGs, see **`docs/UPDATES.md`** and **`RELEASES_BETA.md`** — they do **not** use this file for versioning.

## What to set for a stable release (vs Beta)

| Concept | What it is | Typical value / rule |
| -------- | ----------- | --------------------- |
| **`$ARGUMENTS` in `.claude/commands/release.md`** | Semantic **version string** only (no `v`) | e.g. `0.0.12` — bump in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` |
| **Heading in this file** | Must match: `## v` + same semver | e.g. `## v0.0.12` |
| **Git tag** (created by CI) | `v` + semver | e.g. `v0.0.12` |
| **Bundle `identifier`** (`tauri.conf.json`) | macOS app bundle ID — **not** the release version | Stable app: `com.smastrom.apex-disk`. **Leave unchanged** for normal releases. |
| **Beta bundle ID** | Separate app so Beta can sit beside stable | `com.smastrom.apex-disk.beta` in `tauri.beta.conf.json` only — no change when cutting stable. |

**Order matters (newest-first):** the Release workflow parses **`RELEASES.md` with `grep '^## v' | head -1`**, so the **first** `## vX.Y.Z` heading in this file (directly below the horizontal rule) is the version that must match the three project files. Always **prepend** a new section at the **top**, never at the bottom.

**Beta:** **Actions → Beta → Run workflow** only (no push). Builds the branch you select; publishes a **pre-release** (`beta-<run_id>`) with the DMG. Uses `tauri.beta.conf.json` (different `productName` / `identifier`, no updater artifacts). It does **not** read this file for semver; several Beta builds in a row can show the same app version until you merge a bump.

---

## v0.0.11

- Unified Vue ↔ Rust diagnostic logging (`docs/LOGGING.md`, `APEX_DISK_DEBUG`, scan/disk live traces).
- Rust hardening: store validation and locking, `safe_folders` precompute, faster `system_info`, scan/trash/updater fixes, `tauri` test feature in dev-deps only.
- Nightly (now renamed **Beta**): manual workflow publishes a GitHub pre-release DMG; `docs/UPDATES.md`, `RELEASES_BETA.md`, `src-tauri/README.md`, local `tauri:build:beta`.

## v0.0.10

Testing update system.

## v0.0.9

Testing the release.

## v0.0.8

Initial release.
