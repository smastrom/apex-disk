<!--
ScanScanning

Purpose: Active scan view showing disk/user context, live scan progress, and detailed scan metadata while scanning.

Props: progress (ScanProgress), elapsedSeconds (number)

Example:
 <ScanScanning :progress="progress" :elapsedSeconds="elapsedSeconds" @abort="onAbort" />
-->

<script setup lang="ts">
import Spinner from './ui/Spinner.vue'

import { computed } from 'vue'

import { formatBytes, formatProgressNumber } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

import type { ScanProgress } from '@/types/structs'

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
   <section class="ScanScanning-root" data-testid="scanning-results">
      <div class="ScanScanning-progressBlock">
         <div class="ScanScanning-progressHeader">
            <span class="ScanScanning-spinnerWrap">
               <Spinner :size="18" />
            </span>
            <p class="ScanScanning-progressTitle" data-testid="scan-progress">
               {{
                  t('ScanScanning', 'scanning', {
                     current: formatProgressNumber(progress.current),
                     total: formatProgressNumber(progress.total),
                  })
               }}
            </p>
         </div>
         <div class="ScanScanning-barWrap">
            <div class="ScanScanning-barMain" :style="{ width: percent + '%' }" />
         </div>
      </div>

      <div class="ScanScanning-stats">
         <p>
            <span>{{ t('ScanScanning', 'scannedSize') }}</span>
            <strong>{{ formatBytes(progress.scanned_size_total) }}</strong>
         </p>
         <p>
            <span>{{ t('ScanScanning', 'currentPath') }}</span>
            <strong class="ScanScanning-currentPath" :title="progress.folder || ''">
               {{ progress.folder || t('ScanScanning', 'preparing') }}
            </strong>
         </p>

         <p>
            <span>{{ t('ScanScanning', 'elapsed') }}</span>
            <strong>{{ elapsedDisplay }}</strong>
         </p>
      </div>

      <button
         type="button"
         class="ScanScanning-abortBtn"
         data-testid="scan-abort"
         @click="emit('abort')"
      >
         {{ t('ScanScanning', 'abort') }}
      </button>
   </section>
</template>

<style scoped>
.ScanScanning-root {
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

.ScanScanning-progressBlock {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
}

.ScanScanning-progressHeader {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 10px;
}

.ScanScanning-spinnerWrap {
   display: flex;

   @media (prefers-reduced-motion: reduce) {
      display: none;
   }
}

.ScanScanning-spinnerWrap :deep(.Spinner-root) {
   color: var(--color-accent);
}

.ScanScanning-progressTitle {
   margin: 0;
   font-size: var(--font-size-md);
   font-weight: 500;
   color: var(--color-text);
   flex: 1;
}

.ScanScanning-barWrap {
   position: relative;
   width: 100%;
   height: 6px;
   border-radius: 3px;
   overflow: hidden;
   background: var(--color-surface);
   border: 1px solid var(--color-border);
}

.ScanScanning-barMain {
   height: 100%;
   background: var(--color-accent);
   border-radius: 3px;
   transition: width 0.25s var(--ease-standard);
}

.ScanScanning-stats {
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

.ScanScanning-currentPath {
   max-width: min(65%, 220px);
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   display: block;
}

.ScanScanning-estimate {
   margin: 0;
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.ScanScanning-abortBtn {
   align-self: flex-start;
   padding: 0;
   font-size: var(--font-size-md);
   font-weight: 500;
   color: var(--color-abort);
   background: none;
   border: none;
   cursor: pointer;
   margin-top: var(--spacing-md);

   &:hover {
      opacity: 0.75;
   }
}
</style>
