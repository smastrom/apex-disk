import { createApp } from 'vue'

import AppShell from './components/AppShell.vue'

import { initTauriAppSettings } from '@/stores/app-settings'
import { applyTheme, applyDirection } from '@/lib/theme'

const store = await initTauriAppSettings()

applyTheme(store.getThemeColor())
applyDirection(store.settings.value.language)

const app = createApp(AppShell)
app.mount('#app')
