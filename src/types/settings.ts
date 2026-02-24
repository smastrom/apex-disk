/** Supported app languages. */
export type Language = "en" | "it";

/** Supported theme color presets. */
export type ThemeColor = "oceanic" | "light" | "dark";

/** App settings persisted to disk. */
export interface AppSettings {
   language: Language;
   themeColor: ThemeColor;
   showHiddenFiles: boolean;
   showZeroByteFiles: boolean;
}

export const DEFAULT_SETTINGS: AppSettings = {
   language: "en",
   themeColor: "oceanic",
   showHiddenFiles: false,
   showZeroByteFiles: false,
};
