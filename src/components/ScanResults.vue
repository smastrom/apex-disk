<!--
ScanResults

Purpose: Main content area. Folder/file list with back/forward navigation, selection, and Delete button.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <ScanResults :folders="folders" :loading="loading" :progress="progress" @start-scan="loadFolders" />
-->

<script setup lang="ts">
import ListItem from './ListItem.vue'

import { ref, reactive, watch, computed, inject, type Ref } from 'vue'
import { PhCaretLeft, PhCaretRight } from '@phosphor-icons/vue'
import { useVirtualList } from '@vueuse/core'

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
      scanning?: string
   }
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
}>()

interface NavEntry {
   items: FolderInfo[]
   label: string
}

const backStack = ref<NavEntry[]>([])
const forwardStack = ref<NavEntry[]>([])
const current = ref<NavEntry>({ items: [], label: '' })

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

watch(
   () => props.folders,
   (folders) => {
      if (folders.length > 0) {
         sizeIndex.clear()
         buildSizeIndex(folders)
         backStack.value = []
         forwardStack.value = []
         current.value = { items: folders, label: '' }
      } else {
         sizeIndex.clear()
         backStack.value = []
         forwardStack.value = []
         current.value = { items: [], label: '' }
         selectedMap.clear()
      }
   },
   { immediate: true }
)

const showZeroByteFolders = computed(
   () => storeRef?.value?.settings?.value?.showZeroByteFolders ?? false,
)

const displayedItems = computed(() => {
   const items = current.value.items
   if (showZeroByteFolders.value) return items
   return items.filter((item) => item.is_file || item.size > 0)
})

const { list: virtualList, containerProps, wrapperProps } = useVirtualList(
   displayedItems,
   { itemHeight: 56, overscan: 10 },
)

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
   navDirection.value = 1
   withTransition(async () => {
      backStack.value = [...backStack.value, { ...current.value }]
      forwardStack.value = []
      current.value = { items: item.children, label: item.name }
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

function onDeleteClick() {
   // Logic to be implemented later
}

function onAbort() {
   emit('abort')
}
</script>

<template>
   <main class="ScanResults-root">
      <div v-if="loading" class="ScanResults-loading">
         <p>{{ t('ScanResults', 'scanning', { current: progress.current, total: progress.total }) }}</p>
      </div>
      <div v-else-if="folders.length === 0" class="ScanResults-empty">
         <p>{{ t('ScanResults', 'noDataYet') }}</p>
         <button class="ScanResults-scanBtn" @click="emit('start-scan')">{{ t('ScanResults', 'startScan') }}</button>
      </div>
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
            <button type="button" class="ScanResults-abortBtn" @click="onAbort">
               {{ t('ScanResults', 'abort') }}
            </button>
         </nav>
         <div
            class="ScanResults-listWrap"
            :style="{ '--nav-direction': navDirection }"
         >
            <div class="ScanResults-list" v-bind="containerProps">
               <div v-bind="wrapperProps">
               <ListItem
                  v-for="item in virtualList"
                  :key="item.data.path"
                  :item="item.data"
                  :selected="!!selectedMap.get(item.data.path)"
                  :format-bytes="formatBytes"
                  @select="() => toggleSelect(item.data.path)"
                  @navigate="() => goInto(item.data)"
               />
               </div>
            </div>
         </div>
      </div>
      <div v-if="folders.length > 0" class="ScanResults-footer">
         <button
            type="button"
            class="ScanResults-deleteBtn"
            :disabled="selectedMap.size === 0"
            @click="onDeleteClick"
         >
            {{ t('ScanResults', 'deleteSize', { size: formatBytes(selectedSize) }) }}
         </button>
      </div>
   </main>
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

.ScanResults-loading,
.ScanResults-empty,
.ScanResults-content {
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.ScanResults-empty {
   align-items: center;
   justify-content: center;
   gap: var(--spacing-md);
}

.ScanResults-empty p,
.ScanResults-loading p {
   color: var(--color-text-muted);
   margin: 0;
}

.ScanResults-scanBtn {
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

.ScanResults-scanBtn:hover {
   background: var(--color-accent-hover);
}

.ScanResults-content {
   padding: 0;
   min-height: 0;
   overflow: hidden;
   padding-bottom: var(--delete-footer-height);
}

.ScanResults-nav {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: space-between;
   padding: var(--spacing-md);
   border-bottom: 1px solid var(--color-surface);
}

.ScanResults-navControls {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
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
   transition: background 0.15s;
}

.ScanResults-navBtn:hover:not(:disabled) {
   background: var(--color-surface-hover);
}

.ScanResults-navBtn:disabled {
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

.ScanResults-footer {
   position: absolute;
   bottom: 0;
   left: 0;
   right: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-surface);
   background: var(--color-bg-elevated);
}

.ScanResults-deleteBtn {
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

.ScanResults-deleteBtn:hover:not(:disabled) {
   background: var(--color-accent-hover);
}

.ScanResults-deleteBtn:disabled {
   opacity: 0.5;
   cursor: not-allowed;
}
</style>
