---
description: Load the relevant reference file before starting work
---

Before starting any task, read the relevant reference file from `reference/`
based on the operation. Load BEFORE writing or changing code. Never commit
code that contradicts a reference file — if a reference is wrong, update it
in the same change.

| Operation                                                                               | Read first                     |
| --------------------------------------------------------------------------------------- | ------------------------------ |
| New feature / refactor / anything spanning multiple layers                              | `reference/architecture.md`    |
| Scan/trash flow, `FolderInfo`, progress events, cancellation, Vue/Rust memory lifecycle | `reference/state-lifecycle.md` |
| Adding a Tauri command, event, or settings field                                        | `reference/tauri-commands.md`  |
| Writing or editing a Vue component, `.ts` file, or CSS                                  | `reference/code-style.md`      |
| Adding or changing translations / language support                                      | `reference/translations.md`    |
| Running tests, adding a test, debugging an e2e or unit failure                          | `reference/testing.md`         |
| Adding or changing themes / CSS variables in `theme.css`                                | `reference/themes.md`          |
| Bumping deps, using new JS / CSS / macOS APIs                                           | `reference/compatibility.md`   |
| Writing user-facing prose: README, RELEASES, copy                                       | `reference/voice.md`           |
| Adding logs, diagnostics, or trace channels                                             | `reference/logging.md`         |
| Touching the in-app updater or `latest.json`                                            | `reference/updates.md`         |
| Cutting a stable or beta release                                                        | `reference/releases.md`        |

If a task spans multiple operations (e.g. a new Tauri command that needs a
translation key), read every applicable file.
