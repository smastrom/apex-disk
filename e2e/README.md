# E2E Testing

The E2E suite uses WebdriverIO against a real debug Tauri binary built with the
`e2e` Cargo feature. The app still talks to Rust through Tauri IPC; the test
feature swaps the real home folder and trash operations for deterministic
tempdir fixtures.

Run the suite with:

```bash
pnpm test:e2e
```

The first run is slow because it builds the debug binary. Later runs reuse most
of the Rust build output.

## Runtime Model

`e2e/wdio.conf.ts` builds the app with:

```bash
pnpm tauri build --debug --no-bundle -- --features e2e
```

The config resolves pnpm from `npm_execpath` when available, so nested build
steps use the same package manager version that launched the test. If the build
fails, the WDIO `onPrepare` hook exits immediately instead of letting specs run
against a missing app.

Each WDIO session launches the debug binary from `src-tauri/target/debug/` and
cleans it up on process exit or termination signals.

## Fixture Data

The Rust `e2e_fixtures` module creates a temp home folder with predictable
contents:

| Path                              | Purpose                               |
| --------------------------------- | ------------------------------------- |
| `MyData/`                         | Normal selectable folder              |
| `MyData/big.txt`                  | Normal file over 1 KB                 |
| `MyData/small.txt`                | Under-1 KB filter coverage            |
| `MyData/empty.txt`                | Zero-byte filter coverage             |
| `MyData/.hidden`                  | Hidden-file filter coverage           |
| `MyData/.DS_Store`                | `.DS_Store` dependent-toggle coverage |
| `MyData/SubFolder/Deep/gamma.txt` | Nested selection propagation          |
| `Documents/`                      | Protected folder                      |
| `Projects/Bulk/`                  | Per-folder truncation notice          |
| `.ssh`, `.Trash`                  | Skipped folders, absent from results  |

Tests must use these fixtures rather than the real user home.

## Suites

| Spec                         | Coverage                                                                 |
| ---------------------------- | ------------------------------------------------------------------------ |
| `app-navigation.spec.ts`     | Footer navigation and Information view footer/license rendering          |
| `scan-flow.spec.ts`          | Scan launch, results, sorting, protected rows, truncation, scan dot      |
| `selection-checkbox.spec.ts` | Selection, inherited selection, partial state, protected-folder behavior |
| `settings-flow.spec.ts`      | Settings persistence, scan filters, dependent update and `.DS_Store` UI  |
| `trash-review.spec.ts`       | Review list, countdown, dry-run trash success/error states               |

`settings-flow.spec.ts` intentionally verifies filter effects by invoking the
Rust scan command with the persisted settings. The scan UI lifecycle is already
covered by `scan-flow.spec.ts`, and direct scans keep settings coverage fast.

## Helpers

Shared helpers live in `e2e/helpers/navigation.ts`.

Use existing helpers and selectors before adding new ones. They handle:

- app/view readiness
- scan start retries
- transition settling
- row lookup by displayed text
- checkbox state assertions
- E2E state reset
- dry-run trash mode switching

The helper also disables app/list transition CSS during WebDriver sessions.
This keeps assertions focused on app behavior rather than animation timing.

## Test-Only Commands

The `e2e` Cargo feature exposes commands that production builds do not include:

| Command              | Purpose                                      |
| -------------------- | -------------------------------------------- |
| `reset_e2e_state`    | Restores persisted settings to defaults      |
| `set_e2e_trash_mode` | Switches dry-run trash between success/error |

When adding a production Tauri command, update both `generate_handler!` blocks
in `src-tauri/src/lib.rs`: the default block and the `#[cfg(feature = "e2e")]`
block.

## Gotchas

- Always wait for elements before clicking. Vue transitions and `KeepAlive`
  can leave old elements mounted briefly. Outer view switches skip their
  animation under WebDriver, but inner Scan transitions still run.
- Click row checkboxes for selection. Clicking a folder row navigates into it.
- Reset settings in tests that mutate them.
- Keep fixture expectations in sync with `src-tauri/src/e2e_fixtures.rs`.
- Do not add frontend mocks. The point of this suite is to exercise the real
  Rust, Tauri, and webview boundary.
