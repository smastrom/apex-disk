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

These features degrade gracefully: `::view-transition-*` rules are wrapped in `@supports (view-transition-name: none)` so older WebKit simply doesn't animate; `:popover-open` + `@starting-style` are used only by the selection-count popover (see below) and have no layout fallback needed. Label popovers on list rows use `@floating-ui/dom` with `<Teleport to="body">` instead of the native Popover API so they work on Safari 13+.

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
| Popover API (`popover=` + `:popover-open`) | 17.0 | Only the selection-count popover on result rows uses the native API. On older Safari it just doesn't open via the popover mechanism — the counter remains readable elsewhere in the UI. Row **label** popovers (item name, full path) use `@floating-ui/dom` + `<Teleport to="body">` and work on Safari 13+. |
| `@starting-style` | 17.5 | No entry animation on the popover above |
| View Transitions API (`::view-transition-*`) | 18.0 | All rules wrapped in `@supports (view-transition-name: none)` — navigation simply cuts without animation |
| `overflow: overlay` (native macOS auto-hide scrollbars) | 14.0 (WebKit) | Non-WebKit / older WebKit falls back to the `overflow: auto` declared on the preceding line — classic always-visible scrollbars |
| `color-mix()` with custom-property operands | 16.2 | Passes through unchanged — renderer falls back to the declared base color |
| CSS Nesting | 17.2 | **Transpiled by lightningcss** — no issue |
| Cascade Layers (`@layer`) | 15.4 | **Flattened by lightningcss** — no issue |

---

## Conclusion

The app ships as a **universal binary** supporting both **Intel (x86_64)** and **Apple Silicon (aarch64)** Macs natively — no Rosetta 2 required on either architecture. It is fully functional on **macOS 10.15 Catalina (Safari 13.0)** as the absolute minimum. Core functionality works without issue. Visual enhancements (native-popover animations, view transitions, overlay scrollbars) progressively enhance on newer macOS versions; label popovers use `@floating-ui/dom` so row tooltips work even on the 10.15 baseline.
