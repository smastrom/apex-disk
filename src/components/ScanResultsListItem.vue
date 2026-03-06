<!--
ScanResultsListItem

Purpose: Single row for folder or file. Selection circle, icon, name, item count (folders), size, nav chevron (folders).

Props: item (FolderInfo), isSelected (boolean), isSomeSelected (boolean?), isSelectable (boolean?), formatBytes (fn)

Example:
 <ScanResultsListItem
   :item="folder"
   :isSelected="selectedPaths.has(folder.path)"
   :isSomeSelected="someSelectedPaths.has(folder.path)"
   :isSelectable="!folder.is_protected"
   :formatBytes="formatBytes"
   @select="toggleSelect"
   @navigate="goInto"
 />
-->

<script setup lang="ts">
import ScanResultsListItemIconSwitch from '@/components/ScanResultsListItemIconSwitch.vue'
import SelectionIcon from '@/components/ui/SelectionIcon.vue'

import { PhCaretRight } from '@phosphor-icons/vue'
import { ref, useTemplateRef, computed } from 'vue'

import { useTranslations } from '@/lib/use-translations'
import { useLabelPopover } from '@/lib/use-label-popover'
import { formatDate } from '@/lib/format'
import { useAppSettings } from '@/stores/app-settings'

import type { FolderInfo } from '@/types/structs'

const props = defineProps<{
   item: FolderInfo
   isSelected: boolean
   isSomeSelected?: boolean
   isSelectable?: boolean
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'select'): void
   (e: 'navigate'): void
}>()

/** Determines the selection state for the SelectionIcon component. */
const selectionState = computed(() => {
   if (props.isSelected) return 'selected'
   if (props.isSomeSelected) return 'partial'
   return 'empty'
})

const triggerRef = useTemplateRef<HTMLElement>('triggerRef')
const popoverRef = useTemplateRef<HTMLElement>('popoverRef')
const checkboxTriggerRef = useTemplateRef<HTMLElement>('checkboxTriggerRef')
const checkboxPopoverRef = useTemplateRef<HTMLElement>('checkboxPopoverRef')

const { t } = useTranslations()
const { onPointerEnter, onPointerLeave } = useLabelPopover(triggerRef, popoverRef)
const store = useAppSettings()
const currentLanguage = store.settings.value.language

// Simple tooltip for checkbox (not dependent on text truncation)
const showCheckboxTooltip = ref(false)
let checkboxTimer: ReturnType<typeof setTimeout> | null = null

function onCheckboxPointerEnter() {
   if (checkboxTimer) clearTimeout(checkboxTimer)
   checkboxTimer = setTimeout(() => {
      showCheckboxTooltip.value = true

      // Position the popover relative to the checkbox with viewport clamping
      const trigger = checkboxTriggerRef.value
      const popover = checkboxPopoverRef.value
      if (trigger && popover) {
         const rect = trigger.getBoundingClientRect()
         const EDGE_MARGIN = 16

         // Set initial position
         popover.style.left = `${rect.left}px`
         popover.style.top = `${rect.top - 4}px`

         // Clamp to viewport
         const maxWidth = window.innerWidth - EDGE_MARGIN * 2
         popover.style.maxWidth = `${maxWidth}px`

         const popoverRect = popover.getBoundingClientRect()
         const maxLeft = window.innerWidth - EDGE_MARGIN - popoverRect.width
         const left = Math.max(EDGE_MARGIN, Math.min(rect.left, maxLeft))

         popover.style.left = `${left}px`
      }

      popover?.showPopover()

      // Add scroll listener to dismiss when scrolling
      addCheckboxScrollListener()
   }, 400)
}

function addCheckboxScrollListener() {
   const trigger = checkboxTriggerRef.value
   if (!trigger) return

   // Walk up to find the nearest scrollable ancestor
   let ancestor: HTMLElement | null = trigger.parentElement
   while (ancestor) {
      const { overflow, overflowY } = getComputedStyle(ancestor)
      if (/auto|scroll/.test(overflow + overflowY)) break
      ancestor = ancestor.parentElement
   }
   const target = ancestor ?? document

   target.addEventListener('scroll', dismissCheckboxTooltip, { passive: true, once: true })
}

function onCheckboxPointerLeave() {
   if (checkboxTimer) {
      clearTimeout(checkboxTimer)
      checkboxTimer = null
   }
   if (showCheckboxTooltip.value) {
      showCheckboxTooltip.value = false
      checkboxPopoverRef.value?.hidePopover()
   }
}

function dismissCheckboxTooltip() {
   if (checkboxTimer) {
      clearTimeout(checkboxTimer)
      checkboxTimer = null
   }
   if (showCheckboxTooltip.value) {
      showCheckboxTooltip.value = false
      checkboxPopoverRef.value?.hidePopover()
   }
}
</script>

