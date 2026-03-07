<!--
ScanResultsDeleteListItem

Purpose: Compact list row for delete review. Checkbox (default on), icon, name, size.

Props: item (DeleteListItem), isSelected (boolean), formatBytes (fn)

Example:
 <ScanResultsDeleteListItem
   :item="entry"
   :isSelected="checkedSet.has(entry.path)"
   :formatBytes="formatBytes"
   @toggle="toggle(entry.path)"
 />
-->

<script setup lang="ts">
import ScanResultsListItemIconSwitch from '@/components/ScanResultsListItemIconSwitch.vue'
import CheckboxIcon from '@/components/ui/CheckboxIcon.vue'

import { useTemplateRef } from 'vue'

import { useLabelPopover } from '@/lib/use-label-popover'
import { displayPath, isHidden } from '@/lib/utils'

import type { DeleteListItem } from '@/types/structs'

defineProps<{
   item: DeleteListItem
   isSelected: boolean
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'toggle'): void
}>()

const triggerRef = useTemplateRef<HTMLElement>('triggerRef')
const popoverRef = useTemplateRef<HTMLElement>('popoverRef')

const { onPointerEnter, onPointerLeave } = useLabelPopover(triggerRef, popoverRef)
</script>

<template>
   <div
      class="ScanResultsDeleteListItem-root"
      data-testid="delete-list-row"
      @click="emit('toggle')"
   >
      <button
         type="button"
         class="ScanResultsDeleteListItem-check"
         data-testid="delete-list-row-checkbox"
         :class="{ 'ScanResultsDeleteListItem-check--selected': isSelected }"
         :aria-pressed="isSelected"
         @click.stop="emit('toggle')"
      >
         <CheckboxIcon
            :selected="isSelected"
            :size="16"
            :class="{
               'ScanResultsDeleteListItem-checkEmpty': !isSelected,
               'ScanResultsDeleteListItem-checkFilled': isSelected,
            }"
         />
      </button>
      <div
         class="ScanResultsDeleteListItem-icon"
         :class="{ 'ScanResultsDeleteListItem-icon--hidden': isHidden(item.name) }"
      >
         <ScanResultsListItemIconSwitch :item="item" :size="18" />
      </div>
      <div class="ScanResultsDeleteListItem-info">
         <span
            ref="triggerRef"
            class="ScanResultsDeleteListItem-name"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
            >{{ item.name }}</span
         >
         <span class="ScanResultsDeleteListItem-path">{{ displayPath(item.path) }}</span>
      </div>
      <span class="ScanResultsDeleteListItem-size">{{ formatBytes(item.size) }}</span>
      <div
         ref="popoverRef"
         popover="manual"
         class="Popover"
         @pointerenter="onPointerEnter"
         @pointerleave="onPointerLeave"
      >
         {{ item.name }}
      </div>
   </div>
</template>

<style scoped>
.ScanResultsDeleteListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) var(--spacing-sm);
   min-height: 48px;
   cursor: pointer;
   transition: background 0.2s;
   border-bottom: 1px solid var(--color-accent-bg);

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsDeleteListItem-check {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0;
   background: none;
   border: none;
   cursor: pointer;
   color: var(--color-text-dim);
}

.ScanResultsDeleteListItem-check--selected .ScanResultsDeleteListItem-checkFilled {
   color: var(--color-accent);
}

.ScanResultsDeleteListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsDeleteListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   color: var(--color-accent);
}

.ScanResultsDeleteListItem-icon--hidden {
   opacity: 0.5;
}

.ScanResultsDeleteListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: 1px;
}

.ScanResultsDeleteListItem-name {
   font-size: 0.8125rem;
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsDeleteListItem-path {
   font-size: 0.6875rem;
   color: var(--color-text-dim);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsDeleteListItem-size {
   font-size: 0.875rem;
   color: var(--color-text-muted);
}
</style>
