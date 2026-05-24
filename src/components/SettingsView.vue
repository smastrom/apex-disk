<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- Copyright (C) 2026 Simone Mastromattei -->

<!--
SettingsView

Purpose: Settings screen with Language, Theme, Delete behavior (permanent delete), Scan Settings (hidden files, .DS_Store, 0 B, under 1 KB), Permissions (FDA), and Software Update (manual check + auto-check / auto-install toggles). macOS-style grouped list.

Props: isFdaGranted (boolean), isChecking (boolean), availableVersion (string | null)

Example:
 <SettingsView :isFdaGranted="isFdaGranted" :isChecking="false" :availableVersion="null" />
-->

<script setup lang="ts">
import type { Language, ThemeColor } from '@/types/settings'

import {
   PhArrowCircleUp,
   PhCaretDown,
   PhCheckCircle,
   PhCircle,
   PhWrench as PhGearSix,
   PhArrowClockwise,
} from '@phosphor-icons/vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import { computed, useTemplateRef } from 'vue'

import { useScrollbarVisibility } from '@/lib/use-scrollbar-visibility'
import { useTranslations } from '@/lib/use-translations'
import { useAppSettings } from '@/stores/app-settings'

import { APP_VERSION } from '@/lib/constants'

const props = defineProps<{
   isFdaGranted: boolean
   isChecking: boolean
   isDownloading: boolean
   availableVersion: string | null
   updateReady: boolean
}>()

const emit = defineEmits<{
   (e: 'check-for-updates'): void
}>()

const { t } = useTranslations()

const store = useAppSettings()
const settings = computed(() => store.settings.value)

const scrollRef = useTemplateRef<HTMLElement>('scrollRef')

useScrollbarVisibility(scrollRef, 'hover-only')

const languageOptions = computed(() => [
   { value: 'en' as Language, label: t('SettingsView', 'languageEn') },
   { value: 'it' as Language, label: t('SettingsView', 'languageIt') },
   { value: 'es' as Language, label: t('SettingsView', 'languageEs') },
   { value: 'fr' as Language, label: t('SettingsView', 'languageFr') },
   { value: 'pt' as Language, label: t('SettingsView', 'languagePt') },
   { value: 'de' as Language, label: t('SettingsView', 'languageDe') },
   { value: 'ru' as Language, label: t('SettingsView', 'languageRu') },
   { value: 'zh' as Language, label: t('SettingsView', 'languageZh') },
   { value: 'ja' as Language, label: t('SettingsView', 'languageJa') },
   { value: 'ar' as Language, label: t('SettingsView', 'languageAr') },
])

const themeOptions = computed(() => [
   { value: 'apex' as ThemeColor, label: t('SettingsView', 'themeApex') },
   { value: 'apex-coral' as ThemeColor, label: t('SettingsView', 'themeApexCoral') },
   { value: 'apex-ocean' as ThemeColor, label: t('SettingsView', 'themeApexOcean') },
   { value: 'apex-graphite' as ThemeColor, label: t('SettingsView', 'themeApexGraphite') },
   { value: 'apex-green' as ThemeColor, label: t('SettingsView', 'themeApexGreen') },
   { value: 'apex-light' as ThemeColor, label: t('SettingsView', 'themeApexLight') },
   { value: 'macos-dark' as ThemeColor, label: t('SettingsView', 'themeMacosDark') },
   { value: 'macos-light' as ThemeColor, label: t('SettingsView', 'themeMacosLight') },
])

/**
 * Resolves the software update action button label from the current update state.
 */
const updateActionLabel = computed(() => {
   if (props.isChecking) return t('SettingsView', 'updateChecking')
   if (props.isDownloading) return t('SettingsView', 'updateDownloading')
   if (props.updateReady) return t('SettingsView', 'updateRestartButton')
   if (props.availableVersion) {
      return t('SettingsView', 'updateAvailableButton', { version: props.availableVersion })
   }

   return t('SettingsView', 'updateCheckButton')
})

function toggleHiddenFiles() {
   store.setShowHiddenFiles(!settings.value.showHiddenFiles)
}

