<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
ScanTrashListItem

Purpose: Compact list row for trash review. Checkbox (default on), icon, name, size.

Props: item (TrashListItem), isSelected (boolean), formatBytes (fn)

Example:
 <ScanTrashListItem
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
const pathTriggerRef = useTemplateRef<HTMLElement>('pathTriggerRef')
const pathPopoverRef = useTemplateRef<HTMLElement>('pathPopoverRef')

const { onPointerEnter, onPointerLeave } = useLabelPopover(triggerRef, popoverRef)
const { onPointerEnter: onPathPointerEnter, onPointerLeave: onPathPointerLeave } = useLabelPopover(
   pathTriggerRef,
   pathPopoverRef
)
</script>

<template>
   <div class="ScanTrashListItem-root" data-testid="trash-list-row" @click="emit('toggle')">
      <button
         type="button"
         class="ScanTrashListItem-check"
         data-testid="trash-list-row-checkbox"
         :class="{ 'ScanTrashListItem-check--selected': isSelected }"
         :aria-pressed="isSelected"
         :aria-label="item.name"
         @click.stop="emit('toggle')"
      >
         <CheckboxIcon
            :selected="isSelected"
            :size="16"
            :class="{
               'ScanTrashListItem-checkEmpty': !isSelected,
               'ScanTrashListItem-checkFilled': isSelected,
            }"
         />
      </button>
      <div
         class="ScanTrashListItem-icon"
         :class="{ 'ScanTrashListItem-icon--hidden': isHidden(item.name) }"
      >
         <ScanResultsListItemIconSwitch :item="item" :size="18" />
      </div>
      <div class="ScanTrashListItem-info">
         <span
            ref="triggerRef"
            class="ScanTrashListItem-name"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
            >{{ item.name }}</span
         >
         <span
            ref="pathTriggerRef"
            class="ScanTrashListItem-path"
            @pointerenter="onPathPointerEnter"
            @pointerleave="onPathPointerLeave"
            >{{ displayPath(item.path) }}</span
         >
      </div>
      <span class="ScanTrashListItem-size">{{ formatBytes(item.size) }}</span>
      <Teleport to="body">
         <div
            ref="popoverRef"
            class="Popover"
            role="tooltip"
            @pointerenter="onPointerEnter"
            @pointerleave="onPointerLeave"
         >
            {{ item.name }}
         </div>
         <div
            ref="pathPopoverRef"
            class="Popover"
            role="tooltip"
            @pointerenter="onPathPointerEnter"
            @pointerleave="onPathPointerLeave"
         >
            {{ displayPath(item.path) }}
         </div>
      </Teleport>
   </div>
</template>

<style scoped>
.ScanTrashListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
   padding: var(--spacing-sm) var(--spacing-md);
   min-height: 48px;
   cursor: pointer;
   transition: background 0.2s var(--ease-standard);
   border-bottom: 1px solid var(--color-accent-bg);
   z-index: 2;

   &:hover {
      background: var(--color-accent-bg-hover);
   }
}

.ScanTrashListItem-check {
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

.ScanTrashListItem-check--selected .ScanTrashListItem-checkFilled {
   color: var(--color-accent-alt);
}

.ScanTrashListItem-checkEmpty {
   color: var(--color-text-dim);
}

.ScanTrashListItem-icon {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   justify-content: center;
   width: 24px;
   height: 24px;
   color: var(--color-accent);
}

.ScanTrashListItem-icon--hidden {
   opacity: 0.5;
}

.ScanTrashListItem-info {
   flex: 1;
   min-width: 0;
   display: flex;
   flex-direction: column;
   gap: 1px;
}

.ScanTrashListItem-name {
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanTrashListItem-path {
   font-size: var(--font-size-sm);
   color: var(--color-text-dim);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanTrashListItem-size {
   font-size: var(--font-size-base);
   color: var(--color-text-muted);
}
</style>
