# Releases

Changelog for **stable** builds shipped via the GitHub Release workflow. Newest-first; the Release workflow reads the first `## vX.Y.Z` heading. See [`reference/releases.md`](reference/releases.md) for how to cut a release, and [`reference/updates.md`](reference/updates.md) for the in-app updater.

---

## v0.0.18

### Improvements

#### Frontend

- Animate the outer view switcher with a state-driven slide and keep both views mounted via `v-show` so the active view no longer tears down between transitions.
- Defer the fresh-scan progress dot until the background scan actually completes, and harden the `KeepAlive` wrapper so cached views don't unmount mid-flight.
- Give WebKit scrollbars a JS-owned visibility policy so fade-in and fade-out follow the active surface.

#### Backend

- Tighten Tauri sandboxing: enable a strict CSP in `tauri.conf.json`, drop unused Hardened Runtime entitlements (`allow-unsigned-executable-memory`, `disable-library-validation`), and remove the unused `opener:allow-open-path` capability.
- Run native macOS dialogs through an async channel so a modal no longer pins an IPC worker for the lifetime of the prompt.
- Trim hot-path allocations on every scan: ASCII fast path in safe-folder lookup, cached xattr attribute name, top-N heap key that never compares filenames, and a lazy-format helper for debug traces.
- Route the disk-usage command through `tauri::async_runtime` instead of calling `tokio` directly, and trace `diskutil` failures.

### Bug Fixes

- Scan cancellation can no longer leave the app stuck. An RAII guard releases `SCAN_RUNNING` even on panic or join error, and each scan carries its own cancel token so a stale `cancel_scan` between scans is a no-op.
- `set_settings` now whitelists keys, drops unknown ones, merges over the existing settings instead of replacing them, and runs under the same `STORE_LOCK` as `update_setting` so concurrent writes can't race.
- The updater no longer fetches the feed twice before staging a download (so the staged version always matches the UI), beta and dev builds skip checks by bundle identifier so a release-mode beta can't advertise stable updates, and `restart_app` refuses to restart unless an update is actually staged.
- Canonicalize the home directory before the trash safety filter, so items reached through a symlinked home are no longer silently dropped.
- Switch `AppleLanguages` identifiers to BCP-47 tags (`it-IT`, `zh-Hans`, …) so macOS context menus localize consistently across versions.
- The Full Disk Access probe now distinguishes `PermissionDenied` (truly missing) from `NotFound` (inconclusive), and falls back to a Safari probe when the primary probe path doesn't exist.

### Chores

- Drop the `nix`, `tauri-plugin-os`, `@tauri-apps/plugin-os`, and direct `open` dependencies; route external URL opening through `tauri-plugin-opener`'s Rust API. Bump `dirs` to 6 and `sys-locale` to 0.3 to collapse duplicate versions, move `tokio` to dev-dependencies, and pin `tauri-plugin-webdriver` to `=0.2.1`.
- Sync the `reference/` docs (architecture, scanning, tauri-commands, updates, compatibility) with the new scan-cancel model, the tightened settings flow, the beta gating, the reduced entitlements, and the refreshed MSRV list.
- Ignore `reference/temp/` for git and the formatter so working notes don't clutter the tree.

## v0.0.17

### New Features

- Add the **Apex Light** theme as the default light option, plus a contrast pass on the existing light theme.
- Show the GPL-3.0 license in the footer and the About dialog.
- Add a **Scan .DS_Store** toggle in Settings, gated behind **Show Hidden Files**.
- Split the auto-update toggle into separate **Check** and **Install** controls, so update prompts can be shown without applying automatically.
- Show a scan-progress dot on the footer Scan tab. Yellow while scanning, green when fresh results are waiting.
- Raise the per-folder file cap from 100 to 300 and surface a truncation notice when a folder still exceeds it.

### Improvements

