// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Simone Mastromattei

/// <reference types="vite/client" />

declare module '*.vue' {
   import type { DefineComponent } from 'vue'

   const component: DefineComponent<{}, {}, any>
   export default component
}

declare module '*.yaml' {
   const value: Record<string, Record<string, string>>
   export default value
}
