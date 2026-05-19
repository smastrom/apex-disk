<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewHeader at top; body switches between ScanResults, ScanTrash, or ScanTrashConfirmation. Scanner state is owned by AppLayout.vue (so it survives switches to Settings / Information) and passed in as props.

Props: diskUsage (DiskUsage | null), folders (FolderInfo[]), isScanning (boolean), progress (ScanProgress), elapsedSeconds (number), loadFolders (() => void), onAbort (() => void), onCancel (() => void)

Example:
 <ScanView :diskUsage="diskUsage" :folders="folders" :isScanning="isScanning" :progress="progress" :elapsedSeconds="elapsedSeconds" :loadFolders="loadFolders" :onAbort="onAbort" :onCancel="onCancel" />
-->

<script setup lang="ts">
import ScanLaunch from './ScanLaunch.vue'
import ScanProgress from './ScanProgress.vue'
import ScanResultsList from './ScanResultsList.vue'
import ScanTrashConfirmation from './ScanTrashConfirmation.vue'
import ScanTrashList from './ScanTrashList.vue'
import ScanViewHeader from './ScanViewHeader.vue'

import type { DiskUsage } from '@/types/disk'
import type { FolderInfo, ScanProgress as ScanProgressData, TrashListItem } from '@/types/structs'

import { nextTick, onActivated, onDeactivated, ref, toRef, useTemplateRef, watch } from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'

const props = defineProps<{
   diskUsage?: DiskUsage | null
   folders: FolderInfo[]
   isScanning: boolean
   progress: ScanProgressData
   elapsedSeconds: number
   loadFolders: () => void
   onAbort: () => void
   onCancel: () => void
}>()

const folders = toRef(props, 'folders')
const isScanning = toRef(props, 'isScanning')
const onCancel = () => props.onCancel()

enum ActiveView {
   LAUNCH = 'launch',
   SCANNING = 'scanning',
   RESULTS = 'results',
   TRASH = 'trash',
   TRASH_COMPLETE = 'trashComplete',
}

const activeView = ref<ActiveView>(ActiveView.LAUNCH)
const isScanViewActivated = ref(true)
const pendingActiveView = ref<ActiveView | null>(null)

/**
 * Keeps the rendered body in sync with scanner state without patching the
 * cached subtree while KeepAlive has moved it out of the visible DOM.
 */
function requestActiveView(nextView: ActiveView) {
   if (!isScanViewActivated.value) {
      pendingActiveView.value = nextView

      return
   }

   applyActiveView(nextView)
}

/** Applies the requested scan body and logs the user-visible view state. */
function applyActiveView(nextView: ActiveView) {
   if (nextView === activeView.value) return

   if (nextView === ActiveView.SCANNING) {
      log('view', 'Scan: view scanning')
   } else if (nextView === ActiveView.RESULTS) {
      log('view', 'Scan: view results')
   } else if (nextView === ActiveView.LAUNCH) {
      log('view', 'Scan: view launch')
   }

   activeView.value = nextView
}

watch(
   [() => isScanning.value, () => folders.value.length],
   ([isScanning, folderCount]) => {
      if (isScanning) {
         requestActiveView(ActiveView.SCANNING)
      } else if (folderCount === 0) {
         requestActiveView(ActiveView.LAUNCH)
      } else {
         requestActiveView(ActiveView.RESULTS)
      }
   },
   { immediate: true }
)

const deleteItems = ref<TrashListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const selectedSize = ref(0)
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
const pendingSelection = ref<TrashListItem[] | null>(null)
const pendingReset = ref(false)

/** When Abort/cancel clears folders and we return to ScanLaunch, reset all scan state. */

function resetInternalState() {
   selectedSize.value = 0

   requestActiveView(ActiveView.LAUNCH)

   deleteItems.value = []
   deletedSummary.value = null
   pendingSelection.value = null
}

watch(
   () => folders.value.length,
   (length) => {
      if (length === 0) {
         resetInternalState()
      }
   }
)

/** Skip the inner fade across KeepAlive reactivation so SCANNING -> RESULTS doesn't flash. */
const suppressInnerTransition = ref(false)

onDeactivated(() => {
   isScanViewActivated.value = false
   suppressInnerTransition.value = true

   if (activeView.value === ActiveView.TRASH_COMPLETE) {
      pendingActiveView.value = ActiveView.LAUNCH

      onCancel()
   }
})

onActivated(() => {
   isScanViewActivated.value = true

   if (pendingActiveView.value) {
      applyActiveView(pendingActiveView.value)

      pendingActiveView.value = null
   }

   nextTick(() => {
      suppressInnerTransition.value = false
   })
})

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: TrashListItem[]) {
   log('view', `Scan: view trash review (${items.length} items)`)

   deleteItems.value = items
   activeView.value = ActiveView.TRASH
}

function onBackFromTrash(checkedItems: TrashListItem[]) {
   pendingSelection.value = checkedItems
   activeView.value = ActiveView.RESULTS
}

function onResetFromTrash() {
   pendingReset.value = true
   pendingSelection.value = null
   activeView.value = ActiveView.RESULTS
}

watch(resultsListRef, (ref) => {
   if (!ref) return

   if (pendingReset.value) {
      ref.resetAll()

      pendingReset.value = false
   } else if (pendingSelection.value) {
      ref.setSelectedItems(pendingSelection.value)

      pendingSelection.value = null
   }
})

function onTrashComplete(summary: { count: number; size: number }) {
   log('trash', `Trash: complete ${summary.count} item(s), ${formatBytes(summary.size)}`)

   deletedSummary.value = summary
   activeView.value = ActiveView.TRASH_COMPLETE
}

function onRestart() {
   activeView.value = ActiveView.RESULTS

   onCancel()
}
</script>

<template>
   <section class="ScanView-root" aria-label="Scan">
      <ScanViewHeader :usage="diskUsage" :selectedSize="selectedSize" />

      <Transition :name="suppressInnerTransition ? 'no-fade' : 'fade'" mode="out-in">
         <KeepAlive>
            <ScanLaunch
               v-if="activeView === ActiveView.LAUNCH"
               class="ScanView-body"
               @start-scan="loadFolders"
            />

            <ScanProgress
               v-else-if="activeView === ActiveView.SCANNING"
               class="ScanView-body"
               :progress="progress"
               :elapsedSeconds="elapsedSeconds"
               @abort="onAbort"
            />

            <ScanResultsList
               ref="resultsListRef"
               v-else-if="activeView === ActiveView.RESULTS"
               class="ScanView-body"
               :folders="folders"
               @update:selectedSize="onSelectedSizeUpdate"
               @review="onReview"
               @cancel="onCancel"
            />

            <ScanTrashList
               v-else-if="activeView === ActiveView.TRASH"
               class="ScanView-body"
               :items="deleteItems"
               @back="onBackFromTrash"
               @update:selectedSize="onSelectedSizeUpdate"
               @complete="onTrashComplete"
               @cancel="onCancel"
               @reset="onResetFromTrash"
            />

            <ScanTrashConfirmation
               v-else-if="activeView === ActiveView.TRASH_COMPLETE"
               class="ScanView-body"
               :deletedSummary="deletedSummary"
               @restart="onRestart"
            />
         </KeepAlive>
      </Transition>
   </section>
</template>

<style scoped>
.ScanView-root {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   overflow: hidden;
   background: var(--color-bg);
}

.ScanView-body {
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   overflow: hidden;
}
</style>
