<!--
App

Purpose: Root application component. Bootstraps the settings store, manages scan state, and renders the app shell with header, main content, and footer menu.

Props: none

Example:
 <App />
-->

<script setup lang="ts">
import AppHeader from './AppHeader.vue'
import AppFooter from './AppFooter.vue'
import ScanView from './ScanView.vue'
import SettingsView from './SettingsView.vue'
import InformationView from './InformationView.vue'

import { useTemplateRef, watch } from 'vue'

import { useAppSettings } from '@/stores/app-settings'
import { useAppViews } from '@/lib/use-app-views'
import { useAppUpdate } from '@/lib/use-app-update'
import { useFullDiskAccess } from '@/lib/use-full-disk-access'
import { useSystemInfo } from '@/lib/use-system-info'
import { disableNativeContextMenu } from '@/lib/use-context-menu'
import { applyTheme, applyDirection } from '@/lib/document'
import { setupFocusRing } from '@/lib/use-focus-ring'
import { useDiskUsage } from '@/lib/use-disk-usage'

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'
import '@/assets/css/animations.css'
import '@/assets/css/rtl.css'

const mainContentRef = useTemplateRef<HTMLElement>('mainContentRef')
const settingsStore = useAppSettings()

watch(
   () => settingsStore.getThemeColor(),
   (theme) => applyTheme(theme)
)

watch(
   () => settingsStore.settings.value.language,
   (lang) => applyDirection(lang)
)

const { activeView, setActiveView } = useAppViews(mainContentRef)
const { newAvailableVersion, isChecking, onCheckForUpdates } = useAppUpdate()

const { systemInfo } = await useSystemInfo()
const { isFdaGranted } = await useFullDiskAccess()
const { diskUsage } = await useDiskUsage()

disableNativeContextMenu()
setupFocusRing()
</script>

<template>
   <div class="App-root">
      <AppHeader />

      <div class="App-main">
         <div ref="mainContentRef" class="App-mainContent">
            <KeepAlive>
               <ScanView
                  v-if="activeView === 'scan'"
                  :appActiveView="activeView"
                  :diskUsage="diskUsage"
               />

               <div v-else-if="activeView === 'settings'" class="App-overlay">
                  <SettingsView
                     :newAvailableVersion="newAvailableVersion"
                     :isFdaGranted="isFdaGranted"
                     :isChecking="isChecking"
                     @check-for-updates="onCheckForUpdates"
                  />
               </div>

               <div v-else-if="activeView === 'information'" class="App-overlay">
                  <InformationView :systemInfo="systemInfo" />
               </div>
            </KeepAlive>
         </div>
      </div>

      <AppFooter :activeView="activeView" @select-view="setActiveView" />
   </div>
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
