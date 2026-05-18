// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

import { log } from './log'

export interface UseAppUpdateOptions {
   /** Silently check for updates on app start. Required for autoInstallUpdates. */
   autoCheckUpdates: boolean
   /** Silently download + stage updates after a successful check. Implies autoCheckUpdates. */
   autoInstallUpdates: boolean
}

export function useAppUpdate(options: UseAppUpdateOptions) {
   const { autoCheckUpdates, autoInstallUpdates } = options

   const isChecking = ref(false)
   const isDownloading = ref(false)
   const availableVersion = ref<string | null>(null)
   const updateReady = ref(false)

   /** Silent check on startup. Runs when autoCheckUpdates is enabled. */
   async function checkSilently() {
      if (import.meta.env.DEV) return
      if (!autoCheckUpdates) return
      if (isChecking.value) return

      log('app', 'Updates: silent check…')

      try {
         isChecking.value = true
         availableVersion.value = await invoke<string | null>('check_for_updates_silent')
         if (availableVersion.value) {
            log('app', `Updates: available v${availableVersion.value}`)
         } else {
            log('app', 'Updates: none available')
         }
      } catch (error) {
         log('app', `Updates: silent check failed: ${error}`)
      } finally {
         isChecking.value = false
      }

      if (!availableVersion.value) return

      if (autoInstallUpdates) {
         await downloadSilently()
      } else {
         // Check-only mode: surface availability in the menu without downloading.
         try {
            await invoke('set_update_menu_available', { version: availableVersion.value })
         } catch (error) {
            log('app', `Updates: menu update failed: ${error}`)
         }
      }
   }

   /** Downloads the staged update silently. Sets `updateReady` on success. */
   async function downloadSilently() {
      if (isDownloading.value || updateReady.value) return
      log('app', 'Downloading update…')
      try {
         isDownloading.value = true
         const version = await invoke<string>('download_update')
         updateReady.value = true
         log('app', `Updates: v${version} downloaded, ready to install`)
      } catch (error) {
         log('app', `Updates: download failed: ${error}`)
      } finally {
         isDownloading.value = false
      }
   }

   /**
    * Handles the Settings update button click.
    *
    * - Update already staged → restart immediately (skip the redundant dialog flow).
    * - Otherwise → run the native dialog flow (check → download → restart prompt).
    *   `isChecking` is held for the duration so the button shows a spinner; the Rust
    *   side emits no progress events, so the check + download phases are one state.
    */
   async function onCheckForUpdates() {
      if (updateReady.value) {
         await invoke('restart_app')
         return
      }
      try {
         isChecking.value = true
         await invoke('check_for_updates_dialog')
      } catch (error) {
         log('app', `Updates: dialog flow failed: ${error}`)
      } finally {
         isChecking.value = false
      }
   }

   checkSilently()

   return { isChecking, isDownloading, availableVersion, updateReady, onCheckForUpdates }
}
