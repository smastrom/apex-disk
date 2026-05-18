# Translations

Per-component i18n for the webview. Native menu strings live in Rust ([`tauri-commands.md`](tauri-commands.md) — Locale + native menu).

## Languages

Ten supported languages, keyed under each phrase:

`en, it, es, fr, pt, de, ru, zh, ja, ar`

The default falls back to `en` when a key is missing for the requested language.

## File layout

```
src/assets/translations/
├── index.ts            # Factory: combines all module files, exposes useTranslations()
├── app-shell.yaml      # Per-module YAML files (one per Vue component or feature)
├── scan-launch.yaml
├── scan-results-list.yaml
├── settings-view.yaml
└── ...
```

Each YAML file is **key-first** — one entry per phrase, with a language sub-key:

```yaml
scan_again:
   en: Scan again
   it: Scansiona di nuovo
   es: Volver a escanear
   fr: Analyser à nouveau
   pt: Verificar novamente
   de: Erneut scannen
   ru: Сканировать снова
   zh: 重新扫描
   ja: 再スキャン
   ar: مسح مرة أخرى
```

## Loading

YAML files are imported directly as JavaScript objects via **`@rollup/plugin-yaml`**, registered in `vite.config.ts`. No runtime YAML parsing — the build inlines the structured data.

## Usage in components

```ts
import { useTranslations } from '@/lib/use-translations'

const { t } = useTranslations()

t('scan-launch', 'scan_again') // → "Scan again" / current lang
t('settings-view', 'theme_label', { name: 'Sky' }) // with vars
```

Signature: `t(module, key, vars?)`. The `module` argument is the YAML filename (kebab-case, no extension). Interpolation tokens are `{name}` style.

`useTranslations()` is reactive — when the user changes language in Settings (which writes to the store, which also fires `set_menu_language` for the native menu), all `t()` calls re-evaluate.

## YAML formatting rules

### Folded scalars (`>-`) for long prose

Use the **`>-`** folded-block style only for prose that genuinely needs line wrapping in the source for readability:

```yaml
welcome_paragraph:
   en: >-
      ApexDisk scans your home folder and shows you what's
      taking up space. Select what you want to delete and we'll
      move it to the Trash — recoverable until you empty it.
```

`>-` joins continuation lines with a single space and trims the trailing newline. Useful for English/Latin-script languages where words are space-delimited.

### CJK (Chinese, Japanese) constraint

**`zh` and `ja` values must stay on a single line.** `>-` inserts a space at every line join, and CJK text has no inter-word spaces — folded multi-line CJK would corrupt the output with stray spaces inside words.

```yaml
welcome_paragraph:
   en: >-
      ApexDisk scans your home folder and shows you
      what's taking up space.
   zh: ApexDisk 会扫描你的主目录，告诉你什么文件占用了空间。 # one line, no folding
   ja: ApexDisk はホームフォルダをスキャンし、容量を占有しているものを表示します。 # one line, no folding
```

If a CJK value truly is too long to fit, accept the long line — do not break it with `>-`.

## Adding a new phrase

1. Pick the right module file (matches the component using it). If no module file exists for the component, create one as `kebab-case-of-component.yaml`.
2. Add the key with **all 10 language values**. Use English-first; translate the others.
3. Use the key in the component: `t('module-name', 'new_key')`.

## Adding a new language

1. Add the language code to the supported set in `src/lib/use-translations.ts` (and `src-tauri/src/locale.rs` / `menu_translations.rs` if the native menu should also rebuild for it).
2. Add the language entry under **every** existing key across **every** YAML file. There is no fallback at the file level — a missing key falls back to `en`, but a missing language entry would leave that key as `en` everywhere.
3. Run the app and verify the Settings language picker exposes the new language.
4. Adding RTL languages (e.g. Arabic is already supported) requires `dir="rtl"` handling — see `src/assets/css/rtl.css`.

## Module index

| Location                             | What                                                         |
| ------------------------------------ | ------------------------------------------------------------ |
| `src/assets/translations/*.yaml`     | Per-module key-first translation files                       |
| `src/assets/translations/index.ts`   | Factory: combines all module files                           |
| `src/lib/use-translations.ts`        | `useTranslations()` composable + `t(module, key, vars?)` API |
| `vite.config.ts`                     | `@rollup/plugin-yaml` registration                           |
| `src-tauri/src/locale.rs`            | macOS `AppleLanguages` detection, system language resolution |
| `src-tauri/src/menu_translations.rs` | Native menu labels per language (separate from webview YAML) |
| `src/assets/css/rtl.css`             | Right-to-left layout overrides for Arabic                    |
