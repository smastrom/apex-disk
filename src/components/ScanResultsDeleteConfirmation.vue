<!--
ScanResultsDeleteConfirmation

Purpose: Post-delete screen. Shows resume (items count, size freed) and Scan again button.

Props: deletedSummary ({ count: number, size: number } | null)

Example:
 <ScanResultsDeleteConfirmation :deletedSummary="summary" @restart="onRestart" />
-->

<script setup lang="ts">
import AnimatedCheckCircle from './ui/AnimatedCheckCircle.vue'

import { PhMagnifyingGlass, PhX } from '@phosphor-icons/vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

defineProps<{
   deletedSummary: { count: number; size: number } | null
}>()

defineEmits<{
   (e: 'restart'): void
}>()

const { t } = useTranslations()

function closeApp() {
   getCurrentWindow().close()
}
</script>

<template>
   <div class="ScanResultsDeleteConfirmation-root">
      <div class="ScanResultsDeleteConfirmation-content">
         <AnimatedCheckCircle :size="48" class="ScanResultsDeleteConfirmation-icon" />
         <h2 class="ScanResultsDeleteConfirmation-title">
            {{ t('ScanResultsDeleteConfirmation', 'titleMoveToTrash') }}
         </h2>
         <p v-if="deletedSummary" class="ScanResultsDeleteConfirmation-resume">
            {{
               t('ScanResultsDeleteConfirmation', 'resumeMoveToTrash', {
                  count: deletedSummary.count,
                  size: formatBytes(deletedSummary.size),
               })
            }}
         </p>
         <button
            type="button"
            class="ScanResultsDeleteConfirmation-scanBtn GradientButton"
            data-testid="restart"
            @click="$emit('restart')"
         >
            <PhMagnifyingGlass :size="18" weight="regular" aria-hidden="true" />
            {{ t('ScanResultsDeleteConfirmation', 'restart') }}
         </button>
         <button type="button" class="ScanResultsDeleteConfirmation-closeBtn" @click="closeApp">
            <PhX :size="16" weight="bold" aria-hidden="true" />
            {{ t('ScanResultsDeleteConfirmation', 'closeApp') }}
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsDeleteConfirmation-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   align-items: center;
   justify-content: center;
   min-height: 0;
   padding: var(--spacing-lg);
   background: var(--color-bg);
}

.ScanResultsDeleteConfirmation-content {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-md);
   max-width: var(--content-max-width);
   width: 100%;
}

.ScanResultsDeleteConfirmation-icon {
   color: var(--color-accent);
   flex-shrink: 0;
}

.ScanResultsDeleteConfirmation-title {
   margin: 0;
   font-size: var(--font-size-2xl);
   font-weight: 600;
   color: var(--color-text);
   text-align: center;
}

.ScanResultsDeleteConfirmation-resume {
   margin: 0;
   font-size: var(--font-size-lg);
   color: var(--color-text-muted);
   text-align: center;
}

.ScanResultsDeleteConfirmation-scanBtn {
   height: var(--cta-btn-height);
   width: 100%;
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
   margin-top: var(--spacing-xl);
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-lg);
}

.ScanResultsDeleteConfirmation-closeBtn {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.375rem;
   padding: var(--spacing-sm) var(--spacing-md);
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-text-muted);
   background: none;
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: color 0.2s var(--ease-standard);

   &:hover {
      color: var(--color-text);
   }
}
</style>
