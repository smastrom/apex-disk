<!--
FooterMenu

Purpose: Bottom navigation bar with Scan, Settings, Donate buttons. Mobile-app style footer.

Props: activeView (string?), emit: select-view

Example:
 <FooterMenu :active-view="activeView" @select-view="onSelect" />
-->

<script setup lang="ts">
import { PhMagnifyingGlass, PhGear, PhHeart } from "@phosphor-icons/vue";

defineProps<{
   activeView?: string;
}>();

const emit = defineEmits<{
   (e: "select-view", view: string): void;
}>();

function select(view: string) {
   emit("select-view", view);
}
</script>

<template>
   <footer class="FooterMenu-root">
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'scan' }"
         @click="select('scan')"
      >
         <PhMagnifyingGlass :size="24" weight="regular" />
         <span>Scan</span>
      </button>
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'settings' }"
         @click="select('settings')"
      >
         <PhGear :size="24" weight="regular" />
         <span>Settings</span>
      </button>
      <button
         class="FooterMenu-btn"
         :class="{ 'FooterMenu-btn--active': activeView === 'donate' }"
         @click="select('donate')"
      >
         <PhHeart :size="24" weight="regular" />
         <span>Donate</span>
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
