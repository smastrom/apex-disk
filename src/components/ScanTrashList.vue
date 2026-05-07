<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanTrashList

Purpose: Fullscreen list of items scheduled for trash. Checkboxes (default on) update progress and button size. Red Move to Trash button with countdown then spinner when processing.

Props: items (TrashListItem[]) — countdown starts automatically when component is rendered

Example:
 <ScanTrashList :items="trashItems" @update:selectedSize="onSize" @complete="onComplete" @reset="onReset" />
-->

<script setup lang="ts">
import ScanTrashListItem from './ScanTrashListItem.vue'
import ScanListNav from './ScanListNav.vue'
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
   (e: 'reset'): void
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

/** Checked count and total size for log lines (delete review list). */
function checkedTotalsLabel(): string {
   return `${checkedCount.value} checked, ${formatBytes(selectedSize.value)}`
}

/**
 * Toggles a single item's checked state by cloning the Map and replacing the ref.
 * Clone-and-replace ensures Vue sees a new reference and triggers dependents.
 */
function toggle(path: string) {
   const item = props.items.find((i) => i.path === path)
   const prev = checkedMapRef.value
   const wasChecked = !!prev.get(path)
   const next = new Map(prev)

   next.set(path, !wasChecked)
   checkedMapRef.value = next

   if (!item) {
      return
   }

   const kind = item.is_file ? 'file' : 'folder'
   const action = wasChecked ? 'Uncheck' : 'Check'

   log(
      'trash',
      `Trash: ${action} ${kind} "${item.name}" (${formatBytes(item.size)}) — ${checkedTotalsLabel()}`
   )
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
   log('trash', `Trash: moving ${toTrash.length} item(s) (${formatBytes(selectedSize.value)})`)

   const items = toTrash.map((item) => ({
      path: item.path,
      is_file: item.is_file,
      size: item.size,
   }))

   let summary = { count: toTrash.length, size: selectedSize.value }
   try {
      const result = await invoke<{ count: number; size: number }>('trash_paths', { items })
      summary = result
   } catch (error) {
      log('trash', 'Trash: invoke error, using optimistic values', error)
   }

   log(
      'trash',
      `Trash: moved ${summary.count}/${toTrash.length} item(s) (${formatBytes(summary.size)})`
   )

   await new Promise((r) => setTimeout(r, TRASH_POST_TRASH_SLEEP_MS))
   // Keep isDeleting=true visually — the parent will tear down this component
   // after handling `complete`. Resetting here would flash the ready state.
   emit('complete', summary)
}
</script>

<template>
   <div class="ScanTrashList-root" data-testid="trash-list">
      <ScanListNav
         :isForwardShown="false"
         :isBackDisabled="false"
         pathIcon="trash"
         :pathLabel="t('ScanTrashList', 'navTitle')"
         isActionsShown
         :isResetShown="true"
         :isResetDisabled="false"
         @back="emit('back', getCheckedItems())"
         @reset="emit('reset')"
         @cancel="emit('cancel')"
      />
      <div
         class="ScanTrashList-listWrap"
         :class="{ 'ScanTrashList-listWrap--deleting': isDeleting }"
      >
         <div class="ScanTrashList-list ScanTrashList-listScroll">
            <div class="ScanTrashList-listInner">
               <div v-for="item in items" :key="item.path" class="ScanTrashList-listItem">
                  <ScanTrashListItem
                     :item="item"
                     :isSelected="!!checkedMapRef.get(item.path)"
                     :formatBytes="formatBytes"
                     @toggle="toggle(item.path)"
                  />
               </div>
            </div>
         </div>
      </div>
      <div class="ScanTrashList-footer">
         <button
            type="button"
            class="GradientButton ScanTrashList-moveToTrashBtn"
            :data-deleting="isDeleting || undefined"
            :disabled="countdownRemaining > 0 || checkedCount === 0 || isDeleting"
            data-testid="confirm-trash"
            @click="onTrashClick"
         >
            <Transition name="ScanTrashList-caption" mode="out-in">
               <span v-if="!isDeleting" key="ready" class="ScanTrashList-captionText">
                  {{
                     selectedSize > 0
                        ? t('ScanTrashList', 'moveToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanTrashList', 'moveToTrash')
                  }}
               </span>
               <span
                  v-else-if="prefersReducedMotion"
                  key="deleting-text"
                  class="ScanTrashList-captionText"
               >
                  {{
                     selectedSize > 0
                        ? t('ScanTrashList', 'movingToTrashSize', {
                             size: formatBytes(selectedSize),
                          })
                        : t('ScanTrashList', 'movingToTrash')
                  }}
               </span>
               <Spinner v-else key="deleting-spinner" :size="18" class="ScanTrashList-spinner" />
            </Transition>
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanTrashList-root {
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

.ScanTrashList-listWrap {
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

.ScanTrashList-listWrap--deleting {
   opacity: 0.5;
   pointer-events: none;
}

.ScanTrashList-list {
   flex: 1;
   min-height: 0;
}

.ScanTrashList-listScroll {
   overflow-x: hidden;
   overflow-y: auto;
   overflow-y: overlay;
}

.ScanTrashList-listInner {
   position: relative;
   width: 100%;
}

.ScanTrashList-listItem {
   position: relative;

   &:first-of-type {
      /* Greater than listWrap:before z-index, to prevent first item from partially being hidden by the gradient */
      z-index: 2;
   }
}

.ScanTrashList-footer {
   flex-shrink: 0;
   padding: var(--spacing-md);
   border-top: 1px solid var(--color-bg);
   background: var(--color-bg-elevated);
   box-shadow: 0 -2px 16px var(--color-bg);
}

.ScanTrashList-moveToTrashBtn {
   transition:
      box-shadow 0.2s var(--ease-standard),
      opacity 0.2s var(--ease-standard),
      padding 0.3s var(--ease-standard);

   &:disabled:not([data-deleting]) {
      opacity: 0.5;
   }
}

.ScanTrashList-captionText {
   display: inline-flex;
   align-items: center;
   gap: 0.5rem;
   white-space: nowrap;
}

.ScanTrashList-caption-enter-active,
.ScanTrashList-caption-leave-active {
   transition: opacity 0.3s var(--ease-standard);
}

.ScanTrashList-caption-enter-from,
.ScanTrashList-caption-leave-to {
   opacity: 0;
}

.ScanTrashList-spinner {
   color: var(--color-on-accent);
}

/* Keep opacity transitions; only remove movement (sliding, gap, padding). */
@media (prefers-reduced-motion: reduce) {
   .ScanTrashList-caption-enter-active,
   .ScanTrashList-caption-leave-active {
      transition: opacity 0.25s var(--ease-standard);
   }

   .ScanTrashList-moveToTrashBtn {
      transition: opacity 0.2s var(--ease-standard);
   }
}
</style>
