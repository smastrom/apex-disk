import { inject, computed, type Ref } from 'vue'

import { createT } from '@/assets/translations'

import { SETTINGS_KEY } from '@/stores/settings'

import type { TranslationModule } from '@/assets/translations'
import type { SettingsStore } from '@/stores/settings'

/**
 * Composable that provides a reactive translation function using the settings store language.
 * Use t(module, key, vars?) for interpolation (e.g. t('MainView', 'scanning', { current: 1, total: 10 })).
 * @param storeRef - Optional ref to the settings store; when provided (e.g. by the providing component), inject is skipped.
 */
export function useTranslations(storeRef?: Ref<SettingsStore | null>) {
   const resolved = storeRef ?? inject<Ref<SettingsStore | null>>(SETTINGS_KEY)
   const lang = computed(() => resolved?.value?.settings?.value?.language ?? 'en')
   const t = computed(() => createT(lang.value))
   return {
      t: (module: TranslationModule, key: string, vars?: Record<string, string | number>) =>
         t.value(module, key, vars),
      lang,
   }
}
