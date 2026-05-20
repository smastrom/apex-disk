---
description: Mandatory steps before committing or releasing
---

Before running `git commit` or pushing, complete the following. Do not skip
any step: these commands automate work that is otherwise easy to forget.

This protocol is enforced from the repo by `.claude/hooks/pre-commit-gate.sh`,
wired into Claude Code as a `PreToolUse` hook via `.claude/settings.json` and
into Cursor as a `beforeShellExecution` hook via `.cursor/hooks.json`. The
hook intercepts agent-initiated `git commit` and `git push` and blocks them
unless `.claude/.sync-active` is present. `/sync` and `/force-sync`
create that marker at their first step and remove it at their last, so they
flow through unchanged; any other path to `git commit` / `git push` is
refused with a message pointing here. The marker is gitignored. To bypass
intentionally for a one-off (e.g. a typo fix), ask the user — they can
`touch .claude/.sync-active`, run the command, then `rm` it.

## 1. Sweep every doc in the repo

Run `/sync` (or `/force-sync` if recent commits bypassed the docs sweep).
The sweep must analyze **every** `.md` file in the repository plus
`.coderabbit.yaml` for update eligibility and update any whose contents no
longer match the change. Coverage includes:

- `reference/*.md`: agent-facing deep specs (how the code works)
- `AGENTS.md`: scope + pointers
- `CLAUDE.md`: Claude entrypoint
- `README.md`: user-facing surface
- `RELEASES.md`, `RELEASES_BETA.md`: only when cutting releases (handled by
  `/release` and `/beta-notes`; do not edit by hand here)
- `.claude/rules/*.md`: workflow / convention changes
- `.claude/commands/*.md`: slash command behavior changes
- `.coderabbit.yaml`: CodeRabbit `path_instructions` encode current code
  conventions (Composition API, scoped styles + tokens, store schema,
  translation coverage, e2e fixtures, capability surface, workflow
  expectations). Drift here turns into noisy or wrong PR review comments.
- `LICENSE.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`: rarely change, but verify

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
- If you believe a step should be skipped, **stop and ask the user first**.
  Do not silently skip and commit.
- Never use `--no-verify` or `--force` to bypass hooks.