function toggleDsStore() {
   if (!settings.value.showHiddenFiles) return // Disabled when hidden files is off (gating rule).

   store.setShowDsStore(!settings.value.showDsStore)
}

function toggleUnder1Kb() {
   store.setShowUnder1Kb(!settings.value.showUnder1Kb)
}

function toggleZeroByte() {
   store.setShowZeroByte(!settings.value.showZeroByte)
}

function toggleAutoCheckUpdates() {
   store.setAutoCheckUpdates(!settings.value.autoCheckUpdates)
}

function toggleAutoInstallUpdates() {
   if (!settings.value.autoCheckUpdates) return // Disabled when checking is off (cascade rule).

   store.setAutoInstallUpdates(!settings.value.autoInstallUpdates)
}

async function openSystemSettings() {
   await openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles')
}
</script>

<template>
   <section class="SettingsView-root" data-testid="settings-view" aria-label="Settings">
      <div ref="scrollRef" class="SettingsView-scroll">
         <div class="SettingsView-content" data-testid="settings-content">
            <!-- App Settings -->

            <section class="SettingsGroup">
               <div class="SettingsGroup-row">
                  <span id="label-language" class="SettingsGroup-label">{{
                     t('SettingsView', 'language')
                  }}</span>
                  <div class="SettingsView-selectWrap">
                     <select
                        class="SettingsSelect"
                        aria-labelledby="label-language"
                        :value="settings.language"
                        @change="
                           store.setLanguage(($event.target as HTMLSelectElement).value as Language)
                        "
                     >
                        <option v-for="opt in languageOptions" :key="opt.value" :value="opt.value">
                           {{ opt.label }}
                        </option>
                     </select>
                     <PhCaretDown
                        :size="14"
                        weight="regular"
                        class="SettingsView-selectChevron"
                        aria-hidden="true"
                     />
                  </div>
               </div>
               <div class="SettingsGroup-row">
                  <span id="label-theme" class="SettingsGroup-label">{{
                     t('SettingsView', 'themeColor')
                  }}</span>
                  <div class="SettingsView-selectWrap">
                     <select
                        class="SettingsSelect"
                        aria-labelledby="label-theme"
                        :value="settings.themeColor"
                        @change="
                           store.setThemeColor(
                              ($event.target as HTMLSelectElement).value as ThemeColor
                           )
                        "
                     >
                        <option v-for="opt in themeOptions" :key="opt.value" :value="opt.value">
                           {{ opt.label }}
                        </option>
                     </select>
                     <PhCaretDown
                        :size="14"
                        weight="regular"
                        class="SettingsView-selectChevron"
                        aria-hidden="true"
                     />
                  </div>
               </div>
            </section>

            <!-- Scan Settings -->

            <section class="SettingsGroup">
               <div class="SettingsGroup-row">
                  <span id="label-hidden-files" class="SettingsGroup-label">{{
                     t('SettingsView', 'scanHiddenFiles')
                  }}</span>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{ 'SettingsToggle--on': settings.showHiddenFiles }"
                     :aria-checked="settings.showHiddenFiles"
                     aria-labelledby="label-hidden-files"
                     @click="toggleHiddenFiles"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
               <div class="SettingsGroup-row">
                  <div
                     class="SettingsGroup-labelWrapper"
                     :class="{
                        'SettingsGroup-labelWrapper--disabled': !settings.showHiddenFiles,
                     }"
                  >
                     <span id="label-ds-store" class="SettingsGroup-label">{{
                        t('SettingsView', 'scanDsStore')
                     }}</span>
                     <span class="SettingsView-labelDesc">{{
                        t('SettingsView', 'scanDsStoreDesc')
                     }}</span>
                  </div>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{
                        'SettingsToggle--on': settings.showHiddenFiles && settings.showDsStore,
                        'SettingsToggle--disabled': !settings.showHiddenFiles,
                     }"
                     :aria-checked="settings.showHiddenFiles && settings.showDsStore"
                     :aria-disabled="!settings.showHiddenFiles"
                     :disabled="!settings.showHiddenFiles"
                     aria-labelledby="label-ds-store"
                     @click="toggleDsStore"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
               <div class="SettingsGroup-row">
                  <span id="label-under-1kb" class="SettingsGroup-label">{{
                     t('SettingsView', 'scanUnder1Kb')
                  }}</span>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{ 'SettingsToggle--on': settings.showUnder1Kb }"
                     :aria-checked="settings.showUnder1Kb"
                     aria-labelledby="label-under-1kb"
                     @click="toggleUnder1Kb"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
               <div class="SettingsGroup-row">
                  <span id="label-zero-byte" class="SettingsGroup-label">{{
                     t('SettingsView', 'scanZeroByte')
                  }}</span>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{ 'SettingsToggle--on': settings.showZeroByte }"
                     :aria-checked="settings.showZeroByte"
                     aria-labelledby="label-zero-byte"
                     @click="toggleZeroByte"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
               <p class="SettingsView-resultsNotice">
                  {{ t('SettingsView', 'resultsEffectiveNextScan') }}
               </p>
            </section>

            <!-- FDA -->

            <section class="SettingsGroup">
               <div class="SettingsGroup-row">
                  <span class="SettingsGroup-label">{{ t('SettingsView', 'fdaLabel') }}</span>
                  <span
                     class="SettingsView-fdaStatus"
                     :class="
                        isFdaGranted
                           ? 'SettingsView-fdaStatus--ok'
                           : 'SettingsView-fdaStatus--optional'
                     "
                  >
                     <PhCheckCircle
                        v-if="isFdaGranted"
                        :size="13"
                        weight="fill"
                        aria-hidden="true"
                     />
                     <PhCircle v-else :size="13" weight="regular" aria-hidden="true" />
                     {{
                        isFdaGranted
                           ? t('SettingsView', 'fdaGranted')
                           : t('SettingsView', 'fdaMissing')
                     }}
                  </span>
               </div>
               <template v-if="!isFdaGranted">
                  <p class="SettingsView-fdaDesc">
                     {{ t('SettingsView', 'fdaDesc') }}
                  </p>

                  <div class="SettingsView-fdaControls">
                     <button type="button" class="SettingsView-fdaBtn" @click="openSystemSettings">
                        <PhGearSix :size="13" weight="fill" aria-hidden="true" />
                        {{ t('SettingsView', 'fdaOpenSettings') }}
                     </button>
                  </div>
               </template>
            </section>

            <!-- Software Update -->

            <section class="SettingsGroup">
               <div class="SettingsGroup-row SettingsGroup-row--canWrap">
                  <div class="SettingsGroup-labelWrapper">
                     <span class="SettingsGroup-label">{{ t('SettingsView', 'updateLabel') }}</span>
                     <span class="SettingsView-labelDesc">{{
                        updateReady
                           ? t('SettingsView', 'updateReadyDesc')
                           : availableVersion
                             ? t('SettingsView', 'updateAvailable', { version: availableVersion })
                             : t('SettingsView', 'updateUpToDate', { version: APP_VERSION })
                     }}</span>
                  </div>
                  <button
                     type="button"
                     class="SettingsView-fdaBtn"
                     :class="{ 'SettingsView-fdaBtn--accent': updateReady }"
                     :disabled="isChecking || isDownloading"
                     @click="emit('check-for-updates')"
                  >
                     <PhArrowClockwise
                        v-if="!updateReady && !availableVersion"
                        :size="13"
                        weight="fill"
                        aria-hidden="true"
                        :class="{ 'SettingsView-spinning': isChecking || isDownloading }"
                     />
                     <PhArrowCircleUp
                        v-else-if="updateReady || availableVersion"
                        :size="13"
                        weight="fill"
                        aria-hidden="true"
                     />
                     {{ updateActionLabel }}
                  </button>
               </div>
               <div class="SettingsGroup-row">
                  <div class="SettingsGroup-labelWrapper">
                     <span id="label-auto-check-updates" class="SettingsGroup-label">{{
                        t('SettingsView', 'autoCheckUpdatesLabel')
                     }}</span>
                     <span class="SettingsView-labelDesc">{{
                        t('SettingsView', 'autoCheckUpdatesDesc')
                     }}</span>
                  </div>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{ 'SettingsToggle--on': settings.autoCheckUpdates }"
                     :aria-checked="settings.autoCheckUpdates"
                     aria-labelledby="label-auto-check-updates"
                     @click="toggleAutoCheckUpdates"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
               <div class="SettingsGroup-row">
                  <div
                     class="SettingsGroup-labelWrapper"
                     :class="{
                        'SettingsGroup-labelWrapper--disabled': !settings.autoCheckUpdates,
                     }"
                  >
                     <span id="label-auto-install-updates" class="SettingsGroup-label">{{
                        t('SettingsView', 'autoInstallUpdatesLabel')
                     }}</span>
                     <span class="SettingsView-labelDesc">{{
                        t('SettingsView', 'autoInstallUpdatesDesc')
                     }}</span>
                  </div>
                  <button
                     type="button"
                     role="switch"
                     class="SettingsToggle"
                     :class="{
                        'SettingsToggle--on':
                           settings.autoCheckUpdates && settings.autoInstallUpdates,
                        'SettingsToggle--disabled': !settings.autoCheckUpdates,
                     }"
                     :aria-checked="settings.autoCheckUpdates && settings.autoInstallUpdates"
                     :aria-disabled="!settings.autoCheckUpdates"
                     :disabled="!settings.autoCheckUpdates"
                     aria-labelledby="label-auto-install-updates"
                     @click="toggleAutoInstallUpdates"
                  >
                     <span class="SettingsToggle-knob" aria-hidden="true" />
                  </button>
               </div>
            </section>
         </div>
      </div>
   </section>
