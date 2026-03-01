import { ROOT_THEME } from '@/lib/constants'

/** Applies theme to document. ROOT_THEME uses :root in CSS; others set data-theme. */
export function applyTheme(theme: string): void {
   if (theme === ROOT_THEME) {
      document.documentElement.removeAttribute('data-theme')
   } else {
      document.documentElement.setAttribute('data-theme', theme)
   }
}
