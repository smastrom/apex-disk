<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import Layout from "@/components/Layout.vue";

import type { FolderInfo, ScanProgress } from "@/types/structures";

import "@/assets/css/theme.css";
import "@/assets/css/global.css";
import "@/assets/css/reset.css";

const folders = ref<FolderInfo[]>([]);
const loading = ref(false);
const activeView = ref("scan");
const progress = ref<ScanProgress>({
   current: 0,
   total: 1,
   folder: "",
   size: 0,
});

let unlistenProgress: (() => void) | null = null;

async function loadFolders() {
   loading.value = true;
   progress.value = { current: 0, total: 1, folder: "", size: 0 };

   unlistenProgress = await listen<ScanProgress>("folder-scan-progress", (event) => {
      progress.value = event.payload;
   });

   try {
      folders.value = await invoke<FolderInfo[]>("get_user_folders");
   } catch (error) {
      console.error("Error loading folders:", error);
   } finally {
      unlistenProgress?.();
      unlistenProgress = null;
      loading.value = false;
   }
}

function onSelectView(view: string) {
   activeView.value = view;
}

onUnmounted(() => {
   unlistenProgress?.();
});
</script>

<template>
   <Layout
      :folders="folders"
      :loading="loading"
      :progress="progress"
      :active-view="activeView"
      @select-view="onSelectView"
      @start-scan="loadFolders"
   />
</template>
