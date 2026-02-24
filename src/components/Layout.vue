<!--
Layout

Purpose: App shell with header, main content, and footer menu. Mobile-app style layout.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress), activeView (string?)

Example:
 <Layout
   :folders="folders"
   :loading="loading"
   :progress="progress"
   :active-view="activeView"
   @select-view="onSelectView"
 />
-->

<script setup lang="ts">
import Header from "./Header.vue";
import MainView from "./MainView.vue";
import FooterMenu from "./FooterMenu.vue";

import type { FolderInfo } from "@/types/structures";

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
   activeView?: string;
}>();

defineEmits<{
   (e: "select-view", view: string): void;
   (e: "start-scan"): void;
}>();
</script>

<template>
   <div class="Layout-root">
      <Header />
      <MainView
         :folders="folders"
         :loading="loading"
         :progress="progress"
         @start-scan="$emit('start-scan')"
      />
      <FooterMenu
         :active-view="activeView"
         @select-view="$emit('select-view', $event)"
      />
   </div>
</template>

<style scoped>
.Layout-root {
   display: flex;
   flex-direction: column;
   min-height: 100vh;
   background: var(--color-bg);
}
</style>
