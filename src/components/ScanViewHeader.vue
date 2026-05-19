<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanViewHeader

Purpose: Progress bar showing current disk usage (home volume). Shows "new free" in accent when items are selected for delete.

Props: usage (DiskUsage | null), selectedSize (number?)

Example:
 <ScanViewHeader :usage="diskUsage" :selectedSize="selectedSize" />
-->

<script setup lang="ts">
import type { DiskUsage } from '@/types/disk'

import { PhHardDrive } from '@phosphor-icons/vue'
import { computed } from 'vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'
import { isDev } from '@/lib/utils'

import { DEV_USER_FOLDER_NAME } from '@/lib/constants'

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

const showNewFree = computed(() => {
   if (newFreeSpace.value === null || !props.usage) return false

   return formatBytes(props.usage.free) !== formatBytes(newFreeSpace.value)
})
</script>

<template>
   <div v-if="props.usage" class="ScanViewHeader-root" data-testid="disk-usage">
      <div class="ScanViewHeader-header">
         <span class="ScanViewHeader-volume">
            <PhHardDrive
               :size="14"
               weight="regular"
               class="ScanViewHeader-volumeIcon"
               aria-hidden="true"
            />
            <span class="ScanViewHeader-volumeName">{{ props.usage.volume_name }}</span>
         </span>
         <span class="ScanViewHeader-userBadge">
            /{{ isDev() ? DEV_USER_FOLDER_NAME : props.usage.user_name }}
         </span>
      </div>
      <div class="ScanViewHeader-infoRow">
         <span class="ScanViewHeader-info">
            <span class="ScanViewHeader-label">{{ t('ScanViewHeader', 'total') }}</span>
            <span class="ScanViewHeader-value">
               <span>{{ formatBytes(props.usage.total) }} </span>
            </span>
         </span>

         <span class="ScanViewHeader-info">
            <span class="ScanViewHeader-label">{{ t('ScanViewHeader', 'free') }}</span>
            <span
               :class="['ScanViewHeader-value', { 'ScanViewHeader-value-oldFree': showNewFree }]"
               >{{ formatBytes(props.usage.free) }}</span
            >
            <svg
               v-if="showNewFree"
               xmlns="http://www.w3.org/2000/svg"
               width="14"
               height="14"
               viewBox="0 0 24 24"
               fill="none"
               stroke="currentColor"
               stroke-width="2"
               stroke-linecap="round"
               stroke-linejoin="round"
               class="ScanViewHeader-arrowRight"
               aria-hidden="true"
            >
               <path d="M5 12h14" />
               <path d="m12 5 7 7-7 7" />
            </svg>

            <span v-if="showNewFree" class="ScanViewHeader-info" data-testid="disk-new-free">
               <span class="ScanViewHeader-value ScanViewHeader-value-newFree">
                  {{ formatBytes(newFreeSpace) }}
               </span>
            </span>
         </span>
      </div>
      <div
         class="ScanViewHeader-barWrap"
         role="meter"
         :aria-valuenow="Math.round(mainBarPercent)"
         aria-valuemin="0"
         aria-valuemax="100"
         :aria-label="t('ScanViewHeader', 'total')"
      >
         <div
            v-if="props.selectedSize && props.selectedSize > 0"
            class="ScanViewHeader-barLighter"
            :style="{ width: `${lighterBarPercent}%` }"
            aria-hidden="true"
         />
         <div
            class="ScanViewHeader-barMain"
            :style="{ width: `${mainBarPercent}%` }"
            aria-hidden="true"
         />
      </div>
   </div>
</template>

<style scoped>
.ScanViewHeader-root {
   flex-shrink: 0;
   padding: var(--spacing-sm) var(--spacing-md);
   background: var(--color-bg-elevated);
   border-bottom: 1px solid var(--color-border);
   max-width: var(--content-max-width);
   margin: 0 auto;
   width: 100%;
}

.ScanViewHeader-header {
   display: flex;
   justify-content: space-between;
   align-items: center;
   margin-bottom: var(--spacing-sm);
}

.ScanViewHeader-volume {
   display: flex;
   align-items: center;
   gap: 6px;
   font-size: var(--font-size-base);
   font-weight: 600;
   color: var(--color-text);
}

.ScanViewHeader-volumeIcon {
   flex-shrink: 0;
   color: var(--color-text-muted);
   opacity: 0.85;
}

.ScanViewHeader-volumeName {
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   max-width: 40vw;
}

.ScanViewHeader-userBadge {
   font-size: var(--font-size-sm);
   font-weight: 600;
   color: var(--color-text);
   background: var(--color-surface);
   padding: 3px 10px;
   border-radius: 6px;
   max-width: 40vw;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanViewHeader-infoRow {
   display: flex;
   justify-content: space-between;
   align-items: center;
   font-size: var(--font-size-base);
}

.ScanViewHeader-info {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ScanViewHeader-label {
   color: var(--color-text-muted);
   flex-shrink: 0;
}

.ScanViewHeader-value {
   color: var(--color-text);
   font-weight: 500;
}

.ScanViewHeader-value-oldFree {
   color: var(--color-text-muted);
}

.ScanViewHeader-value-newFree {
   font-weight: 700;
}

.ScanViewHeader-arrowRight {
   flex-shrink: 0;
   opacity: 0.8;
}

.ScanViewHeader-barWrap {
   position: relative;
   margin-top: var(--spacing-sm);
   height: 10px;
   background: var(--color-surface);
   border-radius: 5px;
   overflow: hidden;
   box-shadow: inset 0 0.5px 1px rgba(0, 0, 0, 0.24);
}

.ScanViewHeader-barLighter {
   position: absolute;
   left: 0;
   top: 0;
   height: 100%;
   background: var(--color-accent);
   opacity: 0.45;
   border-radius: 5px;
   transition: width 0.3s var(--ease-out);
   z-index: 0;
}

.ScanViewHeader-barMain {
   position: absolute;
   left: 0;
   top: 0;
   height: 100%;
   background: linear-gradient(90deg, var(--color-accent) 0%, var(--color-accent-hover) 100%);
   border-radius: 5px;
   transition: width 0.3s var(--ease-out);
   box-shadow:
      inset 0 0.5px 0 rgba(255, 255, 255, 0.3),
      var(--glow-sm);
   z-index: 1;
}
</style>
