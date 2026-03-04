<!--
ScanScanningResults

Purpose: Active scan view showing disk/user context, live scan progress, and detailed scan metadata while scanning.

Props: progress ({ current: number; total: number; folder: string; size: number; scanned_size_total: number })

Example:
 <ScanScanningResults :progress="progress" @abort="onAbort" />
-->

<script setup lang="ts">
import Spinner from './Spinner.vue'

import { computed, onActivated, onDeactivated, onUnmounted, ref } from 'vue'

import { formatBytes, formatProgressNumber } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

import type { ScanProgress } from '@/types/structs'

const props = defineProps<{
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'abort'): void
}>()

const { t } = useTranslations()

const elapsedSeconds = ref(0)
let elapsedInterval: ReturnType<typeof setInterval> | null = null

const percent = computed(() => {
   if (props.progress.total <= 0) return 0

   const raw = Math.max(0, Math.min(100, (props.progress.current / props.progress.total) * 100))

   return raw >= 100 ? 99 : Math.round(raw)
})

function startTimer() {
   elapsedSeconds.value = 0
   if (elapsedInterval) clearInterval(elapsedInterval)

   elapsedInterval = setInterval(() => {
      elapsedSeconds.value += 1
   }, 1000)
}

function stopTimer() {
   if (elapsedInterval) {
      clearInterval(elapsedInterval)
      elapsedInterval = null
   }
}

onActivated(startTimer)
onDeactivated(stopTimer)
onUnmounted(stopTimer)
</script>

<template>
   <section class="ScanScanningResults-root" data-testid="scanning-results">
      <div class="ScanScanningResults-progressBlock">
         <div class="ScanScanningResults-progressHeader">
            <span class="ScanScanningResults-spinnerWrap">
               <Spinner :size="18" />
            </span>
            <p class="ScanScanningResults-progressTitle" data-testid="scan-progress">
               {{
                  t('ScanScanningResults', 'scanning', {
                     current: formatProgressNumber(progress.current),
                     total: formatProgressNumber(progress.total),
                  })
               }}
            </p>
            <p v-if="elapsedSeconds >= 1" class="ScanScanningResults-elapsed">
               {{ t('ScanScanningResults', 'elapsed', { seconds: elapsedSeconds }) }}
            </p>
         </div>
         <div class="ScanScanningResults-barWrap">
            <div class="ScanScanningResults-barMain" :style="{ width: percent + '%' }" />
         </div>
      </div>

      <div class="ScanScanningResults-stats">
         <p>
            <span>{{ t('ScanScanningResults', 'stage') }}</span>
            <strong>{{ formatProgressNumber(percent) }}%</strong>
         </p>
         <p>
            <span>{{ t('ScanScanningResults', 'currentPath') }}</span>
            <strong class="ScanScanningResults-currentPath" :title="progress.folder || ''">
               {{ progress.folder || t('ScanScanningResults', 'preparing') }}
            </strong>
         </p>
         <p>
            <span>{{ t('ScanScanningResults', 'scannedSize') }}</span>
            <strong>{{ formatBytes(progress.scanned_size_total) }}</strong>
         </p>
      </div>

      <p class="ScanScanningResults-estimate">{{ t('ScanScanningResults', 'estimate') }}</p>

      <button
         type="button"
         class="ScanScanningResults-abortBtn"
         data-testid="scan-abort"
         @click="emit('abort')"
      >
         {{ t('ScanScanningResults', 'abort') }}
      </button>
   </section>
</template>

<style scoped>
.ScanScanningResults-root {
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

.ScanScanningResults-progressBlock {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
}

.ScanScanningResults-progressHeader {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 10px;
}

.ScanScanningResults-spinnerWrap {
   display: flex;

   @media (prefers-reduced-motion: reduce) {
      display: none;
   }
}

.ScanScanningResults-spinnerWrap :deep(.Spinner-root) {
   color: var(--color-accent);
}

.ScanScanningResults-progressTitle {
   margin: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: var(--color-text);
   flex: 1;
}

.ScanScanningResults-elapsed {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
   flex-shrink: 0;
}

.ScanScanningResults-barWrap {
   position: relative;
   width: 100%;
   height: 6px;
   border-radius: 3px;
   overflow: hidden;
   background: var(--color-surface);
   border: 1px solid var(--color-border);
}

.ScanScanningResults-barMain {
   height: 100%;
   background: var(--color-accent);
   border-radius: 3px;
   transition: width 0.25s ease;
}

.ScanScanningResults-stats {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xs);
   padding: var(--spacing-md);
   border: 1px solid var(--color-border);
   border-radius: 8px;
   background: var(--color-bg-elevated);

   p {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      gap: var(--spacing-sm);
      margin: 0;
      font-size: 0.8125rem;
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

.ScanScanningResults-currentPath {
   max-width: min(65%, 220px);
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   display: block;
}

.ScanScanningResults-estimate {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
}

.ScanScanningResults-abortBtn {
   align-self: flex-start;
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: #ff3b30;
   background: none;
   border: none;
   cursor: pointer;
   margin-top: var(--spacing-md);

   &:hover {
      opacity: 0.75;
   }
}
</style>
