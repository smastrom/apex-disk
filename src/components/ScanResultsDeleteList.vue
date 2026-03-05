<!--
ScanResultsDeleteList

Purpose: Fullscreen list of items scheduled for delete. Checkboxes (default on) update progress and button size. Red Delete button with countdown then spinner when processing.

Props: items (DeleteListItem[]) — countdown starts automatically when component is rendered

Example:
 <ScanResultsDeleteList :items="deleteItems" @update:selectedSize="onSize" @complete="onComplete" />
-->

<script setup lang="ts">
import ScanResultsDeleteListItem from './ScanResultsDeleteListItem.vue'
import ScanResultsNav from './ScanResultsNav.vue'
import Spinner from './Spinner.vue'

import {
   ref,
   shallowRef,
   watch,
   computed,
   onMounted,
   onActivated,
   onDeactivated,
   onUnmounted,
   useTemplateRef,
} from 'vue'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { invoke } from '@tauri-apps/api/core'
import { PhTrashSimple } from '@phosphor-icons/vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'
import { useReducedMotion } from '@/lib/use-reduced-motion'

import { DELETE_COUNTDOWN_MS, DELETE_POST_DELETE_SLEEP_MS } from '@/lib/constants'

import type { DeleteListItem } from '@/types/structs'

const props = defineProps<{
   items: DeleteListItem[]
}>()

