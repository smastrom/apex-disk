<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewHeader at top; body switches between ScanResults, ScanTrash, or ScanTrashConfirmation.

Props: diskUsage (DiskUsage | null)

Example:
 <ScanView :diskUsage="diskUsage" />
-->

<script setup lang="ts">
import ScanViewHeader from './ScanViewHeader.vue'
import ScanTrashList from './ScanTrashList.vue'
import ScanTrashConfirmation from './ScanTrashConfirmation.vue'
import ScanResultsList from './ScanResultsList.vue'
import ScanProgress from './ScanProgress.vue'
import ScanLaunch from './ScanLaunch.vue'

import { ref, watch, onDeactivated, useTemplateRef } from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useScanner } from '@/lib/use-scanner'

import type { TrashListItem } from '@/types/structs'
import type { DiskUsage } from '@/types/disk'

defineProps<{
   diskUsage?: DiskUsage | null
}>()

const { folders, isScanning, progress, elapsedSeconds, loadFolders, onAbort, onCancel } =
   useScanner()

enum ActiveView {
   LAUNCH = 'launch',
   SCANNING = 'scanning',
   RESULTS = 'results',
   TRASH = 'trash',
   TRASH_COMPLETE = 'trashComplete',
}

const activeView = ref<ActiveView>(ActiveView.LAUNCH)

watch(
   [() => isScanning.value, () => folders.value.length],
   ([isScanning, folderCount]) => {
      if (isScanning) {
         log('view', 'Scan: view scanning')
         activeView.value = ActiveView.SCANNING
      } else if (folderCount === 0) {
         log('view', 'Scan: view launch')
         activeView.value = ActiveView.LAUNCH
      } else {
         log('view', 'Scan: view results')
         activeView.value = ActiveView.RESULTS
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
   activeView.value = ActiveView.LAUNCH
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

onDeactivated(() => {
   // If switching app view from this component and we're in TrashResults page
   if (activeView.value === ActiveView.TRASH_COMPLETE) {
      activeView.value = ActiveView.LAUNCH
      onCancel()
   }
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

      <Transition name="fade" mode="out-in">
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
