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
   isSelectable?: boolean | 'deselect-only'
   formatBytes: (bytes: number) => string
}>()

const emit = defineEmits<{
   (e: 'select'): void
   (e: 'navigate'): void
}>()

const selectionState = computed(() => {
   if (props.isSelected) return 'selected'
   if (props.isSomeSelected) return 'partial'
   return 'empty'
})

const isCheckDisabled = computed(() => !props.isSelectable)

/** Suppress click-to-navigate when the pointer drags far enough to read as a text selection. */
const isPressing = ref(false)
const DRAG_THRESHOLD_PX = 4
let pressStartX = 0
let pressStartY = 0
let hasPressStart = false

function onRootPointerDown(e: PointerEvent) {
   if (props.item.is_file) return
   if (e.button !== 0) return
   pressStartX = e.clientX
   pressStartY = e.clientY
   hasPressStart = true
   isPressing.value = true
}

function onRootPointerMove(e: PointerEvent) {
   if (!hasPressStart) return
   const dx = Math.abs(e.clientX - pressStartX)
   const dy = Math.abs(e.clientY - pressStartY)
   if (dx > DRAG_THRESHOLD_PX || dy > DRAG_THRESHOLD_PX) {
      isPressing.value = false
   }
}

function onRootPointerEnd() {
   isPressing.value = false
}

function onRootClick(e: MouseEvent) {
   if (props.item.is_file) return
   if (hasPressStart) {
      const dx = Math.abs(e.clientX - pressStartX)
      const dy = Math.abs(e.clientY - pressStartY)
      hasPressStart = false
      if (dx > DRAG_THRESHOLD_PX || dy > DRAG_THRESHOLD_PX) return
   }
   const selection = window.getSelection()
   if (selection && !selection.isCollapsed && selection.toString().trim().length > 0) return
   emit('navigate')
}

function onRootNavigateKey() {
   if (props.item.is_file) return
   emit('navigate')
}

const triggerRef = useTemplateRef<HTMLElement>('triggerRef')
const popoverRef = useTemplateRef<HTMLElement>('popoverRef')
const checkboxTriggerRef = useTemplateRef<HTMLElement>('checkboxTriggerRef')
const checkboxPopoverRef = useTemplateRef<HTMLElement>('checkboxPopoverRef')

const { t } = useTranslations()
const { onPointerEnter, onPointerLeave } = useLabelPopover(triggerRef, popoverRef)
const store = useAppSettings()
const currentLanguage = store.settings.value.language

const showCheckboxTooltip = ref(false)
let checkboxTimer: ReturnType<typeof setTimeout> | null = null

function onCheckboxPointerEnter() {
   if (checkboxTimer) clearTimeout(checkboxTimer)
   checkboxTimer = setTimeout(() => {
      showCheckboxTooltip.value = true

      const trigger = checkboxTriggerRef.value
      const popover = checkboxPopoverRef.value
      if (trigger && popover) {
         const rect = trigger.getBoundingClientRect()
         const EDGE_MARGIN = 16

         popover.style.left = `${rect.left}px`
         popover.style.top = `${rect.top - 4}px`
         popover.style.maxWidth = `${window.innerWidth - EDGE_MARGIN * 2}px`

         const popoverRect = popover.getBoundingClientRect()
         const maxLeft = window.innerWidth - EDGE_MARGIN - popoverRect.width
         popover.style.left = `${Math.max(EDGE_MARGIN, Math.min(rect.left, maxLeft))}px`
      }

      popover?.showPopover()
      addCheckboxScrollListener()
   }, 400)
}

function addCheckboxScrollListener() {
   const trigger = checkboxTriggerRef.value
   if (!trigger) return

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
         'ScanResultsListItem-root--pressing': isPressing,
      }"
      :role="!item.is_file ? 'button' : undefined"
      :tabindex="!item.is_file ? 0 : undefined"
      :data-testid="item.is_file ? 'results-row-file' : 'results-row-folder'"
      @click="onRootClick"
      @pointerdown="onRootPointerDown"
      @pointermove="onRootPointerMove"
      @pointerup="onRootPointerEnd"
      @pointercancel="onRootPointerEnd"
      @pointerleave="onRootPointerEnd"
      @keydown.enter="onRootNavigateKey"
      @keydown.space.prevent="onRootNavigateKey"
   >
      <button
         ref="checkboxTriggerRef"
         type="button"
         class="ScanResultsListItem-check"
         data-testid="results-row-checkbox"
         :class="{
            'ScanResultsListItem-check--selected': isSelected,
            'ScanResultsListItem-check--some-selected': !isSelected && isSomeSelected,
            'ScanResultsListItem-check--disabled': isCheckDisabled,
         }"
         :aria-pressed="isSelected || isSomeSelected"
         :aria-label="t('ScanResultsListItem', 'selectItem', { name: item.name })"
         :disabled="isCheckDisabled"
         :aria-disabled="isCheckDisabled"
         @click.stop="!isCheckDisabled && emit('select')"
         @pointerdown.stop
         @pointerenter="isCheckDisabled && onCheckboxPointerEnter()"
         @pointerleave="isCheckDisabled && onCheckboxPointerLeave()"
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
   </div>
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
   </Teleport>
