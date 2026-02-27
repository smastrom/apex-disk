<!--
ScanViewInitial

Purpose: Pre-scan view with a safety notice and CTA to start scanning.

Props: none

Example:
 <ScanViewInitial @start-scan="loadFolders" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhShieldWarning } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()

const emit = defineEmits<{
   (e: 'start-scan'): void
}>()
</script>

<template>
   <section class="ScanViewInitial-root">
      <div class="ScanViewInitial-notice">
         <p class="ScanViewInitial-noticeTitle">
            <PhShieldWarning :size="16" weight="regular" />
            {{ t('ScanViewInitial', 'warningTitle') }}
         </p>
         <p class="ScanViewInitial-noticeBody">{{ t('ScanViewInitial', 'warningBody') }}</p>
      </div>

      <button
         type="button"
         class="ScanViewInitial-scanBtn GradientButton"
         @click="emit('start-scan')"
      >
         <PhMagnifyingGlass :size="18" weight="regular" />
         {{ t('ScanViewInitial', 'startScan') }}
      </button>
   </section>
</template>

<style scoped>
.ScanViewInitial-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   justify-content: center;
   gap: var(--spacing-lg);
   max-width: 520px;
   width: 100%;
   margin: 0 auto;
   padding: var(--spacing-lg) var(--spacing-md);
}

.ScanViewInitial-notice {
   border: 1px solid var(--color-border);
   border-radius: 8px;
   padding: var(--spacing-md);
   background: var(--color-bg-elevated);
}

.ScanViewInitial-noticeTitle {
   display: flex;
   align-items: center;
   gap: 8px;
   margin: 0 0 var(--spacing-md) 0;
   font-size: 0.8125rem;
   font-weight: 600;
   color: var(--color-text-muted);
}

.ScanViewInitial-noticeBody {
   margin: 0;
   font-size: 0.8125rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

.ScanViewInitial-scanBtn {
   display: inline-flex;
   align-items: center;
   justify-content: center;
   gap: 10px;
   width: 100%;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--btn-font-size);
}
</style>
