<!--
FooterMenu

Purpose: Bottom navigation bar with Scan, Settings, Informations, Donate buttons. Mobile-app style footer.

Props: activeView (string?), emit: select-view

Example:
 <FooterMenu :active-view="activeView" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhInfo, PhHeart } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()

defineProps<{
   activeView?: string
}>()

const emit = defineEmits<{
   (e: 'select-view', view: string): void
}>()
</script>

<template>
   <footer class="FooterMenu-root">
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'scan' }"
         @click="emit('select-view', 'scan')"
      >
         <PhMagnifyingGlass :size="24" weight="regular" />
         <span>{{ t('FooterMenu', 'scan') }}</span>
      </button>
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'settings' }"
         @click="emit('select-view', 'settings')"
      >
         <PhGear :size="24" weight="regular" />
         <span>{{ t('FooterMenu', 'settings') }}</span>
      </button>
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'informations' }"
         @click="emit('select-view', 'informations')"
      >
         <PhInfo :size="24" weight="regular" />
         <span>{{ t('FooterMenu', 'informations') }}</span>
      </button>
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'donate' }"
         @click="emit('select-view', 'donate')"
      >
         <PhHeart :size="24" weight="regular" />
         <span>{{ t('FooterMenu', 'donate') }}</span>
      </button>
   </footer>
</template>

<style scoped>
.FooterMenu-root {
   height: var(--footer-height);
   display: flex;
   align-items: center;
   justify-content: space-around;
   padding: 0 var(--spacing-md);
   background: var(--color-bg-elevated);
   flex-shrink: 0;
}

.FooterMenu-btn {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-sm) var(--spacing-md);
   color: var(--color-text-muted);
   transition: color 0.15s;
}

.FooterMenu-btn:hover {
   color: var(--color-text);
}

.FooterMenu-btn--active {
   color: var(--color-accent);
}

.FooterMenu-btn span {
   font-size: 0.75rem;
   font-weight: 500;
}
</style>
