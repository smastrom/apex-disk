<!--
ScanResultsNav

Purpose: Shared nav bar for scan results views. Back (and optional forward), center path/title, optional reset/abort actions.

Props: isForwardShown (boolean?), isBackDisabled (boolean?), isForwardDisabled (boolean?), pathLabel (string), pathTitle (string?), pathIcon ('folder' | 'trash'?), isActionsShown (boolean?), isResetDisabled (boolean?)

Example:
 <ScanResultsNav
   :isForwardShown="true"
   :isBackDisabled="backStack.length === 0"
   :isForwardDisabled="forwardStack.length === 0"
   :pathLabel="displayPath"
   :pathTitle="current.path"
   :isActionsShown="true"
   :isResetDisabled="selectedMap.size === 0"
   @back="goBack"
   @forward="goForward"
   @reset="selectedMap.clear()"
   @abort="onAbort"
 />
-->

<script setup lang="ts">
import { PhCaretLeft, PhCaretRight, PhFolderSimple, PhTrashSimple } from '@phosphor-icons/vue'

import { useTemplateRef } from 'vue'
import { useLabelPopover } from '@/lib/use-label-popover'
import { useTranslations } from '@/lib/use-translations'

withDefaults(
   defineProps<{
      isForwardShown?: boolean
      isBackDisabled?: boolean
      isForwardDisabled?: boolean
      pathLabel: string
      pathTitle?: string
      pathIcon?: 'folder' | 'trash'
      isActionsShown?: boolean
      isResetDisabled?: boolean
      isResetShown?: boolean
      isCancelShown?: boolean
   }>(),
   { pathIcon: 'folder', isResetShown: true, isCancelShown: true }
)

const emit = defineEmits<{
   (e: 'back'): void
   (e: 'forward'): void
   (e: 'reset'): void
   (e: 'cancel'): void
}>()

const { t } = useTranslations()

const pathTextRef = useTemplateRef<HTMLElement>('pathTextRef')
const pathPopoverRef = useTemplateRef<HTMLElement>('pathPopoverRef')

const { onPointerEnter, onPointerLeave } = useLabelPopover(pathTextRef, pathPopoverRef)
</script>

<template>
   <nav class="ScanResultsNav-root" data-testid="results-nav">
      <div class="ScanResultsNav-controls">
         <button
            type="button"
            class="ScanResultsNav-btn"
            :disabled="isBackDisabled"
            aria-label="Back"
            data-testid="nav-back"
            @click="emit('back')"
         >
            <PhCaretLeft :size="18" weight="regular" aria-hidden="true" />
         </button>
         <button
            v-if="isForwardShown"
            type="button"
            class="ScanResultsNav-btn"
            :disabled="isForwardDisabled"
            aria-label="Forward"
            data-testid="nav-forward"
            @click="emit('forward')"
         >
            <PhCaretRight :size="18" weight="regular" aria-hidden="true" />
         </button>
      </div>
      <div class="ScanResultsNav-path" data-testid="nav-path-label">
         <PhTrashSimple
            v-if="pathIcon === 'trash'"
            :size="16"
            weight="regular"
            class="ScanResultsNav-pathIcon"
            aria-hidden="true"
         />
         <PhFolderSimple
            v-else
            :size="16"
            weight="regular"
            class="ScanResultsNav-pathIcon"
            aria-hidden="true"
         />
         <span
            ref="pathTextRef"
            class="ScanResultsNav-pathText"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
            >{{ pathLabel }}</span
         >
      </div>
      <div v-if="isActionsShown" class="ScanResultsNav-actions">
         <button
            v-if="isResetShown"
            type="button"
            class="ScanResultsNav-resetBtn"
            :disabled="isResetDisabled"
            @click="emit('reset')"
         >
            {{ t('ScanResultsList', 'resetSelection') }}
         </button>
         <button
            v-if="isCancelShown"
            type="button"
            class="ScanResultsNav-cancelBtn"
            data-testid="results-cancel"
            @click="emit('cancel')"
         >
            {{ t('ScanResultsList', 'cancel') }}
         </button>
      </div>
   </nav>
   <div
      ref="pathPopoverRef"
      popover="manual"
      class="Popover"
      @pointerenter="onPointerEnter"
      @pointerleave="onPointerLeave"
   >
      {{ pathTitle || pathLabel }}
   </div>
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
   min-height: 24px;
   font-size: var(--font-size-base);
   color: var(--color-text-muted);
   text-align: left;
}

/* Explicit 16×16 and block avoid inline SVG subpixel alignment and blur. */
.ScanResultsNav-pathIcon {
   flex-shrink: 0;
   width: 16px;
   height: 16px;
   display: block;
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
   font-size: var(--font-size-md);
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
   font-size: var(--font-size-md);
   font-weight: 500;
   color: var(--color-abort);
   background: none;
   border: none;
   cursor: pointer;

   &:hover {
      opacity: 0.75;
   }
}
</style>
