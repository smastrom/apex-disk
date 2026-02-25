<!--
ScanResultsLoadingView

Purpose: Active scan view showing disk/user context, live scan progress, and detailed scan metadata while scanning.

Props: progress ({ current: number; total: number; folder: string; size: number; scanned_size_total: number })

Example:
 <ScanResultsLoadingView :progress="progress" @abort="onAbort" />
-->

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

import Spinner from './Spinner.vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/useTranslations'

import type { ScanProgress } from '@/types/structures'

const { t } = useTranslations()

const props = defineProps<{
   progress: ScanProgress
}>()

const emit = defineEmits<{
   (e: 'abort'): void
}>()

const elapsedSeconds = ref(0)
let elapsedInterval: ReturnType<typeof setInterval> | null = null

const percent = computed(() => {
   if (props.progress.total <= 0) return 0
   return Math.max(0, Math.min(100, (props.progress.current / props.progress.total) * 100))
})

onMounted(() => {
   elapsedSeconds.value = 0
   elapsedInterval = setInterval(() => {
      elapsedSeconds.value += 1
   }, 1000)
})

onUnmounted(() => {
   if (elapsedInterval) clearInterval(elapsedInterval)
})
</script>

<template>
   <section class="ScanResultsLoadingView-root">
      <div class="ScanResultsLoadingView-progressBlock">
         <div class="ScanResultsLoadingView-progressHeader">
            <span class="ScanResultsLoadingView-spinnerWrap">
               <Spinner :size="18" />
            </span>
            <p class="ScanResultsLoadingView-progressTitle">
               {{
                  t('ScanResultsLoadingView', 'scanning', {
                     current: progress.current,
                     total: progress.total,
                  })
               }}
            </p>
            <p class="ScanResultsLoadingView-elapsed">
               {{ t('ScanResultsLoadingView', 'elapsed', { seconds: elapsedSeconds }) }}
            </p>
         </div>
         <div class="ScanResultsLoadingView-barWrap">
            <div class="ScanResultsLoadingView-barMain" :style="{ width: percent + '%' }" />
         </div>
      </div>

      <div class="ScanResultsLoadingView-stats">
         <p>
            <span>{{ t('ScanResultsLoadingView', 'stage') }}</span>
            <strong>{{ percent.toFixed(1) }}%</strong>
         </p>
         <p>
            <span>{{ t('ScanResultsLoadingView', 'scannedSize') }}</span>
            <strong>{{ formatBytes(progress.scanned_size_total) }}</strong>
         </p>
         <p>
            <span>{{ t('ScanResultsLoadingView', 'currentPath') }}</span>
            <strong class="ScanResultsLoadingView-currentPath" :title="progress.folder || ''">
               {{ progress.folder || t('ScanResultsLoadingView', 'preparing') }}
            </strong>
         </p>
      </div>

      <p class="ScanResultsLoadingView-estimate">{{ t('ScanResultsLoadingView', 'estimate') }}</p>

      <button type="button" class="ScanResultsLoadingView-abortBtn" @click="emit('abort')">
         {{ t('ScanResultsLoadingView', 'abort') }}
      </button>
   </section>
</template>

<style scoped>
.ScanResultsLoadingView-root {
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

.ScanResultsLoadingView-progressBlock {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
}

.ScanResultsLoadingView-progressHeader {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 10px;
}

.ScanResultsLoadingView-spinnerWrap {
   display: flex;
}

.ScanResultsLoadingView-spinnerWrap :deep(.Spinner-root) {
   color: var(--color-accent);
}

.ScanResultsLoadingView-progressTitle {
   margin: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: var(--color-text);
   flex: 1;
}

.ScanResultsLoadingView-elapsed {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
   flex-shrink: 0;
}

.ScanResultsLoadingView-barWrap {
   position: relative;
   width: 100%;
   height: 6px;
   border-radius: 3px;
   overflow: hidden;
   background: var(--color-surface);
   border: 1px solid var(--color-border);
}

.ScanResultsLoadingView-barMain {
   height: 100%;
   background: var(--color-accent);
   border-radius: 3px;
   transition: width 0.25s ease;
}

.ScanResultsLoadingView-stats {
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

.ScanResultsLoadingView-currentPath {
   max-width: min(65%, 220px);
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   display: block;
}

.ScanResultsLoadingView-estimate {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
}

.ScanResultsLoadingView-abortBtn {
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
