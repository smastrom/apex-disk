# Beta QA notes

Per-run notes for **Beta** builds (manual `workflow_dispatch`, tag `beta-<run_id>`). Newest-first; the Beta workflow attaches the **first** `## YYYY-MM-DD` section to the GitHub pre-release body and job summary. See [`docs/RELEASES.md`](docs/RELEASES.md) for how to cut a Beta.

---

## 2026-05-21

Verifies the new `## Release notes` heading that the Release and Beta workflows prepend to the GitHub-release body.

- Confirm the published pre-release body opens with a `## Release notes` heading directly above this dated section.
- Confirm `RELEASES_BETA.md` itself is unchanged on the published artifact: the heading is added only on the rendered release, not the source file.
- Smoke-test the existing scan and trash flow on a small folder to make sure the build is otherwise untouched.
