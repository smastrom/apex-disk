<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
InformationView

Purpose: Displays app information with branding, release details, and system information.

Props: systemInfo (SystemInfo | null)

Example:
   <InformationView :systemInfo="systemInfo" />
-->

<script setup lang="ts">
import InformationFooter from './InformationFooter.vue'
import Logo from './ui/Logo.vue'

import type { SystemInfo } from '@/types/system-info'

import { ref } from 'vue'

import { formatBytes } from '@/lib/format'
import { useTranslations } from '@/lib/use-translations'

import { APP_NAME } from '@/lib/constants'

const COPIED_FEEDBACK_MS = 1600

const { t } = useTranslations()

const props = defineProps<{
   systemInfo: SystemInfo | null
}>()

const isCopied = ref(false)

let copiedTimer: ReturnType<typeof setTimeout> | null = null

function buildSystemInfoText(info: SystemInfo): string {
   const rows: Array<[string, string | null | undefined]> = [
      [t('InformationView', 'macosVersion'), info.macos_version],
      [t('InformationView', 'hardwareModel'), info.hardware_model],
      [t('InformationView', 'cpu'), info.cpu_info],
      [t('InformationView', 'memory'), info.memory_info],
      [t('InformationView', 'systemDisk'), info.system_disk_name],
      [
         t('InformationView', 'diskSize'),
         info.system_disk_size ? formatBytes(info.system_disk_size) : null,
      ],
      [t('InformationView', 'currentUser'), info.current_user],
   ]

   return rows
      .filter(([, value]) => value)
      .map(([label, value]) => `${label}: ${value}`)
      .join('\n')
}

async function copySystemInfo() {
   if (!props.systemInfo) return

   try {
      await navigator.clipboard.writeText(buildSystemInfoText(props.systemInfo))
   } catch (err) {
      console.error('Failed to copy system info:', err)

      return
   }

   isCopied.value = true

   if (copiedTimer) clearTimeout(copiedTimer)

   copiedTimer = setTimeout(() => {
      isCopied.value = false
      copiedTimer = null
   }, COPIED_FEEDBACK_MS)
}
</script>

<template>
   <section class="InformationView-root" data-testid="information-view" aria-label="Information">
      <div class="InformationView-scroll">
         <div class="InformationView-content">
            <!-- App branding section -->
            <section class="InformationView-branding">
               <div class="InformationView-logoContainer">
                  <Logo class="InformationView-logo" />
               </div>
               <h2 class="InformationView-title" data-testid="information-title">{{ APP_NAME }}</h2>

               <p class="InformationView-description">
                  {{ t('InformationView', 'description') }}
               </p>
            </section>

            <!-- System information section -->
            <section
               class="SettingsGroup"
               style="margin-bottom: var(--spacing-sm)"
               v-if="props.systemInfo"
            >
               <div class="SettingsGroup-row" v-if="props.systemInfo.macos_version">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'macosVersion')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     props.systemInfo.macos_version
                  }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.hardware_model">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'hardwareModel')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     props.systemInfo.hardware_model
                  }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.cpu_info">
                  <span class="InformationView-detailLabel">{{ t('InformationView', 'cpu') }}</span>
                  <span class="InformationView-detailValue">{{ props.systemInfo.cpu_info }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.memory_info">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'memory')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     props.systemInfo.memory_info
                  }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.system_disk_name">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'systemDisk')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     props.systemInfo.system_disk_name
                  }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.system_disk_size">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'diskSize')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     formatBytes(props.systemInfo.system_disk_size)
                  }}</span>
               </div>
               <div class="SettingsGroup-row" v-if="props.systemInfo.current_user">
                  <span class="InformationView-detailLabel">{{
                     t('InformationView', 'currentUser')
                  }}</span>
                  <span class="InformationView-detailValue">{{
                     props.systemInfo.current_user
                  }}</span>
               </div>
            </section>

            <div class="InformationView-copyRow" v-if="props.systemInfo">
               <button
                  type="button"
                  class="InformationView-copyButton"
                  :class="{ 'InformationView-copyButton--copied': isCopied }"
                  :aria-live="'polite'"
                  @click="copySystemInfo"
               >
                  {{
                     isCopied
                        ? t('InformationView', 'copied')
                        : t('InformationView', 'copyToClipboard')
                  }}
               </button>
            </div>

            <InformationFooter />
         </div>
      </div>
   </section>
</template>

<style scoped>
.InformationView-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   overflow: hidden;
   background: var(--color-bg);
}

.InformationView-scroll {
   flex: 1;
   min-height: 0;
   overflow-x: hidden;
   overflow-y: auto;
   overflow-y: overlay;
}

.InformationView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
   padding-block: var(--spacing-lg);
   padding-inline-start: var(--spacing-md);
   padding-inline-end: calc(var(--spacing-md) - var(--scrollbar-inline-gutter));
}

.InformationView-branding {
   display: flex;
   flex-direction: column;
   align-items: center;
   text-align: center;
   padding: var(--spacing-sm) 0;
   margin-bottom: var(--spacing-md);
}

.InformationView-logoContainer {
   margin-bottom: var(--spacing-sm);
   opacity: 0.75;
}

.InformationView-logo {
   width: 48px;
   height: 48px;
}

.InformationView-title {
   font-size: var(--font-size-xl);
   font-weight: 700;
   color: var(--color-text);
   margin: 0 0 var(--spacing-sm) 0;
}

.InformationView-description {
   font-size: var(--font-size-base);
   color: var(--color-text);
   margin: 0 0 var(--spacing-sm) 0;
   max-width: 350px;
}

.InformationView-detailLabel {
   font-size: var(--font-size-lg);
   color: var(--color-text);
   font-weight: 500;
}

.InformationView-detailValue {
   font-size: var(--font-size-lg);
   color: var(--color-text-muted);
   white-space: nowrap;
   overflow: hidden;
   text-overflow: ellipsis;
}

.InformationView-copyRow {
   display: flex;
   justify-content: flex-end;
   margin-top: var(--spacing-xs);
   padding-inline-end: var(--spacing-xs);
}

.InformationView-copyButton {
   padding: 0;
   font-size: var(--font-size-sm);
   font-weight: 500;
   color: var(--color-accent);
   background: none;
   border: none;
   cursor: pointer;
   transition: color 0.2s var(--ease-standard);

   &:hover {
      color: var(--color-accent-hover);
   }
}

.InformationView-copyButton--copied {
   color: var(--color-text-muted);
   cursor: default;

   &:hover {
      color: var(--color-text-muted);
   }
}
</style>
