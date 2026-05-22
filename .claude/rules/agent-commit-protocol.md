---
description: Mandatory steps before committing or releasing
---

Before agent-initiated `git commit` or `git push`, run **`/sync`**
(or **`/force-sync`** to recover doc drift from commits that skipped `/sync`).

Full workflow (lint-staged vs `/sync`, gate, scoped doc sweep):
**`reference/agent-workflow.md`**.

## Gate

`.claude/hooks/pre-commit-gate.sh` blocks agent `git commit` / `git push`
unless `.claude/.sync-active` exists. `/sync`, `/force-sync`, `/release`, and
`/release-from-notes` create and remove that marker. To bypass intentionally,
ask the user first.

Never use `--no-verify` or `--force`.

## Also before committing (when applicable)

| Trigger | Action |
| ------- | ------ |
| Any code change | `/sync` (docs + tests + comments + verify + push) |
| Dep bump or new JS/CSS/macOS API | `/compatibility-check` → [`reference/compatibility.md`](../reference/compatibility.md) |
| Beta build | `/beta-notes` before Beta workflow |

If you believe a step should be skipped, **stop and ask the user first**.
