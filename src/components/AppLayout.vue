<!--
AppLayout

Purpose: App shell with header, main content, and footer menu. Mobile-app style layout.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress), activeView (string?)

Example:
 <AppLayout
   :folders="folders"
   :loading="loading"
   :progress="progress"
   :activeView="activeView"
   @select-view="onSelectView"
 />
-->

<script setup lang="ts">
import AppHeader from './AppHeader.vue'
import ScanView from './ScanView.vue'
import SettingsView from './SettingsView.vue'
import AppFooter from './AppFooter.vue'

import { useTranslations } from '@/lib/useTranslations'

import type { FolderInfo, ScanProgress } from '@/types/structures'

const { t } = useTranslations()

defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
   activeView?: string
   fdaGranted?: boolean
}>()

defineEmits<{
   (e: 'select-view', view: string): void
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'cancel'): void
}>()
</script>

<template>
   <div class="AppLayout-root">
      <AppHeader />

      <div class="AppLayout-main">
         <Transition name="AppLayout-fade">
            <KeepAlive>
               <ScanView
                  v-if="activeView === 'scan'"
                  :folders="folders"
                  :loading="loading"
                  :progress="progress"
                  @start-scan="$emit('start-scan')"
                  @abort="$emit('abort')"
                  @cancel="$emit('cancel')"
               />

               <div v-else-if="activeView === 'settings'" key="settings" class="AppLayout-overlay">
                  <SettingsView :fdaGranted="fdaGranted ?? true" />
               </div>

               <div v-else-if="activeView === 'information'" key="other" class="AppLayout-overlay">
                  <main class="AppLayout-placeholder">
                     <p>{{ t('AppLayout', 'informationComingSoon') }}</p>
                  </main>
               </div>

               <div v-else-if="activeView === 'donate'" class="AppLayout-overlay">
                  <main class="AppLayout-placeholder">
                     <p>{{ t('AppLayout', 'donateComingSoon') }}</p>
                  </main>
               </div>
            </KeepAlive>
         </Transition>
      </div>

      <AppFooter
         :activeView="activeView"
         :hasPermissionIssue="fdaGranted === false"
         @select-view="$emit('select-view', $event)"
      />
   </div>
</template>

<style scoped>
.AppLayout-root {
   display: flex;
   flex-direction: column;
   height: 100vh;
   overflow: hidden;
   background: var(--color-bg);
}

.AppLayout-main {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
}

.AppLayout-overlay {
   position: absolute;
   inset: 0;
   display: flex;
   flex-direction: column;
   background: var(--color-bg);
   z-index: 1;
}

.AppLayout-placeholder {
   flex: 1;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: var(--spacing-md);

   p {
      color: var(--color-text-muted);
      margin: 0;
   }
}

.AppLayout-fade-enter-active,
.AppLayout-fade-leave-active {
   transition: opacity 0.18s ease-out;
}

.AppLayout-fade-enter-from,
.AppLayout-fade-leave-to {
   opacity: 0;
}
</style>
