<!--
ScanView

Purpose: Common scan shell. Always shows DiskUsageProgress at top; body switches between ScanResults, ScanResultsDelete, or ScanResultsDeleteComplete.

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
import DiskUsageProgress from './DiskUsageProgress.vue'
import ScanResults from './ScanResults.vue'
import ScanResultsDelete from './ScanResultsDelete.vue'
import ScanResultsDeleteComplete from './ScanResultsDeleteComplete.vue'

import { ref } from 'vue'

import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structures'

const selectedSize = ref(0)
const viewState = ref<'results' | 'delete' | 'deleteComplete'>('results')
const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)

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
   viewState.value = 'deleteComplete'
}

function onScanAgain() {
   viewState.value = 'results'
   emit('start-scan')
}
</script>

<template>
   <main class="ScanView-root">
      <DiskUsageProgress :selectedSize="selectedSize" />
      <ScanResults
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
      <ScanResultsDelete
         v-show="viewState === 'delete'"
         class="ScanView-body"
         :items="deleteItems"
         :active="viewState === 'delete'"
         @back="viewState = 'results'"
         @update:selectedSize="onSelectedSizeUpdate"
         @complete="onDeleteComplete"
      />
      <ScanResultsDeleteComplete
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
