<!--
ScanLaunch

Purpose: Pre-scan view with a safety notice and CTA to start scanning.

Props: none

Example:
 <ScanLaunch @start-scan="loadFolders" />
-->

<script setup lang="ts">
import { PhShieldWarning } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/use-translations'

const emit = defineEmits<{
   (e: 'start-scan'): void
}>()

const { t } = useTranslations()
</script>

<template>
   <section class="ScanLaunch-root" data-testid="scan-launch">
      <div class="ScanLaunch-notice">
         <p class="ScanLaunch-noticeTitle">
            <PhShieldWarning :size="16" weight="regular" aria-hidden="true" />
            {{ t('ScanLaunch', 'warningTitle') }}
         </p>
         <p class="ScanLaunch-noticeBody">
            {{ t('ScanLaunch', 'warningBody') }}
         </p>
      </div>

      <button
         type="button"
         class="ScanLaunch-scanBtn GradientButton"
         data-testid="start-scan"
         @click="emit('start-scan')"
      >
         {{ t('ScanLaunch', 'startScan') }}
      </button>
   </section>
</template>

<style scoped>
.ScanLaunch-root {
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

.ScanLaunch-notice {
   border: 1px solid var(--color-border);
   border-radius: 8px;
   padding: var(--spacing-md);
   background: var(--color-bg-elevated);
}

.ScanLaunch-noticeTitle {
   display: flex;
   align-items: center;
   gap: 8px;
   margin: 0 0 var(--spacing-md) 0;
   font-size: 0.8125rem;
   font-weight: 600;
   color: var(--color-text-muted);
}

.ScanLaunch-noticeBody {
   margin: 0;
   font-size: 0.8125rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

.ScanLaunch-scanBtn {
   display: inline-flex;
   align-items: center;
   justify-content: center;
   gap: 10px;
   width: 100%;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--btn-font-size);
}
</style>