const emit = defineEmits<{
   (e: 'back', checkedItems: DeleteListItem[]): void
   (e: 'update:selectedSize', value: number): void
   (e: 'complete', items: DeleteListItem[]): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()

/**
 * Safety countdown: the delete button stays disabled for DELETE_COUNTDOWN_MS
 * after the view becomes active. Prevents accidental taps when the user just
 * navigated in. A plain `let` timer ID is fine — it's never read reactively.
 */
const countdownRemaining = ref(0)
let countdownInterval: ReturnType<typeof setInterval> | null = null

/** Resets and starts a fresh countdown. Safe to call multiple times. */
function startCountdown() {
   if (countdownInterval) {
      clearInterval(countdownInterval)
      countdownInterval = null
   }

   countdownRemaining.value = DELETE_COUNTDOWN_MS
   countdownInterval = setInterval(() => {
      countdownRemaining.value -= 1000
      if (countdownRemaining.value <= 0 && countdownInterval) {
         clearInterval(countdownInterval)
         countdownInterval = null
      }
   }, 1000)
}

function stopCountdown() {
   if (countdownInterval) {
      clearInterval(countdownInterval)
      countdownInterval = null
   }

   countdownRemaining.value = 0
}

// Start countdown automatically when component is mounted
onMounted(startCountdown)

/**
 * KeepAlive re-activation: restart the countdown when component is re-activated.
 */
onActivated(startCountdown)

onUnmounted(stopCountdown)

/**
 * Cleanup when component becomes inactive (KeepAlive scenario).
 * Reset crucial states to prevent stale state when returning to this view.
 */
onDeactivated(() => {
   stopCountdown()
   isDeleting.value = false
   checkedMapRef.value.clear()
})

/**
 * Checked-state map: Map<path, boolean>.
 *
 * Performance design choices:
 * - shallowRef(Map) instead of reactive(Map): every toggle replaces the whole
 *   Map reference, which triggers a single reactive notification. This keeps
 *   the update path predictable and avoids Vue tracking every individual key.
 * - Map<string, boolean> keyed by path: O(1) per-item lookup in the template
 *   and in size/count computeds without needing a secondary index.
 */
const checkedMapRef = shallowRef(new Map<string, boolean>())
const isDeleting = ref(false)

const { prefersReducedMotion } = useReducedMotion()

const parentRef = useTemplateRef<HTMLElement>('parentRef')

/**
 * Virtual scroller: only renders rows visible in the viewport + overscan buffer.
 * Wrapped in a computed so the virtualizer reactively tracks count/key changes.
 */
const rowVirtualizer = useVirtualizer(
   computed(() => ({
      count: props.items.length,
      getScrollElement: () => parentRef.value,
      estimateSize: () => 48,
      overscan: 5,
      getItemKey: (index: number) => props.items[index]?.path ?? index,
   }))
)

/**
 * Initialises all items as checked when a new set of items arrives.
 * Builds the Map in one shot and assigns it atomically (single reactive write).
 */
watch(
   () => props.items,
   (items) => {
      const next = new Map<string, boolean>()
      for (const item of items) next.set(item.path, true)
      checkedMapRef.value = next
   },
   { immediate: true }
)

/** Total size of currently checked items. Drives the button label and parent disk-usage bar. */
const selectedSize = computed(() => {
   const map = checkedMapRef.value
   let total = 0

   for (const item of props.items) {
      if (map.get(item.path)) total += item.size
   }

   return total
})

/** Emits size changes to parent so the disk usage bar stays in sync. */
watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

/** Number of checked items. Used to disable the delete button when nothing is checked. */
const checkedCount = computed(() => {
   const map = checkedMapRef.value
   let n = 0

   for (const item of props.items) {
      if (map.get(item.path)) n++
   }

   return n
})

/**
 * Toggles a single item's checked state by cloning the Map and replacing the ref.
 * Clone-and-replace ensures Vue sees a new reference and triggers dependents.
 */
function toggle(path: string) {
   const prev = checkedMapRef.value
   const next = new Map(prev)

   next.set(path, !prev.get(path))
   checkedMapRef.value = next
}

/** Returns checked items for the back-navigation emit (restores selection in ScanResultsList). */
function getCheckedItems(): DeleteListItem[] {
   const map = checkedMapRef.value

   return props.items.filter((item) => map.get(item.path))
}

const deleteReady = computed(() => countdownRemaining.value <= 0)

/**
 * Delete handler. Guards against double-clicks and empty selections.
 * Calls the Tauri `delete_paths` command, waits a short delay (so the user
 * sees the spinner), then emits `complete` with the deleted items.
 */
async function onDeleteClick() {
   if (!deleteReady.value || isDeleting.value || checkedCount.value === 0) return

   isDeleting.value = true
   const toDelete = props.items.filter((item) => checkedMapRef.value.get(item.path))

   const items = toDelete.map((item) => ({
      path: item.path,
      is_file: item.is_file,
   }))
   await invoke('delete_paths', { items }).catch(() => {})

   await new Promise((r) => setTimeout(r, DELETE_POST_DELETE_SLEEP_MS))
   // Keep isDeleting=true visually — the parent will tear down this component
   // after handling `complete`. Resetting here would flash the ready state.
   emit('complete', toDelete)
}
</script>

<template>
   <div class="ScanResultsDeleteList-root" data-testid="delete-list">
      <ScanResultsNav
         :isForwardShown="false"
         :isBackDisabled="false"
         pathIcon="trash"
         :pathLabel="t('ScanResultsDeleteList', 'navTitleMoveToTrash')"
         :isActionsShown="true"
         :isResetDisabled="true"
         :isResetShown="false"
         @back="emit('back', getCheckedItems())"
         @cancel="emit('cancel')"
      />
      <div
         class="ScanResultsDeleteList-listWrap"
         :class="{ 'ScanResultsDeleteList-listWrap--deleting': isDeleting }"
      >
         <div ref="parentRef" class="ScanResultsDeleteList-list ScanResultsDeleteList-listScroll">
            <div
               class="ScanResultsDeleteList-listInner"
               :style="{ height: rowVirtualizer.getTotalSize() + 'px' }"
            >
               <div
                  v-for="virtualRow in rowVirtualizer.getVirtualItems()"
                  :key="String(virtualRow.key)"
                  class="ScanResultsDeleteList-listItem"
                  :style="{
                     position: 'absolute',
                     top: 0,
                     left: 0,
                     width: '100%',
                     transform: `translateY(${virtualRow.start}px)`,
                  }"
               >
                  <ScanResultsDeleteListItem
                     :item="items[virtualRow.index]"
                     :isSelected="!!checkedMapRef.get(items[virtualRow.index].path)"
                     :formatBytes="formatBytes"
                     @toggle="toggle(items[virtualRow.index].path)"
                  />
               </div>
            </div>
         </div>
      </div>
      <div class="ScanResultsDeleteList-footer">
         <button
            type="button"
            class="ScanResultsDeleteList-deleteBtn"
            :data-deleting="isDeleting || undefined"
            :disabled="countdownRemaining > 0 || checkedCount === 0 || isDeleting"
            data-testid="confirm-delete"
            @click="onDeleteClick"
         >
            <Transition name="ScanResultsDeleteList-caption" mode="out-in">
               <span v-if="!isDeleting" key="ready" class="ScanResultsDeleteList-captionText">
                  <PhTrashSimple :size="18" weight="bold" aria-hidden="true" />
                  {{
                     selectedSize > 0
                        ? t('ScanResultsDeleteList', 'moveToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanResultsDeleteList', 'moveToTrash')
                  }}
               </span>
               <span
                  v-else-if="prefersReducedMotion"
                  key="deleting-text"
                  class="ScanResultsDeleteList-captionText"
               >
                  {{
                     selectedSize > 0
                        ? t('ScanResultsDeleteList', 'movingToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanResultsDeleteList', 'movingToTrash')
                  }}
               </span>
               <Spinner
                  v-else
                  key="deleting-spinner"
                  :size="18"
                  class="ScanResultsDeleteList-spinner"
               />
            </Transition>
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsDeleteList-root {
   position: relative;
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   overflow: hidden;
   background: var(--color-bg);
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.ScanResultsDeleteList-listWrap {
   position: relative;
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   transition: opacity 0.25s;

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
   }
}

