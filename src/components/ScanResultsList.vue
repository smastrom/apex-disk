<!--
ScanResultsList

Purpose: Main content area. Folder/file list with back/forward navigation, selection, and Delete button.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <ScanResultsList :folders="folders" :loading="loading" :progress="progress" @start-scan="loadFolders" />
-->

<script setup lang="ts">
import ScanResultsListItem from './ScanResultsListItem.vue'
import ScanResultsLoadingView from './ScanResultsLoadingView.vue'
import ScanResultsNav from './ScanResultsNav.vue'
import ScanViewInitial from './ScanViewInitial.vue'

import { ref, reactive, watch, watchEffect, shallowRef, computed, inject, nextTick, type Ref } from 'vue'
import { PhTrash } from '@phosphor-icons/vue'
import { useVirtualizer } from '@tanstack/vue-virtual'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/useTranslations'
import { useViewTransition } from '@/lib/useViewTransition'

import { SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'
import type { DeleteListItem, FolderInfo, ScanProgress } from '@/types/structures'

const { t } = useTranslations()
const { withTransition } = useViewTransition()
const storeRef = inject<Ref<SettingsStore | null>>(SETTINGS_KEY)

const navDirection = ref<1 | -1>(1)

const props = defineProps<{
   folders: FolderInfo[]
   loading: boolean
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'start-scan'): void
   (e: 'abort'): void
   (e: 'update:selectedSize', value: number): void
   (e: 'review', items: DeleteListItem[]): void
}>()

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

watch(
   () => current.value.path,
   () => {
      nextTick(() => parentRef.value?.scrollTo(0, 0))
   }
)

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

/** Build flat list of selected items (no ancestor selected) for delete review, sorted by size descending. */
function buildSelectedItemsForDelete(): DeleteListItem[] {
   const out: DeleteListItem[] = []
   function visit(items: FolderInfo[]) {
      for (const item of items) {
         if (selectedMap.get(item.path) && !hasSelectedAncestor(item.path)) {
            out.push({
               path: item.path,
               name: item.name,
               size: item.size,
               is_file: item.is_file,
            })
         }
         if (!item.is_file) visit(item.children)
      }
   }
   visit(props.folders)
   return out.sort((a, b) => b.size - a.size)
}

/** True if item should appear selected: explicitly in map or inside a selected folder. */
function isSelectedForUI(path: string): boolean {
   return !!selectedMap.get(path) || hasSelectedAncestor(path)
}


// Set of folder paths that have at least one selected descendant but are not
// themselves selected. Updated once per selectedMap mutation — O(entries × depth).
// Template does a simple O(1) Set.has() lookup instead of per-row tree traversal.
const someSelectedPaths = shallowRef(new Set<string>())

watchEffect(() => {
   const set = new Set<string>()
   for (const [path] of selectedMap) {
      let dir = path
      for (;;) {
         const slash = dir.lastIndexOf('/')
         if (slash <= 0) break
         dir = dir.slice(0, slash)
         if (selectedMap.has(dir)) break // ancestor already covers everything below
         set.add(dir)
      }
   }
   someSelectedPaths.value = set
})


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

/** Replaces selection with the given paths (e.g. after returning from DeleteList). */
function setSelectedPaths(paths: Set<string>) {
   selectedMap.clear()
   for (const path of paths) selectedMap.set(path, true)
}

defineExpose({ setSelectedPaths })

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

function onReviewClick() {
   emit('review', buildSelectedItemsForDelete())
}

function onAbort() {
   emit('abort')
}
</script>

<template>
   <div class="ScanResultsList-root">
      <ScanResultsLoadingView v-if="loading" :progress="progress" @abort="onAbort" />
      <ScanViewInitial v-else-if="folders.length === 0" @start-scan="emit('start-scan')" />
      <div v-else class="ScanResultsList-content">
         <ScanResultsNav
            :showForward="true"
            :backDisabled="backStack.length === 0"
            :forwardDisabled="forwardStack.length === 0"
            :pathLabel="displayPath"
            :pathTitle="current.path"
            :showActions="true"
            :resetDisabled="selectedMap.size === 0"
            @back="goBack"
            @forward="goForward"
            @reset="selectedMap.clear()"
            @abort="onAbort"
         />
         <div class="ScanResultsList-listWrap" :style="{ '--nav-direction': navDirection }">
            <div ref="parentRef" class="ScanResultsList-list ScanResultsList-listScroll">
               <div
                  class="ScanResultsList-listInner"
                  :style="{ height: rowVirtualizer.getTotalSize() + 'px' }"
               >
                  <div
                     v-for="virtualRow in rowVirtualizer.getVirtualItems()"
                     :key="String(virtualRow.key)"
                     class="ScanResultsList-listItem"
                     :style="{
                        position: 'absolute',
                        top: 0,
                        left: 0,
                        width: '100%',
                        transform: `translateY(${virtualRow.start}px)`,
                     }"
                  >
                     <ScanResultsListItem
                        :item="displayedItems[virtualRow.index]"
                        :selected="isSelectedForUI(displayedItems[virtualRow.index].path)"
                        :someSelected="someSelectedPaths.has(displayedItems[virtualRow.index].path)"
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
      <div v-if="!loading && folders.length > 0" class="ScanResultsList-footer">
         <button
            type="button"
            class="ScanResultsList-deleteBtn"
            :disabled="selectedMap.size === 0"
            @click="onReviewClick"
         >
            <PhTrash :size="18" weight="bold" />
            <span>{{ t('ScanResultsList', 'reviewSize', { size: formatBytes(selectedSize) }) }}</span>
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsList-root {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   overflow: hidden;
   background: var(--color-bg);
}

.ScanResultsList-content {
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

.ScanResultsList-listWrap {
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   view-transition-name: list-view;
}

.ScanResultsList-list {
   flex: 1;
   min-height: 0;
}

.ScanResultsList-listScroll {
   overflow: auto;
}

.ScanResultsList-listInner {
   position: relative;
   width: 100%;
}

.ScanResultsList-listItem {
   will-change: transform;
}

.ScanResultsList-footer {
   position: absolute;
   bottom: 0;
   left: 0;
   right: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanResultsList-deleteBtn {
   width: 100%;
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
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

   &:hover:not(:disabled) {
      background: var(--color-accent-hover);
      box-shadow: var(--glow-lg);
      transform: translateY(-1px);
   }

   &:active:not(:disabled) {
      transform: translateY(0);
      box-shadow: var(--glow-sm);
   }

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
      box-shadow: none;
   }
}
</style>
