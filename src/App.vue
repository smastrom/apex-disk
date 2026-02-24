<script setup lang="ts">
import Layout from '@/components/Layout.vue'

import { ref, shallowRef, provide, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import { createSettingsStore, SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { FolderInfo, ScanProgress } from '@/types/structures'

import '@/assets/css/theme.css'
import '@/assets/css/global.css'
import '@/assets/css/reset.css'
import '@/assets/css/classes.css'

const settingsStore = shallowRef<SettingsStore | null>(null)
provide(SETTINGS_KEY, settingsStore)

function applyTheme(theme: string) {
   if (theme === 'oceanic') {
      document.documentElement.removeAttribute('data-theme')
   } else {
      document.documentElement.setAttribute('data-theme', theme)
   }
}

onMounted(async () => {
   try {
      settingsStore.value = await createSettingsStore()
      applyTheme(settingsStore.value!.getThemeColor())
   } catch (err) {
      console.error('Failed to load settings:', err)
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
})

let unlistenProgress: (() => void) | null = null

async function loadFolders() {
   aborted.value = false
   loading.value = true
   progress.value = { current: 0, total: 1, folder: '', size: 0 }

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
   progress.value = { current: 0, total: 1, folder: '', size: 0 }
}

function onSelectView(view: string) {
   activeView.value = view
}

onUnmounted(() => {
   unlistenProgress?.()
})
</script>

<template>
   <Layout
      :folders="folders"
      :loading="loading"
      :progress="progress"
      :active-view="activeView"
      @select-view="onSelectView"
      @start-scan="loadFolders"
      @abort="onAbort"
   />
</template>
