<!--
Layout

Purpose: App shell with header, main content, and footer menu. Mobile-app style layout.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress), activeView (string?)

Example:
 <Layout
   :folders="folders"
   :loading="loading"
   :progress="progress"
   :activeView="activeView"
   @select-view="onSelectView"
 />
-->

<script setup lang="ts">
import Header from './Header.vue'
import ScanView from './ScanView.vue'
import SettingsView from './SettingsView.vue'
import FooterNav from './FooterNav.vue'

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
}>()
</script>

<template>
   <div class="Layout-root">
      <Header />

      <div class="Layout-main">
         <ScanView
            v-show="activeView === 'scan'"
            :folders="folders"
            :loading="loading"
            :progress="progress"
            @start-scan="$emit('start-scan')"
            @abort="$emit('abort')"
         />
         <Transition name="Layout-fade">
            <div v-if="activeView === 'settings'" key="settings" class="Layout-overlay">
               <SettingsView :fdaGranted="fdaGranted ?? true" />
            </div>
            <div v-else-if="activeView !== 'scan'" key="other" class="Layout-overlay">
               <main class="Layout-placeholder">
                  <p>
                     {{
                        activeView === 'information'
                           ? t('Layout', 'informationComingSoon')
                           : activeView === 'donate'
                             ? t('Layout', 'donateComingSoon')
                             : ''
                     }}
                  </p>
               </main>
            </div>
         </Transition>
      </div>
      <FooterNav
         :activeView="activeView"
         :hasPermissionIssue="fdaGranted === false"
         @select-view="$emit('select-view', $event)"
      />
   </div>
</template>

<style scoped>
.Layout-root {
   display: flex;
   flex-direction: column;
   height: 100vh;
   overflow: hidden;
   background: var(--color-bg);
}

.Layout-main {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
}

.Layout-overlay {
   position: absolute;
   inset: 0;
   display: flex;
   flex-direction: column;
   background: var(--color-bg);
   z-index: 1;
}

.Layout-placeholder {
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

.Layout-fade-enter-active,
.Layout-fade-leave-active {
   transition: opacity 0.18s ease;
}

.Layout-fade-enter-from,
.Layout-fade-leave-to {
   opacity: 0;
}
</style>
