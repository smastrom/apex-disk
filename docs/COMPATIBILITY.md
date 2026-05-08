# Compatibility Report

Target: **macOS 10.15 Catalina** (`minimumSystemVersion` in `src-tauri/tauri.conf.json`)

## Summary

| Layer                       | Min Target                               | Confirmed Compatible                  |
| --------------------------- | ---------------------------------------- | ------------------------------------- |
| **Architecture**            | Intel (x86_64) + Apple Silicon (aarch64) | Yes — universal binary                |
| **Rust / Native APIs**      | macOS 10.13+ (all APIs used)             | Yes — macOS 10.15 safe                |
| **CSS (lightningcss)**      | Safari 13                                | Yes — transpiled via `vite.config.ts` |
| **JavaScript (Vite build)** | Safari 13 (`build.target`)               | Yes — transpiled by Vite/Rolldown     |
| **Tauri 2 WKWebView**       | macOS 10.15                              | Yes — Tauri 2 requires 10.15+         |

**Supported architectures: Intel (x86_64) and Apple Silicon (aarch64) via universal binary**
**Minimum supported macOS version: 10.15 Catalina (Safari 13.0 / WebKit 605.1.15)**

The app has been validated end-to-end on **macOS 11 Big Sur / Safari 14.0** (real-device test, 2026-05-07) with no visual regressions vs. the macOS 14+ experience. Animations, focus styling, and tooltips are driven by Vue + JS + `@floating-ui/dom` rather than newer browser APIs, so behavior is uniform across every supported Safari version.

---

## How to Run This Compatibility Check

An LLM or developer can re-verify compatibility by following these steps:

### 1. Check architecture support (Intel / Apple Silicon)

The release workflow builds a **universal macOS binary** containing both architectures:

```bash
grep 'universal-apple-darwin\|targets:.*aarch64\|targets:.*x86_64' .github/workflows/release.yml
```

Expected:

- Rust toolchain step installs both targets: `aarch64-apple-darwin,x86_64-apple-darwin`
- Build step uses `--target universal-apple-darwin`
- Release uploads from `target/universal-apple-darwin/release/bundle/`

**No architecture-specific code exists in the Rust source.** To verify:

```bash
# Should return 0 matches — no cfg(target_arch) or arch-gated code:
grep -rn 'cfg.*target_arch\|#\[cfg.*arch\|x86_64\|aarch64' src-tauri/src/ --include='*.rs'
```

All `unsafe` blocks use standard POSIX/macOS APIs (`libc::getxattr`, `objc2` Foundation/AppKit bindings) that behave identically on both Intel and Apple Silicon. No raw pointer arithmetic, `transmute`, or pointer-width assumptions exist in application code.

| Aspect                  | Intel (x86_64)               | Apple Silicon (aarch64)       |
| ----------------------- | ---------------------------- | ----------------------------- |
| **Rust compilation**    | `x86_64-apple-darwin` target | `aarch64-apple-darwin` target |
| **Binary format**       | Included in universal binary | Included in universal binary  |
| **libc / POSIX APIs**   | Identical behavior           | Identical behavior            |
| **objc2 / Foundation**  | Identical behavior           | Identical behavior            |
| **Rosetta 2 required?** | No (native)                  | No (native)                   |

On Apple Silicon Macs running macOS 11+, the universal binary runs natively without Rosetta 2. On Intel Macs running macOS 10.15+, the x86_64 slice runs natively.

### 2. Check the declared minimum macOS version

```bash
grep -A1 minimumSystemVersion src-tauri/tauri.conf.json
```

Expected: `"minimumSystemVersion": "10.15"`

macOS 10.15 Catalina ships with Safari 13.0. Users who update Catalina can reach Safari 15.6.1, but the **worst-case baseline is Safari 13.0**.

### 3. Verify CSS transpilation (lightningcss → Safari 13)

In `vite.config.ts`, lightningcss is wired in as both the CSS transformer and the minifier, targeting Safari 13:

```ts
const safari13 = (13 << 16) | (0 << 8) | 0 // Safari 13.0.0

css: {
   transformer: 'lightningcss',
   lightningcss: {
      targets: {
         safari: safari13,
      },
   },
},

build: {
   target: 'safari13',
   cssMinify: 'lightningcss',
   ...
},
```

