<!--
FooterNav

Purpose: Bottom navigation bar with Scan, Settings, Informations, Donate buttons. Mobile-app style footer.

Props: activeView (string?), emit: select-view

Example:
 <FooterNav :activeView="activeView" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhInfo, PhHeart } from '@phosphor-icons/vue'
import { openUrl } from '@tauri-apps/plugin-opener'

import { DONATE_URL } from '@/lib/constants'
import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()

defineProps<{
   activeView?: string
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
   <footer class="FooterNav-root">
      <button
         class="FooterNav-btn"
         :class="{ 'FooterNav-btn--active': activeView === 'scan' }"
         @click="emit('select-view', 'scan')"
      >
         <PhMagnifyingGlass :size="24" weight="regular" />
         <span>{{ t('FooterNav', 'scan') }}</span>
      </button>
      <button
         class="FooterNav-btn"
         :class="{ 'FooterNav-btn--active': activeView === 'settings' }"
         @click="emit('select-view', 'settings')"
      >
         <PhGear :size="24" weight="regular" />
         <span>{{ t('FooterNav', 'settings') }}</span>
      </button>
      <button
         class="FooterNav-btn"
         :class="{ 'FooterNav-btn--active': activeView === 'informations' }"
         @click="emit('select-view', 'informations')"
      >
         <PhInfo :size="24" weight="regular" />
         <span>{{ t('FooterNav', 'informations') }}</span>
      </button>
      <button class="FooterNav-btn" @click="onDonateClick">
         <PhHeart :size="24" weight="regular" />
         <span>{{ t('FooterNav', 'donate') }}</span>
      </button>
   </footer>
</template>

<style scoped>
.FooterNav-root {
   height: var(--footer-height);
   display: flex;
   align-items: center;
   padding: 0;
   background: var(--color-bg-elevated);
   flex-shrink: 0;
   border-top: 1px solid var(--color-bg);
   box-shadow: 0 -1px 12px var(--color-bg);
}

.FooterNav-btn {
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

   span {
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

.FooterNav-btn--active {
   color: var(--color-accent);

   :deep(svg) {
      filter: drop-shadow(0 0 6px var(--color-accent-glow));
   }

   span {
      text-shadow: 0 0 8px var(--color-accent-glow);
   }
}
</style>
