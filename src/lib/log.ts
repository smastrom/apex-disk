import { invoke } from '@tauri-apps/api/core'

type LogCategory = 'view' | 'nav' | 'scan' | 'file' | 'delete'

let _enabled = false
let _isDev = false

/**
 * Initializes the logger.
 *
 * - Dev builds (`import.meta.env.DEV`): always enabled.
 * - Prod builds: enabled when the user launches the app with
 *   `MAC_DISK_TREE_DEBUG=1` (checked at runtime via Rust).
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

export function log(category: LogCategory, message: string, data?: unknown): void {
   if (!_enabled) return

   const time = new Date().toISOString().slice(11, 23)
   const line = `[${time}] [${category.toUpperCase()}] ${message}`

   // DevTools console (dev only — users won't have it open)
   if (_isDev) {
      if (data !== undefined) console.log(line, data)
      else console.log(line)
   }

   // CLI stdout (always when enabled — visible to both dev and users)
   const cliLine = data !== undefined ? `${line} ${JSON.stringify(data)}` : line
   invoke('log_message', { message: cliLine }).catch(() => {})
}
