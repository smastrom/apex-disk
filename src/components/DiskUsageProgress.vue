<!--
DiskUsageProgress

Purpose: Progress bar showing current disk usage (home volume). Shows "new free" in accent when items are selected for delete.

Props: selectedSize (number?)

Example:
 <DiskUsageProgress :selectedSize="selectedSize" />
-->

<script setup lang="ts">
import { PhHardDrive } from '@phosphor-icons/vue'

import { ref, computed, onMounted } from 'vue'
import { openPath } from '@tauri-apps/plugin-opener'

import { getDiskUsage } from '@/lib/disk'
import { useTranslations } from '@/lib/useTranslations'
import { formatBytes } from '@/lib/format'

import type { DiskUsage } from '@/lib/disk'

const { t } = useTranslations()

const usage = ref<DiskUsage | null>(null)

onMounted(async () => {
   try {
      usage.value = await getDiskUsage()
   } catch (err) {
      console.error('Failed to get disk usage:', err)
   }
})

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
   <div v-if="usage" class="DiskUsageProgress-root">
      <div class="DiskUsageProgress-header">
         <span class="DiskUsageProgress-volume">
            <PhHardDrive :size="14" weight="regular" class="DiskUsageProgress-volumeIcon" />
            {{ usage.volume_name }}
         </span>
         <button
            type="button"
            class="DiskUsageProgress-userBadge"
            @click="openHomeInFinder"
         >
            {{ usage.user_name }}
         </button>
      </div>
      <div class="DiskUsageProgress-infoRow">
         <span class="DiskUsageProgress-info">
            <span class="DiskUsageProgress-label">{{ t('DiskUsageProgress', 'total') }}</span>
            <span class="DiskUsageProgress-value">{{ formatBytes(usage.total) }}</span>
         </span>
         <span class="DiskUsageProgress-info">
            <span class="DiskUsageProgress-label">{{ t('DiskUsageProgress', 'free') }}</span>
            <span class="DiskUsageProgress-value">{{ formatBytes(usage.free) }}</span>
         </span>
         <span v-if="newFreeSpace !== null" class="DiskUsageProgress-info DiskUsageProgress-newFree">
            <span class="DiskUsageProgress-label">{{ t('DiskUsageProgress', 'newFree') }}</span>
            <span class="DiskUsageProgress-value DiskUsageProgress-newFreeValue">
               {{ formatBytes(newFreeSpace) }}
            </span>
         </span>
      </div>
      <div class="DiskUsageProgress-barWrap">
         <div
            v-if="props.selectedSize && props.selectedSize > 0"
            class="DiskUsageProgress-barLighter"
            :style="{ width: lighterBarPercent + '%' }"
         />
         <div
            class="DiskUsageProgress-barMain"
            :style="{ width: mainBarPercent + '%' }"
         />
      </div>
   </div>
</template>

<style scoped>
.DiskUsageProgress-root {
   flex-shrink: 0;
   padding: var(--spacing-sm) var(--spacing-md);
   background: var(--color-bg-elevated);
   border-bottom: 1px solid var(--color-border);
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.DiskUsageProgress-header {
   display: flex;
   justify-content: space-between;
   align-items: center;
   margin-bottom: var(--spacing-sm);
}

.DiskUsageProgress-volume {
   display: flex;
   align-items: center;
   gap: 6px;
   font-size: 0.875rem;
   font-weight: 600;
   color: var(--color-text);
}

.DiskUsageProgress-volumeIcon {
   flex-shrink: 0;
   color: var(--color-text-muted);
   opacity: 0.85;
}

.DiskUsageProgress-userBadge {
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

.DiskUsageProgress-infoRow {
   display: flex;
   justify-content: space-between;
   align-items: center;
   font-size: 0.8125rem;
}

.DiskUsageProgress-info {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.DiskUsageProgress-label {
   color: var(--color-text-muted);
}

.DiskUsageProgress-value {
   color: var(--color-text);
   font-weight: 500;
}

.DiskUsageProgress-newFree {
   .DiskUsageProgress-label {
      color: var(--color-accent);
   }
}

.DiskUsageProgress-newFreeValue {
   color: var(--color-accent);
   font-weight: 600;
}

.DiskUsageProgress-barWrap {
   position: relative;
   margin-top: var(--spacing-sm);
   height: 10px;
   background: var(--color-surface);
   border-radius: 5px;
   overflow: hidden;
   box-shadow: var(--glow-inset);
}

.DiskUsageProgress-barLighter {
   position: absolute;
   left: 0;
   top: 0;
   height: 100%;
   background: color-mix(in srgb, var(--color-accent) 45%, var(--color-surface));
   border-radius: 5px;
   transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1);
   z-index: 0;
}

.DiskUsageProgress-barMain {
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
