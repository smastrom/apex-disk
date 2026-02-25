<!--
ScanViewDiskUsage

Purpose: Progress bar showing current disk usage (home volume). Shows "new free" in accent when items are selected for delete. Exposes refresh() to re-fetch after delete.

Props: selectedSize (number?)

Example:
 <ScanViewDiskUsage ref="diskUsageRef" :selectedSize="selectedSize" />
-->

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { PhHardDrive } from '@phosphor-icons/vue'
import { openPath } from '@tauri-apps/plugin-opener'

import { formatBytes } from '@/lib/format'
import { getDiskUsage } from '@/lib/disk'
import { useTranslations } from '@/lib/useTranslations'

import type { DiskUsage } from '@/lib/disk'

const { t } = useTranslations()

const usage = ref<DiskUsage | null>(null)

async function fetchUsage() {
   try {
      usage.value = await getDiskUsage()
   } catch (err) {
      console.error('Failed to get disk usage:', err)
   }
}

onMounted(fetchUsage)

defineExpose({ refresh: fetchUsage })

const props = defineProps<{
   selectedSize?: number
}>()

/** Main bar: shrinks when selecting to show remaining used after delete. */
const mainBarPercent = computed(() => {
   const u = usage.value
   if (!u || u.total === 0) return 0
   const sel = props.selectedSize ?? 0
   const usedAfterDelete = u.total - u.free - sel
   return Math.max(0, Math.min(100, (usedAfterDelete / u.total) * 100))
})

/** Lighter bar behind: full current used. Extends past main when selecting = "to be wiped". */
const lighterBarPercent = computed(() => {
   const u = usage.value
   if (!u || u.total === 0) return 0
   return Math.min(100, ((u.total - u.free) / u.total) * 100)
})

const newFreeSpace = computed(() => {
   const u = usage.value
   const sel = props.selectedSize ?? 0
   if (!u || sel === 0) return null
   return u.free + sel
})

async function openHomeInFinder() {
   if (!usage.value?.home_path) return
   try {
      await openPath(usage.value.home_path)
   } catch (err) {
      console.error('Failed to open home in Finder:', err)
   }
}
</script>

<template>
   <div v-if="usage" class="ScanViewDiskUsage-root">
      <div class="ScanViewDiskUsage-header">
         <span class="ScanViewDiskUsage-volume">
            <PhHardDrive :size="14" weight="regular" class="ScanViewDiskUsage-volumeIcon" />
            {{ usage.volume_name }}
         </span>
         <button type="button" class="ScanViewDiskUsage-userBadge" @click="openHomeInFinder">
            /{{ usage.user_name }}
         </button>
      </div>
      <div class="ScanViewDiskUsage-infoRow">
         <span class="ScanViewDiskUsage-info">
            <span class="ScanViewDiskUsage-label">{{ t('ScanViewDiskUsage', 'total') }}</span>
            <span class="ScanViewDiskUsage-value">{{ formatBytes(usage.total) }}</span>
         </span>
         <span class="ScanViewDiskUsage-info">
            <span class="ScanViewDiskUsage-label">{{ t('ScanViewDiskUsage', 'free') }}</span>
            <span class="ScanViewDiskUsage-value">{{ formatBytes(usage.free) }}</span>
         </span>
         <span
            v-if="newFreeSpace !== null"
            class="ScanViewDiskUsage-info ScanViewDiskUsage-newFree"
         >
            <span class="ScanViewDiskUsage-label">{{ t('ScanViewDiskUsage', 'newFree') }}</span>
            <span class="ScanViewDiskUsage-value ScanViewDiskUsage-newFreeValue">
               {{ formatBytes(newFreeSpace) }}
            </span>
         </span>
      </div>
      <div class="ScanViewDiskUsage-barWrap">
         <div
            v-if="props.selectedSize && props.selectedSize > 0"
            class="ScanViewDiskUsage-barLighter"
            :style="{ width: lighterBarPercent + '%' }"
         />
         <div class="ScanViewDiskUsage-barMain" :style="{ width: mainBarPercent + '%' }" />
      </div>
   </div>
</template>

<style scoped>
.ScanViewDiskUsage-root {
   flex-shrink: 0;
   padding: var(--spacing-sm) var(--spacing-md);
   background: var(--color-bg-elevated);
   border-bottom: 1px solid var(--color-border);
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.ScanViewDiskUsage-header {
   display: flex;
   justify-content: space-between;
   align-items: center;
   margin-bottom: var(--spacing-sm);
}

.ScanViewDiskUsage-volume {
   display: flex;
   align-items: center;
   gap: 6px;
   font-size: 0.875rem;
   font-weight: 600;
   color: var(--color-text);
}

.ScanViewDiskUsage-volumeIcon {
   flex-shrink: 0;
   color: var(--color-text-muted);
   opacity: 0.85;
}

.ScanViewDiskUsage-userBadge {
   font-size: 0.75rem;
   font-weight: 600;
   color: var(--color-text);
   background: var(--color-surface);
   padding: 3px 10px;
   border-radius: 6px;
   border: none;
   cursor: pointer;
   transition: background 0.2s ease;

   &:hover {
      background: var(--color-surface-hover);
   }
}

.ScanViewDiskUsage-infoRow {
   display: flex;
   justify-content: space-between;
   align-items: center;
   font-size: 0.8125rem;
}

.ScanViewDiskUsage-info {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ScanViewDiskUsage-label {
   color: var(--color-text-muted);
}

.ScanViewDiskUsage-value {
   color: var(--color-text);
   font-weight: 500;
}

.ScanViewDiskUsage-newFree {
   .ScanViewDiskUsage-label {
      color: var(--color-accent);
   }
}

.ScanViewDiskUsage-newFreeValue {
   color: var(--color-accent);
   font-weight: 600;
}

.ScanViewDiskUsage-barWrap {
   position: relative;
   margin-top: var(--spacing-sm);
   height: 10px;
   background: var(--color-surface);
   border-radius: 5px;
   overflow: hidden;
   box-shadow: var(--glow-inset);
}

.ScanViewDiskUsage-barLighter {
   position: absolute;
   left: 0;
   top: 0;
   height: 100%;
   background: color-mix(in srgb, var(--color-accent) 45%, var(--color-surface));
   border-radius: 5px;
   transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
   z-index: 0;
}

.ScanViewDiskUsage-barMain {
   position: absolute;
   left: 0;
   top: 0;
   height: 100%;
   background: linear-gradient(90deg, var(--color-accent) 0%, var(--color-accent-hover) 100%);
   border-radius: 5px;
   transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
   box-shadow: var(--glow-sm);
   z-index: 1;
}
</style>
