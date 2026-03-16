<!--
SettingsView

Purpose: Settings screen with Language, Theme, Delete behavior (permanent delete), Scan Settings (hidden files, 0 B, under 1 KB), and Permissions. macOS-style grouped list.

Props: isFdaGranted (boolean), newAvailableVersion (string | null)

Example:
 <SettingsView :isFdaGranted="isFdaGranted" :newAvailableVersion="null" />
-->

<script setup lang="ts">
import {
   PhArrowCircleUp,
   PhArrowFatDown,
   PhCaretDown,
   PhCheckCircle,
   PhCircle,
   PhWrench as PhGearSix,
   PhArrowClockwise,
} from '@phosphor-icons/vue'

import { computed } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/use-translations'
import { useAppSettings } from '@/stores/app-settings'

import { RELEASE_NOTES_URL, APP_VERSION } from '@/lib/constants'

import type { Language, ThemeColor } from '@/types/settings'

defineProps<{
   newAvailableVersion: string | null
   isFdaGranted: boolean
   isChecking: boolean
}>()

const emit = defineEmits<{
   (e: 'check-for-updates'): void
}>()

const { t } = useTranslations()

const store = useAppSettings()
const settings = computed(() => store.settings.value)

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
   { value: 'smastrom' as ThemeColor, label: t('SettingsView', 'themeApexGreen') },
   { value: 'macos-dark' as ThemeColor, label: t('SettingsView', 'themeMacosDark') },
   { value: 'macos-graphite' as ThemeColor, label: t('SettingsView', 'themeMacosGraphite') },
   { value: 'macos-light' as ThemeColor, label: t('SettingsView', 'themeMacosLight') },
])

function toggleHiddenFiles() {
   store.setShowHiddenFiles(!settings.value.showHiddenFiles)
}

function toggleUnder1Kb() {
   store.setShowUnder1Kb(!settings.value.showUnder1Kb)
}

function toggleZeroByte() {
   store.setShowZeroByte(!settings.value.showZeroByte)
}

async function openSystemSettings() {
   await openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles')
}

async function openReleasesPage() {
   await openUrl(RELEASE_NOTES_URL)
}
</script>

<template>
   <section class="SettingsView-root" data-testid="settings-view" aria-label="Settings">
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
                  <PhCheckCircle v-if="isFdaGranted" :size="13" weight="fill" aria-hidden="true" />
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
               <span class="SettingsGroup-label">{{ t('SettingsView', 'updateLabel') }}</span>
               <span
                  class="SettingsView-updateStatus"
                  :class="
                     newAvailableVersion
                        ? 'SettingsView-updateStatus--available'
                        : 'SettingsView-updateStatus--ok'
                  "
               >
                  <PhArrowCircleUp
                     v-if="newAvailableVersion"
                     :size="13"
                     weight="fill"
                     aria-hidden="true"
                  />
                  <PhCheckCircle v-else :size="13" weight="fill" aria-hidden="true" />
                  {{
                     isChecking
                        ? t('SettingsView', 'updateChecking')
                        : newAvailableVersion
                          ? newAvailableVersion
                          : t('SettingsView', 'updateUpToDate')
                  }}
               </span>
            </div>
            <p v-if="newAvailableVersion" class="SettingsView-updateDesc">
               {{ t('SettingsView', 'updateAvailableHint', { version: newAvailableVersion }) }}
            </p>
            <p v-else class="SettingsView-updateDesc">
               {{ t('SettingsView', 'updateLatestVersion', { version: APP_VERSION }) }}
            </p>
            <div class="SettingsView-updateControls">
               <button
                  v-if="!newAvailableVersion"
                  type="button"
                  class="SettingsView-fdaBtn"
                  :disabled="isChecking"
                  @click="emit('check-for-updates')"
               >
                  <PhArrowClockwise
                     :size="13"
                     weight="fill"
                     aria-hidden="true"
                     :class="{ 'SettingsView-spinning': isChecking }"
                  />
                  {{
                     isChecking
                        ? t('SettingsView', 'updateChecking')
                        : t('SettingsView', 'updateCheckButton')
                  }}
               </button>
               <button
                  v-if="newAvailableVersion"
                  type="button"
                  class="SettingsView-fdaBtn"
                  @click="openReleasesPage"
               >
                  <PhArrowFatDown :size="13" weight="fill" aria-hidden="true" />
                  {{ t('SettingsView', 'updateDownloadsButton') }}
               </button>
            </div>
         </section>
      </div>
   </section>
</template>

<style scoped>
.SettingsView-root {
   flex: 1;
   overflow-y: auto;
   padding: var(--spacing-md);
   background: var(--color-bg);
}

.SettingsView-content {
   max-width: var(--content-max-width);
   margin: 0 auto;
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
.SettingsView-updateStatus {
   display: flex;
   align-items: center;
   gap: 5px;
   font-size: var(--font-size-base);
   font-weight: 500;
}

.SettingsView-updateStatus--ok {
   color: var(--color-success, #22c55e);
}

.SettingsView-updateStatus--available {
   color: var(--color-accent);
}

.SettingsView-updateDesc {
   margin: 0;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: var(--font-size-sm);
   color: var(--color-text-muted);
}

.SettingsView-updateControls {
   display: flex;
   justify-content: space-between;
   gap: 6px;
   padding: 0 var(--spacing-lg);
   margin-bottom: var(--spacing-md);
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
