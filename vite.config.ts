import path from 'path'

import yaml from '@rollup/plugin-yaml'
import vue from '@vitejs/plugin-vue'
import { browserslistToTargets } from 'lightningcss'
import { defineConfig } from 'vite'

const host = process.env.TAURI_DEV_HOST

export default defineConfig(() => ({
   root: 'src',
   plugins: [vue(), yaml()],
   resolve: {
      alias: {
         '@': path.resolve(__dirname, 'src'),
      },
   },

   css: {
      transformer: 'lightningcss' as const,
      lightningcss: {
         targets: browserslistToTargets(['safari 13']), // Matches minimumSystemVersion in tauri.conf.json (macOS 10.15 = Safari 13)
      },
   },

   build: {
      target: 'safari13',
      cssMinify: 'lightningcss' as const,
      outDir: '../dist',
      emptyOutDir: true,
   },

   clearScreen: true,
   server: {
      port: 1420,
      strictPort: true,
      host: host || false,
      hmr: host
         ? {
              protocol: 'ws',
              host,
              port: 1421,
           }
         : undefined,
      watch: {
         ignored: ['**/src-tauri/**'],
      },
   },
}))
