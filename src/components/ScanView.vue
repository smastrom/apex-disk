<!--
ScanView

Purpose: Common scan shell. Always shows ScanViewDiskUsage at top; body switches between ScanResults, ScanResultsDelete, or ScanResultsDeleteConfirmation.

Props: folders (FolderInfo[]), isLoading (boolean), progress (ScanProgress)

Example:
 <ScanView
   :folders="folders"
   :isScanning="isScanning"
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

import { toRef, ref, computed, watch, onDeactivated, useTemplateRef } from 'vue'

import { useTauriAppSettings } from '@/stores/settings'

import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structs'

const props = defineProps<{
   activeView: string
   folders: FolderInfo[]
   isScanning: boolean
   progress: ScanProgress
}>()

const settingsStore = useTauriAppSettings()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'cancel'): void
}>()

enum ViewState {
   // LAUNCH - handled in viewStateLaunch computed
   // SCANNING - handled in viewStateScanning computed
   RESULTS = 'results',
   DELETE = 'delete',
   DELETE_COMPLETE = 'deleteComplete',
}

const viewStateLaunch = computed(
   () => viewState.value === ViewState.RESULTS && props.folders.length === 0
)

const viewStateScanning = toRef(props, 'isScanning')
const viewState = ref<ViewState>(ViewState.RESULTS)

const deleteItems = ref<DeleteListItem[]>([])
const deletedSummary = ref<{ count: number; size: number } | null>(null)
const selectedSize = ref(0)

const diskUsageRef = useTemplateRef<InstanceType<typeof ScanViewDiskUsage>>('diskUsageRef')
const resultsListRef = useTemplateRef<InstanceType<typeof ScanResultsList>>('resultsListRef')
const pendingSelection = ref<DeleteListItem[] | null>(null)

/** When Abort/cancel clears folders and we return to ScanLaunch, reset all scan state. */
watch(
   () => props.folders.length,
   (len) => {
      if (len === 0) {
         selectedSize.value = 0
         viewState.value = ViewState.RESULTS
         deleteItems.value = []
         deletedSummary.value = null
         pendingSelection.value = null
         diskUsageRef.value?.refresh()
      }
   },
   { immediate: true }
)

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}

function onReview(items: DeleteListItem[]) {
   deleteItems.value = items
   viewState.value = ViewState.DELETE
}

function onBackFromDelete(checkedItems: DeleteListItem[]) {
   pendingSelection.value = checkedItems
   viewState.value = ViewState.RESULTS
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
   diskUsageRef.value?.refresh()

   viewState.value = ViewState.DELETE_COMPLETE
}

function onRestart() {
   viewState.value = ViewState.RESULTS
   diskUsageRef.value?.refresh()
   emit('cancel')
}

watch(
   () => props.activeView,
   (newView, oldView) => {
      // Refresh disk usage when returning to the scan view if we are on the launch screen
      if (newView === 'scan' && props.folders.length === 0 && !props.isScanning) {
         diskUsageRef.value?.refresh()
      }

      if (
         oldView === 'scan' &&
         newView !== 'scan' &&
         viewState.value === ViewState.DELETE_COMPLETE
      ) {
         viewState.value = ViewState.RESULTS
         emit('cancel')
      }
   }
)

onDeactivated(() => {
   if (viewState.value === ViewState.DELETE_COMPLETE) {
      viewState.value = ViewState.RESULTS
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
               v-if="viewStateScanning"
               class="ScanView-body"
               :progress="progress"
               @abort="$emit('abort')"
            />

            <ScanLaunch
               v-else-if="viewStateLaunch"
               class="ScanView-body"
               @start-scan="$emit('start-scan')"
            />

            <ScanResultsList
               ref="resultsListRef"
               v-else-if="viewState === ViewState.RESULTS && folders.length > 0"
               class="ScanView-body"
               :folders="folders"
               @update:selectedSize="onSelectedSizeUpdate"
               @review="onReview"
               @cancel="$emit('cancel')"
            />

            <ScanResultsDeleteList
               v-else-if="viewState === ViewState.DELETE"
               class="ScanView-body"
               :items="deleteItems"
               :isActive="viewState === ViewState.DELETE"
               @back="onBackFromDelete"
               @update:selectedSize="onSelectedSizeUpdate"
               @complete="onDeleteComplete"
               @cancel="$emit('cancel')"
            />

            <ScanResultsDeleteConfirmation
               v-else-if="viewState === ViewState.DELETE_COMPLETE"
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
