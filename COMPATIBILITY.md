# Compatibility Report

Target: **macOS 10.15 Catalina** (`minimumSystemVersion` in `src-tauri/tauri.conf.json`)

## Summary

| Layer | Min Target | Confirmed Compatible |
|-------|-----------|---------------------|
| **Rust / Native APIs** | macOS 10.13+ (all APIs used) | Yes — macOS 10.15 safe |
| **CSS (lightningcss)** | Safari 13 | Yes — transpiled via `vite.config.ts` |
| **JavaScript (Vite build)** | Safari 13 (`build.target`) | Yes — transpiled by Vite/Rolldown |
| **Tauri 2 WKWebView** | macOS 10.15 | Yes — Tauri 2 requires 10.15+ |

**Minimum supported macOS version: 10.15 Catalina (Safari 13.0 / WebKit 605.1.15)**

---

## How to Run This Compatibility Check

An LLM or developer can re-verify compatibility by following these steps:

### 1. Check the declared minimum macOS version

```bash
grep -A1 minimumSystemVersion src-tauri/tauri.conf.json
```

Expected: `"minimumSystemVersion": "10.15"`

macOS 10.15 Catalina ships with Safari 13.0. Users who update Catalina can reach Safari 15.6.1, but the **worst-case baseline is Safari 13.0**.

### 2. Verify CSS transpilation (lightningcss → Safari 13)

In `vite.config.ts`, lightningcss is configured to target Safari 13:

```ts
css: {
   transformer: 'lightningcss',
   lightningcss: {
      targets: {
         safari: (13 << 16) | (0 << 8) | 0, // Safari 13.0.0
      },
   },
},
```

**What lightningcss handles** (transpiles or adds prefixes automatically):
- CSS Nesting (`&` selector) → flattened to separate rules
- Vendor prefixes (`-webkit-`) → added where needed
- Modern color functions → converted to compatible equivalents
- Logical properties → converted to physical equivalents

**What lightningcss does NOT handle** (browser APIs in CSS):
- `:popover-open` pseudo-class (Safari 17+) — not a transpilable feature, it's a browser API
- `@starting-style` (Safari 17.5+) — not transpilable
- `::view-transition-*` pseudo-elements (Safari 18+) — not transpilable

These browser API features degrade gracefully: they are behind `@supports` checks or are progressive enhancements (popovers, animations) that don't break core functionality.

To verify CSS output has no untranspilable modern syntax:

```bash
pnpm build
# Check for CSS nesting (should be 0 — fully flattened):
grep -c '&:' dist/assets/*.css
```

### 3. Verify JavaScript transpilation (Vite → Safari 13)

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

### 4. Verify Rust / Native macOS API compatibility

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

### 5. Verify Rust toolchain and dependency MSRV

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

### 6. Tauri 2 platform requirements

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
| Popover API (`:popover-open`) | 17.0 | Popover won't have CSS transitions |
| `@starting-style` | 17.5 | No entry animations for popovers |
| View Transitions API (`::view-transition-*`) | 18.0 | Wrapped in `@supports` — no transition animation |
| CSS Nesting | 17.2 | **Transpiled by lightningcss** — no issue |

---

## Conclusion

The app is fully functional on **macOS 10.15 Catalina (Safari 13.0)** as the absolute minimum. Core functionality works without issue. Visual enhancements (popover animations, view transitions) progressively enhance on newer macOS versions.
