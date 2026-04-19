Add a dated beta-notes section to `RELEASES_BETA.md` (at the repo root). The Beta workflow uses the **first** `## …` section as the pre-release body and job summary, so new sections go **at the top**, newest first.

1. **Gather context** — Read the most recent commits on the current branch since the last section in `RELEASES_BETA.md` (or since the last stable tag if the file is empty): `git log --oneline <since>..HEAD`. Group changes by theme (fixes, features, risky areas to smoke-test).

2. **Determine the date** — Use today's UTC date in `YYYY-MM-DD` form.

3. **Insert a new section** — Add `## YYYY-MM-DD` directly under the horizontal-rule line (the `---` that separates the preamble from entries). Keep it at the **top** of the entries list; do not touch older sections.

4. **Content pattern** — A one-sentence summary line, then a bulleted list of concrete smoke-test items and known issues. Call out:
   - Any UI flows that changed (navigation, scan, trash confirmation)
   - Anything touching the updater, menu, or store
   - Anything platform-sensitive (macOS version behaviour, FDA)

5. **Do not** bump version fields, edit `RELEASES.md`, or commit — this command only prepares the notes file. The user runs **Actions → Beta → Run workflow** manually afterwards.

If `$ARGUMENTS` is provided, treat it as free-form context the user wants included (e.g. "focus on scan cancel flow", "note intermittent permission-prompt issue from yesterday's build"). Weave it into the bullets without dropping the automatically-derived list.
