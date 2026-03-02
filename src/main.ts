import { createApp } from 'vue'

import App from './components/App.vue'

import { createSettingsStore, SETTINGS_KEY } from '@/stores/settings'
import { applyTheme, applyDirection } from '@/lib/theme'

const store = await createSettingsStore()
applyTheme(store.getThemeColor())
applyDirection(store.settings.value.language)

const app = createApp(App)
app.provide(SETTINGS_KEY, store)
app.mount('#app')
