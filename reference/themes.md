# Themes

ApexDisk ships **8 themes**, applied via a `data-theme` attribute on `<html>` and resolved with CSS variables. Pure frontend — no Rust involvement except the persisted setting.

## How it works

1. Settings UI changes the theme.
2. `useAppSettings().setTheme(name)` writes the setting via `update_setting` ([`tauri-commands.md`](tauri-commands.md) — Settings flow).
3. The same setter also writes `data-theme="<name>"` on `document.documentElement`.
4. Every CSS variable in `src/assets/css/theme.css` is defined under a `[data-theme="<name>"]` block. The attribute switch swaps the entire palette instantly — no class toggles, no JS re-renders.

```css
[data-theme='aurora'] {
   --color-bg: oklch(...);
   --color-text: oklch(...);
   --color-accent: oklch(...);
   /* ... */
}

[data-theme='midnight'] {
   --color-bg: oklch(...);
   /* ... */
}
```

The default theme is set on first launch via `src-tauri/src/constants.rs::DEFAULT_THEME` and persisted on first read.

## Variables

Every themed value is a CSS variable. Components **must not hardcode colors, spacing, font sizes, or border radii** — see [`code-style.md`](code-style.md) for the rule. Examples of variable categories:

- **Color** — `--color-bg`, `--color-text`, `--color-accent`, `--color-chrome`, `--color-chrome-border`, `--color-accent-glow`, …
- **Spacing** — `--spacing-xs/sm/md/lg`
- **Font size** — `--font-size-xs/sm/md/lg/xl`
- **Border radius** — `--radius-sm/md/lg`
- **Touch height** — `--touch-height-sm/default/lg`
- **Effects** — `--glow-text`, `--shadow-card`

Spacing, font sizes, and touch-target heights are **shared across themes** — they live at `:root` in `theme.css`, not inside the `[data-theme]` blocks. Only colors and color-derived effects (glows, shadows) vary per theme.

## Theme set

The 8 themes ship in `src/assets/css/theme.css`. Names are kebab-case identifiers used both as the `data-theme` attribute value and as the persisted setting value.

If the persisted theme name doesn't match any defined theme (e.g. after removing one in a release), the `[data-theme]` selector silently fails to match and the fallback `:root` values render — usually unreadable. The Settings picker is the source of truth for valid names; keep it in sync with `theme.css`.

## Adding a new theme

1. Pick a kebab-case name.
2. Add a `[data-theme='<name>']` block to `src/assets/css/theme.css` defining **every** color variable that varies across themes (look at an existing theme block as the template — don't leave gaps, since CSS fallback would inherit from `:root` which is usually wrong).
3. Add the name to the Settings theme picker UI.
4. Add a translation entry for the human-readable label under the appropriate translations module (see [`translations.md`](translations.md)).
5. Visually QA all main views in the new theme — scan results list, settings, information view, trash confirmation. Watch for contrast on the focus ring (`--color-accent-glow`) and on `--color-chrome-border` against `--color-chrome`.

## Reduced motion

Not a theme, but lives in adjacent CSS: `src/lib/use-reduced-motion.ts` plus `@media (prefers-reduced-motion: reduce)` blocks in `src/assets/css/animations.css`. Theme switches respect reduced-motion automatically since they're attribute swaps with no transition.

## Module index

| Location                            | What                                                              |
| ----------------------------------- | ----------------------------------------------------------------- |
| `src/assets/css/theme.css`          | All CSS variables: `:root` (cross-theme) + `[data-theme]` blocks  |
| `src/stores/app-settings.ts`        | `setTheme` — persists + writes `data-theme` on `<html>`           |
| `src/components/SettingsView.vue`   | Theme picker UI                                                   |
| `src-tauri/src/constants.rs`        | `DEFAULT_THEME` — initial value on first launch                   |
| `src/assets/css/animations.css`     | Animation declarations + `prefers-reduced-motion` overrides       |
| `src/lib/use-reduced-motion.ts`     | Reactive `prefers-reduced-motion` query                           |
