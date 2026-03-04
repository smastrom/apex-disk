import { computed } from 'vue'

import { createT } from '@/assets/translations'

import { useAppSettings } from '@/stores/settings'

import type { TranslationModule } from '@/assets/translations'

/**
 * Composable that provides a reactive translation function using the settings store language.
 * Use t(module, key, vars?) for interpolation (e.g. t('MainView', 'scanning', { current: 1, total: 10 })).
 * @param storeRef - Optional ref to the settings store; when provided (e.g. by the providing component), inject is skipped.
 */
export function useTranslations() {
   const settingsStore = useAppSettings()
   const lang = computed(() => settingsStore.settings.value.language)
   const t = computed(() => createT(lang.value))

   return {
      t: (module: TranslationModule, key: string, vars?: Record<string, string | number>) =>
         t.value(module, key, vars),
      lang,
   }
}
