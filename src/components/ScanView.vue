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
import ScanScanning from './ScanScanning.vue'
import ScanLaunch from './ScanLaunch.vue'

import { ref, watch, onDeactivated, useTemplateRef } from 'vue'

import { useScanner } from '@/lib/use-scanner'

import type { DeleteListItem } from '@/types/structs'
import type { DiskUsage } from '@/types/disk'

defineProps<{
   diskUsage?: DiskUsage | null
}>()

const { folders, isScanning, progress, loadFolders, onAbort, onCancel } = useScanner()

enum ActiveView {
   LAUNCH = 'launch',
   SCANNING = 'scanning',
   RESULTS = 'results',
   DELETE = 'delete',
   DELETE_COMPLETE = 'deleteComplete',
}

const activeView = ref<ActiveView>(ActiveView.LAUNCH)

watch(
   [() => isScanning.value, () => folders.value.length],
   ([isScanning, folderCount]) => {
      if (isScanning) {
         activeView.value = ActiveView.SCANNING
      } else if (folderCount === 0) {
         activeView.value = ActiveView.LAUNCH
      } else {
         activeView.value = ActiveView.RESULTS
      }
   },
   { immediate: true }
)

const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const selectedSize = ref(0)
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
const pendingSelection = ref<DeleteListItem[] | null>(null)

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
   // If switching app view from this component and we're in DeleteResults page
   if (activeView.value === ActiveView.DELETE_COMPLETE) {
      activeView.value = ActiveView.LAUNCH
      onCancel()
   }
})

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   activeView.value = ActiveView.DELETE
}

function onBackFromDelete(checkedItems: DeleteListItem[]) {
   pendingSelection.value = checkedItems
   activeView.value = ActiveView.RESULTS
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

   activeView.value = ActiveView.DELETE_COMPLETE
}

function onRestart() {
   activeView.value = ActiveView.RESULTS
   onCancel()
}
</script>

<template>
   <main class="ScanView-root">
      <ScanViewDiskUsage :usage="diskUsage" :selectedSize="selectedSize" />

      <Transition name="fade" mode="out-in">
         <KeepAlive>
            <ScanLaunch
               v-if="activeView === ActiveView.LAUNCH"
               class="ScanView-body"
               @start-scan="loadFolders"
            />

            <ScanScanning
               v-else-if="activeView === ActiveView.SCANNING"
               class="ScanView-body"
               :progress="progress"
               :isScanning="isScanning"
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

            <ScanResultsDeleteList
               v-else-if="activeView === ActiveView.DELETE"
               class="ScanView-body"
               :items="deleteItems"
               @back="onBackFromDelete"
               @update:selectedSize="onSelectedSizeUpdate"
               @complete="onDeleteComplete"
               @cancel="onCancel"
            />

            <ScanResultsDeleteConfirmation
               v-else-if="activeView === ActiveView.DELETE_COMPLETE"
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
