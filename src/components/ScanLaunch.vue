<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

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
         <h2 class="ScanLaunch-noticeTitle">
            <PhShieldWarning :size="16" weight="regular" aria-hidden="true" />
            {{ t('ScanLaunch', 'launchTitle') }}
         </h2>
         <div class="ScanLaunch-noticeBody">
            <p>{{ t('ScanLaunch', 'launchBodyLine1') }}</p>
            <p>{{ t('ScanLaunch', 'launchBodyLine2') }}</p>
         </div>
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
   border: 1px solid var(--color-hairline);
   border-radius: var(--radius-lg);
   padding: var(--spacing-md);
   background: var(--color-bg-elevated);
   box-shadow: var(--shadow-sm);
}

.ScanLaunch-noticeTitle {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   margin: 0 0 var(--spacing-md) 0;
   font-size: var(--font-size-base);
   font-weight: 600;
   color: var(--color-text-muted);
}

.ScanLaunch-noticeTitle :deep(svg) {
   color: var(--color-accent-alt);
   filter: drop-shadow(0 0 4px var(--color-accent-alt-glow));
}

.ScanLaunch-noticeBody {
   margin: 0;
   font-size: var(--font-size-base);
   color: var(--color-text-muted);

   p {
      margin: 0;
   }

   p:not(:last-child) {
      margin-bottom: 1em;
   }
}

.ScanLaunch-scanBtn {
   display: inline-flex;
   gap: 10px;
}
</style>
