<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanProgress

Purpose: Active scan view showing disk/user context, live scan progress, and detailed scan metadata while scanning.

Props: progress (ScanProgress), elapsedSeconds (number)

Example:
 <ScanProgress :progress="progress" :elapsedSeconds="elapsedSeconds" @abort="onAbort" />
-->

<script setup lang="ts">
import Spinner from './ui/Spinner.vue'

import type { ScanProgress } from '@/types/structs'

import { computed } from 'vue'

import { formatBytes, formatProgressNumber } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   progress: ScanProgress
   elapsedSeconds: number
}>()

const emit = defineEmits<{
   (e: 'abort'): void
}>()

const { t } = useTranslations()

const percent = computed(() => {
   if (props.progress.total <= 0) return 0

   return Math.min(99, Math.round((props.progress.current / props.progress.total) * 100))
})

const elapsedDisplay = computed(() => {
   const total = props.elapsedSeconds
   const mins = Math.floor(total / 60)
   const secs = total % 60

   if (mins > 0) return `${mins}m ${String(secs).padStart(2, '0')}s`

   return `${secs}s`
})
</script>

<template>
   <section class="ScanProgress-root" data-testid="scanning-results" aria-label="Scan progress">
      <div class="ScanProgress-progressBlock">
         <div class="ScanProgress-progressHeader">
            <span class="ScanProgress-spinnerWrap">
               <Spinner :size="18" />
            </span>
            <p class="ScanProgress-progressTitle" data-testid="scan-progress">
               {{
                  t('ScanProgress', 'scanning', {
                     current: formatProgressNumber(progress.current),
                     total: formatProgressNumber(progress.total),
                  })
               }}
            </p>
         </div>
         <div
            class="ScanProgress-barWrap"
            role="progressbar"
            :aria-valuenow="percent"
            aria-valuemin="0"
            aria-valuemax="100"
         >
            <div class="ScanProgress-barMain" :style="{ transform: `scaleX(${percent / 100})` }" />
         </div>
      </div>

      <div class="ScanProgress-stats">
         <p>
            <span>{{ t('ScanProgress', 'scannedSize') }}</span>
            <strong>{{ formatBytes(progress.scanned_size_total) }}</strong>
         </p>
         <p>
            <span>{{ t('ScanProgress', 'currentPath') }}</span>
            <strong class="ScanProgress-currentPath" :title="progress.folder || ''">
               {{ progress.folder || t('ScanProgress', 'preparing') }}
            </strong>
         </p>

         <p>
            <span>{{ t('ScanProgress', 'elapsed') }}</span>
            <strong>{{ elapsedDisplay }}</strong>
         </p>
      </div>

      <button
         type="button"
         class="ScanProgress-abortBtn"
         data-testid="scan-abort"
         @click="emit('abort')"
      >
         {{ t('ScanProgress', 'abort') }}
      </button>
   </section>
</template>

<style scoped>
.ScanProgress-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   justify-content: center;
   gap: var(--spacing-md);
   max-width: var(--content-max-width);
   width: 100%;
   margin: 0 auto;
   padding: var(--spacing-lg) var(--spacing-md);
}

.ScanProgress-progressBlock {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
}

.ScanProgress-progressHeader {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 10px;
}

.ScanProgress-spinnerWrap {
   display: flex;

   @media (prefers-reduced-motion: reduce) {
      display: none;
   }
}

.ScanProgress-spinnerWrap :deep(.Spinner-root) {
   color: var(--color-accent);
}

.ScanProgress-progressTitle {
   margin: 0;
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-text);
   flex: 1;
}

.ScanProgress-barWrap {
   position: relative;
   width: 100%;
   height: 8px;
   border-radius: 4px;
   overflow: hidden;
   background: var(--color-surface);
   box-shadow: inset 0 0.5px 1px rgba(0, 0, 0, 0.24);
}

.ScanProgress-barMain {
   width: 100%;
   height: 100%;
   transform-origin: left;
   background: linear-gradient(90deg, var(--btn-gradient-0) 0%, var(--btn-gradient-100) 100%);
   border-radius: 4px;
   box-shadow:
      inset 0 0.5px 0 rgba(255, 255, 255, 0.3),
      0 0 8px var(--color-accent-glow);
   transition: transform 0.3s var(--ease-apple-out);
}

.ScanProgress-stats {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xs);
   padding: var(--spacing-md);
   border: 1px solid var(--color-hairline);
   border-radius: var(--radius-lg);
   background: var(--color-bg-elevated);
   box-shadow: var(--shadow-sm);

   p {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: var(--spacing-sm);
      margin: 0;
      font-size: var(--font-size-base);
   }

   span {
      color: var(--color-text-muted);
   }

   strong {
      max-width: 65%;
      color: var(--color-text);
      text-align: right;
      overflow-wrap: anywhere;
   }
}

.ScanProgress-currentPath {
   max-width: min(65%, 220px);
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   display: block;
}

.ScanProgress-estimate {
   margin: 0;
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.ScanProgress-abortBtn {
   align-self: flex-start;
   padding: 0;
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-abort);
   background: none;
   border: none;
   cursor: pointer;
   margin-top: var(--spacing-md);
   transition:
      opacity 0.15s var(--ease-apple-out),
      transform 0.12s var(--ease-apple-out);

   &:hover {
      opacity: 0.75;
   }

   &:active {
      transform: scale(0.97);
   }
}
</style>
