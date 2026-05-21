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
   onActivated,
   onDeactivated,
   onUnmounted,
} from 'vue'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useListRowPopovers } from '@/lib/use-list-row-popovers'
import { useScrollbarVisibility } from '@/lib/use-scrollbar-visibility'
import { useTranslations } from '@/lib/use-translations'
import { isWebDriverSession } from '@/lib/utils'

const props = defineProps<{
   folders: FolderInfo[]
}>()

const emit = defineEmits<{
   (e: 'update:selectedSize', value: number): void
   (e: 'review', items: TrashListItem[]): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()

/***************************************************************
 * Constants and types
 ***************************************************************/

/** Matches Rust `scan::MAX_FILES_PER_DIR`; see reference/scanning.md. */
const MAX_DISPLAYED_ITEMS = 300

/** First pass on navigation; the rest mounts after enter so slides stay smooth. */
const INITIAL_RENDER_COUNT = 50

/** Fixed window: above 6 rows the overlay scrollbar owns the gutter; at or below we reserve it. */
const SCROLLBAR_VISIBLE_THRESHOLD = 6

interface NavEntry {
   items: FolderInfo[]
   label: string
   path: string
   truncated: boolean
}

/***************************************************************
 * Navigation state
 ***************************************************************/

const backStack = shallowRef<NavEntry[]>([])
const forwardStack = shallowRef<NavEntry[]>([])
const current = shallowRef<NavEntry>({ items: [], label: '', path: '', truncated: false })
const homePath = ref('')
const isListSlideEnabled = ref(false)

/** Manual folder nav only uses `out-in`; programmatic/KeepAlive swaps must not leave the scroll container empty. */
const isListSlideActive = computed(() => isListSlideEnabled.value && !isWebDriverSession)
const listTransitionMode = computed<'out-in' | undefined>(() =>
   isListSlideActive.value ? 'out-in' : undefined
)

/***************************************************************
 * Selection state
 ***************************************************************/

/**
 * Map<path, FolderInfo>. reactive(Map) tracks per-key reads so one toggle re-renders one row.
 * FolderInfo refs are stored at toggle time to avoid tree walks on a 1.5M+ node scan tree.
 */
const selectedMap = reactive(new Map<string, FolderInfo>())

/***************************************************************
 * Result initialization
 ***************************************************************/

function parentDir(path: string): string {
   const i = path.lastIndexOf('/')

   return i <= 0 ? '' : path.slice(0, i)
}

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

const displayPath = computed(() => {
   const path = current.value.path
   const home = homePath.value
   const isAtRoot = backStack.value.length === 0

   if (!path) return '/'
   if (path === home) {
      if (isAtRoot && home) {
         const username = home.split('/').pop()

         if (username) return `/${username}`
      }

      return '~'
   }
   if (home && path.startsWith(home + '/')) return '~' + path.slice(home.length)

   return path
})

/***************************************************************
 * List rendering and transitions
 ***************************************************************/

/** Two-phase render: INITIAL_RENDER_COUNT first, MAX_DISPLAYED_ITEMS after enter. */
const renderedCount = ref(MAX_DISPLAYED_ITEMS)

let expandTimer: ReturnType<typeof setTimeout> | null = null
let isComponentActive = false

function cancelExpand() {
   if (expandTimer !== null) {
      clearTimeout(expandTimer)

      expandTimer = null
   }
}

function scheduleExpand() {
   cancelExpand()

   if (!isComponentActive) {
      return
   }

   expandTimer = setTimeout(() => {
      renderedCount.value = MAX_DISPLAYED_ITEMS

      expandTimer = null
   }, 0)
}

/** Frontend slice; dropped tail is smallest because Rust sorts by size desc. */
const displayedItems = computed(() =>
   current.value.items.slice(0, Math.min(renderedCount.value, MAX_DISPLAYED_ITEMS))
)

/** Rust cap or frontend slice; same notice either way. */
const isListTruncated = computed(
   () => current.value.truncated || current.value.items.length > MAX_DISPLAYED_ITEMS
)

const parentRef = useTemplateRef<HTMLElement>('parentRef')
const namePopoverRef = useTemplateRef<HTMLElement>('namePopoverRef')
const checkboxPopoverRef = useTemplateRef<HTMLElement>('checkboxPopoverRef')
const isTopShadowShown = ref(false)

useScrollbarVisibility(parentRef, 'scroll-and-hover')

const popovers = useListRowPopovers(parentRef, namePopoverRef, checkboxPopoverRef, {
   resolveCheckboxText: (kind) =>
      kind === 'fda'
         ? t('ScanResultsListItem', 'fdaRequiredTooltip')
         : t('ScanResultsListItem', 'protectedTooltip'),
})

/** Avoid top-shadow flicker during list transitions. */
function updateTopShadowVisibility() {
   const listEl = parentRef.value

   if (!listEl) {
      isTopShadowShown.value = false

      return
   }

   const isScrollable = listEl.scrollHeight - listEl.clientHeight > 1

   isTopShadowShown.value = isScrollable && listEl.scrollTop > 10
}

/** Scroll reset runs on after-leave, not on path change, to avoid mid-slide jumps. */
function onAfterListLeave() {
   parentRef.value?.scrollTo(0, 0)
   updateTopShadowVisibility()
}

function onAfterListEnter() {
   scheduleExpand()

   isListSlideEnabled.value = false
}

watch(
   () => current.value.path,
   async () => {
      renderedCount.value = Math.min(INITIAL_RENDER_COUNT, current.value.items.length)

      popovers.dismissAll()

      await nextTick()
      updateTopShadowVisibility()
   },
   { immediate: true }
)

onMounted(() => {
   isComponentActive = true

   updateTopShadowVisibility()
   // appear=false: first inner list mount does not fire after-enter.
   scheduleExpand()
})

onActivated(() => {
   isComponentActive = true

   updateTopShadowVisibility()
   scheduleExpand()
})

onDeactivated(() => {
   isComponentActive = false

   cancelExpand()
   popovers.dismissAll()

   isListSlideEnabled.value = false
})

onUnmounted(() => {
   isComponentActive = false

   cancelExpand()
})

/***************************************************************
 * Selection
 ***************************************************************/

/** True when a selected ancestor would double-count this path in totals/trash. */
function hasSelectedAncestor(path: string): boolean {
   let dir = path

   for (;;) {
      const slash = dir.lastIndexOf('/')

      if (slash <= 0) return false

      dir = dir.slice(0, slash)

      if (selectedMap.has(dir)) return true
   }
}

/** Flat selected list for trash review; ancestor-covered paths excluded, largest first. */
function buildSelectedItemsForTrash(): TrashListItem[] {
   const out: TrashListItem[] = []

   for (const [path, item] of selectedMap) {
      if (!hasSelectedAncestor(path)) {
         out.push({ path, name: item.name, size: item.size, is_file: item.is_file })
      }
   }

   return out.slice().sort((a, b) => b.size - a.size)
}

function isSelectedForUI(path: string): boolean {
   return selectedMap.has(path) || hasSelectedAncestor(path)
}

/** Indeterminate folder paths; O(entries x depth) on mutation, O(1) per row in template. */
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

const selectedSize = computed(() => {
   let total = 0

   for (const [path, item] of selectedMap) {
      if (hasSelectedAncestor(path)) continue

      total += item.size
   }

   return total
})

watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

function selectionTotalsLabel(): string {
   return `${selectedMap.size} selected, ${formatBytes(selectedSize.value)}`
}

function deselectDescendants(folderPath: string) {
   const prefix = folderPath + '/'

   for (const [path] of selectedMap) {
      if (path.startsWith(prefix)) {
         selectedMap.delete(path)
      }
   }
}

function findSelectedAncestor(path: string): string | null {
   let dir = path

   for (;;) {
      const slash = dir.lastIndexOf('/')

      if (slash <= 0) return null

      dir = dir.slice(0, slash)

      if (selectedMap.has(dir)) return dir
   }
}

/** Replace one ancestor selection with all siblings on the path to excludePath. */
function explodeAncestorExcluding(ancestorPath: string, excludePath: string) {
   const ancestor = selectedMap.get(ancestorPath)

   if (!ancestor) return

   selectedMap.delete(ancestorPath)

   let currentFolder = ancestor
   let remainingPath = excludePath.slice(ancestorPath.length + 1) // e.g. "sub/deep/file"

   while (remainingPath) {
      const slashIdx = remainingPath.indexOf('/')
      const segment = slashIdx === -1 ? remainingPath : remainingPath.slice(0, slashIdx)
      const targetChildPath = currentFolder.path + '/' + segment

      for (const child of currentFolder.children) {
         if (child.path !== targetChildPath) {
            selectedMap.set(child.path, child)
         }
      }

      if (slashIdx === -1) break

      const nextFolder = currentFolder.children.find((c) => c.path === targetChildPath)

      if (!nextFolder || nextFolder.is_file) break

      currentFolder = nextFolder
      remainingPath = remainingPath.slice(slashIdx + 1)
   }
}

/**
 * Four-state toggle for item selection:
 * 1. Explicitly selected: deselect.
 * 2. Inherited selection: explode the selected ancestor, excluding this item.
 * 3. Indeterminate: deselect all descendants.
 * 4. Unselected: store the FolderInfo reference in selectedMap.
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

function collectFolderInfosByPath(paths: Set<string>): Map<string, FolderInfo> {
   const found = new Map<string, FolderInfo>()
   const stack = [...props.folders]

   while (stack.length > 0 && found.size < paths.size) {
      const item = stack.pop()!

      if (paths.has(item.path)) {
         found.set(item.path, item)
      }

      if (!item.is_file) {
         for (const child of item.children) {
            stack.push(child)
         }
      }
   }

   return found
}

function fallbackSelectionItem(item: TrashListItem): FolderInfo {
   return {
      name: item.name,
      path: item.path,
      size: item.size,
      is_file: item.is_file,
      is_protected: false,
      is_fda_required: false,
      children: [],
      truncated: false,
   }
}

function selectionItemFromTrashItem(
   item: TrashListItem,
   folderInfoByPath: Map<string, FolderInfo>
): FolderInfo {
   return folderInfoByPath.get(item.path) ?? fallbackSelectionItem(item)
}

/** Restore trash-review selection; keep real FolderInfo nodes when the scan tree still has them. */
function setSelectedItems(items: TrashListItem[]) {
   selectedMap.clear()

   const restoredPaths = new Set(items.map((item) => item.path))
   const folderInfoByPath = collectFolderInfosByPath(restoredPaths)

   for (const item of items) {
      selectedMap.set(item.path, selectionItemFromTrashItem(item, folderInfoByPath))
   }
}

function clearSelectionFromNav() {
   if (selectedMap.size > 0) {
      log('file', `Results: reset selection, was ${selectionTotalsLabel()}`)
   }

   selectedMap.clear()
}

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

/***************************************************************
 * Folder navigation actions
 ***************************************************************/

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

function goBack() {
   if (backStack.value.length === 0) return

   const previous = backStack.value[backStack.value.length - 1]

   if (!previous) return

   log('nav', `Results: back to "${previous.label || '~'}"`)

   isListSlideEnabled.value = true

   document.documentElement.style.setProperty('--nav-direction', '-1')

   forwardStack.value = [...forwardStack.value, { ...current.value }]
   backStack.value = backStack.value.slice(0, -1)
   current.value = previous
}

function goForward() {
   if (forwardStack.value.length === 0) return

   const next = forwardStack.value[forwardStack.value.length - 1]

   if (!next) return

   log('nav', `Results: forward to "${next.label || '~'}"`)

   isListSlideEnabled.value = true

   document.documentElement.style.setProperty('--nav-direction', '1')

   backStack.value = [...backStack.value, { ...current.value }]
   forwardStack.value = forwardStack.value.slice(0, -1)
   current.value = next
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
            @scroll.passive="updateTopShadowVisibility"
         >
            <Transition
               name="list-slide"
               :mode="listTransitionMode"
               :css="isListSlideActive"
               @after-leave="onAfterListLeave"
               @after-enter="onAfterListEnter"
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
      <Teleport to="body">
         <div
            ref="namePopoverRef"
            class="Popover"
            role="tooltip"
            @pointerenter="popovers.onNamePopoverEnter"
            @pointerleave="popovers.onNamePopoverLeave"
         ></div>
         <div
            ref="checkboxPopoverRef"
            class="Popover ScanResultsList-checkboxPopover"
            role="tooltip"
            @pointerenter="popovers.onCheckboxPopoverEnter"
            @pointerleave="popovers.onCheckboxPopoverLeave"
         ></div>
      </Teleport>
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
   /* Clip horizontal overflow so list-slide translateX does not create a
      scrollbar from custom ::-webkit-scrollbar styling. */
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
      /* Keep the first row above the top-shadow gradient. */
      z-index: 5;
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

/* Inherits .Popover from classes.css; wider max-width because it's a sentence. */
.ScanResultsList-checkboxPopover {
   max-width: 280px;
   word-break: normal;
}
</style>
