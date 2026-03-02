import { ref, shallowRef, provide, onMounted, watch } from 'vue'

import { applyTheme, applyDirection } from '@/lib/theme'
import { createSettingsStore, SETTINGS_KEY } from '@/stores/settings'

import type { SettingsStore } from '@/stores/settings'

/**
 * Bootstraps the settings store: creates and provides the store, applies the initial theme and
 * text direction, and keeps both in sync whenever settings change.
 * Call once in the root component (App.vue). Consumers use `useSettingsStore()` to inject.
 */
export function provideSettingsStore() {
   const settingsStore = shallowRef<SettingsStore | null>(null)
   const appReady = ref(false)

   provide(SETTINGS_KEY, settingsStore)

   onMounted(async () => {
      try {
         settingsStore.value = await createSettingsStore()
         applyTheme(settingsStore.value!.getThemeColor())
         applyDirection(settingsStore.value!.settings.value.language)
      } catch (err) {
         console.error('Failed to load settings:', err)
      } finally {
         appReady.value = true
      }
   })

   watch(
      () => settingsStore.value?.getThemeColor(),
      (theme) => {
         if (theme) applyTheme(theme)
      }
   )

   watch(
      () => settingsStore.value?.settings.value.language,
      (lang) => {
         if (lang) applyDirection(lang)
      }
   )

   return { settingsStore, appReady }
}
