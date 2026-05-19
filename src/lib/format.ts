// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { APP_LANGUAGES_TO_LOCALE_MAP } from './constants'

/** Formats bytes into human-readable string (e.g. "1.2 GB").
 *  Uses SI decimal units (1 KB = 1000 B) to match macOS disk size reporting. */
export function formatBytes(bytes: unknown): string {
   if (typeof bytes === 'string') bytes = parseFloat(bytes)
   if (typeof bytes !== 'number') return 'Unknown'
   if (isNaN(bytes)) return 'Unknown'

   if (bytes === 0) return '0 B'

   const k = 1000
   const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
   const i = Math.floor(Math.log(bytes) / Math.log(k))

   return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`
}

/**
 * Formats a progress value for display: integer when whole (e.g. 100.0 → "100"),
 * otherwise one decimal (e.g. 32.5 → "32.5").
 */
export function formatProgressNumber(n: number): string {
   return n % 1 === 0 ? String(Math.round(n)) : n.toFixed(1)
}

/** Formats a year range for display: single year when same (e.g. 2026 → "2026"), otherwise range (e.g. 2026 - 2027 → "2026 - 2027"). */
export function formatYearRange(start: number, end: number): string {
   return start === end ? String(start) : `${start} - ${end}`
}

/** Formats a Unix timestamp into a localized date string based on language. */
export function formatDate(timestamp: number, languageCode: string): string {
   if (!timestamp || timestamp <= 0) return ''

   const locale =
      APP_LANGUAGES_TO_LOCALE_MAP[languageCode as keyof typeof APP_LANGUAGES_TO_LOCALE_MAP]

   if (!locale) {
      console.warn(`Unknown language code: ${languageCode}`)

      return new Date(timestamp * 1000).toLocaleDateString(undefined, { dateStyle: 'short' })
   }

   const date = new Date(timestamp * 1000)

   return date.toLocaleDateString(locale, { dateStyle: 'short' })
}
