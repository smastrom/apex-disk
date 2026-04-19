<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
AppFooter

Purpose: Bottom navigation bar with Scan, Settings, Information buttons. Mobile-app style footer.

Props: activeView (string?), emit: select-view

Example:
 <AppFooter :activeView="activeView" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhInfo } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/use-translations'

defineProps<{
   activeView?: string
}>()

const emit = defineEmits<{
   (e: 'select-view', view: string): void
}>()

const { t } = useTranslations()
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
         <PhMagnifyingGlass :size="24" weight="regular" aria-hidden="true" />
         <span>{{ t('AppFooter', 'scan') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'settings' }"
         :aria-current="activeView === 'settings' ? 'page' : undefined"
         data-testid="footer-settings"
         @click="emit('select-view', 'settings')"
      >
         <PhGear :size="24" weight="regular" aria-hidden="true" />
         <span>{{ t('AppFooter', 'settings') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'information' }"
         :aria-current="activeView === 'information' ? 'page' : undefined"
         data-testid="footer-information"
         @click="emit('select-view', 'information')"
      >
         <PhInfo :size="24" weight="regular" aria-hidden="true" />
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
</style>
