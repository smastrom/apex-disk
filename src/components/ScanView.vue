<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewDiskUsage at top; body switches between ScanResults, ScanResultsDelete, or ScanResultsDeleteConfirmation.

Props: activeView (string), diskUsage (DiskUsage | null)

Example:
 <ScanView
   :activeView="activeView"
   :diskUsage="diskUsage"
 />
-->

<script setup lang="ts">
import ScanViewDiskUsage from './ScanViewDiskUsage.vue'
import ScanResultsDeleteList from './ScanResultsDeleteList.vue'
import ScanResultsDeleteConfirmation from './ScanResultsDeleteConfirmation.vue'
import ScanResultsList from './ScanResultsList.vue'
import ScanScanningResults from './ScanScanningResults.vue'
import ScanLaunch from './ScanLaunch.vue'

import { ref, watch, onDeactivated, useTemplateRef } from 'vue'

import { useAppSettings } from '@/stores/settings'
import { useScanner } from '@/lib/use-scanner'
import { useDiskUsage } from '@/lib/use-disk-usage'

import type { DeleteListItem } from '@/types/structs'
import type { DiskUsage } from '@/types/disk'

const props = defineProps<{
   activeView: string
   diskUsage?: DiskUsage | null
}>()

const settingsStore = useAppSettings()

const { usage: diskUsage } = await useDiskUsage()
const { folders, isScanning, progress, loadFolders, onAbort, onCancel } = useScanner()

enum ScanViewState {
   LAUNCH = 'launch',
   SCANNING = 'scanning',
   RESULTS = 'results',
   DELETE = 'delete',
   DELETE_COMPLETE = 'deleteComplete',
}

// Centralized view state computation - single source of truth

const scanViewState = ref<ScanViewState>(ScanViewState.LAUNCH)

watch([() => isScanning.value, () => folders.value.length], ([isScanning, folderCount]) => {
   if (isScanning) {
      scanViewState.value = ScanViewState.SCANNING
   } else if (folderCount === 0) {
      scanViewState.value = ScanViewState.LAUNCH
   } else {
      scanViewState.value = ScanViewState.RESULTS
   }
})

const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const selectedSize = ref(0)
const diskUsageRefreshKey = ref(0)
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
const pendingSelection = ref<DeleteListItem[] | null>(null)

/** When Abort/cancel clears folders and we return to ScanLaunch, reset all scan state. */

function resetInternalState() {
   selectedSize.value = 0
   scanViewState.value = ScanViewState.LAUNCH
   deleteItems.value = []
   deletedSummary.value = null
   pendingSelection.value = null
   // Refresh disk usage to reflect changes after deletion
   diskUsageRefreshKey.value++
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
   // If switching app view from this component and we're in DeleteResults page
   if (scanViewState.value === ScanViewState.DELETE_COMPLETE) {
      scanViewState.value = ScanViewState.LAUNCH
      onCancel()
   }
})

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   scanViewState.value = ScanViewState.DELETE
}

function onBackFromDelete(checkedItems: DeleteListItem[]) {
   pendingSelection.value = checkedItems
   scanViewState.value = ScanViewState.RESULTS
}

watch(resultsListRef, (ref) => {
   if (ref && pendingSelection.value) {
      ref.setSelectedItems(pendingSelection.value)
      pendingSelection.value = null
   }
})

function onDeleteComplete(items: DeleteListItem[]) {
   deletedSummary.value = {
      count: items.length,
      size: items.reduce((sum, i) => sum + i.size, 0),
   }

   // Space is freed immediately if permanently deleting.
   // If moving to trash, space is only freed after emptying the trash,
   // but we still refresh to get the most accurate current state.
   if (settingsStore.settings.value.permanentlyDelete) {
      selectedSize.value = 0
   }
   diskUsageRefreshKey.value++

   scanViewState.value = ScanViewState.DELETE_COMPLETE
}

function onRestart() {
   scanViewState.value = ScanViewState.RESULTS
   diskUsageRefreshKey.value++
   onCancel()
}
</script>

<template>
   <main class="ScanView-root">
      <ScanViewDiskUsage :usage="diskUsage" :selectedSize="selectedSize" />

      <Transition name="fade" mode="out-in">
         <KeepAlive>
            <ScanLaunch
               v-if="scanViewState === ScanViewState.LAUNCH"
               class="ScanView-body"
               @start-scan="loadFolders"
            />

            <ScanScanningResults
               v-else-if="scanViewState === ScanViewState.SCANNING"
               class="ScanView-body"
               :progress="progress"
               @abort="onAbort"
            />

            <ScanResultsList
               ref="resultsListRef"
               v-else-if="scanViewState === ScanViewState.RESULTS"
               class="ScanView-body"
               :folders="folders"
               @update:selectedSize="onSelectedSizeUpdate"
               @review="onReview"
               @cancel="onCancel"
            />

            <ScanResultsDeleteList
               v-else-if="scanViewState === ScanViewState.DELETE"
               class="ScanView-body"
               :items="deleteItems"
               @back="onBackFromDelete"
               @update:selectedSize="onSelectedSizeUpdate"
               @complete="onDeleteComplete"
               @cancel="onCancel"
            />

            <ScanResultsDeleteConfirmation
               v-else-if="scanViewState === ScanViewState.DELETE_COMPLETE"
               class="ScanView-body"
               :deletedSummary="deletedSummary"
               @restart="onRestart"
            />
         </KeepAlive>
      </Transition>
   </main>
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
