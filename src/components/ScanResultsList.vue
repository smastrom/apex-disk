<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanResultsList

Purpose: Main content area. Folder/file list with back/forward navigation, selection, and Move to Trash button.

Props: folders (FolderInfo[])

Example:
 <ScanResultsList :folders="folders" @update:selectedSize="onSelectedSizeUpdate" @review="onReview" @cancel="onCancel" />
-->

<script setup lang="ts">
import ScanListNav from './ScanListNav.vue'
import ScanResultsListItem from './ScanResultsListItem.vue'

import type { TrashListItem, FolderInfo } from '@/types/structs'

import {
   ref,
   reactive,
   watch,
   watchEffect,
   shallowRef,
   computed,
   useTemplateRef,
   nextTick,
   onMounted,
} from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   folders: FolderInfo[]
}>()

const emit = defineEmits<{
   (e: 'update:selectedSize', value: number): void
   (e: 'review', items: TrashListItem[]): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()

/** Max rows rendered per view. Matches Rust's `scan::MAX_FILES_PER_DIR` so the
 * frontend bound on DOM size and Rust's bound on file payload stay in sync.
 * The Rust cap drops only files; this cap drops the smallest entries of any
 * type. See reference/scanning.md. */
const MAX_DISPLAYED_ITEMS = 300

/** Window size is fixed, so > 6 rows always overflows and the overlay
 * scrollbar paints the right gutter; at or below we reserve it manually. */
const SCROLLBAR_VISIBLE_THRESHOLD = 6

interface NavEntry {
   items: FolderInfo[]
   label: string
   path: string
   /** True when this view's underlying folder had files dropped by the Rust cap. */
   truncated: boolean
}

/**
 * Browser-style navigation stacks. shallowRef avoids deep reactivity on the
 * NavEntry arrays; we only ever replace the whole array, never mutate in place.
 */
const backStack = shallowRef<NavEntry[]>([])
const forwardStack = shallowRef<NavEntry[]>([])
const current = shallowRef<NavEntry>({ items: [], label: '', path: '', truncated: false })
const homePath = ref('')
const isListSlideEnabled = ref(false)
const isWebDriverSession = typeof navigator !== 'undefined' && navigator.webdriver === true

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
 * during setup, with no extra render cycle needed.
 */
watch(
   () => props.folders,
   (folders) => {
      isListSlideEnabled.value = false

      if (folders.length > 0) {
         backStack.value = []
         forwardStack.value = []

         selectedMap.clear()

         const rootPath = parentDir(folders[0].path)

         homePath.value = rootPath
         current.value = { items: folders, label: '', path: rootPath, truncated: false }
      } else {
         backStack.value = []
         forwardStack.value = []
         current.value = { items: [], label: '', path: '', truncated: false }
         homePath.value = ''

         selectedMap.clear()
      }
   },
   { immediate: true }
)

/** Path for display: replaces the home directory prefix with ~ for brevity, or shows /${username} on first screen. */
const displayPath = computed(() => {
   const path = current.value.path
   const home = homePath.value
   const isAtRoot = backStack.value.length === 0

   if (!path) return '/'
   if (path === home) {
      // First screen: show /${username} instead of ~
      if (isAtRoot && home) {
         // Extract username from home path (e.g. "/Users/username" -> "username")
         const username = home.split('/').pop()

         if (username) return `/${username}`
      }

      return '~'
   }
   if (home && path.startsWith(home + '/')) return '~' + path.slice(home.length)

   return path
})

/** Rows actually rendered. Slicing to MAX_DISPLAYED_ITEMS keeps DOM size bounded
 * without virtualization; entries are pre-sorted by size desc in Rust, so the
 * dropped tail is always the smallest items. */
const displayedItems = computed(() => current.value.items.slice(0, MAX_DISPLAYED_ITEMS))

/** True when either Rust dropped files (truncated flag) or the frontend slice
 * hid additional entries. Either way the notice shown is the same. */
const isListTruncated = computed(
   () => current.value.truncated || current.value.items.length > MAX_DISPLAYED_ITEMS
)

const parentRef = useTemplateRef<HTMLElement>('parentRef')
const isTopShadowShown = ref(false)

/**
 * Shows the top overlay only when the list has genuinely scrolled away from
 * the top; this prevents transition-time flicker when switching list views.
 */
function updateTopShadowVisibility() {
   const listEl = parentRef.value

   if (!listEl) {
      isTopShadowShown.value = false

      return
   }

   const isScrollable = listEl.scrollHeight - listEl.clientHeight > 1

   isTopShadowShown.value = isScrollable && listEl.scrollTop > 10
}

function onListScroll() {
   updateTopShadowVisibility()
}

/**
 * Resets scroll to top after the leaving list has finished its transition.
 * Wired to the `<Transition>` `@after-leave` hook below. Running it on
 * `current.path` change directly would scroll the leaving element while it's
 * still mid-slide (visible jump), since `mode="out-in"` keeps it in the DOM
 * for the duration of the leave.
 */
function onAfterListLeave() {
   parentRef.value?.scrollTo(0, 0)
   updateTopShadowVisibility()

   isListSlideEnabled.value = false
}

watch(
   () => current.value.path,
   async () => {
      await nextTick()
      updateTopShadowVisibility()
   }
)

onMounted(() => {
   updateTopShadowVisibility()
})

/**
 * Walks up the directory hierarchy via string slicing to check if any
 * ancestor of `path` is already selected. O(depth) — typically 3-6 levels.
 * Used to avoid double-counting nested selections in size totals and trash lists.
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
 * Builds a flat list of selected items for the trash review screen.
 * Filters out items whose ancestor folder is already selected (to avoid
 * double-counting), then sorts largest first for user visibility.
 */
function buildSelectedItemsForTrash(): TrashListItem[] {
   const out: TrashListItem[] = []

   for (const [path, item] of selectedMap) {
      if (!hasSelectedAncestor(path)) {
         out.push({ path, name: item.name, size: item.size, is_file: item.is_file })
      }
   }

   return out.slice().sort((a, b) => b.size - a.size)
}

/** True if item should appear selected: explicitly in map or inside a selected folder. */
function isSelectedForUI(path: string): boolean {
   return selectedMap.has(path) || hasSelectedAncestor(path)
}

/**
 * Pre-computed set of folder paths that have at least one selected descendant
 * but are not themselves selected (folders in "indeterminate" state).
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

/** Selection count and total size (after ancestor de-dupe) for log lines. */
function selectionTotalsLabel(): string {
   return `${selectedMap.size} selected, ${formatBytes(selectedSize.value)}`
}

/** Removes all selectedMap entries whose path is inside `folderPath`. */
function deselectDescendants(folderPath: string) {
   const prefix = folderPath + '/'

   for (const [path] of selectedMap) {
      if (path.startsWith(prefix)) {
         selectedMap.delete(path)
      }
   }
}

/**
 * Finds the nearest selected ancestor path, or null if none.
 */
function findSelectedAncestor(path: string): string | null {
   let dir = path

   for (;;) {
      const slash = dir.lastIndexOf('/')

      if (slash <= 0) return null

      dir = dir.slice(0, slash)

      if (selectedMap.has(dir)) return dir
   }
}

/**
 * "Explodes" an ancestor selection to exclude a specific descendant.
 * Walks from the ancestor down to the target's parent, at each level
 * selecting all siblings except the one on the path to the target.
 * O(depth × siblings) — typically 3-6 levels with <100 siblings each.
 */
function explodeAncestorExcluding(ancestorPath: string, excludePath: string) {
   const ancestor = selectedMap.get(ancestorPath)

   if (!ancestor) return

   selectedMap.delete(ancestorPath)

   // Walk down from ancestor to the exclude target's parent
   let currentFolder = ancestor
   let remainingPath = excludePath.slice(ancestorPath.length + 1) // e.g. "sub/deep/file"

   while (remainingPath) {
      const slashIdx = remainingPath.indexOf('/')
      const segment = slashIdx === -1 ? remainingPath : remainingPath.slice(0, slashIdx)
      const targetChildPath = currentFolder.path + '/' + segment

      // Select all siblings at this level except the one on the path to exclude
      for (const child of currentFolder.children) {
         if (child.path !== targetChildPath) {
            selectedMap.set(child.path, child)
         }
      }

      if (slashIdx === -1) break // reached the target level

      // Descend into the next level
      const nextFolder = currentFolder.children.find((c) => c.path === targetChildPath)

      if (!nextFolder || nextFolder.is_file) break

      currentFolder = nextFolder
      remainingPath = remainingPath.slice(slashIdx + 1)
   }
}

/**
 * Four-state toggle for item selection:
 * 1. Explicitly selected → deselect (remove from map).
 * 2. Inherited selected (ancestor is selected) → explode ancestor, excluding this item.
 * 3. Indeterminate (has selected descendants) → deselect all descendants.
 * 4. Unselected → select (store FolderInfo reference in map).
 */
function toggleSelect(item: FolderInfo) {
   const kind = item.is_file ? 'file' : 'folder'

   // Always allow deselecting descendants even for protected/FDA folders
   if (someSelectedPaths.value.has(item.path)) {
      deselectDescendants(item.path)
      log(
         'file',
         `Results: deselect descendants under ${kind} "${item.name}" ${selectionTotalsLabel()}`
      )

      return
   }

   if (item.is_protected) {
      return
   }

   if (selectedMap.has(item.path)) {
      selectedMap.delete(item.path)
      log('file', `Results: deselect ${kind} "${item.name}" ${selectionTotalsLabel()}`)
   } else {
      const ancestor = findSelectedAncestor(item.path)

      if (ancestor) {
         explodeAncestorExcluding(ancestor, item.path)
         log(
            'file',
            `Results: explode ancestor (exclude ${kind} "${item.name}") ${selectionTotalsLabel()}`
         )
      } else {
         selectedMap.set(item.path, item)
         log(
            'file',
            `Results: select ${kind} "${item.name}" (${formatBytes(item.size)}) ${selectionTotalsLabel()}`
         )
      }
   }
}

/**
 * Restores selection from TrashListItem[] (e.g. after returning from the trash
 * review screen). Converts each item to a FolderInfo stub so selectedMap stays
 * consistent. O(items) — no tree walk needed.
 */
function setSelectedItems(items: TrashListItem[]) {
   selectedMap.clear()

   for (const item of items) {
      selectedMap.set(item.path, {
         name: item.name,
         path: item.path,
         size: item.size,
         is_file: item.is_file,
         is_protected: false,
         is_fda_required: false,
         children: [],
         truncated: false,
      })
   }
}

/** Clears selection from the nav reset control (selection only; no navigation). */
function clearSelectionFromNav() {
   if (selectedMap.size > 0) {
      log('file', `Results: reset selection, was ${selectionTotalsLabel()}`)
   }

   selectedMap.clear()
}

/** Clears all selections and navigates back to root. */
function resetAll() {
   if (selectedMap.size > 0) {
      log('file', `Results: reset all, was ${selectionTotalsLabel()}`)
   }

   selectedMap.clear()

   backStack.value = []
   forwardStack.value = []
   isListSlideEnabled.value = false

   if (homePath.value && props.folders.length > 0) {
      current.value = {
         items: props.folders,
         label: '',
         path: homePath.value,
         truncated: false,
      }
   }
}

defineExpose({ setSelectedItems, resetAll })

/** Navigates into a folder's children with a forward slide. */
function goInto(item: FolderInfo) {
   if (item.is_file) return

   log('nav', `Results: into "${item.name}" (${item.children.length} children)`)

   isListSlideEnabled.value = true

   document.documentElement.style.setProperty('--nav-direction', '1')

   backStack.value = [...backStack.value, { ...current.value }]
   forwardStack.value = []
   current.value = {
      items: item.children,
      label: item.name,
      path: item.path,
      truncated: item.truncated,
   }
}

/** Navigates to the previous directory with a backward slide. */
function goBack() {
   if (backStack.value.length === 0) return

   log('nav', `Results: back to "${backStack.value[backStack.value.length - 1].label || '~'}"`)

   isListSlideEnabled.value = true

   document.documentElement.style.setProperty('--nav-direction', '-1')

   forwardStack.value = [...forwardStack.value, { ...current.value }]
   current.value = backStack.value.pop()!
}

/** Re-enters a previously visited directory with a forward slide. */
function goForward() {
   if (forwardStack.value.length === 0) return

   log(
      'nav',
      `Results: forward to "${forwardStack.value[forwardStack.value.length - 1].label || '~'}"`
   )

   isListSlideEnabled.value = true

   document.documentElement.style.setProperty('--nav-direction', '1')

   backStack.value = [...backStack.value, { ...current.value }]
   current.value = forwardStack.value.pop()!
}

function onReviewClick() {
   emit('review', buildSelectedItemsForTrash())
}

function onCancel() {
   emit('cancel')
}
</script>

<template>
   <div class="ScanResultsList-root" data-testid="results-list">
      <ScanListNav
         isForwardShown
         :isBackDisabled="backStack.length === 0"
         :isForwardDisabled="forwardStack.length === 0"
         :pathLabel="displayPath"
         :pathTitle="current.path"
         isActionsShown
         :isResetDisabled="selectedMap.size === 0"
         @back="goBack"
         @forward="goForward"
         @reset="clearSelectionFromNav"
         @cancel="onCancel"
      />
      <div
         class="ScanResultsList-listWrap"
         :class="{ 'ScanResultsList-listWrap--topShadowShown': isTopShadowShown }"
      >
         <div
            ref="parentRef"
            class="ScanResultsList-list ScanResultsList-listScroll"
            @scroll.passive="onListScroll"
         >
            <Transition
               name="list-slide"
               mode="out-in"
               :css="isListSlideEnabled && !isWebDriverSession"
               @after-leave="onAfterListLeave"
            >
               <div
                  :key="current.path"
                  class="ScanResultsList-listInner"
                  :style="
                     displayedItems.length > SCROLLBAR_VISIBLE_THRESHOLD
                        ? undefined
                        : { '--list-item-end-gutter': 'var(--scrollbar-inline-gutter)' }
                  "
               >
                  <div
                     v-for="item in displayedItems"
                     :key="item.path"
                     class="ScanResultsList-listItem"
                     v-memo="[
                        item.path,
                        isSelectedForUI(item.path),
                        someSelectedPaths.has(item.path),
                     ]"
                  >
                     <ScanResultsListItem
                        :item="item"
                        :isSelected="isSelectedForUI(item.path)"
                        :isSomeSelected="someSelectedPaths.has(item.path)"
                        :isSelectable="
                           !item.is_protected && !item.is_fda_required
                              ? true
                              : someSelectedPaths.has(item.path)
                                ? 'deselect-only'
                                : false
                        "
                        :formatBytes="formatBytes"
                        @select="() => toggleSelect(item)"
                        @navigate="() => goInto(item)"
                     />
                  </div>
                  <p
                     v-if="isListTruncated"
                     class="ScanResultsList-truncated"
                     data-testid="results-truncated"
                  >
                     {{ t('ScanResultsList', 'truncated') }}
                  </p>
               </div>
            </Transition>
         </div>
      </div>

      <div class="ScanResultsList-footer">
         <button
            type="button"
            class="ScanResultsList-moveToTrashBtn GradientButton"
            :disabled="selectedMap.size === 0"
            data-testid="review-selection"
            @click="onReviewClick"
         >
            {{
               selectedSize > 0
                  ? t(
                       'ScanResultsList',
                       selectedMap.size === 1 ? 'goToReviewSizeOne' : 'goToReviewSize',
                       {
                          size: formatBytes(selectedSize),
                          count: selectedMap.size,
                       }
                    )
                  : t('ScanResultsList', 'goToReview')
            }}
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
   position: relative;
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   padding-bottom: var(--results-footer-height);

   &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 12px;
      background: linear-gradient(var(--color-bg), transparent);
      z-index: 1;
      pointer-events: none;
      opacity: 0;
      transition: opacity 0.12s ease;
   }
}

