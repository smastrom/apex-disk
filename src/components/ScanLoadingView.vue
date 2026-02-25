<!--
ScanLoadingView

Purpose: Active scan view showing disk/user context, live scan progress, and detailed scan metadata while scanning.

Props: progress ({ current: number; total: number; folder: string; size: number })

Example:
 <ScanLoadingView :progress="progress" @abort="onAbort" />
-->

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'

import Spinner from './Spinner.vue'

import { getDiskUsage } from '@/lib/disk'
import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/useTranslations'

import type { DiskUsage } from '@/lib/disk'

const { t } = useTranslations()

const props = defineProps<{
   progress: {
      current: number
      total: number
      folder: string
      size: number
   }
}>()

const emit = defineEmits<{
   (e: 'abort'): void
}>()

const usage = ref<DiskUsage | null>(null)
const elapsedSeconds = ref(0)
let elapsedInterval: ReturnType<typeof setInterval> | null = null

const percent = computed(() => {
   if (props.progress.total <= 0) return 0
   return Math.max(0, Math.min(100, (props.progress.current / props.progress.total) * 100))
})

onMounted(async () => {
   elapsedSeconds.value = 0
   elapsedInterval = setInterval(() => {
      elapsedSeconds.value += 1
   }, 1000)
   try {
      usage.value = await getDiskUsage()
   } catch (err) {
      console.error('Failed to get disk usage:', err)
   }
})

onUnmounted(() => {
   if (elapsedInterval) clearInterval(elapsedInterval)
})
</script>

<template>
   <section class="ScanLoadingView-root">
      <div class="ScanLoadingView-progressBlock">
         <div class="ScanLoadingView-progressHeader">
            <span class="ScanLoadingView-spinnerWrap">
               <Spinner :size="18" />
            </span>
            <p class="ScanLoadingView-progressTitle">
               {{
                  t('ScanLoadingView', 'scanning', {
                     current: progress.current,
                     total: progress.total,
                  })
               }}
            </p>
            <p class="ScanLoadingView-elapsed">
               {{ t('ScanLoadingView', 'elapsed', { seconds: elapsedSeconds }) }}
            </p>
         </div>
         <div class="ScanLoadingView-barWrap">
            <div class="ScanLoadingView-barMain" :style="{ width: percent + '%' }" />
         </div>
      </div>

      <div class="ScanLoadingView-stats">
         <p>
            <span>{{ t('ScanLoadingView', 'stage') }}</span>
            <strong>{{ percent.toFixed(1) }}%</strong>
         </p>
         <p>
            <span>{{ t('ScanLoadingView', 'scannedSize') }}</span>
            <strong>{{ formatBytes(progress.size) }}</strong>
         </p>
         <p>
            <span>{{ t('ScanLoadingView', 'currentPath') }}</span>
            <strong class="ScanLoadingView-currentPath" :title="progress.folder || ''">
               {{ progress.folder || t('ScanLoadingView', 'preparing') }}
            </strong>
         </p>
      </div>

      <p class="ScanLoadingView-estimate">{{ t('ScanLoadingView', 'estimate') }}</p>

      <button type="button" class="ScanLoadingView-abortBtn" @click="emit('abort')">
         {{ t('ScanLoadingView', 'abort') }}
      </button>
   </section>
</template>

<style scoped>
.ScanLoadingView-root {
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

.ScanLoadingView-diskInfo {
   display: flex;
   align-items: center;
   gap: 6px;
   font-size: 0.875rem;
   font-weight: 600;
   color: var(--color-text);
}

.ScanLoadingView-diskIcon {
   color: var(--color-text-muted);
}

.ScanLoadingView-userBadge {
   display: inline-flex;
   align-items: center;
   gap: 6px;
   font-size: 0.75rem;
   font-weight: 500;
   color: var(--color-text-muted);
   background: var(--color-surface);
   padding: 4px 10px;
   border-radius: 6px;
   border: 1px solid var(--color-border);
   cursor: pointer;
}

.ScanLoadingView-userBadge:hover {
   background: var(--color-surface-hover);
   color: var(--color-text);
}

.ScanLoadingView-progressBlock {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-sm);
}

.ScanLoadingView-progressHeader {
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: 10px;
}

.ScanLoadingView-spinnerWrap {
   display: flex;
}

.ScanLoadingView-spinnerWrap :deep(.Spinner-root) {
   color: var(--color-accent);
}

.ScanLoadingView-progressTitle {
   margin: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: var(--color-text);
   flex: 1;
}

.ScanLoadingView-elapsed {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
   flex-shrink: 0;
}

.ScanLoadingView-barWrap {
   position: relative;
   width: 100%;
   height: 6px;
   border-radius: 3px;
   overflow: hidden;
   background: var(--color-surface);
   border: 1px solid var(--color-border);
}

.ScanLoadingView-barMain {
   height: 100%;
   background: var(--color-accent);
   border-radius: 3px;
   transition: width 0.25s ease;
}

.ScanLoadingView-stats {
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xs);
   padding: var(--spacing-md);
   border: 1px solid var(--color-border);
   border-radius: 8px;
   background: var(--color-bg-elevated);
}

.ScanLoadingView-stats p {
   display: flex;
   justify-content: space-between;
   align-items: flex-start;
   gap: var(--spacing-sm);
   margin: 0;
   font-size: 0.8125rem;
}

.ScanLoadingView-stats span {
   color: var(--color-text-muted);
}

.ScanLoadingView-stats strong {
   max-width: 65%;
   color: var(--color-text);
   text-align: right;
   overflow-wrap: anywhere;
}

.ScanLoadingView-currentPath {
   max-width: min(65%, 220px);
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
   display: block;
}

.ScanLoadingView-estimate {
   margin: 0;
   font-size: 0.75rem;
   color: var(--color-text-muted);
}

.ScanLoadingView-abortBtn {
   align-self: flex-start;
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: #ff3b30;
   background: none;
   border: none;
   cursor: pointer;
   margin-top: var(--spacing-md);
}

.ScanLoadingView-abortBtn:hover {
   opacity: 0.75;
}
</style>
