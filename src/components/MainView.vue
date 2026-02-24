<!--
MainView

Purpose: Main content area. Receives folder data and will display the folder tree (to be implemented).

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <MainView :folders="folders" :loading="loading" :progress="progress" />
-->

<script setup lang="ts">
import { useTranslations } from '@/lib/useTranslations'

import type { FolderInfo } from "@/types/structures";

const { t } = useTranslations()

defineProps<{
   folders: FolderInfo[];
   loading: boolean;
   progress: {
      current: number;
      total: number;
      folder: string;
      size: number;
      scanning?: string;
   };
}>();

const emit = defineEmits<{
   (e: "start-scan"): void;
}>();
</script>

<template>
   <main class="MainView-root">
      <div v-if="loading" class="MainView-loading">
         <p>{{ t('MainView', 'scanning', { current: progress.current, total: progress.total }) }}</p>
      </div>
      <div v-else-if="folders.length === 0" class="MainView-empty">
         <p>{{ t('MainView', 'noDataYet') }}</p>
         <button class="MainView-scanBtn" @click="emit('start-scan')">{{ t('MainView', 'startScan') }}</button>
      </div>
      <div v-else class="MainView-content">
         <!-- Folder tree will be implemented here -->
         <p class="MainView-placeholder">{{ t('MainView', 'foldersLoaded', { count: folders.length }) }}</p>
      </div>
   </main>
</template>

<style scoped>
.MainView-root {
   flex: 1;
   overflow-y: auto;
   padding: var(--spacing-md);
   background: var(--color-bg);
}

.MainView-loading,
.MainView-empty,
.MainView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
}

.MainView-empty {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-md);
}

.MainView-empty p,
.MainView-loading p,
.MainView-placeholder {
   color: var(--color-text-muted);
   margin: 0;
}

.MainView-scanBtn {
   padding: var(--spacing-sm) var(--spacing-lg);
   font-size: 0.95rem;
   font-weight: 600;
   color: var(--color-bg);
   background: var(--color-accent);
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: background 0.15s;
}

.MainView-scanBtn:hover {
   background: var(--color-accent-hover);
}
</style>
