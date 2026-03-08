<!--
ScanResultsTrashConfirmation

Purpose: Post-trash screen. Shows resume (items count, size freed) and Scan again button.

Props: deletedSummary ({ count: number, size: number } | null)

Example:
 <ScanResultsTrashConfirmation :deletedSummary="summary" @restart="onRestart" />
-->

<script setup lang="ts">
import AnimatedAlertCircle from './ui/AnimatedAlertCircle.vue'
import AnimatedCheckCircle from './ui/AnimatedCheckCircle.vue'

import { PhMagnifyingGlass, PhX } from '@phosphor-icons/vue'
import { computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   deletedSummary: { count: number; size: number } | null
}>()

defineEmits<{
   (e: 'restart'): void
}>()

const { t } = useTranslations()

const hasErrors = computed(() => !props.deletedSummary || props.deletedSummary.count === 0)

function closeApp() {
   getCurrentWindow().close()
}
</script>

<template>
   <div class="ScanResultsTrashConfirmation-root">
      <div class="ScanResultsTrashConfirmation-content">
         <AnimatedAlertCircle
            v-if="hasErrors"
            :size="48"
            class="ScanResultsTrashConfirmation-icon ScanResultsTrashConfirmation-iconError"
         />
         <AnimatedCheckCircle v-else :size="48" class="ScanResultsTrashConfirmation-icon" />
         <h2 class="ScanResultsTrashConfirmation-title">
            {{ t('ScanResultsTrashConfirmation', hasErrors ? 'titleErrors' : 'title') }}
         </h2>
         <p class="ScanResultsTrashConfirmation-resume">
            {{
               hasErrors
                  ? t('ScanResultsTrashConfirmation', 'resumeErrors')
                  : t('ScanResultsTrashConfirmation', 'resume', {
                       count: deletedSummary!.count,
                       size: formatBytes(deletedSummary!.size),
                    })
            }}
         </p>
         <button
            type="button"
            class="ScanResultsTrashConfirmation-scanBtn GradientButton"
            data-testid="restart"
            @click="$emit('restart')"
         >
            <PhMagnifyingGlass :size="18" weight="regular" aria-hidden="true" />
            {{ t('ScanResultsTrashConfirmation', 'restart') }}
         </button>
         <button type="button" class="ScanResultsTrashConfirmation-closeBtn" @click="closeApp">
            <PhX :size="16" weight="bold" aria-hidden="true" />
            {{ t('ScanResultsTrashConfirmation', 'closeApp') }}
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsTrashConfirmation-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   align-items: center;
   justify-content: center;
   min-height: 0;
   padding: var(--spacing-lg);
   background: var(--color-bg);
}

.ScanResultsTrashConfirmation-content {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-md);
   max-width: var(--content-max-width);
   width: 100%;
}

.ScanResultsTrashConfirmation-icon {
   color: var(--color-accent);
   flex-shrink: 0;
}

.ScanResultsTrashConfirmation-iconError {
   color: var(--color-abort);
}

.ScanResultsTrashConfirmation-title {
   margin: 0;
   font-size: var(--font-size-2xl);
   font-weight: 600;
   color: var(--color-text);
   text-align: center;
}

.ScanResultsTrashConfirmation-resume {
   margin: 0;
   font-size: var(--font-size-lg);
   color: var(--color-text-muted);
   text-align: center;
}

.ScanResultsTrashConfirmation-scanBtn {
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

.ScanResultsTrashConfirmation-closeBtn {
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
