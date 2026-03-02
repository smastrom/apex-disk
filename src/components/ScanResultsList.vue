<!--
ScanResultsList

Purpose: Main content area. Folder/file list with back/forward navigation, selection, and Delete button.

Props: folders (FolderInfo[]), loading (boolean), progress (ScanProgress)

Example:
 <ScanResultsList :folders="folders" :loading="loading" :progress="progress" @start-scan="loadFolders" />
-->

<script setup lang="ts">
import ScanResultsListItem from './ScanResultsListItem.vue'
import ScanResultsNav from './ScanResultsNav.vue'

import {
   ref,
   reactive,
   watch,
   watchEffect,
   shallowRef,
   computed,
   nextTick,
   useTemplateRef,
} from 'vue'
import { PhTrashSimple } from '@phosphor-icons/vue'
import { useVirtualizer } from '@tanstack/vue-virtual'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'
import { useViewTransition } from '@/lib/use-view-transition'

import type { DeleteListItem, FolderInfo } from '@/types/structs'

const props = defineProps<{
   folders: FolderInfo[]
}>()

const emit = defineEmits<{
   (e: 'update:selectedSize', value: number): void
   (e: 'review', items: DeleteListItem[]): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()
const { withTransition } = useViewTransition()

interface NavEntry {
   items: FolderInfo[]
   label: string
   path: string
}

/**
 * Browser-style navigation stacks. shallowRef avoids deep reactivity on the
 * NavEntry arrays — we only ever replace the whole array, never mutate in place.
 */
const backStack = shallowRef<NavEntry[]>([])
const forwardStack = shallowRef<NavEntry[]>([])
const current = shallowRef<NavEntry>({ items: [], label: '', path: '' })
const homePath = ref('')

/**
 * Selection state: Map<path, FolderInfo>.
 *
 * Performance design choices:
 * - reactive(Map) instead of reactive(Set): Vue tracks Map.get(key) per-key
 *   (not ITERATE_KEY), so toggling one item only re-renders that row, not the
 *   entire list.
 * - Stores FolderInfo references directly: gives O(1) access to item size/name
 *   without needing a separate lookup index. Items are captured at toggle time,
 *   avoiding any upfront tree walk (the folder tree can have 1.5M+ nodes).
 */
const selectedMap = reactive(new Map<string, FolderInfo>())

/** Extracts the parent directory from a path string. */
function parentDir(path: string): string {
   const i = path.lastIndexOf('/')
   return i <= 0 ? '' : path.slice(0, i)
}

/**
 * Resets navigation and sets the root view when scan results arrive.
 * `{ immediate: true }` so the initial prop value is handled synchronously
 * during setup — no extra render cycle needed.
 */
watch(
   () => props.folders,
   (folders) => {
      if (folders.length > 0) {
         backStack.value = []
         forwardStack.value = []
         selectedMap.clear()
         const rootPath = parentDir(folders[0].path)
         homePath.value = rootPath
         current.value = { items: folders, label: '', path: rootPath }
      } else {
         backStack.value = []
         forwardStack.value = []
         current.value = { items: [], label: '', path: '' }
         homePath.value = ''
         selectedMap.clear()
      }
   },
   { immediate: true }
)

/** Path for display: replaces the home directory prefix with ~ for brevity. */
const displayPath = computed(() => {
   const path = current.value.path
   const home = homePath.value
   if (!path) return '/'
   if (path === home) return '~'
   if (home && path.startsWith(home + '/')) return '~' + path.slice(home.length)
   return path
})

/** Visible items for the current directory (filtering is done at scan time in Rust). */
const displayedItems = computed(() => current.value.items)

const parentRef = useTemplateRef<HTMLElement>('parentRef')

/** Resets scroll position to top when navigating into a different directory. */
watch(
   () => current.value.path,
   () => {
      nextTick(() => parentRef.value?.scrollTo(0, 0))
   }
)

/**
 * Virtual scroller: only renders rows visible in the viewport + overscan buffer.
 * Wrapped in a computed so the virtualizer reactively tracks count/key changes.
 */
const rowVirtualizer = useVirtualizer(
   computed(() => ({
      count: displayedItems.value.length,
      getScrollElement: () => parentRef.value,
      estimateSize: () => 72,
      overscan: 10,
      getItemKey: (index: number) => displayedItems.value[index]?.path ?? index,
   }))
)
/**
 * Walks up the directory hierarchy via string slicing to check if any
 * ancestor of `path` is already selected. O(depth) — typically 3-6 levels.
 * Used to avoid double-counting nested selections in size totals and delete lists.
 */
function hasSelectedAncestor(path: string): boolean {
   let dir = path
   for (;;) {
      const slash = dir.lastIndexOf('/')
      if (slash <= 0) return false
      dir = dir.slice(0, slash)
      if (selectedMap.has(dir)) return true
   }
}

/**
 * Builds a flat list of selected items for the delete review screen.
 * Filters out items whose ancestor folder is already selected (to avoid
 * double-counting), then sorts largest first for user visibility.
 */
function buildSelectedItemsForDelete(): DeleteListItem[] {
   const out: DeleteListItem[] = []
   for (const [path, item] of selectedMap) {
      if (!hasSelectedAncestor(path)) {
         out.push({ path, name: item.name, size: item.size, is_file: item.is_file })
      }
   }
   return out.sort((a, b) => b.size - a.size)
}

/** True if item should appear selected: explicitly in map or inside a selected folder. */
function isSelectedForUI(path: string): boolean {
   return selectedMap.has(path) || hasSelectedAncestor(path)
}

/**
 * Pre-computed set of folder paths that have at least one selected descendant
 * but are not themselves selected — i.e. folders in "indeterminate" state.
 *
 * Rebuilt automatically on every selectedMap mutation via watchEffect.
 * Cost: O(entries × depth) per mutation. The template then does O(1) Set.has()
 * per row instead of a per-row tree traversal.
 */
const someSelectedPaths = shallowRef(new Set<string>())

watchEffect(() => {
   const set = new Set<string>()
   for (const [path] of selectedMap) {
      let dir = path
      for (;;) {
         const slash = dir.lastIndexOf('/')
         if (slash <= 0) break
         dir = dir.slice(0, slash)
         if (selectedMap.has(dir)) break
         set.add(dir)
      }
   }
   someSelectedPaths.value = set
})

/**
 * Total size of selected items, excluding items already covered by a selected
 * ancestor. Reads item.size directly from the stored FolderInfo — O(entries × depth).
 */
const selectedSize = computed(() => {
   let total = 0
   for (const [path, item] of selectedMap) {
      if (hasSelectedAncestor(path)) continue
      total += item.size
   }
   return total
})

/** Emits size changes to parent so the disk usage bar stays in sync. */
watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

/** Removes all selectedMap entries whose path is inside `folderPath`. */
function deselectDescendants(folderPath: string) {
   const prefix = folderPath + '/'
   for (const [path] of selectedMap) {
      if (path.startsWith(prefix)) selectedMap.delete(path)
   }
}

/**
 * Three-state toggle for item selection:
 * 1. Selected → deselect (remove from map).
 * 2. Indeterminate (has selected descendants) → deselect all descendants.
 * 3. Unselected → select (store FolderInfo reference in map).
 */
function toggleSelect(item: FolderInfo) {
   if (item.is_protected) return
   if (selectedMap.has(item.path)) {
      selectedMap.delete(item.path)
   } else if (someSelectedPaths.value.has(item.path)) {
      deselectDescendants(item.path)
   } else {
      selectedMap.set(item.path, item)
   }
}

/**
 * Restores selection from DeleteListItem[] (e.g. after returning from the delete
 * review screen). Converts each item to a FolderInfo stub so selectedMap stays
 * consistent. O(items) — no tree walk needed.
 */
function setSelectedItems(items: DeleteListItem[]) {
   selectedMap.clear()
   for (const item of items) {
      selectedMap.set(item.path, {
         name: item.name,
         path: item.path,
         size: item.size,
         is_file: item.is_file,
         is_protected: false,
         children: [],
      })
   }
}

defineExpose({ setSelectedItems })

const listWrapRef = useTemplateRef<HTMLElement>('listWrapRef')
const footerRef = useTemplateRef<HTMLElement>('footerRef')

/**
 * View transition name helpers. Names are applied just before a transition
 * and removed after, so they don't interfere with other transitions on the page.
 */
function enableListTransitionNames() {
   listWrapRef.value?.style.setProperty('view-transition-name', 'list-view')
   footerRef.value?.style.setProperty('view-transition-name', 'list-footer')
}

function clearListTransitionNames() {
   listWrapRef.value?.style.removeProperty('view-transition-name')
   footerRef.value?.style.removeProperty('view-transition-name')
}

/** Navigates into a folder's children with a forward view transition. */
async function goInto(item: FolderInfo) {
   if (item.is_file) return
   document.documentElement.style.setProperty('--nav-direction', '1')
   enableListTransitionNames()
   await withTransition(async () => {
      backStack.value = [...backStack.value, { ...current.value }]
      forwardStack.value = []
      current.value = { items: item.children, label: item.name, path: item.path }
   })
   clearListTransitionNames()
}

/** Navigates to the previous directory with a backward view transition. */
async function goBack() {
   if (backStack.value.length === 0) return
   document.documentElement.style.setProperty('--nav-direction', '-1')
   enableListTransitionNames()
   await withTransition(async () => {
      forwardStack.value = [...forwardStack.value, { ...current.value }]
      current.value = backStack.value.pop()!
   })
   clearListTransitionNames()
}

/** Re-enters a previously visited directory with a forward view transition. */
async function goForward() {
   if (forwardStack.value.length === 0) return
   document.documentElement.style.setProperty('--nav-direction', '1')
   enableListTransitionNames()
   await withTransition(async () => {
      backStack.value = [...backStack.value, { ...current.value }]
      current.value = forwardStack.value.pop()!
   })
   clearListTransitionNames()
}

function onReviewClick() {
   emit('review', buildSelectedItemsForDelete())
}

function onCancel() {
   emit('cancel')
}
</script>

<template>
   <div class="ScanResultsList-root">
      <ScanResultsNav
         showForward
         :backDisabled="backStack.length === 0"
         :forwardDisabled="forwardStack.length === 0"
         :pathLabel="displayPath"
         :pathTitle="current.path"
         showActions
         :resetDisabled="selectedMap.size === 0"
         @back="goBack"
         @forward="goForward"
         @reset="selectedMap.clear()"
         @cancel="onCancel"
      />
      <div ref="listWrapRef" class="ScanResultsList-listWrap">
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

      <div ref="footerRef" class="ScanResultsList-footer">
         <button
            type="button"
            class="ScanResultsList-deleteBtn GradientButton"
            :disabled="selectedMap.size === 0"
            @click="onReviewClick"
         >
            <PhTrashSimple :size="18" weight="bold" aria-hidden="true" />
            <span>{{
               selectedSize > 0
                  ? t('ScanResultsList', 'reviewSize', { size: formatBytes(selectedSize) })
                  : t('ScanResultsList', 'review')
            }}</span>
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

.ScanResultsList-listWrap {
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   padding-bottom: var(--delete-footer-height);
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

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
      box-shadow: none;
   }
}
</style>
