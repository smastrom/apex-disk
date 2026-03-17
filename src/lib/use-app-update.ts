import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

/**
 * Dev mock for the silent auto-check on app start.
 * Set to a version string (e.g. '1.2.0') to simulate "update available" in the
 * SettingsView UI, or `null` to simulate "up to date".
 *
 * This does NOT affect the "Check for Updates" button or menu item — those
 * always invoke the real Rust updater command with native dialogs.
 *
 * Only active when running `pnpm tauri dev`.
 */
const DEV_MOCK_VERSION: string | null = null // '1.2.0'

export function useAppUpdate() {
   const isChecking = ref(false)
   const availableVersion = ref<string | null>(null)

   /** Silent check — updates the reactive state without showing dialogs. */
   async function checkSilently() {
      if (isChecking.value) return
      try {
         isChecking.value = true
         if (import.meta.env.DEV) {
            availableVersion.value = DEV_MOCK_VERSION
         } else {
            availableVersion.value = await invoke<string | null>('check_for_updates_silent')
         }
      } catch (error) {
         console.error('Silent update check failed:', error)
      } finally {
         isChecking.value = false
      }
   }

   /** Full update flow — triggers native dialogs (check → confirm → download → restart). */
   async function onCheckForUpdates() {
      if (isChecking.value) return
      try {
         isChecking.value = true
         await invoke('check_for_updates')
         // After the dialog flow completes, refresh the inline status
         if (!import.meta.env.DEV) {
            availableVersion.value = await invoke<string | null>('check_for_updates_silent')
         }
      } catch (error) {
         console.error('Error checking for updates:', error)
      } finally {
         isChecking.value = false
      }
   }

   // Auto-check on app start
   checkSilently()

   return { isChecking, availableVersion, onCheckForUpdates }
}
