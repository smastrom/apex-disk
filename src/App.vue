<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import FolderNode from './components/FolderNode.vue'

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
const progress = ref<ScanProgress>({
   current: 0,
   total: 1,
   folder: '',
   size: 0,
})

let unlistenProgress: (() => void) | null = null

async function loadFolders() {
   loading.value = true
   expandedPaths.value = new Set()
   progress.value = { current: 0, total: 1, folder: '', size: 0 }

   unlistenProgress = await listen<ScanProgress>('folder-scan-progress', (event) => {
      progress.value = event.payload
   })

   try {
      folders.value = await invoke<FolderInfo[]>('get_user_folders')
   } catch (error) {
      console.error('Error loading folders:', error)
   } finally {
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

onMounted(() => loadFolders())
onUnmounted(() => unlistenProgress?.())
</script>

<template>
   <main class="container">
      <div v-if="loading" class="loader">
         <div class="progress-bar">
            <div
               class="progress-fill"
               :style="{
                  width: `${progress.total ? (progress.current / progress.total) * 100 : 0}%`,
               }"
            ></div>
         </div>
         <p class="progress-text">{{ progress.current }} of {{ progress.total }} folders scanned</p>
         <p v-if="progress.scanning" class="progress-scanning">Scanning {{ progress.scanning }}…</p>
         <p v-else-if="progress.folder" class="progress-size">
            Last: {{ progress.folder }} — {{ formatBytes(progress.size) }}
         </p>
      </div>
      <div v-else class="tree-container">
         <h2>User Folders (sorted by size)</h2>
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

.tree-container {
   text-align: left;
   max-width: 600px;
   margin: 0 auto;
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
