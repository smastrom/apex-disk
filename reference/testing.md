# Testing

Keywords: cargo test, e2e, wdio, typecheck, fmt, headers, test sweep.

Three test layers plus static checks. The frontend has **no unit tests** and **no ESLint** — only the TypeScript checker (`vue-tsc`) + e2e. The Rust side has full integration coverage. Oxlint is a formatter adjunct (import/padding), not a linter.

## Suites

| Suite                | Command                                               | Scope                                                                         |
| -------------------- | ----------------------------------------------------- | ----------------------------------------------------------------------------- |
| **Format**           | `pnpm fmt:check` / `pnpm fmt`                         | Oxfmt — general `.ts` / `.tsx` / `.vue` / CSS / JSON formatting.              |
| **Import / padding** | `oxlint` / `oxlint --fix`                             | Oxlint adjunct — import order + statement padding only (see `code-style.md`). |
| **License headers**  | `pnpm headers:check` / `pnpm headers`                 | SPDX + copyright on every first-party source file.                            |
| **Typecheck**        | `pnpm typecheck` (= `vue-tsc --noEmit`)               | Frontend linter — TypeScript / Vue type correctness only.                     |
| **Rust unit / int.** | `pnpm test:unit` (= `cargo test -- --test-threads=1`) | Rust modules + integration tests in `src-tauri/tests/`. **Serial.**           |
| **E2E**              | `pnpm test:e2e`                                       | WebdriverIO drives a debug Tauri build with the `e2e` cargo feature.          |

## When to run

When to run lint-staged vs `/sync` verify: [`agent-workflow.md`](agent-workflow.md).
Do not duplicate work lint-staged already did at commit time.

**E2E note:** `/sync` skips `pnpm test:e2e` locally (CI runs it). Step 4 of
`/sync` includes a **test sweep** of specs/helpers/fixtures. `/force-sync`
runs e2e when the commit window touched frontend, Rust, or e2e. Selector and
fixture contract: [`e2e.md`](e2e.md).

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

The E2E suite drives a real Tauri build via WebDriver. Selector registry,
fixture names, and interaction rules: [`e2e.md`](e2e.md). Key pieces:

- **Cargo feature** — `cargo build --features e2e` enables `tauri_plugin_webdriver` and the test-only commands (`trash::set_e2e_trash_mode`, `store::reset_e2e_state`). These live in a second `generate_handler!` block under `#[cfg(feature = "e2e")]`. See [`tauri-commands.md`](tauri-commands.md) for the dual-block rule when adding new commands.
- **Trash dry-run** — `set_e2e_trash_mode` swaps in a dry-run trash that records intent without touching disk. Specs assert on the recorded calls instead of actual macOS Trash interactions.
- **Scan slow-start** — `get_user_folders_sync_with_progress` holds for up to 500 ms (polling the cancel token) when built with `--features e2e`. Without this the tiny fixture can finish scanning before WebdriverIO catches the SCANNING view, breaking "abort during scan" specs.
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

| Location                               | What                                                                                                                                                         |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src-tauri/tests/scan_test.rs`         | Scan walker + filter behavior                                                                                                                                |
| `src-tauri/tests/trash_test.rs`        | Trash + `filter_items` defense                                                                                                                               |
| `src-tauri/tests/store_test.rs`        | Store concurrency, defaults merging, key validation                                                                                                          |
| `src-tauri/tests/safe_folders_test.rs` | Protected + skipped path correctness                                                                                                                         |
| `src-tauri/tests/scaling_probe.rs`     | Opt-in (`#[ignore]`) IPC payload / node-count probe against real paths. Run with `cargo test --test scaling_probe -- --ignored --nocapture --test-threads=1` |
| `src-tauri/tests/support/mod.rs`       | Shared helpers (temp dirs, fixtures)                                                                                                                         |
| `src-tauri/src/e2e_fixtures.rs`        | `reset_e2e_state`, dry-run trash mode (compiled under `e2e` feature)                                                                                         |
| `src-tauri/src/lib.rs`                 | Two `generate_handler!` blocks (default + `#[cfg(feature = "e2e")]`)                                                                                         |
| `e2e/specs/`                           | WebdriverIO test scenarios                                                                                                                                   |
| `package.json`                         | `test:unit`, `test:e2e`, `typecheck`, `fmt`, `headers` scripts                                                                                               |
| [`e2e.md`](e2e.md)                     | `data-testid` contract, `sel` registry, fixture names                                                                                                        |
