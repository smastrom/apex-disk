<!--
ScanView

Purpose: Common scan shell. Always shows DiskUsageProgress at top; body switches between ScanLoadingView, ScanSplashScreen, or folder list via ScanResults.

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

import { ref } from 'vue'

import type { FolderInfo, ScanProgress } from '@/types/structures'

const selectedSize = ref(0)

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
}>()

defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
}>()

function onSelectedSizeUpdate(value: number) {
   selectedSize.value = value
}
</script>

<template>
   <main class="ScanView-root">
      <DiskUsageProgress :selectedSize="selectedSize" />
      <ScanResults
         class="ScanView-body"
         :folders="folders"
         :loading="loading"
         :progress="progress"
         @start-scan="$emit('start-scan')"
         @abort="$emit('abort')"
         @update:selectedSize="onSelectedSizeUpdate"
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
