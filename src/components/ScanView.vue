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

import { ref } from 'vue'

import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structures'

const selectedSize = ref(0)
const viewState = ref<'results' | 'delete' | 'deleteComplete'>('results')
const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const diskUsageRef = ref<InstanceType<typeof ScanViewDiskUsage> | null>(null)

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
}>()

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   viewState.value = 'delete'
}

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
   emit('start-scan')
}
</script>

<template>
   <main class="ScanView-root">
      <ScanViewDiskUsage ref="diskUsageRef" :selectedSize="selectedSize" />
      <ScanResultsList
         v-show="viewState === 'results'"
         class="ScanView-body"
         :folders="folders"
         :loading="loading"
         :progress="progress"
         @start-scan="$emit('start-scan')"
         @abort="$emit('abort')"
         @update:selectedSize="onSelectedSizeUpdate"
         @review="onReview"
      />
      <ScanResultsDeleteList
         v-if="viewState === 'delete'"
         class="ScanView-body"
         :items="deleteItems"
         :active="true"
         @back="viewState = 'results'"
         @update:selectedSize="onSelectedSizeUpdate"
         @complete="onDeleteComplete"
      />
      <ScanResultsDeleteConfirmation
         v-if="viewState === 'deleteComplete'"
         class="ScanView-body"
         :deletedSummary="deletedSummary"
         @scan-again="onScanAgain"
      />
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