- Replace the Suspense-based system bootstrap with a synchronous `useSystem` composable, so the loading screen no longer flashes on slow setups and the lifecycle hooks inside `App.vue` bind before any async work resolves.
- Anchor the disabled-checkbox explainer tooltip directly to its icon and keep it interactive on hover.
- Scope `user-select: text` to the surfaces that need it (tooltips, system info, scan result names).
- Reserve a scrollbar gutter on the scan results list so the layout no longer shifts when overflow appears.
- Skip the inner fade across `KeepAlive` reactivation in ScanView so returning to the Scan tab keeps cached content stable.
- Show the top shadow overlay on the scan results list only once it has scrolled past its top.
- Gate the list-slide view transition to manual navigation only, skipping the slide on programmatic view changes.
- Shorten "Copy to clipboard" to "Copy" across all 10 languages.
- Soften the GradientButton corner radius from full-round to large.

### Bug Fixes

- Theme the AppLoadingScreen with CSS tokens so it no longer flashes a dark background on light themes.
- Constrain long folder names in the scan results to their container width so they no longer push sibling controls off-screen.

### Chores

- Adopt nightly rustfmt with import grouping and comment wrapping; apply across `src-tauri/`.
- Demote Oxlint to a formatter adjunct (import order, statement padding, assignment/call padding); apply across `src/` and `e2e/` and wire it into the `/sync` and `/force-sync` verify gates.
- Enforce `/sync` via a PreToolUse pre-commit gate hook that refuses agent-initiated `git commit` / `git push` outside `/sync` or `/force-sync`.
- Include `.coderabbit.yaml` in the `/sync` docs sweep.
- Drop narration comments across the Rust backend and codify the no-narration rule in `AGENTS.md` and `reference/code-style.md`.
- Always nest `@media` and `prefers-reduced-motion` rules inside their selector; promote CSS style rules into their own section.
- Add tests for the per-folder file cap and the truncation notice; document the truncation contract in the architecture reference.
- Add a voice guide and em-dash prose rules for user-facing docs; sweep em-dash interrupts and en-dashes across the repo.

## v0.0.16

### Improvements

- Ship per-architecture installers and updates: dedicated Apple Silicon (`aarch64`) and Intel (`x86_64`) DMGs alongside the existing universal one. Each per-arch DMG is roughly 50% smaller than the universal. The in-app updater fetches only the slice matching the running Mac, so updates also halve in size. Existing universal-binary installs migrate to a per-arch binary automatically on next update.
- Re-export `apex-disk-hero.png` at a smaller file size.

### Bug Fixes

- Rename per-architecture update bundles to architecture-qualified filenames before upload so the in-app updater's signature verification succeeds on both Apple Silicon and Intel.

### Chores

- Reorganize agent-facing documentation into a dedicated `reference/` directory and `.claude/rules/` always-loaded routing, with a repo-wide `.md` sweep + file-top comment check inside `/sync` and `/force-sync`. Move outcome-facing content (FAQ) into `marketing/`. Add `CLAUDE.md` as a one-line `@AGENTS.md` import.
- Split `/release` into auto (generates notes from git log) and curated (`/release-from-notes`, trusts hand-written notes) paths so existing release notes can't be accidentally overwritten.
- Expand `/compatibility-check`'s Web API audit to ~20 newer-Safari patterns (`view-transition`, `:has()`, `@container`, `structuredClone`, `Promise.any`, …) and wire it into the command checklist; was previously skipped entirely.

## v0.0.15

### Improvements

- Ship per-architecture installers and updates: dedicated Apple Silicon (`aarch64`) and Intel (`x86_64`) DMGs alongside the existing universal one. Each per-arch DMG is roughly 50% smaller than the universal. The in-app updater fetches only the slice matching the running Mac, so updates also halve in size. Existing universal-binary installs migrate to a per-arch binary automatically on next update.
- Re-export `apex-disk-hero.png` at a smaller file size.

### Chores

- Reorganize agent-facing documentation into a dedicated `reference/` directory and `.claude/rules/` always-loaded routing, with a repo-wide `.md` sweep + file-top comment check inside `/sync` and `/force-sync`. Move outcome-facing content (FAQ) into `marketing/`. Add `CLAUDE.md` as a one-line `@AGENTS.md` import. No user-visible effect.
- Split `/release` into auto (generates notes from git log) and curated (`/release-from-notes`, trusts hand-written notes) paths so existing release notes can't be accidentally overwritten.
- Expand `/compatibility-check`'s Web API audit to ~20 newer-Safari patterns (`view-transition`, `:has()`, `@container`, `structuredClone`, `Promise.any`, …) and wire it into the command checklist; was previously skipped entirely.

