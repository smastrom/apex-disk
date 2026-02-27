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

import { ref } from 'vue'

import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structures'

const selectedSize = ref(0)
const viewState = ref<'results' | 'delete' | 'deleteComplete'>('results')
const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const diskUsageRef = ref<InstanceType<typeof ScanViewDiskUsage> | null>(null)
const resultsListRef = ref<InstanceType<typeof ScanResultsList> | null>(null)

defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'cancel'): void
}>()

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   viewState.value = 'delete'
}

function onBackFromDelete(checkedPaths: string[]) {
   resultsListRef.value?.setSelectedPaths(new Set(checkedPaths))
   viewState.value = 'results'
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

      <Transition name="ScanView-fade" mode="out-in">
         <KeepAlive>
            <ScanScanningResults
               v-if="loading"
               class="ScanView-body"
               :progress="progress"
               @abort="$emit('abort')"
            />

            <ScanLaunch
               v-else-if="viewState === 'results' && folders.length === 0"
               class="ScanView-body"
               @start-scan="$emit('start-scan')"
            />

            <ScanResultsList
               ref="resultsListRef"
               v-else-if="viewState === 'results'"
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

.ScanView-fade-enter-active,
.ScanView-fade-leave-active {
   transition: opacity 0.22s cubic-bezier(0.4, 0, 0.2, 1);
}

.ScanView-fade-enter-from,
.ScanView-fade-leave-to {
   opacity: 0;
}
</style>
