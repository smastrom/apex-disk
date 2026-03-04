<!--
SettingsView

Purpose: Settings screen with Language, Theme, Delete behavior (permanent delete), Scan Settings (hidden files, 0 B, under 1 KB), and Permissions. macOS-style grouped list.

Props: isFdaGranted (boolean), availableUpdate (string | null)

Example:
 <SettingsView :isFdaGranted="isFdaGranted" :availableUpdate="null" />
-->

<script setup lang="ts">
// import SettingsFooter from '@/components/SettingsFooter.vue'

import {
   PhArrowCircleUp,
   PhCaretDown,
   PhCheckCircle,
   PhCircle,
   PhWrench as PhGearSix,
} from '@phosphor-icons/vue'

import { computed } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'

import { useTranslations } from '@/lib/use-translations'

import { useSettingsStore } from '@/stores/settings'

import type { Language, ThemeColor } from '@/types/settings'

defineProps<{
   isFdaGranted: boolean
   availableUpdate: string | null
}>()

const { t } = useTranslations()

const store = useSettingsStore()
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
   { value: 'mac-disk-tree' as ThemeColor, label: t('SettingsView', 'themeMacDiskTree') },
   { value: 'macos-dark' as ThemeColor, label: t('SettingsView', 'themeMacosDark') },
   { value: 'macos-light' as ThemeColor, label: t('SettingsView', 'themeMacosLight') },
   { value: 'macos-graphite' as ThemeColor, label: t('SettingsView', 'themeMacosGraphite') },
   { value: 'coral-orange' as ThemeColor, label: t('SettingsView', 'themeCoralOrange') },
   { value: 'ayu' as ThemeColor, label: t('SettingsView', 'themeAyu') },
   { value: 'smastrom' as ThemeColor, label: t('SettingsView', 'themeSmastrom') },
])

function togglePermanentlyDelete() {
   store.setPermanentlyDelete(!settings.value.permanentlyDelete)
}

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
</script>

<template>
   <main class="SettingsView-root" data-testid="settings-view">
      <div class="SettingsView-content">
         <!-- App Settings -->

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'language') }}</span>
               <div class="SettingsView-selectWrap">
                  <select
                     class="SettingsSelect"
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
               <span class="SettingsGroup-label">{{ t('SettingsView', 'themeColor') }}</span>
               <div class="SettingsView-selectWrap">
                  <select
                     class="SettingsSelect"
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

         <!-- Delete behavior -->

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <div class="SettingsGroup-labelWrapper">
                  <span class="SettingsGroup-label">{{
                     t('SettingsView', 'permanentlyDelete')
                  }}</span>
                  <p class="SettingsView-settingDesc">
                     {{ t('SettingsView', 'permanentlyDeleteDesc') }}
                  </p>
               </div>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.permanentlyDelete }"
                  :aria-pressed="settings.permanentlyDelete"
                  @click="togglePermanentlyDelete"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
         </section>

         <!-- Scan Settings -->

         <section class="SettingsGroup">
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'indexHiddenFiles') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showHiddenFiles }"
                  :aria-pressed="settings.showHiddenFiles"
                  @click="toggleHiddenFiles"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'indexUnder1Kb') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showUnder1Kb }"
                  :aria-pressed="settings.showUnder1Kb"
                  @click="toggleUnder1Kb"
               >
                  <span class="SettingsToggle-knob" aria-hidden="true" />
               </button>
            </div>
            <div class="SettingsGroup-row">
               <span class="SettingsGroup-label">{{ t('SettingsView', 'indexZeroByte') }}</span>
               <button
                  type="button"
                  class="SettingsToggle"
                  :class="{ 'SettingsToggle--on': settings.showZeroByte }"
                  :aria-pressed="settings.showZeroByte"
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
                     availableUpdate
                        ? 'SettingsView-updateStatus--available'
                        : 'SettingsView-updateStatus--ok'
                  "
               >
                  <PhArrowCircleUp
                     v-if="availableUpdate"
                     :size="13"
                     weight="fill"
                     aria-hidden="true"
                  />
                  <PhCheckCircle v-else :size="13" weight="fill" aria-hidden="true" />
                  {{ availableUpdate ? availableUpdate : t('SettingsView', 'updateUpToDate') }}
               </span>
            </div>
            <p v-if="availableUpdate" class="SettingsView-updateDesc">
               {{ t('SettingsView', 'updateAvailableHint', { version: availableUpdate }) }}
            </p>
         </section>

         <!-- App info (name, version, author, links) -->
         <!-- <SettingsFooter /> -->
      </div>
   </main>
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
   font-size: 0.8125rem;
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
   font-size: 0.75rem;
   font-weight: 500;
   cursor: pointer;
   transition: opacity 0.15s ease;
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
   font-size: 0.75rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

.SettingsView-settingDesc {
   margin: 0;
   font-size: 0.75rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

.SettingsView-resultsNotice {
   margin: 0;
   padding: var(--spacing-md) var(--spacing-lg);
   font-size: 0.75rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}

/* Update row */
.SettingsView-updateStatus {
   display: flex;
   align-items: center;
   gap: 5px;
   font-size: 0.8125rem;
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
   font-size: 0.75rem;
   line-height: 1.5;
   color: var(--color-text-muted);
}
</style>
