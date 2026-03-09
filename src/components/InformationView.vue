<!--
InformationView

Purpose: Displays app information with branding, release details, and system information.

Props: systemInfo (SystemInfo | null)

Example:
   <InformationView :systemInfo="systemInfo" />
-->

<script setup lang="ts">
import type { SystemInfo } from '@/types/system-info'

import Logo from './ui/Logo.vue'
import InformationFooter from './InformationFooter.vue'

import { useTranslations } from '@/lib/use-translations'
import { formatBytes } from '@/lib/format'

import { APP_NAME } from '@/lib/constants'

const { t } = useTranslations()

const props = defineProps<{
   systemInfo: SystemInfo | null
}>()
</script>

<template>
   <main class="InformationView-root" data-testid="information-view">
      <div class="InformationView-content">
         <!-- App branding section -->
         <section class="InformationView-branding">
            <div class="InformationView-logoContainer">
               <Logo class="InformationView-logo" />
            </div>
            <h1 class="InformationView-title" data-testid="information-title">{{ APP_NAME }}</h1>

            <p class="InformationView-description">
               {{ t('InformationView', 'description') }}
            </p>
         </section>

         <!-- System information section -->
         <section class="SettingsGroup" v-if="props.systemInfo">
            <div class="SettingsGroup-row" v-if="props.systemInfo.macos_version">
               <span class="InformationView-detailLabel">{{
                  t('InformationView', 'macosVersion')
               }}</span>
               <span class="InformationView-detailValue">{{ props.systemInfo.macos_version }}</span>
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
               <span class="InformationView-detailLabel">{{ t('InformationView', 'memory') }}</span>
               <span class="InformationView-detailValue">{{ props.systemInfo.memory_info }}</span>
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
               <span class="InformationView-detailValue">{{ props.systemInfo.current_user }}</span>
            </div>
         </section>

         <InformationFooter />
      </div>
   </main>
</template>

<style scoped>
.InformationView-root {
   flex: 1;
   overflow-y: auto;
   padding: var(--spacing-lg) var(--spacing-md);
   background: var(--color-bg);
}

.InformationView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
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
   line-height: 1.4;
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
</style>
