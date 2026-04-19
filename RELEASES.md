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

## v0.0.12

- Rename the manual QA release channel from **Nightly** to **Beta** end-to-end: workflow (`beta.yml`, tag prefix `beta-<run_id>`), Tauri config (`tauri.beta.conf.json`, bundle id `com.smastrom.apex-disk.beta`, product name "ApexDisk Beta"), `pnpm tauri:build:beta`, and the companion `RELEASES_BETA.md`. The slash command is now `/beta-notes`.
- Reorganize project docs: `COMPATIBILITY.md`, `LOGGING.md`, `UPDATES.md` move to `docs/` (agent-facing reference material); user-/CI-facing files stay at root. `AGENTS.md` grows a "check `docs/` before every commit" rule.
- Flip `RELEASES.md` to **newest-first**: the Release workflow now reads the first `## v` heading (`head -1`) so new entries go at the top; the Release pre-release input is retitled to disambiguate semver pre-releases from the Beta channel.
- Refresh `docs/COMPATIBILITY.md` for the current frontend: label popovers use `@floating-ui/dom` + `<Teleport>` (Safari 13+); only the result-row selection-count popover still uses the native Popover API. Document the `overflow: overlay` / `auto` fallback used for native macOS overlay scrollbars.
- Frontend polish: switch from the custom `::-webkit-scrollbar` to native macOS overlay scrollbars across the four scroll containers; swap label popovers to Teleport + Floating UI (escapes ancestor overflow clipping); tune view transitions (`--ease-apple`, 320 ms / 40 px drill-down, 260 ms / 20 px tab switch) and pin the results footer above the sliding list with its own `view-transition-name`; shrink default window to 420×800.
- Theming: restore **Apex** as the single root theme after a brief Core-as-default experiment; soften selected-row tints in both base themes; round the selection checkbox focus ring.
- CSS build: compile through **lightningcss** (targets Safari 13) for both `css.transformer` and `cssMinify`, so modern CSS downlevels to flat Safari 13 syntax matching the declared minimum.
- Rust cleanup: refresh stale module docs across `src-tauri/src/`, drop dead code (collapsed `to_supported_language`, merged `resolve_app_language_inner`, removed a one-shot `is_fda_required` local, etc.), and silence a dead-code warning on a shared test helper (`create_test_home_with_system_files`).
- Other: CodeRabbit config adapted to the project structure; GitHub Actions bumped to Node-24-compatible majors; comments policy added to `AGENTS.md`; scan-row navigation made drag-safe via JS-driven press animation.

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
