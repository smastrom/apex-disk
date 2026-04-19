<!--
ScanTrashConfirmation

Purpose: Post-trash screen. Shows resume (items count, size freed) and Scan again button. When debug logging is on, **Scan again** uses category `trash` (see `docs/LOGGING.md`).

Props: deletedSummary ({ count: number, size: number } | null)

Example:
 <ScanTrashConfirmation :deletedSummary="summary" @restart="onRestart" />
-->

<script setup lang="ts">
import AnimatedAlertCircle from './ui/AnimatedAlertCircle.vue'
import AnimatedCheckCircle from './ui/AnimatedCheckCircle.vue'

import { PhMagnifyingGlass, PhX } from '@phosphor-icons/vue'
import { computed } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { formatBytes } from '@/lib/format'
import { log } from '@/lib/log'
import { useTranslations } from '@/lib/use-translations'

const props = defineProps<{
   deletedSummary: { count: number; size: number } | null
}>()

const emit = defineEmits<{
   (e: 'restart'): void
}>()

const { t } = useTranslations()

const hasErrors = computed(() => !props.deletedSummary || props.deletedSummary.count === 0)

/** Emits `restart` after logging (returns user to results to scan again). */
function onScanAgain() {
   const s = props.deletedSummary

   if (s && s.count > 0) {
      log('trash', `Trash: scan again — last delete ${s.count} item(s), ${formatBytes(s.size)}`)
   } else {
      log('trash', 'Trash: scan again')
   }

   emit('restart')
}

function closeApp() {
   getCurrentWindow().close()
}
</script>

<template>
   <div class="ScanTrashConfirmation-root" role="status" aria-live="polite">
      <div class="ScanTrashConfirmation-content">
         <AnimatedAlertCircle
            v-if="hasErrors"
            :size="48"
            class="ScanTrashConfirmation-icon ScanTrashConfirmation-iconError"
         />
         <AnimatedCheckCircle v-else :size="48" class="ScanTrashConfirmation-icon" />
         <h2 class="ScanTrashConfirmation-title">
            {{ t('ScanTrashConfirmation', hasErrors ? 'titleErrors' : 'title') }}
         </h2>
         <p class="ScanTrashConfirmation-resume">
            {{
               hasErrors
                  ? t('ScanTrashConfirmation', 'resumeErrors')
                  : t(
                       'ScanTrashConfirmation',
                       deletedSummary!.count === 1 ? 'resumeOne' : 'resume',
                       {
                          count: deletedSummary!.count,
                          size: formatBytes(deletedSummary!.size),
                       }
                    )
            }}
         </p>
         <button
            type="button"
            class="ScanTrashConfirmation-scanBtn GradientButton"
            data-testid="restart"
            @click="onScanAgain"
         >
            <PhMagnifyingGlass :size="18" weight="regular" aria-hidden="true" />
            {{ t('ScanTrashConfirmation', 'restart') }}
         </button>
         <button type="button" class="ScanTrashConfirmation-closeBtn" @click="closeApp">
            <PhX :size="16" weight="bold" aria-hidden="true" />
            {{ t('ScanTrashConfirmation', 'closeApp') }}
         </button>
      </div>
   </div>
</template>

<style scoped>
.ScanTrashConfirmation-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   align-items: center;
   justify-content: center;
   min-height: 0;
   padding: var(--spacing-lg);
   background: var(--color-bg);
}

.ScanTrashConfirmation-content {
   display: flex;
   flex-direction: column;
   align-items: center;
   gap: var(--spacing-md);
   max-width: var(--content-max-width);
   width: 100%;
}

.ScanTrashConfirmation-icon {
   color: var(--color-accent-alt);
   flex-shrink: 0;
}

.ScanTrashConfirmation-iconError {
   color: var(--color-abort);
}

.ScanTrashConfirmation-title {
   margin: 0;
   font-size: var(--font-size-2xl);
   font-weight: 600;
   color: var(--color-text);
   text-align: center;
}

.ScanTrashConfirmation-resume {
   margin: 0;
   font-size: var(--font-size-lg);
   color: var(--color-text-muted);
   text-align: center;
}

.ScanTrashConfirmation-scanBtn {
   height: var(--cta-btn-height);
   width: 100%;
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.5rem;
   margin-top: var(--spacing-xl);
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-lg);
}

.ScanTrashConfirmation-closeBtn {
   display: flex;
   align-items: center;
   justify-content: center;
   gap: 0.375rem;
   padding: var(--spacing-sm) var(--spacing-md);
   font-size: var(--font-size-base);
   font-weight: 500;
   color: var(--color-text-muted);
   background: none;
   border: none;
   border-radius: 6px;
   cursor: pointer;
   transition: color 0.2s var(--ease-standard);

   &:hover {
      color: var(--color-text);
   }
}
</style>