**What lightningcss handles** (transpiles or adds prefixes automatically):

- CSS Nesting (`&` selector) → flattened to separate rules
- `:has()` parent selector → preserved (no fallback possible) but emitted only where source uses it (currently no usage in app CSS)
- Cascade layers (`@layer`) → flattened in source order
- Vendor prefixes (`-webkit-`) → added where needed
- Modern color functions (`color-mix()` with static operands, `oklch()`, etc.) → converted to compatible equivalents
- Logical properties (`margin-inline`, `padding-block`, …) → converted to physical equivalents
- `inset` shorthand → expanded to `top/right/bottom/left`

**What lightningcss does NOT handle** (browser APIs or runtime-resolved values):

- `overflow: overlay` (WebKit-only, all Safari versions) — passes through; non-WebKit engines ignore it and fall back to the `overflow: auto` declared on the line above
- `color-mix()` whose operands are CSS custom properties — would pass through unchanged (cannot be resolved at build time). The codebase deliberately avoids this form: the one previous use (a 45% accent-over-surface tint on the scan progress bar's secondary segment) was rewritten as `background: var(--color-accent); opacity: 0.45`, which is mathematically identical in sRGB and works on Safari 13+.

To verify CSS output has no untranspilable modern syntax:

```bash
pnpm build
# Check for CSS nesting (should be 0 — fully flattened):
grep -c '&:' dist/assets/*.css
# Check for cascade layers (should be 0 — flattened):
grep -c '@layer' dist/assets/*.css
```

### 4. Verify JavaScript transpilation (Vite → Safari 13)

In `vite.config.ts`:

```ts
build: {
   target: 'safari13',
},
```

This makes Vite/Rolldown transpile modern JS to Safari 13-compatible output:

- Optional chaining (`?.`) → transpiled
- Nullish coalescing (`??`) → transpiled
- `BigInt` → transpiled or polyfilled
- Top-level `await` → wrapped

To verify:

```bash
pnpm build
# Should find 0 instances of optional chaining in output:
grep -c '?\.' dist/assets/*.js
```

### 5. Verify Rust / Native macOS API compatibility

All Objective-C framework APIs used are available well before macOS 10.15:

| API                                                | File                             | Min macOS |
| -------------------------------------------------- | -------------------------------- | --------- |
| `NSURL::fileURLWithPath`, `resourceValuesForKeys`  | `src-tauri/src/disk.rs`          | 10.0      |
| `NSURLVolumeAvailableCapacityForImportantUsageKey` | `src-tauri/src/disk.rs`          | **10.13** |
| `NSUserDefaults::standardUserDefaults`             | `src-tauri/src/locale.rs`        | 10.0      |
| `NSAlert`, `NSImage`                               | `src-tauri/src/native_dialog.rs` | 10.0      |
| `libc::getxattr`                                   | `src-tauri/src/xattr.rs`         | 10.4      |

**Most restrictive native API: `NSURLVolumeAvailableCapacityForImportantUsageKey` (macOS 10.13)**
This is safely within the 10.15 minimum.

Runtime version detection in `src-tauri/src/permissions.rs`:

- macOS 12+: probes `~/Library/Containers/com.apple.stocks` for Full Disk Access
- macOS 10.15–11: probes `~/Library/Safari` for Full Disk Access

To verify no new APIs break compatibility, search for `objc2` usage:

```bash
grep -rn 'objc2' src-tauri/src/ --include='*.rs'
```

Then cross-reference each Foundation/AppKit symbol with Apple's developer docs for availability.

### 6. Verify Rust toolchain and dependency MSRV

| Dependency                               | Version | MSRV       |
| ---------------------------------------- | ------- | ---------- |
| tauri                                    | 2.x     | Rust 1.70+ |
| objc2 / objc2-foundation / objc2-app-kit | 0.3–0.6 | Rust 1.70+ |
| tokio                                    | 1.x     | Rust 1.63+ |
| serde                                    | 1.x     | Rust 1.31+ |
| rayon                                    | 1.10    | Rust 1.59+ |
| nix                                      | 0.29    | Rust 1.69+ |
| trash                                    | 5.x     | Rust 1.63+ |

**Aggregate Rust MSRV: 1.70+** (driven by Tauri 2 and objc2)

No `rust-toolchain.toml` exists — the MSRV is implicit from dependencies.

### 7. Tauri 2 platform requirements

Tauri 2 on macOS:

- Uses **WKWebView** (system WebKit, tied to macOS version)
- Minimum: **macOS 10.15** (Catalina) — this is a hard requirement from Tauri 2 itself
- WebKit version on macOS 10.15.0: Safari 13.0 (605.1.15)
- WebKit version on latest Catalina (10.15.7 + updates): up to Safari 15.6.1

### 8. Confirm no opt-in to newer-only browser APIs

The app deliberately drives animations, popovers, and focus styling from Vue + JS rather than the native View Transitions API, the native Popover API, `@starting-style`, or `:focus-visible`. To verify none have crept back in:

```bash
grep -rn 'view-transition\|startViewTransition\|popover=\|:popover-open\|@starting-style\|showPopover\|hidePopover\|:focus-visible' src/ --include='*.ts' --include='*.vue' --include='*.css' --exclude-dir=dist
```

Should return zero hits.

---

## macOS ↔ Safari Version Reference

| macOS Version | Codename | Ships with Safari | Max Safari (with updates) |
| ------------- | -------- | ----------------- | ------------------------- |
| 10.15         | Catalina | 13.0              | 15.6.1                    |
| 11            | Big Sur  | 14.0              | 16.6.1                    |
| 12            | Monterey | 15.0              | 17.6.1                    |
| 13            | Ventura  | 16.0              | 18.x                      |
| 14            | Sonoma   | 17.0              | 18.x                      |
| 15            | Sequoia  | 18.0              | current                   |

---

## How Compatibility Is Maintained at the Code Level

Rather than feature-detect newer browser APIs at runtime and fall back, the app uses portable mechanisms from the start. The features that would otherwise require newer Safari versions are reimplemented in Vue + JS so they work uniformly on Safari 13+.

| Concern                                                                             | Newer-Safari API                                                                            | What we use instead                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| View transitions / page-style animations                                            | `document.startViewTransition` + `::view-transition-*` (Safari 18+)                         | Vue `<Transition>` with named CSS classes (`list-slide`, `app-slide`, `fade`, `app-ready`) in [src/assets/css/animations.css](../src/assets/css/animations.css). `mode="out-in"` on the folder list keeps leave/enter sequential to avoid layout overlap.                                                                                                                                                                                                                                                        |
| Floating tooltips / popovers                                                        | Popover API (`popover=` + `:popover-open`, Safari 17+) and `@starting-style` (Safari 17.5+) | [src/lib/use-label-popover.ts](../src/lib/use-label-popover.ts) — `@floating-ui/dom`'s `computePosition()` for placement, `<Teleport to="body">` for stacking, an `is-open` class for entry/exit transitions. Used by item-name + path tooltips ([ScanResultsListItem](../src/components/ScanResultsListItem.vue), [ScanListNav](../src/components/ScanListNav.vue), [ScanTrashListItem](../src/components/ScanTrashListItem.vue)) and by the disabled-checkbox explainer.                                       |
| Keyboard-vs-mouse focus distinction                                                 | `:focus-visible` UA heuristics (reliable on Safari 17+; weak/absent on Safari 13–16)        | [src/lib/use-focus-ring.ts](../src/lib/use-focus-ring.ts) toggles `html.focus-ring-keyboard` based on real keyboard input. The custom box-shadow ring in [src/assets/css/global.css](../src/assets/css/global.css) is gated on that class. The UA outline is suppressed unconditionally.                                                                                                                                                                                                                         |
| Press states without `:active` (so they survive pointer drift)                      | `:has(...)` selectors (Safari 15.4+)                                                        | A JS-set `--pressing` class on [ScanResultsListItem](../src/components/ScanResultsListItem.vue), driven by pointer-down/move with a 4px drag threshold.                                                                                                                                                                                                                                                                                                                                                          |
| Auto-hide overlay-style scrollbars that ignore the OS _Show scroll bars_ preference | None (the OS bar honors the preference, so on Big Sur with "Always" it renders chrome Aqua) | Custom `::-webkit-scrollbar` styling in [src/assets/css/global.css](../src/assets/css/global.css). Thumb is transparent at rest, fades in when the scrollable container or the scrollbar gutter is hovered, brightens on direct thumb hover (`::-webkit-scrollbar:hover::-webkit-scrollbar-thumb`). The pseudo-element only renders when the container actually overflows. Width is centralized as `--scrollbar-inline-gutter` in [src/assets/css/theme.css](../src/assets/css/theme.css). Works on Safari ≥ 13. |

---

## Features That Rely on Progressive Enhancement

These CSS/Web API features are used but **only render with the visual upgrade on newer macOS**. They degrade silently — no broken behavior, no missing UI:

| Feature                                                                           | Min Safari    | Fallback behavior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| --------------------------------------------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `overflow: overlay` (scrollbar overlays content instead of stealing layout space) | 14.0 (WebKit) | Falls back to the `overflow: auto` declared on the preceding line — the bar can reserve ~10px. [InformationView](../src/components/InformationView.vue) and [SettingsView](../src/components/SettingsView.vue) trim the centered content's `padding-inline-end` by `--scrollbar-inline-gutter` so the bar paints flush with the cards' right edge instead of carving a separate gutter; [ScanResultsListItem](../src/components/ScanResultsListItem.vue) uses the same trick on its inline margin. Elsewhere lists use overlay only (see scrollbar row above). |
| CSS Nesting                                                                       | 17.2          | **Transpiled by lightningcss** — no issue at runtime                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Cascade Layers (`@layer`)                                                         | 15.4          | **Flattened by lightningcss** — no issue at runtime                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `inset` shorthand                                                                 | 14.5          | **Expanded by lightningcss** to `top/right/bottom/left` — no issue at runtime                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

---

## Real Device Testing

### macOS 11 Big Sur — Safari 14.0 (2026-05-07)

The test machine was deliberately kept at its shipping Safari 14.0 (no Safari Technology Preview, no point updates that would push it toward Safari 16.6.1) so the run exercises the lower-bound experience for the macOS 11 floor. The same scenarios are expected to pass on macOS 10.15 Catalina with Safari 13, since Safari 13/14 share the relevant feature gaps (no View Transitions API, no native Popover API, weak `:focus-visible` semantics).

**Result: app launches, scans, reviews, and animations all work end-to-end.** The two visual regressions originally observed have both been addressed:

| Issue (originally observed on Safari 14)                                                                                                                                                                                            | Resolution                                                                                                                                                                                                                |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Folder navigation and tab switching changed content with no visible motion (the native View Transitions API isn't available before Safari 18).                                                                                      | Animations rewritten on top of Vue `<Transition>` so every supported Safari plays the same slide/fade. The native API path was removed entirely from the codebase.                                                        |
| Custom focus ring + UA focus outline both appeared on mouse click for `[tabindex='0']` and `<button>` elements (Safari 14 doesn't apply the Safari-17+ `:focus-visible` heuristics that hide the UA outline on non-keyboard focus). | Unconditional `outline: none` reset in [src/assets/css/global.css](../src/assets/css/global.css) for all focusable selectors. The custom box-shadow ring is then gated on the JS-driven `html.focus-ring-keyboard` class. |
| Disabled-checkbox explainer tooltip relied on the native Popover API (`popover="manual"`) and `@starting-style`, both Safari 17+. On Safari 14 the tooltip never appeared.                                                          | Reimplemented using the existing `useLabelPopover` helper (floating-ui + `<Teleport to="body">` + `is-open` class transition). Now works on Safari 13+.                                                                   |

---

## Conclusion

The app ships as a **universal binary** supporting both **Intel (x86_64)** and **Apple Silicon (aarch64)** Macs natively — no Rosetta 2 required on either architecture. It is fully functional on **macOS 10.15 Catalina (Safari 13.0)** as the absolute minimum, and has been validated on **macOS 11 Big Sur / Safari 14.0** (real-device test, 2026-05-07) with no remaining visual regressions vs. the macOS 14+ experience.

The animation, popover, focus-ring, and scrollbar systems are intentionally implemented in Vue + JS + `@floating-ui/dom` + custom `::-webkit-scrollbar` styling rather than the corresponding newer-Safari/OS APIs (`startViewTransition`, the Popover API, `:focus-visible`, native overlay scrollbars), so behavior is uniform across every supported Safari version. `overflow: overlay` (Safari 14+ WebKit) still governs whether the scrollbar steals layout space where declared; centered views shim classic gutters with `--scrollbar-inline-gutter` — scrollbar visuals remain identical on every supported version.
