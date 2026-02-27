<!--
AppFooter

Purpose: Bottom navigation bar with Scan, Settings, Information, Donate buttons. Mobile-app style footer.

Props: activeView (string?), hasPermissionIssue (boolean?), emit: select-view

Example:
 <AppFooter :activeView="activeView" :hasPermissionIssue="!fdaGranted" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhInfo, PhHeart } from '@phosphor-icons/vue'
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/useTranslations'

import { DONATE_URL } from '@/lib/constants'

const { t } = useTranslations()

defineProps<{
   activeView?: string
   hasPermissionIssue?: boolean
}>()

const emit = defineEmits<{
   (e: 'select-view', view: string): void
}>()

async function onDonateClick() {
   try {
      await openUrl(DONATE_URL)
   } catch (err) {
      console.error('Failed to open donate URL:', err)
   }
}
</script>

<template>
   <footer class="AppFooter-root">
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'scan' }"
         @click="emit('select-view', 'scan')"
      >
         <PhMagnifyingGlass :size="24" weight="regular" />
         <span>{{ t('AppFooter', 'scan') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'settings' }"
         @click="emit('select-view', 'settings')"
      >
         <span class="AppFooter-iconWrap">
            <PhGear :size="24" weight="regular" />
            <span v-if="hasPermissionIssue" class="AppFooter-badge" aria-hidden="true" />
         </span>
         <span>{{ t('AppFooter', 'settings') }}</span>
      </button>
      <button
         class="AppFooter-btn"
         :class="{ 'AppFooter-btn--active': activeView === 'information' }"
         @click="emit('select-view', 'information')"
      >
         <PhInfo :size="24" weight="regular" />
         <span>{{ t('AppFooter', 'information') }}</span>
      </button>
      <button class="AppFooter-btn" @click="onDonateClick">
         <PhHeart :size="24" weight="regular" />
         <span>{{ t('AppFooter', 'donate') }}</span>
      </button>
   </footer>
</template>

<style scoped>
.AppFooter-root {
   height: var(--footer-height);
   display: flex;
   align-items: center;
   padding: 0;
   background: var(--color-bg-elevated);
   flex-shrink: 0;
   border-top: 1px solid var(--color-bg);
   box-shadow: 0 -1px 12px var(--color-bg);
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
   border-radius: 8px;
   transition:
      color 0.2s,
      text-shadow 0.3s;

   &:hover {
      color: var(--color-text);
   }

   > span:not(.AppFooter-iconWrap) {
      font-size: 0.75rem;
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
   color: var(--color-accent);

   :deep(svg) {
      filter: drop-shadow(0 0 6px var(--color-accent-glow));
   }

   span {
      text-shadow: 0 0 8px var(--color-accent-glow);
   }
}

/* Icon wrapper to position the badge relative to the icon only */
.AppFooter-iconWrap {
   position: relative;
   display: inline-flex;
   /* Override the inherited span styles that stretch to full button width */
   width: auto;
   overflow: visible;
   text-overflow: unset;
   white-space: unset;
   text-shadow: none;
}

.AppFooter-badge {
   position: absolute;
   top: -2px;
   right: -4px;
   width: 7px;
   height: 7px;
   border-radius: 50%;
   background: #ef4444;
   border: 1.5px solid var(--color-bg-elevated);
   font-size: 0;
   overflow: visible;
}
</style>
