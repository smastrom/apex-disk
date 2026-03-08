<!--
ScanResultsTrashListItem

Purpose: Compact list row for trash review. Checkbox (default on), icon, name, size.

Props: item (TrashListItem), isSelected (boolean), formatBytes (fn)

Example:
 <ScanResultsTrashListItem
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

import type { TrashListItem } from '@/types/structs'

defineProps<{
   item: TrashListItem
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
   <div class="ScanResultsTrashListItem-root" data-testid="trash-list-row" @click="emit('toggle')">
      <button
         type="button"
         class="ScanResultsTrashListItem-check"
         data-testid="trash-list-row-checkbox"
         :class="{ 'ScanResultsTrashListItem-check--selected': isSelected }"
         :aria-pressed="isSelected"
         @click.stop="emit('toggle')"
      >
         <CheckboxIcon
            :selected="isSelected"
            :size="16"
            :class="{
               'ScanResultsTrashListItem-checkEmpty': !isSelected,
               'ScanResultsTrashListItem-checkFilled': isSelected,
            }"
         />
      </button>
      <div
         class="ScanResultsTrashListItem-icon"
         :class="{ 'ScanResultsTrashListItem-icon--hidden': isHidden(item.name) }"
      >
         <ScanResultsListItemIconSwitch :item="item" :size="18" />
      </div>
      <div class="ScanResultsTrashListItem-info">
         <span
            ref="triggerRef"
            class="ScanResultsTrashListItem-name"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
            >{{ item.name }}</span
         >
         <span class="ScanResultsTrashListItem-path">{{ displayPath(item.path) }}</span>
      </div>
      <span class="ScanResultsTrashListItem-size">{{ formatBytes(item.size) }}</span>
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
.ScanResultsTrashListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-xs) var(--spacing-sm);
   min-height: 48px;
   cursor: pointer;
   transition: background 0.2s var(--ease-standard);
   border-bottom: 1px solid var(--color-accent-bg);

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanResultsTrashListItem-check {
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

.ScanResultsTrashListItem-check--selected .ScanResultsTrashListItem-checkFilled {
   color: var(--color-accent);
}

.ScanResultsTrashListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanResultsTrashListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   color: var(--color-accent);
}

.ScanResultsTrashListItem-icon--hidden {
   opacity: 0.5;
}

.ScanResultsTrashListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: 1px;
}

.ScanResultsTrashListItem-name {
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsTrashListItem-path {
   font-size: var(--font-size-xs);
   color: var(--color-text-dim);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsTrashListItem-size {
   font-size: var(--font-size-md);
   color: var(--color-text-muted);
}
</style>
