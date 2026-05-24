<!-- Thanks for contributing to ApexDisk! -->

**Base branch:** open pull requests against **`development`**, not `main`. The `main` branch is reserved for release merges.

## Summary

<!-- What does this change and why? One or two sentences is fine. -->

## Linked issue

<!-- e.g. Closes #123 — or "n/a" for small fixes -->

## Changes

<!-- Bullet list of notable changes. Skip if the summary already covers it. -->

-

## Checklist

- [ ] Code follows the conventions in `AGENTS.md` (Vue `<script setup>`, Rust import order, naming, comments policy)
- [ ] `pnpm fmt:check` passes
- [ ] `pnpm test:unit` passes (required for any Rust / `src-tauri/` change)
- [ ] `pnpm test:e2e` passes if UI flows changed
- [ ] `reference/` and root `.md` files reviewed — use the **Changed paths → re-verify** table in `reference/index.md`, then skim remaining docs; update any that drifted
- [ ] Commits follow the repo style: imperative subject, short body only when the _why_ isn't obvious, `Co-Authored-By` trailer when a commit was made by an agent
- [ ] No new platform-specific code for Windows/Linux (macOS-only project)

## Screenshots / screen recording

<!-- Required for UI changes. Before/after if applicable. -->

## Notes for reviewers

<!-- Anything non-obvious: tradeoffs considered, follow-ups intentionally deferred, areas you'd like a careful look at. -->

---

<details>
<summary>Using Claude Code? Helpful slash commands</summary>

- **`/sync`** — groups your uncommitted work into logical commits, runs the repo-wide `.md` sweep (`reference/`, `AGENTS.md`, `CLAUDE.md`, …), and pushes.
- **`/force-sync`** — reconciles every `.md` file against recent commits that bypassed the sweep.
- **`/compatibility-check`** — verifies the change stays within the supported macOS / Safari / architecture targets declared in `reference/compatibility.md`.

Release-channel commands (`/release`, `/beta-notes`) are maintainer-only — please don't run them in a PR.

</details>