<template>
   <div
      class="ScanResultsListItem-root"
      :class="{
         'ScanResultsListItem-root--selected': isSelected,
         'ScanResultsListItem-root--folder': !item.is_file,
      }"
      :data-testid="item.is_file ? 'results-row-file' : 'results-row-folder'"
      @click="!item.is_file && emit('navigate')"
   >
      <button
         ref="checkboxTriggerRef"
         type="button"
         class="ScanResultsListItem-check"
         data-testid="results-row-checkbox"
         :class="{
            'ScanResultsListItem-check--selected': isSelected,
            'ScanResultsListItem-check--some-selected': !isSelected && isSomeSelected,
            'ScanResultsListItem-check--disabled': !isSelectable,
         }"
         :aria-pressed="isSelected || isSomeSelected"
         :disabled="!isSelectable"
         :aria-disabled="!isSelectable"
         @click.stop="isSelectable && emit('select')"
         @pointerenter="!isSelectable && onCheckboxPointerEnter()"
         @pointerleave="!isSelectable && onCheckboxPointerLeave()"
      >
         <SelectionIcon
            :state="selectionState"
            :size="22"
            :class="{
               'ScanResultsListItem-checkEmpty': selectionState === 'empty',
               'ScanResultsListItem-checkPartial': selectionState === 'partial',
               'ScanResultsListItem-checkFilled': selectionState === 'selected',
            }"
         />
      </button>
      <div
         class="ScanResultsListItem-icon"
         :class="{ 'ScanResultsListItem-icon--hidden': item.name.startsWith('.') }"
      >
         <ScanResultsListItemIconSwitch :item="item" :size="28" />
      </div>
      <div class="ScanResultsListItem-info">
         <span
            ref="triggerRef"
            class="ScanResultsListItem-name"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
            >{{ item.name }}</span
         >
         <span class="ScanResultsListItem-details">
            <span v-if="!item.is_file">
               {{
                  item.children.length === 1
                     ? t('ScanResultsListItem', 'itemOne')
                     : t('ScanResultsListItem', 'itemsCount', { count: item.children.length })
               }}<span v-if="item.last_modified" style="opacity: 0.5">,</span>
            </span>
            <span style="opacity: 0.5" v-if="item.last_modified">
               {{ ' ' }} {{ t('ScanResultsListItem', 'lastModified').toLocaleLowerCase() }}
               {{ formatDate(item.last_modified, currentLanguage) }}
            </span>
         </span>
      </div>
      <div class="ScanResultsListItem-meta">
         <span class="ScanResultsListItem-size">{{ formatBytes(item.size) }}</span>
         <PhCaretRight
            v-if="!item.is_file"
            :size="18"
            weight="regular"
            class="ScanResultsListItem-chevron"
            aria-hidden="true"
         />
      </div>
      <div
         ref="popoverRef"
         popover="manual"
         class="ScanResultsListItem-popover"
         @pointerenter="onPointerEnter"
         @pointerleave="onPointerLeave"
      >
         {{ item.name }}
      </div>
      <div
         v-if="!isSelectable"
         ref="checkboxPopoverRef"
         popover="manual"
         class="ScanResultsListItem-checkboxPopover"
      >
         {{
            item.is_fda_required
               ? t('ScanResultsListItem', 'fdaRequiredTooltip')
               : t('ScanResultsListItem', 'protectedTooltip')
         }}
      </div>
   </div>
</template>

<style scoped>
.ScanResultsListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   padding: var(--spacing-sm);
   height: 64px;
   min-height: 64px;
   margin: calc(var(--spacing-sm) / 2) var(--spacing-sm);
   border-radius: 8px;
   box-sizing: border-box;
   transition: background 0.2s;

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsListItem-root--folder {
   cursor: pointer;
}

.ScanResultsListItem-root--selected {
   background: var(--color-accent-bg);

   &:hover {
      background: var(--color-accent-bg);
   }
}

.ScanResultsListItem-check {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0;
   background: none;
   border: none;
   cursor: pointer;
   color: var(--color-text-dim);
   transition: filter 0.2s;
}

.ScanResultsListItem-check--selected .ScanResultsListItem-checkFilled {
   color: var(--color-accent);
   filter: drop-shadow(0 0 4px var(--color-accent-glow));
}

.ScanResultsListItem-check--some-selected .ScanResultsListItem-checkPartial {
   color: var(--color-accent);
}

.ScanResultsListItem-check--disabled {
   opacity: 0.5;
   cursor: not-allowed;
}

.ScanResultsListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   color: var(--color-accent);
}

.ScanResultsListItem-icon--hidden {
   opacity: 0.4;
}

.ScanResultsListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: var(--spacing-xxs);
}

.ScanResultsListItem-name {
   font-size: 0.9375rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsListItem-details {
   font-size: 0.75rem;
   color: var(--color-text-muted);
}

.ScanResultsListItem-meta {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ScanResultsListItem-size {
   font-size: 0.875rem;
   color: var(--color-text-muted);
}

.ScanResultsListItem-chevron {
   color: var(--color-text-dim);
}

/* ── Base popover styles ── */

.ScanResultsListItem-popover,
.ScanResultsListItem-checkboxPopover {
   position: fixed;
   margin: 0;
   border: 1px solid var(--color-border);
   border-radius: 6px;
   background: var(--color-bg-elevated);
   color: var(--color-text);
   font-weight: 500;
   line-height: 1.4;
   box-shadow: 0 2px 12px rgba(0, 0, 0, 0.25);
   transform: translateY(-100%);
   pointer-events: auto;
   opacity: 0;
   filter: blur(4px);
   transition:
      opacity 0.2s var(--ease-standard),
      filter 0.2s var(--ease-standard);

   &:popover-open {
      opacity: 1;
      filter: blur(0);
   }

   @starting-style {
      &:popover-open {
         opacity: 0;
         filter: blur(4px);
      }
   }

   @media (prefers-reduced-motion: reduce) {
      transition: none;
      filter: none;
   }
}

.ScanResultsListItem-popover {
   padding: 6px 10px;
   max-width: 420px;
   font-size: 0.75rem;
   word-break: break-all;
}

.ScanResultsListItem-checkboxPopover {
   padding: 8px 12px;
   max-width: 280px;
   font-size: 0.75rem;
   line-height: 1.5;
}
</style>
