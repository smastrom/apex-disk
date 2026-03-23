import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

export function useAppUpdate() {
   const isChecking = ref(false)
   const isDownloading = ref(false)
   const availableVersion = ref<string | null>(null)
   const updateReady = ref(false)

   /** Silent check — updates the reactive state without showing dialogs. */
   async function checkSilently() {
      if (isChecking.value) return
      try {
         isChecking.value = true
         availableVersion.value = await invoke<string | null>('check_for_updates_silent')
      } catch (error) {
         console.error('Silent update check failed:', error)
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
      try {
         isDownloading.value = true
         await invoke<string>('download_update')
         updateReady.value = true
      } catch (error) {
         console.error('Silent update download failed:', error)
      } finally {
         isDownloading.value = false
      }
   }

   /** Restarts the app to apply the staged update. */
   async function restartToUpdate() {
      await invoke('restart_app')
   }

   /**
    * Handles the "Check for Updates" / "Restart to Update" button click.
    * If an update is already staged, restarts. Otherwise checks + downloads.
    */
   async function onCheckForUpdates() {
      if (updateReady.value) {
         await restartToUpdate()
         return
      }

      if (isChecking.value || isDownloading.value) return

      try {
         isChecking.value = true
         availableVersion.value = await invoke<string | null>('check_for_updates_silent')
      } catch (error) {
         console.error('Error checking for updates:', error)
      } finally {
         isChecking.value = false
      }

      // If an update was found, download it
      if (availableVersion.value) {
         await downloadSilently()
      }
   }

   // Auto-check on app start
   checkSilently()

   return { isChecking, isDownloading, availableVersion, updateReady, onCheckForUpdates }
}
