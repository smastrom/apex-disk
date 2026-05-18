---
description: Load the relevant reference file before starting work
---

Before starting any task, read the relevant reference file from `reference/`
based on the operation. Load BEFORE writing or changing code. Never commit
code that contradicts a reference file — if a reference is wrong, update it
in the same change.

| Operation                                                        | Read first                       |
| ---------------------------------------------------------------- | -------------------------------- |
| New feature / refactor / anything spanning multiple layers       | `reference/architecture.md`      |
| Bumping deps, using new JS / CSS / macOS APIs                    | `reference/compatibility.md`     |
| Adding logs, diagnostics, or trace channels                      | `reference/logging.md`           |
| Cutting a stable or beta release                                 | `reference/releases.md`          |
| Touching the in-app updater or `latest.json`                     | `reference/updates.md`           |
| Adding or changing translations / language support               | `reference/translations.md`      |
| Adding or changing themes / CSS variables in `theme.css`         | `reference/themes.md`            |
| Scan tree, `FolderInfo`, progress events, trash flow             | `reference/scanning.md`          |
| Adding a Tauri command, event, or settings field                 | `reference/tauri-commands.md`    |
| Writing or editing a Vue component, `.ts` file, or CSS           | `reference/code-style.md`        |
| Running tests, adding a test, debugging an e2e or unit failure   | `reference/testing.md`           |

If a task spans multiple operations (e.g. a new Tauri command that needs a
translation key), read every applicable file.
