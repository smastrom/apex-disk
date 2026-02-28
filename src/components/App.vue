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

import { ref, shallowRef, provide, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import { applyTheme } from '@/lib/theme'
import { useTranslations } from '@/lib/useTranslations'
import { createSettingsStore, SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { FolderInfo, ScanProgress } from '@/types/structures'

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'

const { t } = useTranslations()

const settingsStore = shallowRef<SettingsStore | null>(null)
const appReady = ref(false)

// We cannot detect FDA without probing a protected path (which would ask for that path, not FDA).
// Show instructions and open on settings so the user grants FDA and relaunches.
const fdaGranted = ref(false)

provide(SETTINGS_KEY, settingsStore)

onMounted(async () => {
   try {
      settingsStore.value = await createSettingsStore()
      applyTheme(settingsStore.value!.getThemeColor())
      activeView.value = 'settings'
   } catch (err) {
      console.error('Failed to load settings:', err)
   } finally {
      appReady.value = true
   }
})

watch(
   () => settingsStore.value?.getThemeColor(),
   (theme) => {
      if (theme) applyTheme(theme)
   }
)

const folders = ref<FolderInfo[]>([])
const loading = ref(false)
const activeView = ref('scan')
const scanGeneration = ref(0)
const progress = ref<ScanProgress>({
   current: 0,
   total: 1,
   folder: '',
   size: 0,
   scanned_size_total: 0,
})

let unlistenProgress: (() => void) | null = null

async function loadFolders() {
   // Clean up any previous scan's listener before starting a new one
   unlistenProgress?.()
   unlistenProgress = null

   scanGeneration.value += 1
   const gen = scanGeneration.value

   loading.value = true
   progress.value = { current: 0, total: 1, folder: '', size: 0, scanned_size_total: 0 }

   unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
      if (gen === scanGeneration.value) progress.value = event.payload
   })

   try {
      const result = await invoke<FolderInfo[]>('get_user_folders')
      if (gen === scanGeneration.value) folders.value = result
   } catch (error) {
      if (gen === scanGeneration.value) console.error('Error loading folders:', error)
   } finally {
      if (gen === scanGeneration.value) {
         unlistenProgress?.()
         unlistenProgress = null
         loading.value = false
      }
   }
}

function onAbort() {
   scanGeneration.value += 1
   unlistenProgress?.()
   unlistenProgress = null
   folders.value = []
   loading.value = false
   progress.value = { current: 0, total: 1, folder: '', size: 0, scanned_size_total: 0 }
}

function onCancel() {
   folders.value = []
}

onUnmounted(() => {
   unlistenProgress?.()
})
</script>

<template>
   <Transition name="App-ready" mode="out-in">
      <AppLoadingScreen v-if="!appReady" key="loading" />

      <div v-else key="layout" class="App-root">
         <AppHeader />

         <div class="App-main">
            <Transition name="App-fade">
               <KeepAlive>
                  <ScanView
                     v-if="activeView === 'scan'"
                     :folders="folders"
                     :loading="loading"
                     :progress="progress"
                     @start-scan="loadFolders"
                     @abort="onAbort"
                     @cancel="onCancel"
                  />

                  <div v-else-if="activeView === 'settings'" key="settings" class="App-overlay">
                     <SettingsView :fdaGranted="fdaGranted" />
                  </div>

                  <div v-else-if="activeView === 'information'" key="other" class="App-overlay">
                     <main class="App-placeholder">
                        <p>{{ t('App', 'informationComingSoon') }}</p>
                     </main>
                  </div>

                  <div v-else-if="activeView === 'donate'" class="App-overlay">
                     <main class="App-placeholder">
                        <p>{{ t('App', 'donateComingSoon') }}</p>
                     </main>
                  </div>
               </KeepAlive>
            </Transition>
         </div>

         <AppFooter
            :activeView="activeView"
            :hasPermissionIssue="!fdaGranted"
            @select-view="activeView = $event"
         />
      </div>
   </Transition>
</template>

<style scoped>
.App-ready-leave-active {
   transition: opacity 0.2s ease-out;
}

.App-ready-leave-to {
   opacity: 0;
}

.App-ready-enter-active {
   transition: opacity 0.15s ease-out;
}

.App-ready-enter-from {
   opacity: 0;
}

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

.App-fade-enter-active,
.App-fade-leave-active {
   transition: opacity 0.18s ease-out;
}

.App-fade-enter-from,
.App-fade-leave-to {
   opacity: 0;
}
</style>
