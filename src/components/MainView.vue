<!--
MainView

Purpose: Main content area. Folder/file list with navigation, selection, and Delete button.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <MainView :folders="folders" :loading="loading" :progress="progress" @start-scan="loadFolders" />
-->

<script setup lang="ts">
import ListItem from './ListItem.vue'

import { ref, reactive, watch, computed } from 'vue'

import { PhCaretLeft, PhCaretRight, PhFolder } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'
import { formatBytes } from '@/lib/format'

import type { FolderInfo } from '@/types/structures'

const { t } = useTranslations()

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: {
      current: number
      total: number
      folder: string
      size: number
      scanning?: string
   }
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
}>()

interface NavEntry {
   items: FolderInfo[]
   label: string
}

const backStack = ref<NavEntry[]>([])
const forwardStack = ref<NavEntry[]>([])
const current = ref<NavEntry>({ items: [], label: 'start' })

// Flat path→size index built once on scan, gives O(1) size lookup
const sizeIndex = new Map<string, number>()

function buildSizeIndex(items: FolderInfo[]) {
   for (const item of items) {
      sizeIndex.set(item.path, item.size)
      if (!item.is_file) buildSizeIndex(item.children)
   }
}

watch(
   () => props.folders,
   (folders) => {
      if (folders.length > 0) {
         sizeIndex.clear()
         buildSizeIndex(folders)
         backStack.value = []
         forwardStack.value = []
         current.value = { items: folders, label: t('MainView', 'rootLabel') }
      }
   },
   { immediate: true }
)

// Map instead of Set: Map.get(key) tracks per-key, not ITERATE_KEY.
// This means only the toggled ListItem re-renders, not the entire list.
const selectedMap = reactive(new Map<string, boolean>())
const selectedSize = computed(() => {
   let total = 0
   for (const [path] of selectedMap) total += sizeIndex.get(path) ?? 0
   return total
})

function toggleSelect(path: string) {
   if (selectedMap.get(path)) selectedMap.delete(path)
   else selectedMap.set(path, true)
}

function goInto(item: FolderInfo) {
   if (item.is_file) return
   backStack.value = [...backStack.value, { ...current.value }]
   forwardStack.value = []
   current.value = { items: item.children, label: item.name }
}

function goBack() {
   if (backStack.value.length === 0) return
   forwardStack.value = [...forwardStack.value, { ...current.value }]
   const prev = backStack.value.pop()!
   current.value = prev
}

function goForward() {
   if (forwardStack.value.length === 0) return
   backStack.value = [...backStack.value, { ...current.value }]
   const next = forwardStack.value.pop()!
   current.value = next
}

function startOver() {
   if (props.folders.length === 0) return
   backStack.value = []
   forwardStack.value = []
   current.value = { items: props.folders, label: t('MainView', 'rootLabel') }
}

function onDeleteClick() {
   // Logic to be implemented later
}
</script>

<template>
   <main class="MainView-root">
      <div v-if="loading" class="MainView-loading">
         <p>{{ t('MainView', 'scanning', { current: progress.current, total: progress.total }) }}</p>
      </div>
      <div v-else-if="folders.length === 0" class="MainView-empty">
         <p>{{ t('MainView', 'noDataYet') }}</p>
         <button class="MainView-scanBtn" @click="emit('start-scan')">{{ t('MainView', 'startScan') }}</button>
      </div>
      <div v-else class="MainView-content">
         <nav class="MainView-nav">
            <button type="button" class="MainView-navLink" @click="startOver">
               <PhCaretLeft :size="18" weight="regular" />
               {{ t('MainView', 'startOver') }}
            </button>
            <div class="MainView-navControls">
               <button
                  type="button"
                  class="MainView-navBtn"
                  :disabled="backStack.length === 0"
                  aria-label="Back"
                  @click="goBack"
               >
                  <PhCaretLeft :size="18" weight="regular" />
               </button>
               <button
                  type="button"
                  class="MainView-navBtn"
                  :disabled="forwardStack.length === 0"
                  aria-label="Forward"
                  @click="goForward"
               >
                  <PhCaretRight :size="18" weight="regular" />
               </button>
               <span class="MainView-navLabel">
                  <PhFolder :size="16" weight="regular" class="MainView-navIcon" />
                  {{ current.label }}
               </span>
            </div>
         </nav>
         <div class="MainView-list">
            <ListItem
               v-for="item in current.items"
               :key="item.path"
               :item="item"
               :selected="!!selectedMap.get(item.path)"
               :format-bytes="formatBytes"
               @select="() => toggleSelect(item.path)"
               @navigate="() => goInto(item)"
            />
         </div>
      </div>
      <div v-if="folders.length > 0" class="MainView-footer">
         <button
            type="button"
            class="MainView-deleteBtn"
            :disabled="selectedMap.size === 0"
            @click="onDeleteClick"
         >
            {{ t('MainView', 'deleteSize', { size: formatBytes(selectedSize) }) }}
         </button>
      </div>
   </main>
</template>

<style scoped>
.MainView-root {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   overflow: hidden;
   background: var(--color-bg);
}

.MainView-loading,
.MainView-empty,
.MainView-content {
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.MainView-empty {
   align-items: center;
   justify-content: center;
   gap: var(--spacing-md);
}

.MainView-empty p,
.MainView-loading p {
   color: var(--color-text-muted);
   margin: 0;
}

.MainView-scanBtn {
   padding: var(--spacing-sm) var(--spacing-lg);
   font-size: 0.95rem;
   font-weight: 600;
   color: var(--color-bg);
   background: var(--color-accent);
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: background 0.15s;
}

.MainView-scanBtn:hover {
   background: var(--color-accent-hover);
}

.MainView-content {
   padding: 0;
   min-height: 0;
   overflow: hidden;
   padding-bottom: var(--delete-footer-height);
}

.MainView-nav {
   flex-shrink: 0;
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
   padding: var(--spacing-md);
   border-bottom: 1px solid var(--color-surface);
}

.MainView-navLink {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: 0;
   font-size: 0.875rem;
   color: var(--color-accent);
   background: none;
   border: none;
   cursor: pointer;
}

.MainView-navLink:hover {
   text-decoration: underline;
}

.MainView-navControls {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
}

.MainView-navBtn {
   display: flex;
   align-items: center;
   justify-content: center;
   width: 32px;
   height: 28px;
   font-size: 0.875rem;
   color: var(--color-text);
   background: var(--color-surface);
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: background 0.15s;
}

.MainView-navBtn:hover:not(:disabled) {
   background: var(--color-surface-hover);
}

.MainView-navBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

.MainView-navLabel {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   font-size: 0.875rem;
   color: var(--color-text-muted);
}

.MainView-navIcon {
   flex-shrink: 0;
   color: var(--color-accent);
}

.MainView-list {
   flex: 1;
   min-height: 0;
   overflow-y: auto;
   padding: var(--spacing-sm);
   padding-bottom: var(--spacing-lg);
}

.MainView-footer {
   position: absolute;
   bottom: 0;
   left: 0;
   right: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-surface);
   background: var(--color-bg-elevated);
}

.MainView-deleteBtn {
   width: 100%;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.9375rem;
   font-weight: 600;
   color: #fff;
   background: var(--color-accent);
   border: none;
   border-radius: 8px;
   cursor: pointer;
   transition: background 0.15s;
}

.MainView-deleteBtn:hover:not(:disabled) {
   background: var(--color-accent-hover);
}

.MainView-deleteBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}
</style>
