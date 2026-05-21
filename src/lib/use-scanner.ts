// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

import type { FolderInfo, ScanProgress, ScanResult } from '@/types/structs'

import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { markRaw, onMounted, onUnmounted, ref, shallowRef } from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useAppSettings } from '@/stores/app-settings'

const INITIAL_PROGRESS: ScanProgress = {
   current: 0,
   total: 1,
   folder: '',
   size: 0,
   scanned_size_total: 0,
   completed_size: 0,
}

/** Materialize each node's `path` from the home root + chain of `name`s
 *  (Rust skips `path` on the wire to shrink the IPC payload), then make the
 *  tree immutable. `markRaw` keeps Vue from ever wrapping a node in a Proxy;
 *  `Object.freeze` locks the shape so JSC/V8 can optimize property access. */
function hydrateTree(nodes: FolderInfo[], parentPath: string): void {
   for (const node of nodes) {
      node.path = `${parentPath}/${node.name}`

      if (node.children.length > 0) {
         hydrateTree(node.children, node.path)
      }

      Object.freeze(node.children)
      markRaw(node)
      Object.freeze(node)
   }
}

export function useScanner() {
   const settingsStore = useAppSettings()

   const folders = shallowRef<FolderInfo[]>([])
   const isScanning = ref(false)
   const hasFreshResults = ref(false)

   function markResultsSeen() {
      hasFreshResults.value = false
   }

   function markResultsPending() {
      hasFreshResults.value = true
   }

   /**
    * Generation counter that invalidates in-flight scans. Bumped on every new scan
    * (`loadFolders`) and on abort (`onAbort`). Each async callback compares the
    * captured `gen` snapshot against the current value; if they diverge, the
    * callback was spawned by a stale scan and is silently discarded. This avoids
    * race conditions when the user re-scans or aborts while a scan is still running.
    */
   const scanGeneration = ref(0)
   const progress = ref<ScanProgress>({ ...INITIAL_PROGRESS })

   /** Teardown handle for the Tauri `folder-scan-progress` event listener. */
   let unlistenProgress: (() => void) | null = null

   const elapsedSeconds = ref(0)

   let elapsedInterval: ReturnType<typeof setInterval> | null = null

   function startElapsed() {
      stopElapsed()

      elapsedSeconds.value = 0

      if (elapsedInterval) return // Guard against multiple concurrent timers

      elapsedInterval = setInterval(() => {
         elapsedSeconds.value++
      }, 1000)
   }

   function stopElapsed() {
      if (elapsedInterval) {
         clearInterval(elapsedInterval)

         elapsedInterval = null
      }

      elapsedSeconds.value = 0
   }

   async function loadFolders() {
      const settings = settingsStore.settings.value

      log('scan', 'Scan: started', {
         showHiddenFiles: settings.showHiddenFiles,
         showDsStore: settings.showDsStore,
         showUnder1Kb: settings.showUnder1Kb,
         showZeroByte: settings.showZeroByte,
      })

      // Clean up any previous scan's listener before starting a new one
      unlistenProgress?.()
      unlistenProgress = null

      scanGeneration.value += 1

      const gen = scanGeneration.value // snapshot — all callbacks below bail if gen is stale

      isScanning.value = true
      progress.value = { ...INITIAL_PROGRESS }

      startElapsed()

      unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
         if (gen === scanGeneration.value) progress.value = event.payload
      })

      try {
         const options = {
            show_hidden_files: settings.showHiddenFiles,
            show_ds_store: settings.showDsStore,
            show_under_1kb: settings.showUnder1Kb,
            show_zero_byte: settings.showZeroByte,
         }

         const result = await invoke<ScanResult>('get_user_folders', { options })

         if (gen === scanGeneration.value) {
            hydrateTree(result.folders, result.root)

            const totalSize = formatBytes(progress.value.scanned_size_total)

            log(
               'scan',
               `Scan: complete ${result.folders.length} folders, ${totalSize}, ${elapsedSeconds.value}s`
            )

            folders.value = result.folders
         }
      } catch (error) {
         log('scan', 'Scan: error', error)
      } finally {
         if (gen === scanGeneration.value) {
            unlistenProgress?.()
            unlistenProgress = null
            isScanning.value = false

            stopElapsed()
         }
      }
   }

   // Fire-and-forget on window reload: stops the Rust walker and releases
   // SCAN_RUNNING before the reloaded page tries to start a new scan.
   function abortOnUnload() {
      if (!isScanning.value) return

      void invoke('cancel_scan').catch(() => {})
   }

   onMounted(() => {
      window.addEventListener('beforeunload', abortOnUnload)
   })

   onUnmounted(() => {
      window.removeEventListener('beforeunload', abortOnUnload)
   })

   async function onAbort() {
      log('scan', 'Scan: aborted')

      scanGeneration.value += 1
      unlistenProgress?.()
      unlistenProgress = null

      folders.value = []
      isScanning.value = false
      hasFreshResults.value = false
      progress.value = { ...INITIAL_PROGRESS }

      stopElapsed()

      // Cancel any ongoing Rust scan to free memory
      try {
         await invoke('cancel_scan')
      } catch (error) {
         log('scan', 'Scan: failed to cancel', error)
      }
   }

   return {
      folders,
      isScanning,
      hasFreshResults,
      markResultsPending,
      markResultsSeen,
      progress,
      elapsedSeconds,
      loadFolders,
      onAbort,
      onCancel: onAbort,
   }
}
