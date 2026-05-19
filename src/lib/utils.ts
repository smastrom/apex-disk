// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * Simple debounce function.
 */
export function debounce<T extends (...args: any[]) => any>(
   fn: T,
   delay: number
): (...args: Parameters<T>) => void {
   let timeoutId: ReturnType<typeof setTimeout> | null = null

   return (...args: Parameters<T>) => {
      if (timeoutId) clearTimeout(timeoutId)

      timeoutId = setTimeout(() => fn(...args), delay)
   }
}

/**
 * No-operation function.
 */
export const noop = () => {}

/**
 * Extracts lowercase extension from filename (e.g. "file.PDF" → "pdf").
 */
export function getExtension(name: string): string {
   const lastDot = name.lastIndexOf('.')

   if (lastDot === -1) return ''

   return name.slice(lastDot + 1).toLowerCase()
}

/**
 * Formats a full path for display: /Users/<name>/… → ~/…
 */
export function displayPath(path: string): string {
   return path.replace(/^\/Users\/[^/]+/, '~')
}

/**
 * Checks if a file/folder name starts with a dot (hidden file).
 */
export function isHidden(name: string): boolean {
   return name.startsWith('.')
}

/** True when the app is running under WebDriver automation. */
export const isWebDriverSession = typeof navigator !== 'undefined' && navigator.webdriver === true

export const isDev = () => import.meta.env.DEV
