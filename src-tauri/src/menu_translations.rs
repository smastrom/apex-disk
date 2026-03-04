//! Menu label translations for all supported languages.
//!
//! ## Multilanguage Behavior
//!
//! This file provides translations ONLY for custom menu items that cannot be
//! automatically localized by macOS. There are two categories of menu items:
//!
//! **Custom items (translated here)**:
//! - Check for Updates, Release Notes, License (app-specific functionality)
//! - Window and Help submenu titles (these are custom submenu headers)
//! - These follow the APP language setting and update immediately when language changes
//!
//! **Native items (NOT translated here)**:
//! - About, Services, Hide, Show All, Quit, Minimize, Close Window
//! - These use `None` as label in menu.rs and are automatically localized by macOS
//! - They follow the macOS SYSTEM language, not the app language setting
//!
//! ## Language Coordination
//!
//! - The `lang` parameter comes from the current app language setting
//! - This is typically the same as system language on first launch, or user's chosen language
//! - When user changes app language, `set_menu_language` rebuilds menu with new translations
//! - Native items remain in system language; custom items follow app language
//!
//! Keep translations in sync with frontend language list in
//! `src/lib/constants.ts` (`APP_LANGUAGES`).

/// Localized labels for custom menu items only.
///
/// These items require manual translation because they're app-specific or
/// submenu headers that macOS cannot automatically localize.
///
/// Native items (About, Hide, Quit, Minimize, etc.) use `None` as label
/// in menu.rs and are automatically localized by macOS to system language.
pub struct MenuLabels {
    pub check_for_updates: &'static str,
    pub release_notes: &'static str,
    pub license: &'static str,
    pub website_label: &'static str,
    pub window: &'static str,
    pub help: &'static str,
}

/// Returns menu labels for custom items only.
///
/// Native menu items (About, Hide, Quit, etc.) are automatically localized
/// by macOS and should use `None` as label in menu.rs.
pub fn labels_for(lang: &str) -> MenuLabels {
    match lang {
        "it" => MenuLabels {
            check_for_updates: "Controlla aggiornamenti…",
            release_notes: "Note di versione",
            license: "Licenza",
            website_label: "Note di versione",
            window: "Finestra",
            help: "Aiuto",
        },
        "es" => MenuLabels {
            check_for_updates: "Buscar actualizaciones…",
            release_notes: "Notas de versión",
            license: "Licencia",
            website_label: "Notas de versión",
            window: "Ventana",
            help: "Ayuda",
        },
        "fr" => MenuLabels {
            check_for_updates: "Rechercher des mises à jour…",
            release_notes: "Notes de version",
            license: "Licence",
            website_label: "Notes de version",
            window: "Fenêtre",
            help: "Aide",
        },
        "pt" => MenuLabels {
            check_for_updates: "Verificar atualizações…",
            release_notes: "Notas de versão",
            license: "Licença",
            website_label: "Notas de versão",
            window: "Janela",
            help: "Ajuda",
        },
        "de" => MenuLabels {
            check_for_updates: "Nach Updates suchen…",
            release_notes: "Versionshinweise",
            license: "Lizenz",
            website_label: "Versionshinweise",
            window: "Fenster",
            help: "Hilfe",
        },
        "ru" => MenuLabels {
            check_for_updates: "Поиск обновлений…",
            release_notes: "Заметки о выпуске",
            license: "Лицензия",
            website_label: "Заметки о выпуске",
            window: "Окно",
            help: "Справка",
        },
        "zh" => MenuLabels {
            check_for_updates: "检查更新…",
            release_notes: "发行说明",
            license: "许可证",
            website_label: "发行说明",
            window: "窗口",
            help: "帮助",
        },
        "ja" => MenuLabels {
            check_for_updates: "更新を確認…",
            release_notes: "リリースノート",
            license: "ライセンス",
            website_label: "リリースノート",
            window: "ウインドウ",
            help: "ヘルプ",
        },
        "ar" => MenuLabels {
            check_for_updates: "البحث عن تحديثات…",
            release_notes: "ملاحظات الإصدار",
            license: "الرخصة",
            website_label: "ملاحظات الإصدار",
            window: "نافذة",
            help: "المساعدة",
        },
        // English is the default
        _ => MenuLabels {
            check_for_updates: "Check for Updates…",
            release_notes: "Release Notes",
            license: "License",
            website_label: "Release Notes",
            window: "Window",
            help: "Help",
        },
    }
}
