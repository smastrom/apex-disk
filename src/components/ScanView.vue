<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewDiskUsage at top; body switches between ScanResults, ScanResultsDelete, or ScanResultsDeleteConfirmation.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <ScanView
   :folders="folders"
   :loading="loading"
   :progress="progress"
   @start-scan="loadFolders"
   @abort="onAbort"
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

import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structs'

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'cancel'): void
}>()

const selectedSize = ref(0)
const viewState = ref<'results' | 'delete' | 'deleteComplete'>('results')
const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const diskUsageRef = useTemplateRef<InstanceType<typeof ScanViewDiskUsage>>('diskUsageRef')
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
const pendingSelection = ref<DeleteListItem[] | null>(null)

/** When Abort/cancel clears folders and we return to ScanLaunch, reset all scan state. */
watch(
   () => props.folders.length,
   (len) => {
      if (len === 0) {
         selectedSize.value = 0
         viewState.value = 'results'
         deleteItems.value = []
         deletedSummary.value = null
         pendingSelection.value = null
      }
   },
   { immediate: true }
)

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   viewState.value = 'delete'
}

function onBackFromDelete(checkedItems: DeleteListItem[]) {
   pendingSelection.value = checkedItems
   viewState.value = 'results'
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

   selectedSize.value = 0
   diskUsageRef.value?.refresh()
   viewState.value = 'deleteComplete'
}

function onScanAgain() {
   viewState.value = 'results'
   emit('cancel')
}

onDeactivated(() => {
   if (viewState.value === 'deleteComplete') {
      viewState.value = 'results'
      emit('cancel')
   }
})
</script>

<template>
   <main class="ScanView-root">
      <ScanViewDiskUsage ref="diskUsageRef" :selectedSize="selectedSize" />

      <Transition name="fade" mode="out-in">
         <KeepAlive>
            <ScanScanningResults
               v-if="loading"
               class="ScanView-body"
               :progress="progress"
               @abort="$emit('abort')"
            />

            <ScanLaunch
               v-else-if="
                  viewState === 'results' && folders.length === 0 /* TODO: Add its own viewState */
               "
               class="ScanView-body"
               @start-scan="$emit('start-scan')"
            />

            <ScanResultsList
               ref="resultsListRef"
               v-else-if="viewState === 'results' && folders.length > 0"
               class="ScanView-body"
               :folders="folders"
               @update:selectedSize="onSelectedSizeUpdate"
               @review="onReview"
               @cancel="$emit('cancel')"
            />

            <ScanResultsDeleteList
               v-else-if="viewState === 'delete'"
               class="ScanView-body"
               :items="deleteItems"
               :active="viewState === 'delete'"
               @back="onBackFromDelete"
               @update:selectedSize="onSelectedSizeUpdate"
               @complete="onDeleteComplete"
               @cancel="$emit('cancel')"
            />

            <ScanResultsDeleteConfirmation
               v-else-if="viewState === 'deleteComplete'"
               class="ScanView-body"
               :deletedSummary="deletedSummary"
               @scan-again="onScanAgain"
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
