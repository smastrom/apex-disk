<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
AppLayout

Purpose: Application layout rendered once system bootstrap is complete. Manages scan state and renders the header, the view-switching main area, and the footer menu.

Props: systemInfo (SystemInfo | null), isFdaGranted (boolean), diskUsage (DiskUsage | null)

Example:
 <AppLayout :systemInfo="systemInfo" :isFdaGranted="isFdaGranted" :diskUsage="diskUsage" />
-->

<script setup lang="ts">
import AppFooter from './AppFooter.vue'
import AppHeader from './AppHeader.vue'
import AppViewAnnouncer from './AppViewAnnouncer.vue'
import InformationView from './InformationView.vue'
import ScanView from './ScanView.vue'
import SettingsView from './SettingsView.vue'

import type { DiskUsage } from '@/types/disk'
import type { SystemInfo } from '@/types/system-info'

import { watch } from 'vue'

import { applyTheme, applyDirection } from '@/lib/dom'
import { useAppUpdate } from '@/lib/use-app-update'
import { useAppViews } from '@/lib/use-app-views'
import { disableNativeContextMenu } from '@/lib/use-context-menu'
import { setupFocusRing } from '@/lib/use-focus-ring'
import { useScanner } from '@/lib/use-scanner'
import { useAppSettings } from '@/stores/app-settings'

defineProps<{
   systemInfo: SystemInfo | null
   isFdaGranted: boolean
   diskUsage: DiskUsage | null
}>()

const settingsStore = useAppSettings()

watch(
   () => settingsStore.getThemeColor(),
   (theme) => applyTheme(theme)
)

watch(
   () => settingsStore.settings.value.language,
   (lang) => applyDirection(lang)
)

const {
   folders,
   isScanning,
   hasFreshResults: hasPendingScanResults,
   markResultsSeen,
   progress,
   elapsedSeconds,
   loadFolders,
   onAbort,
   onCancel,
} = useScanner()

const { activeView, setActiveView } = useAppViews({
   onEnter: {
      scan: markResultsSeen,
   },
})
const { isChecking, isDownloading, availableVersion, updateReady, onCheckForUpdates } =
   useAppUpdate({
      autoCheckUpdates: settingsStore.settings.value.autoCheckUpdates,
      autoInstallUpdates: settingsStore.settings.value.autoInstallUpdates,
   })

disableNativeContextMenu()
setupFocusRing()
</script>

<template>
   <div class="App-root">
      <AppHeader />

      <div class="App-main" role="main">
         <div class="App-mainContent">
            <Transition name="app-slide" mode="out-in">
               <KeepAlive>
                  <ScanView
                     v-if="activeView === 'scan'"
                     key="scan"
                     :diskUsage="diskUsage"
                     :folders="folders"
                     :isScanning="isScanning"
                     :progress="progress"
                     :elapsedSeconds="elapsedSeconds"
                     :loadFolders="loadFolders"
                     :onAbort="onAbort"
                     :onCancel="onCancel"
                  />

                  <SettingsView
                     v-else-if="activeView === 'settings'"
                     key="settings"
                     :isFdaGranted="isFdaGranted"
                     :isChecking="isChecking"
                     :isDownloading="isDownloading"
                     :availableVersion="availableVersion"
                     :updateReady="updateReady"
                     @check-for-updates="onCheckForUpdates"
                  />

                  <InformationView
                     v-else-if="activeView === 'information'"
                     key="information"
                     :systemInfo="systemInfo"
                  />
               </KeepAlive>
            </Transition>
         </div>
      </div>

      <AppFooter
         :activeView="activeView"
         :isScanning="isScanning"
         :hasPendingScanResults="hasPendingScanResults"
         @select-view="setActiveView"
      />

      <AppViewAnnouncer :activeView="activeView" />
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
</style>
