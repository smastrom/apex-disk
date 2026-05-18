# Releases

Changelog for **stable** builds shipped via the GitHub Release workflow. Newest-first; the Release workflow reads the first `## vX.Y.Z` heading. See [`reference/releases.md`](reference/releases.md) for how to cut a release, and [`reference/updates.md`](reference/updates.md) for the in-app updater.

---

## v0.0.15-rc.1

Test pre-release for the per-architecture CI workflow. Marked as a GitHub pre-release so stable users on `v0.0.14` are not auto-updated to it.

### Improvements

- Ship per-architecture installers and updates: dedicated Apple Silicon (`aarch64`) and Intel (`x86_64`) DMGs alongside the existing universal one — each per-arch DMG is roughly 50% smaller than the universal. The in-app updater fetches only the slice matching the running Mac, so updates also halve in size. Existing universal-binary installs migrate to a per-arch binary automatically on next update.
- Re-export `apex-disk-hero.png` at a smaller file size.

### Chores

- Reorganize agent-facing documentation into a dedicated `reference/` directory and `.claude/rules/` always-loaded routing, with a repo-wide `.md` sweep + file-top comment check inside `/sync` and `/force-sync`. Move outcome-facing content (FAQ) into `marketing/`. Add `CLAUDE.md` as a one-line `@AGENTS.md` import. No user-visible effect.
- Split `/release` into auto (generates notes from git log) and curated (`/release-from-notes`, trusts hand-written notes) paths so existing release notes can't be accidentally overwritten.
- Expand `/compatibility-check`'s Web API audit to ~20 newer-Safari patterns (`view-transition`, `:has()`, `@container`, `structuredClone`, `Promise.any`, …) and wire it into the command checklist — was previously skipped entirely.

## v0.0.14

### Improvements

- Implement the macOS 11 UI normalization plan: replace the View Transitions API, the native Popover API, and reliance on `:focus-visible` UA heuristics with portable Vue `<Transition>` + `@floating-ui/dom` + JS-driven focus tracking, so folder/tab slide animations, the disabled-checkbox explainer tooltip, and the keyboard-only focus ring all work uniformly on every supported Safari (≥ 13) instead of just Safari 17/18+.
- Style scrollbars via custom `::-webkit-scrollbar` rules (transparent thumb that fades in on hover and brightens on direct thumb hover), bypassing the OS _Show scroll bars_ preference for a consistent look on macOS 11+.
- Drop runtime `color-mix()` in favor of `background + opacity` composition so the scan-progress bar's secondary segment renders correctly on Safari 13–16.1 as well.
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
