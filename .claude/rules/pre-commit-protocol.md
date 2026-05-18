---
description: Mandatory steps before committing or releasing
---

Before running `git commit` or pushing, complete the following. Do not skip
any step — these commands automate work that is otherwise easy to forget.

## 1. Sweep every `.md` in the repo

Run `/sync` (or `/force-sync` if recent commits bypassed the docs sweep).
The sweep must analyze **every** `.md` file in the repository for update
eligibility and update any whose contents no longer match the change.
Coverage includes:

- `reference/*.md` — agent-facing deep specs (how the code works)
- `marketing/*.md` — outcome-facing content (FAQ, descriptions). Only touch
  when user-visible outcomes change, not for internal refactors.
- `AGENTS.md` — scope + pointers
- `CLAUDE.md` — Claude entrypoint
- `README.md` — user-facing surface
- `RELEASES.md`, `RELEASES_BETA.md` — only when cutting releases (handled by
  `/release` and `/beta-notes`; do not edit by hand here)
- `.claude/rules/*.md` — workflow / convention changes
- `.claude/commands/*.md` — slash command behavior changes
- `LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md` — rarely change, but verify

Never commit code that contradicts any of these files.

## 2. Compatibility check (when applicable)

If the change adds or bumps a dependency, or introduces new JS / CSS /
macOS API surface, run `/compatibility-check` and fix any findings before
committing. The check verifies the change stays within the declared
minimum target documented in `reference/compatibility.md` (macOS 10.15 /
Safari 13 / Rust MSRV 1.70+).

## 3. Beta cut (when applicable)

When preparing a beta build, run `/beta-notes` to add the dated entry to
`RELEASES_BETA.md` before triggering the Beta workflow.

## Notes

- These rules supplement, not replace, the test gates inside `/sync` and
  `/force-sync` (typecheck, unit tests, e2e). Those still run.
- If you believe a step should be skipped, **stop and ask the user first**
  — do not silently skip and commit.
- Never use `--no-verify` or `--force` to bypass hooks.
