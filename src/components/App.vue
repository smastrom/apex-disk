<!--
App

Purpose: Root application component. Bootstraps the settings store, manages scan state, and renders the app shell with header, main content, and footer menu.

Props: none

Example:
 <App />
-->

<script setup lang="ts">
import AppLoadingScreen from '@/components/AppLoadingScreen.vue'
import AppHeader from './AppHeader.vue'
import ScanView from './ScanView.vue'
import SettingsView from './SettingsView.vue'
import AppFooter from './AppFooter.vue'

import { useTemplateRef } from 'vue'

import { provideSettingsStore } from '@/lib/provide-settings-store'
import { useFocusRing } from '@/lib/use-focus-ring'
import { useFullDiskAccess } from '@/lib/use-full-disk-access'
import { useScan } from '@/lib/use-scan'
import { useViews } from '@/lib/use-views'
import { useUpdater } from '@/lib/use-updater'
import { useTranslations } from '@/lib/use-translations'

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'
import '@/assets/css/animations.css'
import '@/assets/css/rtl.css'

const mainContentRef = useTemplateRef<HTMLElement>('mainContentRef')

const { settingsStore, appReady } = provideSettingsStore()

useFocusRing()

const { t } = useTranslations(settingsStore)
const { fdaGranted } = useFullDiskAccess()
const { folders, loading, progress, loadFolders, onAbort, onCancel } = useScan(settingsStore)
const { activeView, setActiveView } = useViews(mainContentRef)
const { availableUpdate } = useUpdater(t)
</script>

<template>
   <Transition name="app-ready" mode="out-in">
      <AppLoadingScreen v-if="!appReady" key="loading" />

      <div v-else class="App-root">
         <AppHeader />

         <div class="App-main">
            <div ref="mainContentRef" class="App-mainContent">
               <ScanView
                  v-show="activeView === 'scan'"
                  :folders="folders"
                  :loading="loading"
                  :progress="progress"
                  @start-scan="loadFolders"
                  @abort="onAbort"
                  @cancel="onCancel"
               />

               <div v-if="activeView === 'settings'" class="App-overlay">
                  <SettingsView :fdaGranted="fdaGranted" :availableUpdate="availableUpdate" />
               </div>

               <!--                <div v-else-if="activeView === 'information'" class="App-overlay">
                  <main class="App-placeholder">
                     <p>{{ t('App', 'informationComingSoon') }}</p>
                  </main>
               </div> -->
            </div>
         </div>

         <AppFooter :activeView="activeView" @select-view="setActiveView" />
      </div>
   </Transition>
</template>

<style scoped>
.App-root {
   display: flex;
   flex-direction: column;
   height: 100vh;
   overflow: hidden;
   background: var(--color-bg);
}

.App-main {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
}

.App-mainContent {
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
}

.App-overlay {
   position: absolute;
   inset: 0;
   display: flex;
   flex-direction: column;
   background: var(--color-bg);
   z-index: 1;
}

.App-placeholder {
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
</style>