.ScanResultsList-listWrap--topShadowShown {
   &::before {
      opacity: 1;
   }
}

.ScanResultsList-list {
   flex: 1;
   min-height: 0;
}

.ScanResultsList-listScroll {
   /* Clip horizontal overflow so the list-slide transition (translateX ±10px
      during folder navigation) doesn't trigger a transient horizontal
      scrollbar from our custom ::-webkit-scrollbar styling. */
   overflow-x: hidden;
   overflow-y: auto;
   overflow-y: overlay;
}

.ScanResultsList-listInner {
   position: relative;
   width: 100%;
}

.ScanResultsList-listItem {
   position: relative;

   &:first-of-type {
      /* Greater than listWrap:before z-index, to prevent first item from partially being hidden by the gradient */
      z-index: 2;
   }
}

.ScanResultsList-truncated {
   text-align: center;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-base);
   color: var(--color-text-muted);
}

.ScanResultsList-footer {
   position: absolute;
   bottom: 0;
   left: 0;
   right: 0;
   z-index: 3;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanResultsList-moveToTrashBtn {
   background: linear-gradient(
      90deg,
      var(--btn-step-0) 0%,
      var(--btn-step-50) 50%,
      var(--btn-step-100) 100%
   );

   & svg {
      filter: drop-shadow(0 0 3px rgba(0, 0, 0, 0.2));
   }
}
</style>
