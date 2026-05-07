# Compatibility Report

Target: **macOS 10.15 Catalina** (`minimumSystemVersion` in `src-tauri/tauri.conf.json`)

## Summary

| Layer | Min Target | Confirmed Compatible |
|-------|-----------|---------------------|
| **Architecture** | Intel (x86_64) + Apple Silicon (aarch64) | Yes — universal binary |
| **Rust / Native APIs** | macOS 10.13+ (all APIs used) | Yes — macOS 10.15 safe |
| **CSS (lightningcss)** | Safari 13 | Yes — transpiled via `vite.config.ts` |
| **JavaScript (Vite build)** | Safari 13 (`build.target`) | Yes — transpiled by Vite/Rolldown |
| **Tauri 2 WKWebView** | macOS 10.15 | Yes — Tauri 2 requires 10.15+ |

**Supported architectures: Intel (x86_64) and Apple Silicon (aarch64) via universal binary**
**Minimum supported macOS version: 10.15 Catalina (Safari 13.0 / WebKit 605.1.15)**

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

| Aspect | Intel (x86_64) | Apple Silicon (aarch64) |
|--------|---------------|------------------------|
| **Rust compilation** | `x86_64-apple-darwin` target | `aarch64-apple-darwin` target |
| **Binary format** | Included in universal binary | Included in universal binary |
| **libc / POSIX APIs** | Identical behavior | Identical behavior |
| **objc2 / Foundation** | Identical behavior | Identical behavior |
| **Rosetta 2 required?** | No (native) | No (native) |

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
- `:has()` parent selector → preserved (no fallback possible) but emitted only where source uses it
- Cascade layers (`@layer`) → flattened in source order
- Vendor prefixes (`-webkit-`) → added where needed
- Modern color functions (`color-mix()` with static operands, `oklch()`, etc.) → converted to compatible equivalents
- Logical properties (`margin-inline`, `padding-block`, …) → converted to physical equivalents

**What lightningcss does NOT handle** (browser APIs or runtime-resolved values):
- `:popover-open` pseudo-class (Safari 17+) — browser API, not transpilable
- `@starting-style` (Safari 17.5+) — not transpilable
- `::view-transition-*` pseudo-elements (Safari 18+) — not transpilable
- `overflow: overlay` (WebKit-only, all Safari versions) — passes through; non-WebKit engines ignore it and fall back to the `overflow: auto` declared on the line above
- `color-mix()` whose operands are CSS custom properties — passes through unchanged (cannot be resolved at build time)

These features degrade gracefully: `::view-transition-*` rules are wrapped in `@supports (view-transition-name: none)` so older WebKit simply doesn't animate; `:popover-open` + `@starting-style` are currently used by exactly one element — the disabled-checkbox explanatory tooltip in `ScanResultsListItem` (see *Features That Rely on Progressive Enhancement* below) — and on older Safari that tooltip simply does not appear. **All other popovers (item-name and path tooltips in `ScanResultsListItem`, `ScanListNav`, `ScanTrashListItem`) already use `@floating-ui/dom` with `<Teleport to="body">` via [src/lib/use-label-popover.ts](../src/lib/use-label-popover.ts), so they work on Safari 13+.** Fix 3 in *UI Normalization Plan* migrates the last native-Popover holdout to the same helper, dropping the Popover-API dependency entirely.

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

| API | File | Min macOS |
|-----|------|-----------|
| `NSURL::fileURLWithPath`, `resourceValuesForKeys` | `src-tauri/src/disk.rs` | 10.0 |
| `NSURLVolumeAvailableCapacityForImportantUsageKey` | `src-tauri/src/disk.rs` | **10.13** |
| `NSUserDefaults::standardUserDefaults` | `src-tauri/src/locale.rs` | 10.0 |
| `NSAlert`, `NSImage` | `src-tauri/src/native_dialog.rs` | 10.0 |
| `libc::getxattr` | `src-tauri/src/xattr.rs` | 10.4 |

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

| Dependency | Version | MSRV |
|------------|---------|------|
| tauri | 2.x | Rust 1.70+ |
| objc2 / objc2-foundation / objc2-app-kit | 0.3–0.6 | Rust 1.70+ |
| tokio | 1.x | Rust 1.63+ |
| serde | 1.x | Rust 1.31+ |
| rayon | 1.10 | Rust 1.59+ |
| nix | 0.29 | Rust 1.69+ |
| trash | 5.x | Rust 1.63+ |