.ScanResultsDeleteList-listWrap--isDeleting {
   opacity: 0.5;
   pointer-events: none;
}

.ScanResultsDeleteList-list {
   flex: 1;
   min-height: 0;
}

.ScanResultsDeleteList-listScroll {
   overflow: auto;
}

.ScanResultsDeleteList-listInner {
   position: relative;
   width: 100%;
}

.ScanResultsDeleteList-listItem {
   will-change: transform;
}

.ScanResultsDeleteList-footer {
   flex-shrink: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanResultsDeleteList-deleteBtn {
   height: var(--cta-btn-height);
   width: 100%;
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.9375rem;
   font-weight: 600;
   color: #fff;
   background: #ff3b30;
   border: none;
   border-radius: 8px;
   cursor: pointer;
   transition:
      opacity 0.2s,
      padding 0.3s var(--ease-standard);

   &:hover:not(:disabled) {
      opacity: 0.9;
   }

   &:disabled:not([data-deleting]) {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

.ScanResultsDeleteList-captionText {
   display: inline-flex;
   align-items: center;
   gap: 0.5rem;
   white-space: nowrap;
}

.ScanResultsDeleteList-caption-enter-active,
.ScanResultsDeleteList-caption-leave-active {
   transition: opacity 0.3s var(--ease-standard);
}

.ScanResultsDeleteList-caption-enter-from,
.ScanResultsDeleteList-caption-leave-to {
   opacity: 0;
}

.ScanResultsDeleteList-spinner {
   color: #fff;
}

/* Keep opacity transitions; only remove movement (sliding, gap, padding). */
@media (prefers-reduced-motion: reduce) {
   .ScanResultsDeleteList-caption-enter-active,
   .ScanResultsDeleteList-caption-leave-active {
      transition: opacity 0.25s var(--ease-standard);
   }

   .ScanResultsDeleteList-deleteBtn {
      transition: opacity 0.2s;
   }
}
</style>
