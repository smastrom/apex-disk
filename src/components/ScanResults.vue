<!--
ScanResults

Purpose: Main content area. Folder/file list with back/forward navigation, selection, and Delete button.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <ScanResults :folders="folders" :loading="loading" :progress="progress" @start-scan="loadFolders" />
-->

<script setup lang="ts">
import ListItem from './ListItem.vue'
import ScanLoadingView from './ScanLoadingView.vue'
import ScanSplashScreen from './ScanSplashScreen.vue'

import { ref, reactive, watch, computed, inject, type Ref } from 'vue'
import { PhCaretLeft, PhCaretRight, PhFolder, PhTrash } from '@phosphor-icons/vue'
import { openPath } from '@tauri-apps/plugin-opener'
import { useVirtualizer } from '@tanstack/vue-virtual'

import { useTranslations } from '@/lib/useTranslations'
import { useViewTransition } from '@/lib/useViewTransition'
import { formatBytes } from '@/lib/format'
import { SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { FolderInfo } from '@/types/structures'

const { t } = useTranslations()
const { withTransition } = useViewTransition()
const storeRef = inject<Ref<SettingsStore | null>>(SETTINGS_KEY)

const navDirection = ref<1 | -1>(1)

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: {
      current: number
      total: number
      folder: string
      size: number
      scanned_size_total: number
      scanning?: string
   }
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'update:selectedSize', value: number): void
}>()

const showDeleteResults = ref(false)

interface NavEntry {
   items: FolderInfo[]
   label: string
   path: string
}

const backStack = ref<NavEntry[]>([])
const forwardStack = ref<NavEntry[]>([])
const current = ref<NavEntry>({ items: [], label: '', path: '' })
const homePath = ref('')

// Map instead of Set: Map.get(key) tracks per-key, not ITERATE_KEY.
// This means only the toggled ListItem re-renders, not the entire list.
const selectedMap = reactive(new Map<string, boolean>())

// Flat path→size index built once on scan, gives O(1) size lookup
const sizeIndex = new Map<string, number>()

function buildSizeIndex(items: FolderInfo[]) {
   for (const item of items) {
      sizeIndex.set(item.path, item.size)
      if (!item.is_file) buildSizeIndex(item.children)
   }
}

function parentDir(path: string): string {
   const i = path.lastIndexOf('/')
   return i <= 0 ? '' : path.slice(0, i)
}

watch(
   () => props.folders,
   (folders) => {
      if (folders.length > 0) {
         sizeIndex.clear()
         buildSizeIndex(folders)
         backStack.value = []
         forwardStack.value = []
         const rootPath = parentDir(folders[0].path)
         homePath.value = rootPath
         current.value = { items: folders, label: '', path: rootPath }
      } else {
         sizeIndex.clear()
         backStack.value = []
         forwardStack.value = []
         current.value = { items: [], label: '', path: '' }
         homePath.value = ''
         selectedMap.clear()
      }
   },
   { immediate: true }
)

const showZeroByteFolders = computed(
   () => storeRef?.value?.settings?.value?.showZeroByteFolders ?? false
)

/** Path for display: home directory shown as ~. */
const displayPath = computed(() => {
   const path = current.value.path
   const home = homePath.value
   if (!path) return '/'
   if (path === home) return '~'
   if (home && path.startsWith(home + '/')) return '~' + path.slice(home.length)
   return path
})

const displayedItems = computed(() => {
   const items = current.value.items
   if (showZeroByteFolders.value) return items
   return items.filter((item) => item.is_file || item.size > 0)
})

const parentRef = ref<HTMLElement | null>(null)
const rowVirtualizer = useVirtualizer(
   computed(() => ({
      count: displayedItems.value.length,
      getScrollElement: () => parentRef.value,
      estimateSize: () => 72,
      overscan: 10,
      getItemKey: (index: number) => displayedItems.value[index]?.path ?? index,
   }))
)

/** True if any key in selectedMap is a strict ancestor of path (path is inside that folder). */
function hasSelectedAncestor(path: string): boolean {
   let dir = path
   for (;;) {
      const slash = dir.lastIndexOf('/')
      if (slash <= 0) return false
      dir = dir.slice(0, slash)
      if (selectedMap.get(dir)) return true
   }
}

