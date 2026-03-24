import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

import { log } from './log'

export function useAppUpdate(options: { autoUpdates: boolean }) {
   const { autoUpdates } = options
   const isChecking = ref(false)
   const isDownloading = ref(false)
   const availableVersion = ref<string | null>(null)
   const updateReady = ref(false)

   /** Silent check + auto-download on startup. Only runs when autoUpdates is enabled. */
   async function checkSilently() {
      if (import.meta.env.DEV) return
      if (!autoUpdates) return
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
         log('app', `Updates: silent check failed — ${error}`)
      } finally {
         isChecking.value = false
      }

      // Auto-download if an update was found
      if (availableVersion.value) {
         await downloadSilently()
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
         log('app', `Updates: download failed — ${error}`)
      } finally {
         isDownloading.value = false
      }
   }

   /**
    * Handles the "Check for Updates" button click.
    * Always triggers the native dialog flow (check → download → restart prompt).
    * Same behavior as the menu item click.
    */
   async function onCheckForUpdates() {
      await invoke('check_for_updates_dialog')
   }

   // Auto-check on app start (production only, auto-updates only)
   checkSilently()

   return { isChecking, isDownloading, availableVersion, updateReady, onCheckForUpdates }
}
