<!--
InformationView

Purpose: Displays app information with branding, release details, and system information.

Props: systemInfo (SystemInfo | null)

Example:
   <InformationView :systemInfo="systemInfo" />
-->

<script setup lang="ts">
import Logo from './ui/Logo.vue'
import InformationFooter from './InformationFooter.vue'

import { useTranslations } from '@/lib/use-translations'
import { formatBytes } from '@/lib/format'

import type { SystemInfo } from '@/types/system-info'

import { APP_NAME } from '@/lib/constants'

const { t } = useTranslations()

const props = defineProps<{
   systemInfo: SystemInfo | null
}>()
</script>

<template>
   <main class="InformationView-root">
      <div class="InformationView-content">
         <!-- App branding section -->
         <section class="InformationView-branding">
            <div class="InformationView-logoContainer">
               <Logo class="InformationView-logo" />
            </div>
            <h1 class="InformationView-title">{{ APP_NAME }}</h1>

            <p class="InformationView-description">
               {{ t('InformationView', 'description') }}
            </p>
         </section>

         <!-- System information section -->
         <section class="SettingsGroup">
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'hardwareModel') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">{{ props.systemInfo.hardware_model }}</div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">{{ t('InformationView', 'cpu') }}</div>
               </div>
               <div class="InformationView-detailValue">{{ props.systemInfo.cpu_info }}</div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'memory') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">{{ props.systemInfo.memory_info }}</div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'systemDisk') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">
                  {{ props.systemInfo.system_disk_name }}
               </div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'diskSize') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">
                  {{ formatBytes(props.systemInfo.system_disk_size) }}
               </div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'macosVersion') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">{{ props.systemInfo.macos_version }}</div>
            </div>
            <div class="SettingsGroup-row" v-if="props.systemInfo">
               <div class="SettingsGroup-labelWrapper">
                  <div class="InformationView-detailLabel">
                     {{ t('InformationView', 'currentUser') }}
                  </div>
               </div>
               <div class="InformationView-detailValue">{{ props.systemInfo.current_user }}</div>
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
   font-size: 1.125rem;
   font-weight: 700;
   color: var(--color-text);
   margin: 0 0 var(--spacing-sm) 0;
}

.InformationView-version {
   font-size: 0.875rem;
   color: var(--color-text-muted);
   margin: 0 0 var(--spacing-sm) 0;
}

.InformationView-description {
   font-size: 0.8125rem;
   color: var(--color-text);
   margin: 0 0 var(--spacing-sm) 0;
   max-width: 350px;
   line-height: 1.4;
}

.InformationView-author {
   font-size: 0.75rem;
   color: var(--color-text-muted);
   margin: 0;
}

.InformationView-detailLabel {
   font-size: 0.9375rem;
   color: var(--color-text);
   font-weight: 500;
}

.InformationView-detailValue {
   font-size: 0.9375rem;
   color: var(--color-text-muted);
   white-space: nowrap;
}

.InformationView-linksContainer {
   display: flex;
   align-items: center;
   gap: var(--spacing-sm);
   flex-wrap: wrap;
}

.InformationView-link {
   padding: 4px 10px;
   background: var(--color-surface);
   border: 1px solid var(--color-border);
   border-radius: 5px;
   color: var(--color-text);
   font-size: 0.75rem;
   font-weight: 500;
   cursor: pointer;
   transition: opacity 0.15s ease;
   white-space: nowrap;

   &:hover {
      opacity: 0.75;
   }
}
</style>