**Aggregate Rust MSRV: 1.70+** (driven by Tauri 2 and objc2)

No `rust-toolchain.toml` exists — the MSRV is implicit from dependencies.

### 7. Tauri 2 platform requirements

Tauri 2 on macOS:
- Uses **WKWebView** (system WebKit, tied to macOS version)
- Minimum: **macOS 10.15** (Catalina) — this is a hard requirement from Tauri 2 itself
- WebKit version on macOS 10.15.0: Safari 13.0 (605.1.15)
- WebKit version on latest Catalina (10.15.7 + updates): up to Safari 15.6.1

---

## macOS ↔ Safari Version Reference

| macOS Version | Codename | Ships with Safari | Max Safari (with updates) |
|---------------|----------|-------------------|---------------------------|
| 10.15 | Catalina | 13.0 | 15.6.1 |
| 11 | Big Sur | 14.0 | 16.6.1 |
| 12 | Monterey | 15.0 | 17.6.1 |
| 13 | Ventura | 16.0 | 18.x |
| 14 | Sonoma | 17.0 | 18.x |
| 15 | Sequoia | 18.0 | current |

---

## Features That Rely on Progressive Enhancement

These CSS/Web API features are used but **only work on newer macOS** (they degrade gracefully):

| Feature | Min Safari | Fallback behavior |
|---------|-----------|-------------------|
| Popover API (`popover=` + `:popover-open`) | 17.0 | **Last remaining native-Popover element**: the disabled-checkbox explainer tooltip in [ScanResultsListItem.vue:279-290](../src/components/ScanResultsListItem.vue#L279-L290) (shown when a checkbox is non-toggleable because the item is system-protected or requires Full Disk Access). On Safari < 17 the tooltip never appears — users see a disabled checkbox with no explanation. **Fix 3 below** migrates this to the existing `useLabelPopover` helper so it works on Safari 13+ and the native Popover API dependency goes away entirely. All other popovers (item-name + path tooltips on result/trash rows and in `ScanListNav`) already use `@floating-ui/dom` + `<Teleport to="body">` and work on Safari 13+. |
| `@starting-style` | 17.5 | Used only for the entry animation of the same disabled-checkbox tooltip above. Eliminated by Fix 3 (the floating-ui-based helper drives entry animation via an `is-open` class transition that works on Safari 13+). |
| View Transitions API (`::view-transition-*`) | 18.0 | All rules wrapped in `@supports (view-transition-name: none)`. **Until Fix 1 below ships, navigation cuts instantly on Safari < 18** (confirmed on macOS 11 / Safari 14, see *Real Device Testing*). After Fix 1: Vue `<Transition>` provides a slide/fade fallback. |
| `overflow: overlay` (native macOS auto-hide scrollbars) | 14.0 (WebKit) | Non-WebKit / older WebKit falls back to the `overflow: auto` declared on the preceding line — classic always-visible scrollbars |
| `color-mix()` with custom-property operands | 16.2 | Passes through unchanged — renderer falls back to the declared base color |
| CSS Nesting | 17.2 | **Transpiled by lightningcss** — no issue |
| Cascade Layers (`@layer`) | 15.4 | **Flattened by lightningcss** — no issue |

---

## Real Device Testing

### macOS 11 Big Sur — Safari 14.0 (2026-05-07)

The test machine was deliberately kept at its shipping Safari 14.0 (no Safari Technology Preview, no point updates that would push it toward Safari 16.6.1) so the run exercises the lower-bound experience for the macOS 11 floor. The same scenarios are expected to pass on macOS 10.15 Catalina with Safari 13, since Safari 13/14 share the relevant feature gaps (no View Transitions API, no native `:focus-visible` UA semantics).

**Result: app launches, scans, and reviews work end-to-end.** Two visual regressions vs. the macOS 14+ experience were observed:

| # | Issue | Root cause |
|---|-------|------------|
| 1 | View transitions degrade to instant DOM swaps. Folder navigation in `ScanResultsList` (goInto/goBack/goForward) and tab switching between Scan/Settings/Information change content with no visible motion. | `document.startViewTransition` ships in Safari 18.0+, so the `VIEW_TRANSITION_SUPPORTED` check in [src/lib/use-view-transition.ts](../src/lib/use-view-transition.ts) takes its else branch and applies the update immediately. The `@supports (view-transition-name: none)` keyframe blocks in [src/assets/css/animations.css](../src/assets/css/animations.css) never engage. There is no non-API fallback animation. |
| 2 | Focus ring appears on mouse click for `ScanResultsListItem` rows and other `[tabindex='0']` / `<button>` elements. Expected behavior (verified on macOS 14 / Safari 17+ on an M1 MacBook Air): ring appears only after keyboard navigation. | The keyboard-detection JS in [src/lib/use-focus-ring.ts](../src/lib/use-focus-ring.ts) toggles `html.focus-ring-keyboard` correctly on Safari 14. However, the only `outline: none` reset in [src/assets/css/global.css:49-58](../src/assets/css/global.css#L49-L58) is *inside* the `html.focus-ring-keyboard …:focus` selector, so when the class is absent (after a mouse click) nothing suppresses Safari's native outline. Safari 17+ hides the UA outline for non-keyboard focus via its built-in `:focus-visible` heuristics, which masks the missing reset on newer macOS. |

Neither issue blocks core functionality — both are visual polish.

---

## UI Normalization Plan

### Fix 1 — View transition fallback animation

**Goal:** when `document.startViewTransition` is unavailable, deliver a comparable slide/fade animation using Vue's `<Transition>` component instead of an instant DOM swap.

**Approach:**
- Keep `useViewTransition()` as-is for the View Transitions API path (Safari 18+).
- Export `VIEW_TRANSITION_SUPPORTED` from [src/lib/use-view-transition.ts](../src/lib/use-view-transition.ts) so templates can branch.
- For app-view tab switching in [src/components/App.vue:70-94](../src/components/App.vue#L70-L94), wrap the `<KeepAlive>` content in `<Transition mode="out-in" name="vt-fallback-app">`. Bind `:css="!VIEW_TRANSITION_SUPPORTED"` so Vue's CSS hooks no-op on browsers where the View Transitions API already animates the swap (avoids double animation).
- For folder list navigation in [src/components/ScanResultsList.vue](../src/components/ScanResultsList.vue), wrap `.ScanResultsList-listInner` in `<Transition mode="out-in" name="vt-fallback-list">` keyed on `current.path` so goInto/goBack/goForward trigger leave/enter; same `:css` gating.
- Add `vt-fallback-app-*` and `vt-fallback-list-*` enter/leave classes to [src/assets/css/animations.css](../src/assets/css/animations.css) that translate by `40px * var(--nav-direction) * var(--rtl-flip)` (list) or `20px * …` (app), mirroring the existing `vt-slide-*` keyframes used by the supported path.
- Mirror the existing `prefers-reduced-motion` block: opacity-only fade.
- The existing `view-transition-name: scan-list-footer` declaration stays put — Safari 14 ignores the unknown property, so it is harmless.

**Out of scope:** matching the View Transitions API's pixel-perfect cross-fade (the API screenshots both states; Vue can only animate live DOM). The fallback is intentionally a "good enough" slide.

### Fix 2 — Suppress default focus outline on non-keyboard interaction

**Goal:** only the custom box-shadow focus ring is ever drawn. The UA outline never appears, regardless of Safari version.

**Approach:**
- In [src/assets/css/global.css](../src/assets/css/global.css), add an unconditional outline reset on focusable selectors:

  ```css
  button:focus,
  a:focus,
  select:focus,
  input:focus,
  [tabindex='0']:focus {
     outline: none;
  }
  ```

- Keep the existing `html.focus-ring-keyboard …:focus { box-shadow: … }` block as the only place that paints the keyboard ring. Its inner `outline: none` becomes redundant but can stay as a safety belt.
- One-line CSS change. No Vue or Rust impact. No `:focus-visible` is introduced, so Safari 13/14 (which ignore `:focus-visible`) keep working.

**Trade-off accepted:** if the `setupFocusRing()` JS hook ever fails to mount, *no* focus ring is drawn (instead of falling back to the UA outline). Acceptable because the hook is wired in at app root and a JS failure already breaks the app entirely.

### Fix 3 — Drop the native Popover API dependency

**Goal:** make the disabled-checkbox explainer tooltip work on Safari 13+ instead of Safari 17+, by reusing the existing `@floating-ui/dom`-based helper. After this fix, the app no longer relies on the Popover API or `@starting-style` anywhere.

**Background:** [src/lib/use-label-popover.ts](../src/lib/use-label-popover.ts) already implements a tooltip pattern that:
- Positions via `@floating-ui/dom`'s `computePosition()` (works in every Safari).
- Renders into `<body>` via `<Teleport to="body">` so ancestor `overflow` clipping is bypassed without needing the browser's "top layer".
- Animates entry/exit via an `is-open` class — a plain `transition`, no `@starting-style` needed.
- Is currently used by item-name + path tooltips in [ScanResultsListItem.vue](../src/components/ScanResultsListItem.vue), [ScanListNav.vue](../src/components/ScanListNav.vue), and [ScanTrashListItem.vue](../src/components/ScanTrashListItem.vue).

The disabled-checkbox tooltip in [ScanResultsListItem.vue:115-178, 279-290, 427-469](../src/components/ScanResultsListItem.vue#L115-L178) is the only place that opted into the native Popover API instead.

**Approach:**
- Replace the `popover="manual"` element + `showPopover()` / `hidePopover()` calls with a `useLabelPopover(checkboxTriggerRef, checkboxPopoverRef)` instance, mirroring how the item-name tooltip is wired.
- Keep the existing 400 ms hover delay by composing the wrapper's `onPointerEnter` over `useLabelPopover`'s `onPointerEnter` (or by passing a delay option if the helper grows one — small generalization).
- Replace the manual `getBoundingClientRect()` + `EDGE_MARGIN` placement math (lines 123–135) with floating-ui's `computePosition({ placement: 'top-start', middleware: [shift({ padding: 16 })] })`. This is what `useLabelPopover` already does internally.
- Replace the `:popover-open` rule and `@starting-style` block with `.ScanResultsListItem-checkboxPopover.is-open { … }` and a regular `transition`, mirroring the existing `.Popover` styles.
- Drop the bespoke scroll-listener that calls `dismissCheckboxTooltip` (lines 143–156): `useLabelPopover` already handles pointer-leave dismiss, and floating-ui can autoUpdate position if the user wants the tooltip to track on scroll instead of dismiss. Pick whichever matches the current UX best — likely "dismiss on scroll" is fine to drop since the tooltip is short-lived.

**Net effect:**
- Removes `popover=` attribute, `showPopover()` / `hidePopover()` calls, `:popover-open`, and `@starting-style` from the codebase.
- Removes two rows from *Features That Rely on Progressive Enhancement* (Popover API and `@starting-style`).
- Tooltip animates correctly on Safari 13/14/15/16, not just 17+.
- One fewer fork in browser-support reasoning.

### Verification after all fixes ship

1. Re-run on the same macOS 11 / Safari 14 install: tab switching and folder navigation should slide; mouse clicks should not draw any outline; Tab-key navigation should still draw the custom box-shadow ring; hovering a disabled checkbox should show the explainer tooltip with a fade/blur entry.
2. Re-run on macOS 14+ / Safari 17+: behavior should be unchanged (View Transitions API still owns the animation; keyboard-only ring still works; tooltip looks identical, just driven by floating-ui instead of the native Popover API).
3. Update *Features That Rely on Progressive Enhancement*: the View Transitions row gets the Vue-`<Transition>` fallback note; the Popover API and `@starting-style` rows can be removed entirely once Fix 3 ships.
4. Confirm zero remaining references in the codebase: `grep -rn 'popover=\|:popover-open\|@starting-style\|showPopover\|hidePopover' src/` should return no hits.

---

## Conclusion

The app ships as a **universal binary** supporting both **Intel (x86_64)** and **Apple Silicon (aarch64)** Macs natively — no Rosetta 2 required on either architecture. It is fully functional on **macOS 10.15 Catalina (Safari 13.0)** as the absolute minimum, and has been validated on **macOS 11 Big Sur / Safari 14.0** (real-device test, 2026-05-07): core functionality works end-to-end, with two visual regressions tracked in *UI Normalization Plan* above. Visual enhancements (native-popover animations, view transitions, overlay scrollbars) progressively enhance on newer macOS versions; label popovers use `@floating-ui/dom` so row tooltips work even on the 10.15 baseline.
