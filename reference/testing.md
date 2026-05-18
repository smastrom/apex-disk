# Testing

Three test layers plus three static checks. The frontend has **no unit tests** — only typecheck + e2e. The Rust side has full integration coverage.

## Suites

| Suite                | Command                                               | Scope                                                                |
| -------------------- | ----------------------------------------------------- | -------------------------------------------------------------------- |
| **Format**           | `pnpm fmt:check` / `pnpm fmt`                         | Oxfmt — `.ts` / `.tsx` / `.vue` formatting + import sorting.         |
| **License headers**  | `pnpm headers:check` / `pnpm headers`                 | SPDX + copyright on every first-party source file.                   |
| **Typecheck**        | `pnpm typecheck` (= `vue-tsc --noEmit`)               | Frontend type correctness. Committed code must be type-clean.        |
| **Rust unit / int.** | `pnpm test:unit` (= `cargo test -- --test-threads=1`) | Rust modules + integration tests in `src-tauri/tests/`. **Serial.**  |
| **E2E**              | `pnpm test:e2e`                                       | WebdriverIO drives a debug Tauri build with the `e2e` cargo feature. |

## When to run

| Change                                                       | Run                                    |
| ------------------------------------------------------------ | -------------------------------------- |
| Always (before every commit)                                 | `pnpm fmt:check`, `pnpm headers:check` |
| Any `*.ts` / `*.tsx` / `*.vue`                               | + `pnpm typecheck`                     |
| Any `src-tauri/**`                                           | + `pnpm test:unit`                     |
| Any user-visible change (`src/**`, `src-tauri/**`, `e2e/**`) | + `pnpm test:e2e`                      |

`/sync` and `/force-sync` run the relevant subset automatically. They never push red code and never bypass with `--no-verify` / `--force`.

**`pnpm test:e2e` is slow** — it rebuilds the debug Tauri binary on first run; subsequent runs are faster but still measured in minutes. Don't skip it on the assumption that "frontend-only" changes can't break e2e — they routinely do (selectors, transitions, focus handling, scroll behavior).

## Rust integration tests

Layout:

```
src-tauri/
├── src/                   # Modules under test
├── tests/                 # Integration tests (each file = a binary)
│   ├── scan_test.rs
│   ├── trash_test.rs
│   ├── store_test.rs
│   ├── safe_folders_test.rs
│   └── support/
│       └── mod.rs         # Shared helpers (temp dirs, fixtures)
└── Cargo.toml
```

Conventions:

- **Serial execution.** `cargo test -- --test-threads=1`. Some tests mutate process-global state (locale, store, `e2e` flag) — parallel runs would race. The `pnpm test:unit` script bakes this in.
- **Temp dirs only.** Tests must never touch the real user home. `support/mod.rs` provides helpers that create + clean up temp directories. If you add a new module that needs a writable path, route it through the same helper.
- **Fixture state reset.** `e2e_fixtures.rs` exposes `reset_e2e_state` (under `#[cfg(feature = "e2e")]`) so E2E specs can scrub between scenarios. Don't replicate this logic ad-hoc in tests.
- **Filter at the boundary.** When testing scan/trash behavior, drive through the public command surface (`get_user_folders`, `trash_paths`) rather than calling internal walkers — that way the `safe_folders.rs` filter is exercised once, the same way production does it.

## E2E (WebdriverIO + `e2e` cargo feature)

The E2E suite drives a real Tauri build via WebDriver. Key pieces:

- **Cargo feature** — `cargo build --features e2e` enables `tauri_plugin_webdriver` and a handful of test-only commands (`trash::set_e2e_trash_mode`, `store::reset_e2e_state`). These commands exist in a **second** `tauri::generate_handler!` block in `src-tauri/src/lib.rs` (one for default builds, one for `#[cfg(feature = "e2e")]`). **When you add a new command, update both blocks** — the macro can't expand nested macros, hence the duplication.
- **Trash dry-run** — `set_e2e_trash_mode` swaps in a dry-run trash that records intent without touching disk. Specs assert on the recorded calls instead of actual macOS Trash interactions.
- **Spec location** — `e2e/specs/`, plus shared helpers under `e2e/`.
- **First run is slow** — the debug Tauri build is compiled lazily. The CI machine and local dev share this cost.

## Frontend

No frontend unit tests today. `pnpm typecheck` is the only static guarantee; behavioral coverage lives in E2E. **Don't add Vitest or Vue Test Utils unless asked** — see the "What not to do" list in [`AGENTS.md`](../AGENTS.md).

## Adding tests

Per the project policy in [`AGENTS.md`](../AGENTS.md): **do not create tests unless asked.** If the user asks for one:

- Rust: add an integration test to `src-tauri/tests/<area>_test.rs` (one file per subsystem). Use `support::*` helpers for temp dirs.
- E2E: add a `.ts` spec under `e2e/specs/`. Reuse existing helpers; avoid new selectors when an existing one exists.

If a test breaks because the source change is correct and the test is outdated, fix the test in the same change. If a test breaks for an unknown reason, stop and surface it — never amend or revert source commits behind the user's back to make the suite pass.

## Module index

| Location                               | What                                                                 |
| -------------------------------------- | -------------------------------------------------------------------- |
| `src-tauri/tests/scan_test.rs`         | Scan walker + filter behavior                                        |
| `src-tauri/tests/trash_test.rs`        | Trash + `filter_items` defense                                       |
| `src-tauri/tests/store_test.rs`        | Store concurrency, defaults merging, key validation                  |
| `src-tauri/tests/safe_folders_test.rs` | Protected + skipped path correctness                                 |
| `src-tauri/tests/support/mod.rs`       | Shared helpers (temp dirs, fixtures)                                 |
| `src-tauri/src/e2e_fixtures.rs`        | `reset_e2e_state`, dry-run trash mode (compiled under `e2e` feature) |
| `src-tauri/src/lib.rs`                 | Two `generate_handler!` blocks (default + `#[cfg(feature = "e2e")]`) |
| `e2e/specs/`                           | WebdriverIO test scenarios                                           |
| `package.json`                         | `test:unit`, `test:e2e`, `typecheck`, `fmt`, `headers` scripts       |
