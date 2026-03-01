import { ROOT_THEME, RTL_LANGUAGES } from '@/lib/constants'

/** Applies theme to document. ROOT_THEME uses :root in CSS; others set data-theme. */
export function applyTheme(theme: string): void {
   if (theme === ROOT_THEME) {
      document.documentElement.removeAttribute('data-theme')
   } else {
      document.documentElement.setAttribute('data-theme', theme)
   }
}

/** Sets dir and lang attributes on <html> based on the active language. */
export function applyDirection(lang: string): void {
   const dir = RTL_LANGUAGES.has(lang) ? 'rtl' : 'ltr'
   document.documentElement.setAttribute('dir', dir)
   document.documentElement.setAttribute('lang', lang)
}
