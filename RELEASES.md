# Releases

Changelog for **stable** builds shipped via the GitHub **Release** workflow (DMG + signed updater bundle + `latest.json`). For nightly QA DMGs, see **`UPDATES.md`** and **`RELEASES_NIGHTLY.md`** — they do **not** use this file for versioning.

## What to set for a stable release (vs nightly)

| Concept | What it is | Typical value / rule |
| -------- | ----------- | --------------------- |
| **`$ARGUMENTS` in `.claude/commands/release.md`** | Semantic **version string** only (no `v`) | e.g. `0.0.11` — bump in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` |
| **Heading in this file** | Must match: `## v` + same semver | e.g. `## v0.0.11` |
| **Git tag** (created by CI) | `v` + semver | e.g. `v0.0.11` |
| **Bundle `identifier`** (`tauri.conf.json`) | macOS app bundle ID — **not** the release version | Stable app: `com.smastrom.apex-disk`. **Leave unchanged** for normal releases. |
| **Nightly bundle ID** | Separate app so nightly can sit beside stable | `com.smastrom.apex-disk.nightly` in `tauri.nightly.conf.json` only — no change when cutting stable. |

**Order matters:** the Release workflow parses **`RELEASES.md` with `grep '^## v' | tail -1`**, so the **last** `## vX.Y.Z` in this file is the version that must match the three project files. Always **append** a new section at the **bottom**, never at the top.

**Nightly:** **Actions → Nightly → Run workflow** only (no push). Builds the branch you select; publishes a **pre-release** (`nightly-<run_id>`) with the DMG. Uses `tauri.nightly.conf.json` (different `productName` / `identifier`, no updater artifacts). It does **not** read this file for semver; several nightlies in a row can show the same app version until you merge a bump.

---

## v0.0.8

Initial release.

## v0.0.9

Testing the release.

## v0.0.10

Testing update system.

## v0.0.11

- Unified Vue ↔ Rust diagnostic logging (`LOGGING.md`, `APEX_DISK_DEBUG`, scan/disk live traces).
- Rust hardening: store validation and locking, `safe_folders` precompute, faster `system_info`, scan/trash/updater fixes, `tauri` test feature in dev-deps only.
- Nightly: manual workflow publishes a GitHub pre-release DMG; `UPDATES.md`, `RELEASES_NIGHTLY.md`, `src-tauri/README.md`, local `tauri:build:nightly`.