/** True if item should appear selected: explicitly in map or inside a selected folder. */
function isSelectedForUI(path: string): boolean {
   return !!selectedMap.get(path) || hasSelectedAncestor(path)
}

const selectedSize = computed(() => {
   let total = 0
   for (const [path] of selectedMap) {
      if (hasSelectedAncestor(path)) continue
      total += sizeIndex.get(path) ?? 0
   }
   return total
})

watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

function toggleSelect(item: FolderInfo) {
   if (item.is_protected) return
   if (selectedMap.get(item.path)) selectedMap.delete(item.path)
   else selectedMap.set(item.path, true)
}

function goInto(item: FolderInfo) {
   if (item.is_file) return
   navDirection.value = 1
   withTransition(async () => {
      backStack.value = [...backStack.value, { ...current.value }]
      forwardStack.value = []
      current.value = { items: item.children, label: item.name, path: item.path }
   })
}

function goBack() {
   if (backStack.value.length === 0) return
   navDirection.value = -1
   withTransition(async () => {
      forwardStack.value = [...forwardStack.value, { ...current.value }]
      current.value = backStack.value.pop()!
   })
}

function goForward() {
   if (forwardStack.value.length === 0) return
   navDirection.value = 1
   withTransition(async () => {
      backStack.value = [...backStack.value, { ...current.value }]
      current.value = forwardStack.value.pop()!
   })
}

async function openCurrentInFinder() {
   const path = current.value.path
   if (!path) return
   try {
      await openPath(path)
   } catch (err) {
      console.error('Failed to open folder in Finder:', err)
   }
}

function onDeleteClick() {
   showDeleteResults.value = true
}

function onAbort() {
   emit('abort')
}
</script>

<template>
   <div class="ScanResults-root">
      <ScanLoadingView v-if="loading" :progress="progress" @abort="onAbort" />
      <ScanSplashScreen v-else-if="folders.length === 0" @start-scan="emit('start-scan')" />
      <div v-else class="ScanResults-content">
         <nav class="ScanResults-nav">
            <div class="ScanResults-navControls">
               <button
                  type="button"
                  class="ScanResults-navBtn"
                  :disabled="backStack.length === 0"
                  aria-label="Back"
                  @click="goBack"
               >
                  <PhCaretLeft :size="18" weight="regular" />
               </button>
               <button
                  type="button"
                  class="ScanResults-navBtn"
                  :disabled="forwardStack.length === 0"
                  aria-label="Forward"
                  @click="goForward"
               >
                  <PhCaretRight :size="18" weight="regular" />
               </button>
            </div>
            <button
               type="button"
               class="ScanResults-navPath"
               :title="current.path"
               :aria-label="t('ScanResults', 'openInFinder')"
               :disabled="!current.path"
               @click.stop="openCurrentInFinder"
            >
               <PhFolder :size="16" weight="regular" class="ScanResults-navPathIcon" />
               <span class="ScanResults-navPathText">{{ displayPath }}</span>
            </button>
            <div class="ScanResults-navActions">
               <button
                  type="button"
                  class="ScanResults-resetBtn"
                  :disabled="selectedMap.size === 0"
                  @click="selectedMap.clear()"
               >
                  {{ t('ScanResults', 'resetSelection') }}
               </button>
               <button type="button" class="ScanResults-abortBtn" @click="onAbort">
                  {{ t('ScanResults', 'abort') }}
               </button>
            </div>
         </nav>
         <div class="ScanResults-listWrap" :style="{ '--nav-direction': navDirection }">
            <div ref="parentRef" class="ScanResults-list ScanResults-listScroll">
               <div
                  class="ScanResults-listInner"
                  :style="{ height: rowVirtualizer.getTotalSize() + 'px' }"
               >
                  <div
                     v-for="virtualRow in rowVirtualizer.getVirtualItems()"
                     :key="String(virtualRow.key)"
                     class="ScanResults-listItem"
                     :style="{
                        position: 'absolute',
                        top: 0,
                        left: 0,
                        width: '100%',
                        transform: `translateY(${virtualRow.start}px)`,
                     }"
                  >
                     <ListItem
                        :item="displayedItems[virtualRow.index]"
                        :selected="isSelectedForUI(displayedItems[virtualRow.index].path)"
                        :selectable="!displayedItems[virtualRow.index].is_protected"
                        :formatBytes="formatBytes"
                        @select="() => toggleSelect(displayedItems[virtualRow.index])"
                        @navigate="() => goInto(displayedItems[virtualRow.index])"
                     />
                  </div>
               </div>
            </div>
         </div>
      </div>
      <div v-if="!loading && folders.length > 0 && !showDeleteResults" class="ScanResults-footer">
         <button
            type="button"
            class="ScanResults-deleteBtn"
            :disabled="selectedMap.size === 0"
            @click="onDeleteClick"
         >
            <PhTrash :size="18" weight="bold" />
            {{
               selectedMap.size === 0
                  ? t('ScanResults', 'delete')
                  : t('ScanResults', 'deleteSize', { size: formatBytes(selectedSize) })
            }}
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResults-root {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   overflow: hidden;
   background: var(--color-bg);
}

