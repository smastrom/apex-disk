//! Menu label translations for all supported languages.
//!
//! ## Multilanguage Behavior
//!
//! This file provides translations for ALL menu items to ensure they follow the app language setting:
//!
//! **All menu items (translated here)**:
//! - Native items: About, Services, Hide, Show All, Quit, Minimize, Close Window
//! - Custom items: Release Notes, License (app-specific functionality)
//! - Submenu titles: Window and Help (custom submenu headers)
//! - All items follow the APP language setting and update immediately when language changes
//!
//! ## Language Coordination
//!
//! - The `lang` parameter comes from the current app language setting
//! - This is typically the same as system language on first launch, or user's chosen language
//! - When user changes app language, `set_menu_language` rebuilds menu with new translations
//! - All menu items now follow the app language, not the macOS system language
//!
//! Keep translations in sync with frontend language list in
//! `src/lib/constants.ts` (`APP_LANGUAGES`).

/// Localized labels for all menu items.
///
/// Includes both custom items (app-specific functionality) and native items
/// (About, Services, Hide, etc.) to ensure they follow the app language setting
/// rather than the macOS system language.
pub struct MenuLabels {
    pub release_notes: &'static str,
    pub license: &'static str,
    pub website_label: &'static str,
    pub window: &'static str,
    pub minimize: &'static str,
    pub close_window: &'static str,
    pub help: &'static str,
    // Native menu items (now translated to follow app language)
    pub about: &'static str,
    pub services: &'static str,
    pub hide: &'static str,
    pub hide_others: &'static str,
    pub show_all: &'static str,
    pub quit: &'static str,
}

/// Returns menu labels for all menu items.
///
/// All menu items (both native and custom) are now translated to follow
/// the app language setting rather than the macOS system language.
pub fn labels_for(lang: &str) -> MenuLabels {
    match lang {
        "it" => MenuLabels {
            release_notes: "Note di versione",
            license: "Licenza",
            website_label: "Note di versione",
            window: "Finestra",
            minimize: "Minimizza",
            close_window: "Chiudi finestra",
            help: "Aiuto",
            about: "Informazioni su MacDiskTree",
            services: "Servizi",
            hide: "Nascondi MacDiskTree",
            hide_others: "Nascondi altre",
            show_all: "Mostra tutto",
            quit: "Esci da MacDiskTree",
        },
        "es" => MenuLabels {
            release_notes: "Notas de versión",
            license: "Licencia",
            website_label: "Notas de versión",
            window: "Ventana",
            minimize: "Minimizar",
            close_window: "Cerrar ventana",
            help: "Ayuda",
            about: "Acerca de MacDiskTree",
            services: "Servicios",
            hide: "Ocultar MacDiskTree",
            hide_others: "Ocultar otras",
            show_all: "Mostrar todo",
            quit: "Salir de MacDiskTree",
        },
        "fr" => MenuLabels {
            release_notes: "Notes de version",
            license: "Licence",
            website_label: "Notes de version",
            window: "Fenêtre",
            minimize: "Réduire",
            close_window: "Fermer la fenêtre",
            help: "Aide",
            about: "À propos de MacDiskTree",
            services: "Services",
            hide: "Masquer MacDiskTree",
            hide_others: "Masquer les autres",
            show_all: "Tout afficher",
            quit: "Quitter MacDiskTree",
        },
        "pt" => MenuLabels {
            release_notes: "Notas de versão",
            license: "Licença",
            website_label: "Notas de versão",
            window: "Janela",
            minimize: "Minimizar",
            close_window: "Fechar janela",
            help: "Ajuda",
            about: "Sobre o MacDiskTree",
            services: "Serviços",
            hide: "Ocultar MacDiskTree",
            hide_others: "Ocultar outras",
            show_all: "Mostrar tudo",
            quit: "Sair do MacDiskTree",
        },
        "de" => MenuLabels {
            release_notes: "Versionshinweise",
            license: "Lizenz",
            website_label: "Versionshinweise",
            window: "Fenster",
            minimize: "Minimieren",
            close_window: "Fenster schließen",
            help: "Hilfe",
            about: "Über MacDiskTree",
            services: "Dienste",
            hide: "MacDiskTree ausblenden",
            hide_others: "Andere ausblenden",
            show_all: "Alle einblenden",
            quit: "MacDiskTree beenden",
        },
        "ru" => MenuLabels {
            release_notes: "Заметки о выпуске",
            license: "Лицензия",
            website_label: "Заметки о выпуске",
            window: "Окно",
            minimize: "Свернуть",
            close_window: "Закрыть окно",
            help: "Справка",
            about: "О программе MacDiskTree",
            services: "Службы",
            hide: "Скрыть MacDiskTree",
            hide_others: "Скрыть остальные",
            show_all: "Показать все",
            quit: "Выйти из MacDiskTree",
        },
        "zh" => MenuLabels {
            release_notes: "发行说明",
            license: "许可证",
            website_label: "发行说明",
            window: "窗口",
            minimize: "最小化",
            close_window: "关闭窗口",
            help: "帮助",
            about: "关于 MacDiskTree",
            services: "服务",
            hide: "隐藏 MacDiskTree",
            hide_others: "隐藏其他",
            show_all: "显示全部",
            quit: "退出 MacDiskTree",
        },
        "ja" => MenuLabels {
            release_notes: "リリースノート",
            license: "ライセンス",
            website_label: "リリースノート",
            window: "ウインドウ",
            minimize: "最小化",
            close_window: "ウィンドウを閉じる",
            help: "ヘルプ",
            about: "MacDiskTree について",
            services: "サービス",
            hide: "MacDiskTree を隠す",
            hide_others: "ほかを隠す",
            show_all: "すべてを表示",
            quit: "MacDiskTree を終了",
        },
        "ar" => MenuLabels {
            release_notes: "ملاحظات الإصدار",
            license: "الرخصة",
            website_label: "ملاحظات الإصدار",
            window: "نافذة",
            minimize: "تصغير",
            close_window: "إغلاق النافذة",
            help: "المساعدة",
            about: "حول MacDiskTree",
            services: "الخدمات",
            hide: "إخفاء MacDiskTree",
            hide_others: "إخفاء الآخرين",
            show_all: "إظهار الكل",
            quit: "إنهاء MacDiskTree",
        },
        // English is the default
        _ => MenuLabels {
            release_notes: "Release Notes",
            license: "License",
            website_label: "Release Notes",
            window: "Window",
            minimize: "Minimize",
            close_window: "Close Window",
            help: "Help",
            about: "About MacDiskTree",
            services: "Services",
            hide: "Hide MacDiskTree",
            hide_others: "Hide Others",
            show_all: "Show All",
            quit: "Quit MacDiskTree",
        },
    }
}