</template>

<style scoped>
.SettingsView-root {
   flex: 1;
   display: flex;
   flex-direction: column;
   min-height: 0;
   overflow: hidden;
   background: var(--color-bg);
}

.SettingsView-scroll {
   flex: 1;
   min-height: 0;
   overflow-x: hidden;
   overflow-y: auto;
   overflow-y: overlay;
}

.SettingsView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
   padding-block: var(--spacing-md);
   padding-inline-start: var(--spacing-md);
   padding-inline-end: calc(var(--spacing-md) - var(--scrollbar-inline-gutter));
}

.SettingsView-selectWrap {
   position: relative;
   display: inline-block;

   & select {
      min-width: 160px;
   }

   .SettingsSelect {
      background-image: none;
   }
}

.SettingsView-selectChevron {
   position: absolute;
   right: 10px;
   top: 50%;
   transform: translateY(-50%);
   pointer-events: none;
   color: var(--color-text-muted);
}

/* FDA row */
.SettingsView-fdaStatus {
   display: flex;
   align-items: center;
   gap: 5px;
   font-size: var(--font-size-base);
   font-weight: 500;
}

.SettingsView-fdaStatus--ok {
   color: var(--color-success, #22c55e);
}

.SettingsView-fdaStatus--optional {
   color: var(--color-text-muted);
}

.SettingsView-fdaControls {
   display: flex;
   justify-content: space-between;
   gap: 6px;
   padding: 0 var(--spacing-lg);
   margin-bottom: var(--spacing-md);
}

.SettingsView-fdaBtn {
   display: inline-flex;
   align-items: center;
   gap: 4px;
   padding: 4px 10px;
   border: 1px solid var(--color-border);
   border-radius: 5px;
   background: var(--color-bg);
   color: var(--color-text);
   font-size: var(--font-size-sm);
   font-weight: 500;
   cursor: pointer;
   transition: opacity 0.15s var(--ease-standard);
}

.SettingsView-fdaBtn:hover:not(:disabled) {
   opacity: 0.75;
}

.SettingsView-fdaBtn:disabled {
   opacity: 0.45;
   cursor: default;
}

.SettingsView-fdaBtn--accent {
   background: var(--color-accent);
   color: var(--color-bg);
   border-color: var(--color-accent);
}

.SettingsView-fdaDesc {
   margin: 0;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.SettingsView-resultsNotice {
   margin: 0;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

/* Update row */
.SettingsView-labelDesc {
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.SettingsView-spinning {
   animation: spin 1s linear infinite;
}

@keyframes spin {
   from {
      transform: rotate(0deg);
   }
   to {
      transform: rotate(360deg);
   }
}
</style>
