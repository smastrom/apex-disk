# Beta QA notes

Per-run notes for **Beta** builds (manual `workflow_dispatch`, tag `beta-v<version>-<run_id>`). Newest-first; the Beta workflow attaches the **first** `## YYYY-MM-DD` section to the GitHub pre-release body and job summary. See [`reference/releases.md`](reference/releases.md) for how to cut a Beta.

---

## 2026-05-22

Verifies the cap-alignment fix that restores the "list truncated" notice on directories past the per-folder file cap, and tightens the matching subfolder cap from 500 to 100.

- Scan a home that includes at least one directory with more than 100 files (large `Downloads/`, a project repo's build output, etc.). Navigate into it: the "list truncated" message must appear under the row list, and the visible files must cap at 100.
- Scan a directory with more than 100 immediate subfolders (a `node_modules/` parent is the easy case). Navigate into the parent: the same notice must appear, the visible subfolders must cap at 100, and the parent's headline size must still reflect every subfolder's bytes (the cap is applied after recursion, so dropped subfolders still contribute to the total).
- Confirm folders well under the caps (e.g. `~/Documents` with a handful of items) do not show the notice.
- Regression pass: scan, Move to Trash, restore from `~/.Trash` on a small in-cap folder. Behavior should be identical to v0.0.24 for directories that stay below the caps.
- On Intel and Apple Silicon, confirm a real-home scan still completes without the renderer freezing or memory ballooning. The reduced folder cap means pathological `node_modules`-style parents now ship up to 200 children per node instead of 600.

## 2026-05-21

Verifies the new `## Release notes` heading that the Release and Beta workflows prepend to the GitHub-release body.

- Confirm the published pre-release body opens with a `## Release notes` heading directly above this dated section.
- Confirm `RELEASES_BETA.md` itself is unchanged on the published artifact: the heading is added only on the rendered release, not the source file.
- Smoke-test the existing scan and trash flow on a small folder to make sure the build is otherwise untouched.