## v0.0.14

### Improvements

- Implement the macOS 11 UI normalization plan: replace the View Transitions API, the native Popover API, and reliance on `:focus-visible` UA heuristics with portable Vue `<Transition>` + `@floating-ui/dom` + JS-driven focus tracking, so folder/tab slide animations, the disabled-checkbox explainer tooltip, and the keyboard-only focus ring all work uniformly on every supported Safari (≥ 13) instead of just Safari 17/18+.
- Style scrollbars via custom `::-webkit-scrollbar` rules (transparent thumb that fades in on hover and brightens on direct thumb hover), bypassing the OS _Show scroll bars_ preference for a consistent look on macOS 11+.
- Drop runtime `color-mix()` in favor of `background + opacity` composition so the scan-progress bar's secondary segment renders correctly on Safari 13 to 16.1 as well.
- Adjust the default window size to 425×785.

### Chores

- Document the macOS 11 real-device test, the executed normalization plan, and the codebase-level workarounds for newer-Safari APIs in `docs/COMPATIBILITY.md`.
- Add product FAQs and tighten the README tagline.
- Swap `tsx` for Bun in external scripts and document the convention (`Bun.YAML.parse`, `Bun.file`, `Bun.Glob`; `tsdown` for `.d.ts`).
- Migrate translations from TypeScript modules to YAML.
- Apply SPDX `GPL-3.0-or-later` + copyright headers to all first-party source; add the `pnpm headers` tooling and require it before `/sync` commits; preserve source file mode in `add-license-headers.sh`.
- Stop ignoring `pnpm-lock.yaml` so installs are reproducible.
- Add `SECURITY.md`, a contributor PR template, and `CODEOWNERS`.
- Populate `src-tauri/Cargo.toml` with publication metadata.
- Consolidate Rust backend reference content into `docs/ARCHITECTURE.md`.

## v0.0.13

### Improvements

- Rework `GradientButton` shadows so the rest→hover transition interpolates cleanly (matching layer counts) and the drop/glow/inset are theme tokens. macOS Light softens the drops and strengthens the hover glow so it's visible on a light surface.

### Chores

- Refresh the README hero image.
- Consolidate release guidance into `docs/RELEASES.md`; slim root `RELEASES.md` and `RELEASES_BETA.md` to pointers, and scope `docs/UPDATES.md` to updater behavior only.
- Add `docs/ARCHITECTURE.md` documenting the Rust/webview boundary: per-side responsibilities, IPC, subsystem walkthroughs, and directory map.
- Require stable `RELEASES.md` entries to be grouped under `### New Features / Improvements / Bug Fixes / Chores`; `/release` and the authoring conventions in `docs/RELEASES.md` now enforce the format.
- Strengthen the `AGENTS.md` pre-commit rule: analyze **every** file under `docs/` for update eligibility, with an expanded trigger list (renames, boundary shifts, workflow edits).
- Trigger a Cloudflare Pages rebuild from the Release workflow via the `CF_PAGES_DEPLOY_HOOK_URL` secret so the marketing site re-publishes against each new tag.
- Add the `/sync` slash command for splitting in-flight work into logical commits, sweeping `docs/`, and pushing.
- Cover the core flows with e2e specs (`selection-checkbox`, `trash-review`, `settings-flow`) on top of the existing Rust-side `e2e` fixture; add shared WDIO helpers in `e2e/helpers/navigation.ts` for checkbox state assertions, trash list traversal, settings toggles, and byte parsing from the review button.
- Run the e2e workflow as a matrix across `macos-13`, `macos-14`, and `macos-15` with `fail-fast: false`; older hosted runners have been retired by GitHub, so the 10.15 `minimumSystemVersion` stays enforced via the static compatibility check rather than a runner.

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
