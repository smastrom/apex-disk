// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/**
 * ApexDisk diagnostic logging (Vue → Terminal via Tauri IPC).
 *
 * **Purpose:** Structured `[apex:vue:…]` lines for dev and bug reports (`APEX_DISK_DEBUG=1`), plus global handlers for uncaught web errors on stderr.
 *
 * **Docs:** Categories, Rust `[apex:rust:…]` channels, streams, and source index — **`docs/LOGGING.md`**.
 *
 * **Example:**
 *
 * ```ts
 * import { log } from '@/lib/log'
 * log('scan', 'Scan started', options)
 * ```
 */

import { invoke } from '@tauri-apps/api/core'

import type { App } from 'vue'

export type LogCategory = 'app' | 'disk' | 'view' | 'nav' | 'scan' | 'file' | 'trash' | 'settings'

let _enabled = false
let _isDev = false

function formatTime(): string {
   return new Date().toISOString().slice(11, 23)
}

function serializeDetail(detail: unknown): string {
   if (detail instanceof Error) {
      return detail.stack ?? detail.message
   }

   try {
      return JSON.stringify(detail)
   } catch {
      return String(detail)
   }
}

/**
 * Initializes the logger.
 *
 * - Dev builds (`import.meta.env.DEV`): always enabled (authoring / local QA).
 * - Prod builds: enabled only when the user launches with `APEX_DISK_DEBUG=1`
 *   (checked via Rust). Intended for **bug reports**—capture Terminal output and
 *   attach to an issue; not for everyday use.
 */
export async function initLog(): Promise<void> {
   if (import.meta.env.DEV) {
      _enabled = true
      _isDev = true
      return
   }

   try {
      _enabled = await invoke<boolean>('is_debug_mode')
   } catch {
      _enabled = false
   }
}

/**
 * Registers window-level and Vue error hooks so failures are labeled `[apex:vue/…]`.
 * In dev, uncaught issues always go to the WebView console; with diagnostic logging
 * they also mirror to the Terminal (stderr for errors).
 */
export function registerDiagnosticHandlers(app: App): void {
   window.addEventListener('error', (event) => {
      const where = event.filename ? `${event.filename}:${event.lineno}` : ''
      const summary = [event.message, where].filter(Boolean).join(' ')
      reportWebProblem('window-error', summary || 'window error', event.error)
   })

   window.addEventListener('unhandledrejection', (event) => {
      reportWebProblem('unhandled-rejection', String(event.reason), event.reason)
   })

   app.config.errorHandler = (err, _instance, info) => {
      const summary = err instanceof Error ? err.message : String(err)
      reportWebProblem('vue', `${summary} (${info})`, err)
   }
}

function reportWebProblem(kind: string, summary: string, detail?: unknown): void {
   const label = `[apex:vue/${kind}]`

   if (_isDev) {
      if (detail !== undefined) {
         console.error(label, summary, detail)
      } else {
         console.error(label, summary)
      }
   }

   if (!_enabled) {
      return
   }

   const time = formatTime()
   let line = `[${time}] ${label} ${summary}`

   if (detail !== undefined) {
      line = `${line} ${serializeDetail(detail)}`
   }

   invoke('log_error_message', { message: line }).catch(() => {})
}

export function log(category: LogCategory, message: string, data?: unknown): void {
   if (!_enabled) {
      return
   }

   const time = formatTime()
   const line = `[${time}] [apex:vue:${category}] ${message}`

   if (_isDev) {
      if (data !== undefined) {
         console.log(line, data)
      } else {
         console.log(line)
      }
   }

   const cliLine = data !== undefined ? `${line} ${JSON.stringify(data)}` : line
   invoke('log_message', { message: cliLine }).catch(() => {})
}
