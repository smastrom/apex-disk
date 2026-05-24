<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewHeader at top; body switches between ScanResults, ScanTrash, or ScanTrashConfirmation. Scanner state is owned by AppLayout.vue (so it survives switches to Settings / Information) and passed in as props.

Props: isActive (boolean), diskUsage (DiskUsage | null), folders (FolderInfo[]), isScanning (boolean), progress (ScanProgress), elapsedSeconds (number), loadFolders (() => void), onAbort (() => void), onCancel (() => void)

Example:
 <ScanView :isActive="activeView === 'scan'" :diskUsage="diskUsage" :folders="folders" :isScanning="isScanning" :progress="progress" :elapsedSeconds="elapsedSeconds" :loadFolders="loadFolders" :onAbort="onAbort" :onCancel="onCancel" />
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

import { ref, toRef, useTemplateRef, watch } from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { isWebDriverSession } from '@/lib/utils'

const props = defineProps<{
   isActive: boolean
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

function setBodyView(nextView: ActiveView) {
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
         setBodyView(ActiveView.SCANNING)
      } else if (folderCount === 0) {
         setBodyView(ActiveView.LAUNCH)
      } else {
         setBodyView(ActiveView.RESULTS)
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

   setBodyView(ActiveView.LAUNCH)

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

/** Leaving Scan while the success screen is up resets the flow for next entry. */
watch(
   () => props.isActive,
   (isActive, wasActive) => {
      if (wasActive && !isActive && activeView.value === ActiveView.TRASH_COMPLETE) {
         activeView.value = ActiveView.LAUNCH

         onCancel()
      }
   }
)

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

      <Transition name="fade" mode="out-in" :css="!isWebDriverSession">
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
