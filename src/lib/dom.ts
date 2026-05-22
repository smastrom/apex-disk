// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { RTL_LANGUAGES } from '@/lib/constants'

export function applyTheme(theme: string): void {
   document.documentElement.setAttribute('data-theme', theme)
}

/** Sets dir and lang attributes on <html> based on the active language. */
export function applyDirection(lang: string): void {
   const dir = RTL_LANGUAGES.has(lang) ? 'rtl' : 'ltr'

   document.documentElement.setAttribute('dir', dir)
   document.documentElement.setAttribute('lang', lang)
}
