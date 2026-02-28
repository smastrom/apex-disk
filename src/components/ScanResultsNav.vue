<!--
ScanResultsNav

Purpose: Shared nav bar for scan results views. Back (and optional forward), center path/title, optional reset/abort actions.

Props: showForward (boolean), backDisabled (boolean), forwardDisabled (boolean?), pathLabel (string), pathTitle (string?), showActions (boolean), resetDisabled (boolean?)

Example:
 <ScanResultsNav
   :showForward="true"
   :backDisabled="backStack.length === 0"
   :forwardDisabled="forwardStack.length === 0"
   :pathLabel="displayPath"
   :pathTitle="current.path"
   :showActions="true"
   :resetDisabled="selectedMap.size === 0"
   @back="goBack"
   @forward="goForward"
   @reset="selectedMap.clear()"
   @abort="onAbort"
 />
-->

<script setup lang="ts">
import { PhCaretLeft, PhCaretRight, PhFolder } from '@phosphor-icons/vue'

import { useTranslations } from '@/lib/useTranslations'

const { t } = useTranslations()

defineProps<{
   showForward?: boolean
   backDisabled?: boolean
   forwardDisabled?: boolean
   pathLabel: string
   pathTitle?: string
   showActions?: boolean
   resetDisabled?: boolean
}>()

const emit = defineEmits<{
   (e: 'back'): void
   (e: 'forward'): void
   (e: 'reset'): void
   (e: 'cancel'): void
}>()
</script>

<template>
   <nav class="ScanResultsNav-root">
      <div class="ScanResultsNav-controls">
         <button
            type="button"
            class="ScanResultsNav-btn"
            :disabled="backDisabled"
            aria-label="Back"
            @click="emit('back')"
         >
            <PhCaretLeft :size="18" weight="regular" aria-hidden="true" />
         </button>
         <button
            v-if="showForward"
            type="button"
            class="ScanResultsNav-btn"
            :disabled="forwardDisabled"
            aria-label="Forward"
            @click="emit('forward')"
         >
            <PhCaretRight :size="18" weight="regular" aria-hidden="true" />
         </button>
      </div>
      <div class="ScanResultsNav-path" :title="pathTitle">
         <PhFolder :size="16" weight="regular" class="ScanResultsNav-pathIcon" aria-hidden="true" />
         <span class="ScanResultsNav-pathText">{{ pathLabel }}</span>
      </div>
      <div v-if="showActions" class="ScanResultsNav-actions">
         <button
            type="button"
            class="ScanResultsNav-resetBtn"
            :disabled="resetDisabled"
            @click="emit('reset')"
         >
            {{ t('ScanResultsList', 'resetSelection') }}
         </button>
         <button type="button" class="ScanResultsNav-cancelBtn" @click="emit('cancel')">
            {{ t('ScanResultsList', 'cancel') }}
         </button>
      </div>
   </nav>
</template>

<style scoped>
.ScanResultsNav-root {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: space-between;
   gap: var(--spacing-md);
   padding: var(--spacing-md);
   border-bottom: 1px solid var(--color-bg);
}

.ScanResultsNav-controls {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
}

.ScanResultsNav-btn {
   display: flex;
   align-items: center;
   justify-content: center;
   width: 32px;
   height: 28px;
   color: var(--color-text);
   background: var(--color-surface);
   border: none;
   border-radius: 6px;
   cursor: pointer;
   opacity: 0.9;
   transition:
      background 0.2s,
      opacity 0.2s;

   &:hover:not(:disabled) {
      background: var(--color-surface-hover);
      opacity: 1;
   }

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

.ScanResultsNav-path {
   flex: 1;
   min-width: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) 0;
   font-size: 0.8125rem;
   color: var(--color-text-muted);
   text-align: left;
}

.ScanResultsNav-pathIcon {
   flex-shrink: 0;
   color: var(--color-accent);
}

.ScanResultsNav-pathText {
   overflow: hidden;
   text-overflow: ellipsis;
   white-space: nowrap;
}

.ScanResultsNav-actions {
   display: flex;
   align-items: center;
   gap: var(--spacing-md);
}

.ScanResultsNav-resetBtn {
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: var(--color-text-muted);
   background: none;
   border: none;
   cursor: pointer;

   &:hover:not(:disabled) {
      color: var(--color-text);
      opacity: 0.85;
   }

   &:disabled {
      opacity: 0.5;
      cursor: not-allowed;
   }
}

.ScanResultsNav-cancelBtn {
   padding: 0;
   font-size: 0.875rem;
   font-weight: 500;
   color: #ff3b30;
   background: none;
   border: none;
   cursor: pointer;

   &:hover {
      opacity: 0.75;
   }
}
</style>
