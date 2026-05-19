<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
AppFooter

Purpose: Bottom navigation bar with Scan, Settings, Information buttons. Mobile-app style footer. Shows a yellow dot on the Scan icon while a scan is running and the user is on Settings or Information, then a green dot once the scan completes until the user returns to Scan.

Props: activeView (string?), isScanning (boolean?), hasPendingScanResults (boolean?), emit: select-view

Example:
 <AppFooter :activeView="activeView" :isScanning="isScanning" :hasPendingScanResults="hasPendingScanResults" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhInfo } from '@phosphor-icons/vue'
import { computed } from 'vue'

import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   activeView?: string
   isScanning?: boolean
   hasPendingScanResults?: boolean
}>()

const emit = defineEmits<{
   (e: 'select-view', view: string): void
}>()

const { t } = useTranslations()

const showScanBusyDot = computed(() => Boolean(props.isScanning) && props.activeView !== 'scan')
const showScanReadyDot = computed(
   () => !props.isScanning && Boolean(props.hasPendingScanResults) && props.activeView !== 'scan'
)
</script>

<template>
   <nav class="AppFooter-root" aria-label="Main">
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'scan' }"
         :aria-current="activeView === 'scan' ? 'page' : undefined"
         data-testid="footer-scan"
         @click="emit('select-view', 'scan')"
      >
         <span class="AppFooter-iconWrap">
            <PhMagnifyingGlass :size="24" weight="regular" aria-hidden="true" />
            <span
               v-if="showScanBusyDot || showScanReadyDot"
               class="AppFooter-scanDot"
               :class="showScanBusyDot ? 'AppFooter-scanDot--busy' : 'AppFooter-scanDot--ready'"
               data-testid="footer-scan-dot"
               :aria-label="t('AppFooter', showScanBusyDot ? 'scanInProgress' : 'scanComplete')"
               role="status"
            />
         </span>
         <span>{{ t('AppFooter', 'scan') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'settings' }"
         :aria-current="activeView === 'settings' ? 'page' : undefined"
         data-testid="footer-settings"
         @click="emit('select-view', 'settings')"
      >
         <span class="AppFooter-iconWrap">
            <PhGear :size="24" weight="regular" aria-hidden="true" />
         </span>
         <span>{{ t('AppFooter', 'settings') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'information' }"
         :aria-current="activeView === 'information' ? 'page' : undefined"
         data-testid="footer-information"
         @click="emit('select-view', 'information')"
      >
         <span class="AppFooter-iconWrap">
            <PhInfo :size="24" weight="regular" aria-hidden="true" />
         </span>
         <span>{{ t('AppFooter', 'information') }}</span>
      </button>
   </nav>
</template>

<style scoped>
.AppFooter-root {
   position: relative;
   height: var(--footer-height);
   display: flex;
   align-items: center;
   padding: 0;
   background: var(--color-chrome);
   -webkit-backdrop-filter: saturate(180%) blur(30px);
   backdrop-filter: saturate(180%) blur(30px);
   flex-shrink: 0;
   border-top: 1px solid var(--color-chrome-border);
   z-index: 2;
}

.AppFooter-btn {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-sm);
   color: var(--color-text-muted);
   border-radius: var(--radius-sm);
   transition:
      color 0.2s var(--ease-apple-out),
      transform 0.15s var(--ease-apple-out),
      text-shadow 0.3s var(--ease-apple-out);

   &:hover {
      color: var(--color-text);
   }

   &:active {
      transform: scale(0.96);
   }

   > span:not(.AppFooter-iconWrap) {
      font-size: var(--font-size-sm);
      font-weight: 500;
      width: 100%;
      min-width: 0;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      text-align: center;
   }
}

.AppFooter-btn--active {
   color: var(--footer-nav-active-color);

   &:hover {
      color: var(--footer-nav-active-color);
   }

   :deep(svg) {
      filter: drop-shadow(0 0 6px var(--color-accent-glow));
   }

   span {
      text-shadow: 0 0 8px var(--color-accent-glow);
   }
}

.AppFooter-iconWrap {
   position: relative;
   display: inline-flex;
   line-height: 0;
}

.AppFooter-scanDot {
   position: absolute;
   top: -2px;
   right: -2px;
   width: 8px;
   height: 8px;
   border-radius: 50%;
}

.AppFooter-scanDot--busy {
   background: var(--color-scan-busy);
   box-shadow: 0 0 6px var(--color-scan-busy-glow);
   animation: AppFooter-scanDotPulse 1.6s var(--ease-apple-out) infinite;
}

.AppFooter-scanDot--ready {
   background: var(--color-scan-ready);
   box-shadow: 0 0 6px var(--color-scan-ready-glow);
}

@keyframes AppFooter-scanDotPulse {
   0%,
   100% {
      opacity: 1;
      transform: scale(1);
   }
   50% {
      opacity: 0.55;
      transform: scale(0.85);
   }
}
</style>
