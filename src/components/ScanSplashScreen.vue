<!--
ScanSplashScreen

Purpose: Pre-scan view with a safety notice and CTA to start scanning.

Props: none

Example:
 <ScanSplashScreen @start-scan="loadFolders" />
-->

<script setup lang="ts">
import { PhPlay, PhShieldWarning } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()

const emit = defineEmits<{
   (e: 'start-scan'): void
}>()
</script>

<template>
   <section class="ScanSplashScreen-root">
      <div class="ScanSplashScreen-notice">
         <p class="ScanSplashScreen-noticeTitle">
            <PhShieldWarning :size="16" weight="regular" />
            {{ t('ScanSplashScreen', 'warningTitle') }}
         </p>
         <p class="ScanSplashScreen-noticeBody">{{ t('ScanSplashScreen', 'warningBody') }}</p>
      </div>

      <button type="button" class="ScanSplashScreen-scanBtn" @click="emit('start-scan')">
         <PhPlay :size="18" weight="regular" />
         {{ t('ScanSplashScreen', 'startScan') }}
      </button>
   </section>
</template>

<style scoped>
.ScanSplashScreen-root {
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

.ScanSplashScreen-notice {
   border: 1px solid var(--color-border);
   border-radius: 8px;
   padding: var(--spacing-md);
   background: var(--color-bg-elevated);
}

.ScanSplashScreen-noticeTitle {
   display: flex;
   align-items: center;
   gap: 8px;
   margin: 0 0 var(--spacing-md) 0;
   font-size: 0.8125rem;
   font-weight: 600;
   color: var(--color-text-muted);
}

.ScanSplashScreen-noticeBody {
   margin: 0;
   font-size: 0.8125rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

.ScanSplashScreen-scanBtn {
   display: inline-flex;
   align-items: center;
   justify-content: center;
   gap: 10px;
   width: 100%;
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
      background 0.2s ease,
      box-shadow 0.3s ease,
      transform 0.2s ease;

   &:hover {
      background: var(--color-accent-hover);
      box-shadow: var(--glow-lg);
      transform: translateY(-1px);
   }

   &:active {
      box-shadow: var(--glow-sm);
      transform: translateY(0);
   }
}
</style>
