<!--
ScanResultsDeleteComplete

Purpose: Post-delete screen. Shows resume (items count, size freed) and Scan again button.

Props: deletedSummary ({ count: number, size: number } | null)

Example:
 <ScanResultsDeleteComplete :deletedSummary="summary" @scan-again="onScanAgain" />
-->

<script setup lang="ts">
import { PhCheckCircle, PhMagnifyingGlass } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'
import { formatBytes } from '@/lib/format'

const { t } = useTranslations()

defineProps<{
   deletedSummary: { count: number; size: number } | null
}>()

defineEmits<{
   (e: 'scan-again'): void
}>()
</script>

<template>
   <div class="ScanResultsDeleteComplete-root">
      <div class="ScanResultsDeleteComplete-content">
         <PhCheckCircle :size="48" weight="fill" class="ScanResultsDeleteComplete-icon" />
         <h2 class="ScanResultsDeleteComplete-title">
            {{ t('ScanResultsDeleteComplete', 'title') }}
         </h2>
         <p v-if="deletedSummary" class="ScanResultsDeleteComplete-resume">
            {{
               t('ScanResultsDeleteComplete', 'resume', {
                  count: deletedSummary.count,
                  size: formatBytes(deletedSummary.size),
               })
            }}
         </p>
         <button
            type="button"
            class="ScanResultsDeleteComplete-scanBtn"
            @click="$emit('scan-again')"
         >
            <PhMagnifyingGlass :size="18" weight="regular" />
            {{ t('ScanResultsDeleteComplete', 'scanAgain') }}
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanResultsDeleteComplete-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   align-items: center;
   justify-content: center;
   min-height: 0;
   padding: var(--spacing-lg);
   background: var(--color-bg);
}

.ScanResultsDeleteComplete-content {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-md);
   max-width: var(--content-max-width);
   width: 100%;
}

.ScanResultsDeleteComplete-icon {
   color: var(--color-accent);
   flex-shrink: 0;
}

.ScanResultsDeleteComplete-title {
   margin: 0;
   font-size: 1.25rem;
   font-weight: 600;
   color: var(--color-text);
   text-align: center;
}

.ScanResultsDeleteComplete-resume {
   margin: 0;
   font-size: 0.9375rem;
   color: var(--color-text-muted);
   text-align: center;
}

.ScanResultsDeleteComplete-scanBtn {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
   margin-top: var(--spacing-sm);
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.9375rem;
   font-weight: 600;
   color: var(--color-on-accent);
   background: var(--color-accent);
   border: none;
   border-radius: 8px;
   cursor: pointer;
   box-shadow: var(--glow-md);
   transition:
      background 0.2s,
      box-shadow 0.3s;

   &:hover {
      background: var(--color-accent-hover);
      box-shadow: var(--glow-lg);
   }
}
</style>
