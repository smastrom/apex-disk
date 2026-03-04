import { createApp } from 'vue'

import App from './components/App.vue'

import { initTauriAppSettings } from '@/stores/settings'
import { applyTheme, applyDirection } from '@/lib/theme'

const store = await initTauriAppSettings()

applyTheme(store.getThemeColor())
applyDirection(store.settings.value.language)

const app = createApp(App)
app.mount('#app')
