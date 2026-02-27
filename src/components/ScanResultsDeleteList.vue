<!--
ScanResultsDeleteList

Purpose: Fullscreen list of items scheduled for delete. Checkboxes (default on) update progress and button size. Red Delete button with countdown then spinner when processing.

Props: items (DeleteListItem[]), active (boolean?) — when true, countdown starts

Example:
 <ScanResultsDeleteList :items="deleteItems" :active="isDeleteView" @update:selectedSize="onSize" @complete="onComplete" />
-->

<script setup lang="ts">
import ScanResultsDeleteListItem from './ScanResultsDeleteListItem.vue'
import ScanResultsNav from './ScanResultsNav.vue'
import Spinner from './Spinner.vue'

import { ref, shallowRef, watch, computed, onUnmounted } from 'vue'
import { useVirtualizer } from '@tanstack/vue-virtual'
import { invoke } from '@tauri-apps/api/core'
import { PhTrash } from '@phosphor-icons/vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/useTranslations'

import {
   DELETE_COUNTDOWN_SECONDS,
   DELETE_POST_DELETE_SLEEP_MS,
   MOCK_DELETE_DURATION_MS,
} from '@/lib/constants'

import type { DeleteListItem } from '@/types/structures'

const MOCK_DELETE = false // import.meta.env.DEV

const { t } = useTranslations()

const props = defineProps<{
   items: DeleteListItem[]
   active?: boolean
}>()

const countdownRemaining = ref(0)
let countdownInterval: ReturnType<typeof setInterval> | null = null

watch(
   () => props.active,
   (active) => {
      if (countdownInterval) {
         clearInterval(countdownInterval)
         countdownInterval = null
      }
      if (active) {
         countdownRemaining.value = DELETE_COUNTDOWN_SECONDS
         countdownInterval = setInterval(() => {
            countdownRemaining.value -= 1
            if (countdownRemaining.value <= 0 && countdownInterval) {
               clearInterval(countdownInterval)
               countdownInterval = null
            }
         }, 1000)
      } else {
         countdownRemaining.value = 0
      }
   },
   { immediate: true }
)

onUnmounted(() => {
   if (countdownInterval) clearInterval(countdownInterval)
})

const emit = defineEmits<{
   (e: 'back', checkedPaths: string[]): void
   (e: 'update:selectedSize', value: number): void
   (e: 'complete', items: DeleteListItem[]): void
}>()

/** ShallowRef so we replace the whole Map on load (one reactive write) instead of N .set() calls. */
const checkedMapRef = shallowRef(new Map<string, boolean>())
const deleting = ref(false)

const parentRef = ref<HTMLElement | null>(null)

const rowVirtualizer = useVirtualizer(
   computed(() => ({
      count: props.items.length,
      getScrollElement: () => parentRef.value,
      estimateSize: () => 40,
      overscan: 5,
      getItemKey: (index: number) => props.items[index]?.path ?? index,
   }))
)

watch(
   () => props.items,
   (items) => {
      const next = new Map<string, boolean>()
      for (const item of items) next.set(item.path, true)
      checkedMapRef.value = next
   },
   { immediate: true }
)

const selectedSize = computed(() => {
   const map = checkedMapRef.value
   let total = 0
   for (const item of props.items) {
      if (map.get(item.path)) total += item.size
   }
   return total
})

watch(selectedSize, (size) => emit('update:selectedSize', size), { immediate: true })

const checkedCount = computed(() => {
   const map = checkedMapRef.value
   let n = 0
   for (const item of props.items) {
      if (map.get(item.path)) n++
   }
   return n
})

function toggle(path: string) {
   const prev = checkedMapRef.value
   const next = new Map(prev)
   next.set(path, !prev.get(path))
   checkedMapRef.value = next
}

function getCheckedPaths(): string[] {
   const map = checkedMapRef.value
   return props.items.filter((item) => map.get(item.path)).map((item) => item.path)
}

const deleteReady = computed(() => countdownRemaining.value <= 0)

async function onDeleteClick() {
   if (!deleteReady.value || deleting.value || checkedCount.value === 0) return
   deleting.value = true
   const toDelete = props.items.filter((item) => checkedMapRef.value.get(item.path))

   if (MOCK_DELETE) {
      await new Promise((r) => setTimeout(r, MOCK_DELETE_DURATION_MS))
   } else {
      const items = toDelete.map((item) => ({
         path: item.path,
         is_file: item.is_file,
      }))
      await invoke('delete_paths', { items }).catch(() => {})
   }

   await new Promise((r) => setTimeout(r, DELETE_POST_DELETE_SLEEP_MS))
   emit('complete', toDelete)
   deleting.value = false
}
</script>

<template>
   <div class="ScanResultsDeleteList-root">
      <ScanResultsNav
         :showForward="false"
         :backDisabled="false"
         :pathLabel="t('ScanResultsDeleteList', 'navTitle')"
         :showActions="false"
         @back="emit('back', getCheckedPaths())"
      />
      <div
         class="ScanResultsDeleteList-listWrap"
         :class="{ 'ScanResultsDeleteList-listWrap--deleting': deleting }"
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
                     :selected="!!checkedMapRef.get(items[virtualRow.index].path)"
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
            :disabled="countdownRemaining > 0 || checkedCount === 0 || deleting"
            @click="onDeleteClick"
         >
            <Spinner v-if="deleting" :size="18" class="ScanResultsDeleteList-spinner" />
            <PhTrash v-else :size="18" weight="bold" />
            <span v-if="!deleting">{{
               selectedSize > 0
                  ? t('ScanResultsDeleteList', 'deleteSize', { size: formatBytes(selectedSize) })
                  : t('ScanResultsDeleteList', 'delete')
            }}</span>
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
   flex: 1;
   min-height: 0;
   display: flex;
   flex-direction: column;
   transition: opacity 0.25s;
}

.ScanResultsDeleteList-listWrap--deleting {
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
   transition: opacity 0.2s;

   &:hover:not(:disabled) {
      opacity: 0.9;
   }

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

.ScanResultsDeleteList-spinner {
   color: #fff;
}
</style>
