# Voice for user-facing docs

Keywords: README, RELEASES, copy, tone, em dash, second person.

Canonical example: [`../README.md`](../README.md). When in doubt, open the
README and copy its cadence sentence-by-sentence.

This guide governs every user-facing surface:

- `README.md`
- `RELEASES.md`, `RELEASES_BETA.md` (bullet bodies; the structural headings
  follow [`releases.md`](releases.md))
- `SECURITY.md`, `CODE_OF_CONDUCT.md`, `LICENSE.md`
- in-app strings (translations, error copy, dialog text)
- code comments in `.ts` / `.tsx` / `.vue` / `.rs` / `.sh` / CSS (rules 2-5
  only; rule 1 about second person does not apply, since comments address
  the next maintainer)
- commit messages and PR descriptions

Files under `.claude/` and `reference/` are exempt: they are agent-facing
internal docs.

## How the voice sounds

Five rules, deliberately short. Adding more rules pushes drafts toward
over-application of every rule at once; keep this list tight.

### 1. Second person, direct

**Applies to:** `README.md`, in-app strings.

**Does not apply to:**

- Code comments — they talk to the next maintainer, not the end user.
- `RELEASES.md` / `RELEASES_BETA.md` — these are technical changelogs;
  bullets describe what changed in the product, not what the reader does.
  Past-tense action verbs ("Fixed…", "Added support for…", "Improved scan
  cancellation on large trees") are the norm there, not "you".

Address the reader as "you" and their machine as "your Mac". Never "the
user", never third person.

- Good: "Use it when your disk is nearly full and you need to reclaim space
  fast."
- Bad: "Users should run the tool when their disk is nearly full."

### 2. Plain English, no marketing jargon

Banned words and phrases: _leverage, unleash, powerful, blazing-fast,
seamless, effortless, intuitive, next-generation, revolutionary, simply,
just, whether you're a beginner or a power user_.

- Good: "Scans your Mac user folder and sorts everything by size."
- Bad: "Leverage our powerful disk-analysis engine for a seamless cleanup
  experience."

### 3. Honest, no overselling

State limits plainly. Acknowledge what the product does not do.

- "There is no catch. ApexDisk is 100% free and open source under the
  GPL-3.0 license."
- "Full-disk scanning and other volumes (such as external drives) may be
  considered in future releases, but they are not currently on the roadmap."

Never imply a feature exists if it doesn't. Never imply benchmarks that
weren't measured.

### 4. Short paragraphs, conversational rhythm

One to three sentences per paragraph. Break long thoughts apart rather than
stacking clauses. Contractions ("it's", "you'd", "what's", "don't") match
natural spoken English and are encouraged.

### 5. Action-first for instructions

Start instruction sentences with the verb the reader performs.

- "Download the latest .dmg from the Releases page, then drag the app to
  your Applications folder."
- "Grant Full Disk Access in System Settings."

## Punctuation

### Em dashes (`—`)

**Allowed as label separators**, where the dash sits between a short label
(noun, term, name, section title) and a description or gloss of it. This
covers bullets, list items, table headings, CSS/Rust/TS section-header
comments, file-title comments, complexity annotations, and any other
single-line `[label] — [description]` construct.

```markdown
- **Fast** — scans hundreds of thousands of files in seconds.
- **Safe** — every removal goes to the Trash, never a hard delete.
- **New Features** — user-visible additions.
```

```css
/* Theme: macOS Dark — Apple's system dark appearance */
/* Easing — use consistently for transitions and animations */
```

```rust
/// - `MyData` — files of varying sizes, hidden file
/// Silent update check — returns the available version string or `null`.
```

**Not allowed as parenthetical interrupts in running prose.** Rewrite with a
comma, parentheses, period, or colon. The test: if the words before and
after the dash form a flowing sentence and the dash is interrupting it to
add a side note or reason, it's an interrupt and must go.

- Bad: "ApexDisk is built with Rust — a fast systems language — and runs
  natively."
- Good: "ApexDisk is built with Rust, a fast systems language, and runs
  natively."
- Bad: "Scanning is fast — even on huge folders."
- Good: "Scanning is fast, even on huge folders."
- Bad: "We only ever replace the whole array — never mutate in place."
  (this is a flowing sentence with the dash interrupting it)
- Good: "We only ever replace the whole array, never mutate in place."

**Log strings and structured output:** even when they look like
`[category] — [data]`, prefer removing the dash (the existing convention is
`"Scan: complete 5 folders"`, not `"Scan: complete — 5 folders"`). These
aren't prose and don't need the separator.

### En dashes (`–`)

Not used in this project. If you need a separator inside a numeric range, use
"to" or a hyphen ("10 to 20 MB", "1-3 sentences").

### Hyphens (`-`)

Normal and encouraged in compound modifiers: "user-facing", "open-source",
"single-core", "size-sorted". Unrelated to the em-dash rule.

## How to apply

1. Drafting a user-facing doc? Open [`../README.md`](../README.md) first and
   read a couple of sections to re-calibrate.
2. Before saving, scan for: banned words (rule 2), em dashes that aren't
   list separators, third-person "the user" (in surfaces where rule 1
   applies), paragraphs longer than three sentences.
3. For release-note bullets, the **section heading** uses the
   label-separator pattern (e.g. `**Improvements** — enhancements to
existing behavior`), but the **bullet bodies** under each section follow
   this voice guide and must not use em dashes as interrupts.
