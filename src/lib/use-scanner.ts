import { log } from '@/lib/log'
import { useAppSettings } from '@/stores/app-settings'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref, shallowRef } from 'vue'

import type { FolderInfo, ScanProgress } from '@/types/structs'

const INITIAL_PROGRESS: ScanProgress = {
   current: 0,
   total: 1,
   folder: '',
   size: 0,
   scanned_size_total: 0,
   completed_size: 0,
}

export function useScanner() {
   const settingsStore = useAppSettings()

   const folders = shallowRef<FolderInfo[]>([])
   const isScanning = ref(false)

   /**
    * Generation counter that invalidates in-flight scans. Bumped on every new scan
    * (`loadFolders`) and on abort (`onAbort`). Each async callback compares the
    * captured `gen` snapshot against the current value — if they diverge, the
    * callback was spawned by a stale scan and is silently discarded. This avoids
    * race conditions when the user re-scans or aborts while a scan is still running.
    */
   const scanGeneration = ref(0)
   const progress = ref<ScanProgress>({ ...INITIAL_PROGRESS })

   /** Teardown handle for the Tauri `folder-scan-progress` event listener. */
   let unlistenProgress: (() => void) | null = null

   async function loadFolders() {
      log('scan', 'Scan started')

      // Clean up any previous scan's listener before starting a new one
      unlistenProgress?.()
      unlistenProgress = null

      scanGeneration.value += 1
      const gen = scanGeneration.value // snapshot — all callbacks below bail if gen is stale
      isScanning.value = true
      progress.value = { ...INITIAL_PROGRESS }

      unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
         if (gen === scanGeneration.value) progress.value = event.payload
      })

      try {
         const settings = settingsStore.settings.value
         const options = {
            show_hidden_files: settings.showHiddenFiles,
            show_under_1kb: settings.showUnder1Kb,
            show_zero_byte: settings.showZeroByte,
         }

         const result = await invoke<FolderInfo[]>('get_user_folders', { options })

         if (gen === scanGeneration.value) {
            log('scan', `Scan complete: ${result.length} folders`)
            folders.value = result
         }
      } catch (error) {
         log('scan', 'Scan error', error)
         if (gen === scanGeneration.value) console.error('Error isLoading folders:', error)
      } finally {
         if (gen === scanGeneration.value) {
            unlistenProgress?.()
            unlistenProgress = null
            isScanning.value = false
         }
      }
   }

   async function onAbort() {
      log('scan', 'Scan aborted')
      scanGeneration.value += 1
      unlistenProgress?.()
      unlistenProgress = null

      folders.value = []
      isScanning.value = false
      progress.value = { ...INITIAL_PROGRESS }

      // Cancel any ongoing Rust scan to free memory
      try {
         await invoke('cancel_scan')
      } catch (error) {
         console.error('Failed to cancel scan:', error)
      }
   }

   return {
      folders,
      isScanning,
      progress,
      loadFolders,
      onAbort,
      onCancel: onAbort,
   }
}
