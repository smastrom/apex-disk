# Compatibility

The supported target and how to verify it. ApexDisk runs portable Vue + JS implementations of animations, popovers, focus styling, and scrollbars; the LLM does not need to know which newer Safari APIs we "could have used" — only what's available today.

## Targets

| Layer                  | Min target                           | Notes                                            |
| ---------------------- | ------------------------------------ | ------------------------------------------------ |
| **Architecture**       | Intel x86_64 + Apple Silicon aarch64 | Universal binary, no Rosetta 2 required.         |
| **Rust / Native APIs** | macOS 10.13+ APIs only               | Safely within the 10.15 minimum.                 |
| **CSS**                | Safari 13                            | Transpiled by lightningcss in `vite.config.ts`.  |
| **JavaScript**         | Safari 13                            | Transpiled by Vite (`build.target: 'safari13'`). |
| **Tauri 2 WKWebView**  | macOS 10.15                          | Hard requirement from Tauri 2 itself.            |

Minimum supported macOS: **10.15 Catalina** (Safari 13.0 / WebKit 605.1.15). The minimum is set by `minimumSystemVersion` in `src-tauri/tauri.conf.json`.

## What lightningcss handles (you can use these freely)

- CSS Nesting (`&` selector)
- Cascade layers (`@layer`)
- Vendor prefixes (`-webkit-`, etc.)
- Modern color functions (`color-mix()` with static operands, `oklch()`)
- Logical properties (`margin-inline`, `padding-block`)
- `inset` shorthand

## What lightningcss does NOT handle (avoid)

- `color-mix()` whose operands are CSS custom properties — cannot be resolved at build time. Use `opacity` on a solid color instead.
- `:has()` selector — would emit unchanged; Safari 13–15.3 don't support it. Don't use `:has()`.

## Progressive-enhancement features (still allowed)

These CSS features render the visual upgrade on newer Safari and degrade silently on older ones — no broken behavior:

| Feature                   | Min Safari | Fallback                                                       |
| ------------------------- | ---------- | -------------------------------------------------------------- |
| `overflow: overlay`       | 14.0       | Falls back to the `overflow: auto` declared on the line above. |
| CSS Nesting               | 17.2       | Transpiled by lightningcss.                                    |
| Cascade Layers (`@layer`) | 15.4       | Flattened by lightningcss.                                     |
| `inset` shorthand         | 14.5       | Expanded by lightningcss.                                      |

## How to verify (re-run when changing deps or APIs)

Each step is a single shell check. Run them after bumping deps, adding a new JS / CSS / macOS API, or touching `vite.config.ts` / `tauri.conf.json`.

### 1. Architecture (universal binary)

```bash
grep 'universal-apple-darwin\|targets:.*aarch64\|targets:.*x86_64' .github/workflows/release.yml
```

Expected: Rust toolchain installs both `aarch64-apple-darwin,x86_64-apple-darwin`; build uses `--target universal-apple-darwin`; upload from `target/universal-apple-darwin/release/bundle/`.

```bash
# Should return 0 — no arch-gated code in app source:
grep -rn 'cfg.*target_arch\|#\[cfg.*arch\|x86_64\|aarch64' src-tauri/src/ --include='*.rs'
```

### 2. Declared minimum macOS

```bash
grep -A1 minimumSystemVersion src-tauri/tauri.conf.json
```

Expected: `"minimumSystemVersion": "10.15"`.

### 3. CSS transpilation (lightningcss → Safari 13)

`vite.config.ts` must have:

```ts
const safari13 = (13 << 16) | (0 << 8) | 0

css: {
   transformer: 'lightningcss',
   lightningcss: { targets: { safari: safari13 } },
},

build: {
   target: 'safari13',
   cssMinify: 'lightningcss',
}
```

Then:

```bash
pnpm build
# CSS nesting flattened — should be 0:
grep -c '&:' dist/assets/*.css
# Cascade layers flattened — should be 0:
grep -c '@layer' dist/assets/*.css
```

### 4. JavaScript transpilation

`vite.config.ts` must have `build.target: 'safari13'`. Then:

```bash
pnpm build
# Optional chaining transpiled — should be 0:
grep -c '?\.' dist/assets/*.js
```

### 5. Native macOS API surface

All `objc2` usage must target APIs available on macOS 10.13 or earlier:

```bash
grep -rn 'objc2' src-tauri/src/ --include='*.rs'
```

Cross-reference each Foundation/AppKit symbol with Apple's developer docs for availability. The most restrictive symbol currently in use is `NSURLVolumeAvailableCapacityForImportantUsageKey` (macOS 10.13) in `src-tauri/src/disk.rs`.

### 6. Rust MSRV

Driven by Tauri 2 and objc2: **Rust 1.70+**. No `rust-toolchain.toml` — the MSRV is implicit from dependencies.

Other MSRVs in the tree: tokio (1.63), serde (1.31), rayon (1.59), nix (1.69), trash (1.63). Aggregate stays at 1.70.

### 7. No opt-in to newer-only browser APIs

```bash
grep -rn 'view-transition\|startViewTransition\|popover=\|:popover-open\|@starting-style\|showPopover\|hidePopover\|:focus-visible' src/ --include='*.ts' --include='*.vue' --include='*.css' --exclude-dir=dist
```

Should return zero hits.

### 8. Tauri 2 platform requirement

Tauri 2 on macOS uses WKWebView (system WebKit, tied to macOS version) and requires **macOS 10.15** as the hard minimum. WebKit on macOS 10.15.0: Safari 13.0 (605.1.15). On fully-updated Catalina: up to Safari 15.6.1.

## macOS ↔ Safari version reference

| macOS | Codename | Ships with Safari | Max Safari (with updates) |
| ----- | -------- | ----------------- | ------------------------- |
| 10.15 | Catalina | 13.0              | 15.6.1                    |
| 11    | Big Sur  | 14.0              | 16.6.1                    |
| 12    | Monterey | 15.0              | 17.6.1                    |
| 13    | Ventura  | 16.0              | 18.x                      |
| 14    | Sonoma   | 17.0              | 18.x                      |
| 15    | Sequoia  | 18.0              | current                   |

## Validation history

Real-device tested end-to-end on **macOS 11 Big Sur / Safari 14.0** (2026-05-07) with no visual regressions. The Safari 13 floor is expected to behave identically since the relevant feature gaps (View Transitions, native Popover API, `:focus-visible` heuristics) all exist on Safari 13 and 14 the same way.
