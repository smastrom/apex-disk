# Agent workflow

Keywords: sync, lint-staged, husky, pre-commit gate, verify, headers, typecheck.

How agents should use the reference docs, git hooks, and `/sync`. Read this
when you are unsure what runs automatically vs what you must do by hand.

Task routing: [`index.md`](index.md). Suite details: [`testing.md`](testing.md).
Commit gate and release triggers: [`.claude/rules/agent-commit-protocol.md`](../.claude/rules/agent-commit-protocol.md).

## Branching

| Branch            | Role                                                                                                             |
| ----------------- | ---------------------------------------------------------------------------------------------------------------- |
| **`development`** | Default integration branch. Day-to-day work, agent `/sync` pushes, and **all external pull requests** land here. |
| **`main`**        | Release line. Merge from `development` when shipping; the Release workflow runs against this branch.             |

Feature branches branch off `development` and merge back into `development`.
Release commits (version bumps + `RELEASES.md`) also land on `development` first,
then merge to `main` in a separate PR before triggering **Actions → Release**.

## Three phases

| Phase            | When                            | Who runs it                                | What it covers                                                                                                                                                                                              |
| ---------------- | ------------------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **While coding** | Before writing or changing code | Agent                                      | Read the relevant `reference/*.md` file(s) from [`index.md`](index.md) **Task routing**. Update a reference in the same change if it is wrong.                                                              |
| **At commit**    | Every `git commit`              | **Husky + lint-staged** (automatic)        | Staged `*.{js,ts,vue}` → `oxlint --fix` + `oxfmt`; staged CSS/JSON/MD → `oxfmt`; staged `*.rs` → `rustfmt`. Do **not** manually re-run fmt/oxlint on files lint-staged already covers unless a hook failed. |
| **Before push**  | End of a work session           | Agent via **`/sync`** or **`/force-sync`** | Logical commit grouping, license headers, typecheck, doc/coderabbit/test/comment sweeps, `test:unit` when Rust changed, push. See [`.claude/commands/sync.md`](../.claude/commands/sync.md).                |

Human commits from a terminal follow the same husky hook at commit time. Only
**agent-initiated** `git commit` / `git push` are gated (next section).

## Pre-commit gate (agents only)

`.claude/hooks/pre-commit-gate.sh` blocks agent `git commit` and `git push`
unless `.claude/.sync-active` exists. `/sync`, `/force-sync`, `/release`, and
`/release-from-notes` create that marker at start and remove it at end. Any other
agent path to commit/push is refused.

To bypass intentionally for a one-off, ask the user first. They can
`touch .claude/.sync-active`, run the command, then `rm` it.

Never use `--no-verify` or `--force` to skip hooks.

## What `/sync` verify runs (and what it skips)

After commits, `/sync` step 7 runs checks on `HEAD` that lint-staged does
**not** cover:

| Check                            | In lint-staged?                | In `/sync` verify?                                                                                                                                                  |
| -------------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| oxlint + oxfmt (TS/Vue/CSS/JSON) | Yes, on staged files at commit | `fmt:check` + `oxlint` on whole tree (belt-and-suspenders)                                                                                                          |
| rustfmt                          | Yes, on staged `.rs` at commit | `fmt:check` covers TS side; Rust fmt via lint-staged per commit                                                                                                     |
| License headers                  | No                             | `headers:check`                                                                                                                                                     |
| Typecheck (`vue-tsc`)            | No                             | Yes, when TS/Vue touched                                                                                                                                            |
| Rust unit tests                  | No                             | Yes, when `src-tauri/**` touched                                                                                                                                    |
| E2E                              | No                             | **`/sync` skips locally**; CI runs it. Step 4 **test sweep** reads specs instead. **`/force-sync` runs e2e** when the commit window touched frontend, Rust, or e2e. |

Docs-only syncs can skip the verify step entirely (see `/sync` step 7).

## Doc sweep at sync time

`/sync` step 4 runs a **scoped doc sweep** first using the **Changed paths →
re-verify** table in [`index.md`](index.md), then skims remaining `.md`
files and `.coderabbit.yaml`, plus test files and file-top comments. Reading
`reference/*.md` while coding reduces drift but does not replace the sweep.