.ScanResults-content {
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
   padding: 0;
   overflow: hidden;
   padding-bottom: var(--delete-footer-height);
}

.ScanResults-nav {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: var(--spacing-md);
   padding: var(--spacing-md);
   border-bottom: 1px solid var(--color-bg);
}

.ScanResults-navControls {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
}

.ScanResults-navActions {
   display: flex;
   align-items: center;
   gap: var(--spacing-md);
}

.ScanResults-navBtn {
   display: flex;
   align-items: center;
   justify-content: center;
   width: 32px;
   height: 28px;
   color: var(--color-text);
   background: var(--color-surface);
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition:
      background 0.2s,
      box-shadow 0.25s;
}

.ScanResults-navBtn:hover:not(:disabled) {
   background: var(--color-surface-hover);
   box-shadow: var(--glow-sm);
}

.ScanResults-navBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

.ScanResults-navPath {
   flex: 1;
   min-width: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) 0;
   font-size: 0.8125rem;
   color: var(--color-text-muted);
   background: none;
   border: none;
   border-radius: 0;
   cursor: pointer;
   text-align: left;
   position: relative;
}

.ScanResults-navPath::after {
   content: '';
   position: absolute;
   left: 0;
   right: 0;
   bottom: 0;
   height: 1px;
   background: var(--color-accent);
   transform: scaleX(0);
   transition: transform 0.2s ease;
   pointer-events: none;
}

.ScanResults-navPath:hover {
   color: var(--color-text);
}

.ScanResults-navPath:hover:not(:disabled)::after {
   transform: scaleX(1);
}

.ScanResults-navPath:disabled {
   cursor: default;
}

.ScanResults-navPathIcon {
   flex-shrink: 0;
   color: var(--color-accent);
}

.ScanResults-navPathText {
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
}

.ScanResults-resetBtn {
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: var(--color-text-muted);
   background: none;
   border: none;
   cursor: pointer;
}

.ScanResults-resetBtn:hover:not(:disabled) {
   color: var(--color-text);
   opacity: 0.85;
}

.ScanResults-resetBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

.ScanResults-abortBtn {
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: #ff3b30;
   background: none;
   border: none;
   cursor: pointer;
}

.ScanResults-abortBtn:hover {
   opacity: 0.75;
}

.ScanResults-listWrap {
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   view-transition-name: list-view;
}

.ScanResults-list {
   flex: 1;
   min-height: 0;
}

.ScanResults-listScroll {
   overflow: auto;
}

.ScanResults-listInner {
   position: relative;
   width: 100%;
}

.ScanResults-listItem {
   will-change: transform;
}

.ScanResults-footer {
   position: absolute;
   bottom: 0;
   left: 0;
   right: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanResults-deleteBtn {
   width: 100%;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.9375rem;
   font-weight: 600;
   color: var(--color-on-accent);
   background: var(--color-accent);
   border: none;
   border-radius: 8px;
   cursor: pointer;
   box-shadow: var(--glow-md);
   transition:
      background 0.2s,
      box-shadow 0.3s,
      transform 0.15s;
}

.ScanResults-deleteBtn:hover:not(:disabled) {
   background: var(--color-accent-hover);
   box-shadow: var(--glow-lg);
   transform: translateY(-1px);
}

.ScanResults-deleteBtn:active:not(:disabled) {
   transform: translateY(0);
   box-shadow: var(--glow-sm);
}

.ScanResults-deleteBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
   box-shadow: none;
}
</style>
