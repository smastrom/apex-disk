<!--
ScanViewDiskUsage

Purpose: Progress bar showing current disk usage (home volume). Shows "new free" in accent when items are selected for delete.

Props: usage (DiskUsage | null), selectedSize (number?)

Example:
 <ScanViewDiskUsage :usage="diskUsage" :selectedSize="selectedSize" />
-->

<script setup lang="ts">
import { computed } from 'vue'
import { PhHardDrive } from '@phosphor-icons/vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

import type { DiskUsage } from '@/types/disk'

const props = defineProps<{
   usage?: DiskUsage | null
   selectedSize?: number
}>()

const { t } = useTranslations()

/** Main bar: shrinks when selecting to show remaining used after delete. */
const mainBarPercent = computed(() => {
   const u = props.usage

   if (!u || u.total === 0) return 0

   const sel = props.selectedSize ?? 0
   const usedAfterDelete = u.total - u.free - sel

   return Math.max(0, Math.min(100, (usedAfterDelete / u.total) * 100))
})

/** Lighter bar behind: full current used. Extends past main when selecting = "to be wiped". */
const lighterBarPercent = computed(() => {
   const u = props.usage

   if (!u || u.total === 0) return 0

   return Math.min(100, ((u.total - u.free) / u.total) * 100)
})

const newFreeSpace = computed(() => {
   const u = props.usage
   const sel = props.selectedSize ?? 0

   if (!u || sel === 0) return null

   return u.free + sel
})
</script>

<template>
   <div v-if="props.usage" class="ScanViewDiskUsage-root" data-testid="disk-usage">
      <div class="ScanViewDiskUsage-header">
         <span class="ScanViewDiskUsage-userBadge">
            {{ props.usage.user_name }}
         </span>

         <span class="ScanViewDiskUsage-volume">
            <PhHardDrive
               :size="14"
               weight="regular"
               class="ScanViewDiskUsage-volumeIcon"
               aria-hidden="true"
            />
            {{ props.usage.volume_name }}
         </span>
      </div>
      <div class="ScanViewDiskUsage-infoRow">
         <span class="ScanViewDiskUsage-info">
            <span class="ScanViewDiskUsage-label">{{ t('ScanViewDiskUsage', 'total') }}</span>
            <span class="ScanViewDiskUsage-value">{{ formatBytes(props.usage.total) }}</span>
         </span>
         <span class="ScanViewDiskUsage-info">
            <span class="ScanViewDiskUsage-label">{{ t('ScanViewDiskUsage', 'free') }}</span>
            <span class="ScanViewDiskUsage-value">{{ formatBytes(props.usage.free) }}</span>
         </span>
         <span
            v-if="newFreeSpace !== null"
            class="ScanViewDiskUsage-info ScanViewDiskUsage-newFree"
            data-testid="disk-new-free"
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
   font-size: var(--font-size-md);
   font-weight: 600;
   color: var(--color-text);
}

.ScanViewDiskUsage-volumeIcon {
   flex-shrink: 0;
   color: var(--color-text-muted);
   opacity: 0.85;
}

.ScanViewDiskUsage-userBadge {
   font-size: var(--font-size-sm);
   font-weight: 600;
   color: var(--color-text);
   background: var(--color-surface);
   padding: 3px 10px;
   border-radius: 6px;
}

.ScanViewDiskUsage-infoRow {
   display: flex;
   justify-content: space-between;
   align-items: center;
   font-size: var(--font-size-base);
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
