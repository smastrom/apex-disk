<!--
ScanResultsTrashList

Purpose: Fullscreen list of items scheduled for trash. Checkboxes (default on) update progress and button size. Red Move to Trash button with countdown then spinner when processing.

Props: items (TrashListItem[]) — countdown starts automatically when component is rendered

Example:
 <ScanResultsTrashList :items="trashItems" @update:selectedSize="onSize" @complete="onComplete" />
-->

<script setup lang="ts">
import ScanResultsTrashListItem from './ScanResultsTrashListItem.vue'
import ScanResultsNav from './ScanResultsNav.vue'
import Spinner from './ui/Spinner.vue'

import {
   ref,
   shallowRef,
   watch,
   computed,
   onMounted,
   onActivated,
   onDeactivated,
   onUnmounted,
} from 'vue'
import { invoke } from '@tauri-apps/api/core'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useTranslations } from '@/lib/use-translations'
import { useReducedMotion } from '@/lib/use-reduced-motion'

import { TRASH_COUNTDOWN_MS, TRASH_POST_TRASH_SLEEP_MS } from '@/lib/constants'

import type { TrashListItem } from '@/types/structs'

const props = defineProps<{
   items: TrashListItem[]
}>()

const emit = defineEmits<{
   (e: 'back', checkedItems: TrashListItem[]): void
   (e: 'update:selectedSize', value: number): void
   (e: 'complete', summary: { count: number; size: number }): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()

/**
 * Safety countdown: the trash button stays disabled for TRASH_COUNTDOWN_MS
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

   countdownRemaining.value = TRASH_COUNTDOWN_MS
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
 * KeepAlive re-activation: restart the countdown and re-sync checkedMapRef
 * if props.items changed while deactivated (e.g. user went back to results
 * and reviewed a different selection). AppView switches don't change
 * props.items, so selections are preserved in that case.
 */
onActivated(() => {
   startCountdown()

   if (props.items !== lastInitializedItems) {
      initCheckedMap(props.items)
   }
})

onUnmounted(stopCountdown)

/**
 * Cleanup when component becomes inactive (KeepAlive scenario).
 * Only reset transient UI states — checkedMapRef is intentionally preserved
 * so selections survive AppView switches. It is cleared on abort or after
 * a successful trash (via parent handling of 'cancel' / 'complete' events).
 */
onDeactivated(() => {
   stopCountdown()
   isDeleting.value = false
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

/** Tracks which items array was last used to build checkedMapRef. */
let lastInitializedItems: TrashListItem[] | null = null

/** Builds a fresh all-checked Map from items and assigns it atomically. */
function initCheckedMap(items: TrashListItem[]) {
   const next = new Map<string, boolean>()
   for (const item of items) next.set(item.path, true)
   checkedMapRef.value = next
   lastInitializedItems = items
}

/**
 * Initialises all items as checked when a new set of items arrives.
 * Builds the Map in one shot and assigns it atomically (single reactive write).
 */
watch(() => props.items, initCheckedMap, { immediate: true })

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

/** Number of checked items. Used to disable the trash button when nothing is checked. */
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
function getCheckedItems(): TrashListItem[] {
   const map = checkedMapRef.value

   return props.items.filter((item) => map.get(item.path))
}

const trashReady = computed(() => countdownRemaining.value <= 0)

/**
 * Trash handler. Guards against double-clicks and empty selections.
 * Calls the Tauri `trash_paths` command, waits a short delay (so the user
 * sees the spinner), then emits `complete` with the trashed items.
 */
async function onTrashClick() {
   if (!trashReady.value || isDeleting.value || checkedCount.value === 0) return

   isDeleting.value = true
   const toTrash = props.items.filter((item) => checkedMapRef.value.get(item.path))
   log('trash', `Moving ${toTrash.length} items to Trash (${formatBytes(selectedSize.value)})`)

   const items = toTrash.map((item) => ({
      path: item.path,
      is_file: item.is_file,
      size: item.size,
   }))

   let summary = { count: toTrash.length, size: selectedSize.value }
   try {
      const result = await invoke<{ count: number; size: number }>('trash_paths', { items })
      summary = result
   } catch {
      // Fall back to optimistic values
   }

   log(
      'trash',
      `Moved ${summary.count}/${toTrash.length} items to Trash (${formatBytes(summary.size)})`
   )

   await new Promise((r) => setTimeout(r, TRASH_POST_TRASH_SLEEP_MS))
   // Keep isDeleting=true visually — the parent will tear down this component
   // after handling `complete`. Resetting here would flash the ready state.
   emit('complete', summary)
}
</script>

<template>
   <div class="ScanResultsTrashList-root" data-testid="trash-list">
      <ScanResultsNav
         :isForwardShown="false"
         :isBackDisabled="false"
         pathIcon="trash"
         :pathLabel="t('ScanResultsTrashList', 'navTitle')"
         isActionsShown
         isResetDisabled
         :isResetShown="false"
         @back="emit('back', getCheckedItems())"
         @cancel="emit('cancel')"
      />
      <div
         class="ScanResultsTrashList-listWrap"
         :class="{ 'ScanResultsTrashList-listWrap--deleting': isDeleting }"
      >
         <div class="ScanResultsTrashList-list ScanResultsTrashList-listScroll">
            <div class="ScanResultsTrashList-listInner">
               <div v-for="item in items" :key="item.path" class="ScanResultsTrashList-listItem">
                  <ScanResultsTrashListItem
                     :item="item"
                     :isSelected="!!checkedMapRef.get(item.path)"
                     :formatBytes="formatBytes"
                     @toggle="toggle(item.path)"
                  />
               </div>
            </div>
         </div>
      </div>
      <div class="ScanResultsTrashList-footer">
         <button
            type="button"
            class="GradientButton ScanResultsTrashList-moveToTrashBtn"
            :data-deleting="isDeleting || undefined"
            :disabled="countdownRemaining > 0 || checkedCount === 0 || isDeleting"
            data-testid="confirm-trash"
            @click="onTrashClick"
         >
            <Transition name="ScanResultsTrashList-caption" mode="out-in">
               <span v-if="!isDeleting" key="ready" class="ScanResultsTrashList-captionText">
                  {{
                     selectedSize > 0
                        ? t('ScanResultsTrashList', 'moveToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanResultsTrashList', 'moveToTrash')
                  }}
               </span>
               <span
                  v-else-if="prefersReducedMotion"
                  key="deleting-text"
                  class="ScanResultsTrashList-captionText"
               >
                  {{
                     selectedSize > 0
                        ? t('ScanResultsTrashList', 'movingToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanResultsTrashList', 'movingToTrash')
                  }}
               </span>
               <Spinner
                  v-else
                  key="deleting-spinner"
                  :size="18"
                  class="ScanResultsTrashList-spinner"
               />
            </Transition>
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsTrashList-root {
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

.ScanResultsTrashList-listWrap {
   position: relative;
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   transition: opacity 0.25s var(--ease-standard);

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

.ScanResultsTrashList-listWrap--deleting {
   opacity: 0.5;
   pointer-events: none;
}

.ScanResultsTrashList-list {
   flex: 1;
   min-height: 0;
}

.ScanResultsTrashList-listScroll {
   overflow: auto;
}

.ScanResultsTrashList-listInner {
   position: relative;
   width: 100%;
}

.ScanResultsTrashList-footer {
   flex-shrink: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanResultsTrashList-moveToTrashBtn {
   height: var(--cta-btn-height);
   width: 100%;
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-lg);
   border-radius: 8px;
   transition:
      box-shadow 0.2s var(--ease-standard),
      opacity 0.2s var(--ease-standard),
      padding 0.3s var(--ease-standard);

   &:hover:not(:disabled) {
      box-shadow: 0 0 14px var(--color-action-glow);
   }

   &:disabled:not([data-deleting]) {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

.ScanResultsTrashList-captionText {
   display: inline-flex;
   align-items: center;
   gap: 0.5rem;
   white-space: nowrap;
}

.ScanResultsTrashList-caption-enter-active,
.ScanResultsTrashList-caption-leave-active {
   transition: opacity 0.3s var(--ease-standard);
}

.ScanResultsTrashList-caption-enter-from,
.ScanResultsTrashList-caption-leave-to {
   opacity: 0;
}

.ScanResultsTrashList-spinner {
   color: var(--color-on-accent);
}

/* Keep opacity transitions; only remove movement (sliding, gap, padding). */
@media (prefers-reduced-motion: reduce) {
   .ScanResultsTrashList-caption-enter-active,
   .ScanResultsTrashList-caption-leave-active {
      transition: opacity 0.25s var(--ease-standard);
   }

   .ScanResultsTrashList-moveToTrashBtn {
      transition: opacity 0.2s var(--ease-standard);
   }
}
</style>
