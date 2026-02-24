<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import FolderNode from './components/FolderNode.vue'

import '@/assets/css/reset.css'

interface FolderInfo {
   name: string
   path: string
   size: number
   icon?: string
   children: FolderInfo[]
   is_file: boolean
}

interface ScanProgress {
   current: number
   total: number
   folder: string
   size: number
   scanning?: string
}

const folders = ref<FolderInfo[]>([])
const loading = ref(false)
const expandedPaths = ref<Set<string>>(new Set())
const elapsedMs = ref(0)
const scanDuration = ref<number | null>(null)
const progress = ref<ScanProgress>({
   current: 0,
   total: 1,
   folder: '',
   size: 0,
})

let unlistenProgress: (() => void) | null = null
let timerInterval: ReturnType<typeof setInterval> | null = null

async function loadFolders() {
   loading.value = true
   scanDuration.value = null
   expandedPaths.value = new Set()
   progress.value = { current: 0, total: 1, folder: '', size: 0 }

   const startTime = performance.now()
   elapsedMs.value = 0
   timerInterval = setInterval(() => {
      elapsedMs.value = Math.round(performance.now() - startTime)
   }, 50)

   unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
      progress.value = event.payload
   })

   try {
      folders.value = await invoke<FolderInfo[]>('get_user_folders')
   } catch (error) {
      console.error('Error loading folders:', error)
   } finally {
      scanDuration.value = Math.round(performance.now() - startTime)
      if (timerInterval) clearInterval(timerInterval)
      timerInterval = null
      unlistenProgress?.()
      unlistenProgress = null
      loading.value = false
   }
}

function toggleExpand(path: string) {
   const next = new Set(expandedPaths.value)
   if (next.has(path)) next.delete(path)
   else next.add(path)
   expandedPaths.value = next
}

function formatBytes(bytes: number) {
   if (bytes === 0) return '0 Bytes'
   const k = 1024
   const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
   const i = Math.floor(Math.log(bytes) / Math.log(k))
   return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

onUnmounted(() => {
   unlistenProgress?.()
   if (timerInterval) clearInterval(timerInterval)
})
</script>

<template>
   <main class="container">
      <div v-if="!loading && folders.length === 0" class="start-screen">
         <h2>Mac Disk Lens</h2>
         <p>Scan your home directory to see what's taking up space.</p>
         <button class="scan-btn" @click="loadFolders">Start Scan</button>
      </div>

      <div v-else-if="loading" class="loader">
         <div class="progress-bar">
            <div
               class="progress-fill"
               :style="{
                  width: `${progress.total ? (progress.current / progress.total) * 100 : 0}%`,
               }"
            ></div>
         </div>
         <p class="progress-text">{{ progress.current }} of {{ progress.total }} folders scanned</p>
         <p class="timer">{{ (elapsedMs / 1000).toFixed(1) }}s</p>
         <p v-if="progress.scanning" class="progress-scanning">Scanning {{ progress.scanning }}…</p>
         <p v-else-if="progress.folder" class="progress-size">
            Last: {{ progress.folder }} — {{ formatBytes(progress.size) }}
         </p>
      </div>

      <div v-else class="tree-container">
         <div class="tree-header">
            <h2>User Folders (sorted by size)</h2>
            <div class="tree-header-right">
               <span v-if="scanDuration !== null" class="scan-duration"
                  >{{ (scanDuration / 1000).toFixed(2) }}s</span
               >
               <button class="scan-btn small" @click="loadFolders">Rescan</button>
            </div>
         </div>
         <p class="tree-hint">Click a folder to expand and navigate into it</p>
         <ul class="folder-tree">
            <FolderNode
               v-for="folder in folders"
               :key="folder.path"
               :folder="folder"
               :depth="0"
               :expanded-paths="expandedPaths"
               :format-bytes="formatBytes"
               :toggle-expand="toggleExpand"
            />
         </ul>
      </div>
   </main>
</template>

<style scoped>
.loader {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: 0.75rem;
   padding: 1.5rem;
   max-width: 400px;
   margin: 0 auto;
}

.progress-bar {
   width: 100%;
   height: 8px;
   background: #e0e0e0;
   border-radius: 4px;
   overflow: hidden;
}

.progress-fill {
   height: 100%;
   background: linear-gradient(90deg, #646cff 0%, #535bf2 100%);
   border-radius: 4px;
   transition: width 0.2s ease-out;
}

.progress-text {
   font-size: 0.9rem;
   color: #444;
   margin: 0;
}

.progress-scanning {
   font-size: 0.85rem;
   color: #646cff;
   font-weight: 500;
   margin: 0;
}

.progress-size {
   font-size: 0.8rem;
   color: #888;
   margin: 0;
}

.timer {
   font-size: 1.1rem;
   font-variant-numeric: tabular-nums;
   color: #646cff;
   font-weight: 600;
   margin: 0;
}

.start-screen {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: 1rem;
   padding: 2rem;
}

.start-screen p {
   color: #666;
   margin: 0;
}

.scan-btn {
   padding: 0.6rem 1.6rem;
   font-size: 0.95rem;
   font-weight: 600;
   color: #fff;
   background: #646cff;
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: background 0.15s;
}

.scan-btn:hover {
   background: #535bf2;
}

.scan-btn.small {
   padding: 0.35rem 0.9rem;
   font-size: 0.8rem;
}

.tree-container {
   text-align: left;
   max-width: 600px;
   margin: 0 auto;
}

.tree-header {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 1rem;
}

.tree-header h2 {
   margin: 0;
}

.tree-header-right {
   display: flex;
   align-items: center;
   gap: 0.75rem;
}

.scan-duration {
   font-size: 0.85rem;
   font-variant-numeric: tabular-nums;
   color: #888;
}

.tree-hint {
   font-size: 0.85rem;
   color: #666;
   margin: 0 0 1rem 0;
}

.folder-tree {
   list-style: none;
   padding: 0;
   margin: 0;
}

.folder-children {
   list-style: none;
   padding: 0;
   margin: 0;
}

.folder-item {
   margin: 0;
}

.folder-row {
   display: flex;
   align-items: center;
   gap: 0.5rem;
   padding: 4px 8px;
   cursor: default;
   border-radius: 4px;
}

.folder-row.expandable {
   cursor: pointer;
}

.folder-row.expandable:hover {
   background: rgba(100, 108, 255, 0.1);
}

.folder-arrow {
   width: 12px;
   font-size: 0.7rem;
   color: #666;
}

.folder-arrow-placeholder {
   width: 12px;
   display: inline-block;
}

.folder-icon {
   font-size: 0.85rem;
   flex-shrink: 0;
}

.folder-name {
   flex: 1;
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
}

.folder-row.is-file .folder-name {
   color: #555;
}

.folder-size {
   font-size: 0.85rem;
   color: #888;
   flex-shrink: 0;
}
</style>

<style>
:root {
   font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
   font-size: 16px;
   line-height: 24px;
   font-weight: 400;
   color: #0f0f0f;
   background-color: #f6f6f6;
   font-synthesis: none;
   text-rendering: optimizeLegibility;
   -webkit-font-smoothing: antialiased;
   -moz-osx-font-smoothing: grayscale;
   -webkit-text-size-adjust: 100%;
}

.container {
   margin: 0;
   padding-top: 10vh;
   display: flex;
   flex-direction: column;
   justify-content: center;
   text-align: center;
}
</style>