</template>

<style scoped>
.ScanResultsListItem-root {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   padding: var(--spacing-xs) var(--spacing-sm);
   height: 64px;
   min-height: 64px;
   margin: calc(var(--spacing-sm) / 2) var(--spacing-sm);
   border-radius: var(--radius-md);
   box-sizing: border-box;
   background: var(--color-row-idle);
   transition:
      background 0.18s var(--ease-apple-out),
      transform 0.12s var(--ease-apple-out),
      box-shadow 0.2s var(--ease-apple-out);

   &:hover {
      background: var(--color-row-hover);
   }
}

.ScanResultsListItem-root--folder {
   cursor: pointer;
}

.ScanResultsListItem-root--selected {
   background: var(--color-row-selected);

   &:hover {
      background: var(--color-row-selected-hover);
   }
}

/* JS-driven press state — only set on clean clicks (pointer didn't drag
 * and release didn't land on the checkbox). Works on all supported
 * macOS versions; the previous :has()-based rule required Safari 15.4+. */
.ScanResultsListItem-root--folder.ScanResultsListItem-root--pressing {
   transform: scale(0.992);
   background: var(--color-row-press);
}

@media (prefers-reduced-motion: reduce) {
   .ScanResultsListItem-root--folder.ScanResultsListItem-root--pressing {
      transform: none;
   }
}

.ScanResultsListItem-check {
   flex-shrink: 0;
   align-self: stretch;
   display: flex;
   align-items: center;
   justify-content: center;
   padding: 0;
   background: none;
   border-radius: var(--radius-sm);
   border: none;
   cursor: pointer;
   color: var(--color-text-dim);
   transition: filter 0.2s var(--ease-standard);
}

.ScanResultsListItem-check--selected .ScanResultsListItem-checkFilled {
   color: var(--color-accent-alt);
   filter: drop-shadow(0 0 4px var(--color-accent-alt-glow));
}

.ScanResultsListItem-check--some-selected .ScanResultsListItem-checkPartial {
   color: var(--color-accent-alt);
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
   font-size: var(--font-size-lg);
   font-weight: 500;
   color: var(--color-text);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.ScanResultsListItem-details {
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.ScanResultsListItem-meta {
   flex-shrink: 0;
   display: flex;
   align-items: center;
   gap: var(--spacing-xs);
}

.ScanResultsListItem-size {
   font-size: var(--font-size-base);
   color: var(--color-text-muted);
}

.ScanResultsListItem-chevron {
   color: var(--color-text-dim);
}

/* ── Checkbox tooltip popover ── */

.ScanResultsListItem-checkboxPopover {
   position: fixed;
   margin: 0;
   padding: var(--spacing-sm) var(--spacing-md);
   max-width: 280px;
   border: 1px solid var(--color-chrome-border);
   border-radius: var(--radius-sm);
   background: var(--color-chrome);
   -webkit-backdrop-filter: saturate(180%) blur(30px);
   backdrop-filter: saturate(180%) blur(30px);
   color: var(--color-text);
   font-size: var(--font-size-sm);
   font-weight: 500;
   box-shadow: var(--shadow-md);
   transform: translateY(-100%);
   pointer-events: auto;
   opacity: 0;
   filter: blur(4px);
   transition:
      opacity 0.2s var(--ease-apple-out),
      filter 0.2s var(--ease-apple-out);
}

.ScanResultsListItem-checkboxPopover:popover-open {
   opacity: 1;
   filter: blur(0);
}

@starting-style {
   .ScanResultsListItem-checkboxPopover:popover-open {
      opacity: 0;
      filter: blur(4px);
   }
}

@media (prefers-reduced-motion: reduce) {
   .ScanResultsListItem-checkboxPopover {
      transition: none;
      filter: none;
   }
}
</style>
