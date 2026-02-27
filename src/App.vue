<!--
App

Purpose: Root application component. Bootstraps the settings store, manages scan state, and renders the app shell.

Props: none

Example:
 <App />
-->

<script setup lang="ts">
import AppLoadingScreen from '@/components/AppLoadingScreen.vue'
import Layout from '@/components/Layout.vue'

import { ref, shallowRef, provide, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import { applyTheme } from '@/lib/theme'
import { createSettingsStore, SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { FolderInfo, ScanProgress } from '@/types/structures'

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'

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
const aborted = ref(false)
const progress = ref<ScanProgress>({
   current: 0,
   total: 1,
   folder: '',
   size: 0,
   scanned_size_total: 0,
})

let unlistenProgress: (() => void) | null = null

async function loadFolders() {
   aborted.value = false
   loading.value = true
   progress.value = { current: 0, total: 1, folder: '', size: 0, scanned_size_total: 0 }

   unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
      progress.value = event.payload
   })

   try {
      const result = await invoke<FolderInfo[]>('get_user_folders')
      if (!aborted.value) folders.value = result
   } catch (error) {
      if (!aborted.value) console.error('Error loading folders:', error)
   } finally {
      unlistenProgress?.()
      unlistenProgress = null
      loading.value = false
   }
}

function onAbort() {
   aborted.value = true
   folders.value = []
   loading.value = false
   progress.value = { current: 0, total: 1, folder: '', size: 0, scanned_size_total: 0 }
}

function onSelectView(view: string) {
   activeView.value = view
}

onUnmounted(() => {
   unlistenProgress?.()
})
</script>

<template>
   <Transition name="App-ready" mode="out-in">
      <AppLoadingScreen v-if="!appReady" key="loading" />
      <Layout
         v-else
         key="app"
         :folders="folders"
         :loading="loading"
         :progress="progress"
         :activeView="activeView"
         :fdaGranted="fdaGranted"
         @select-view="onSelectView"
         @start-scan="loadFolders"
         @abort="onAbort"
      />
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
</style>
