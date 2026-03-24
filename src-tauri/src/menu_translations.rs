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
    pub check_for_updates: &'static str,
    pub checking_for_updates: &'static str,
    pub downloading_update: &'static str,
    pub restart_to_update: &'static str,
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
            check_for_updates: "Controlla aggiornamenti…",
            checking_for_updates: "Controllo aggiornamenti…",
            downloading_update: "Download aggiornamento…",
            restart_to_update: "Riavvia per aggiornare",
            website_label: "Note di versione",
            window: "Finestra",
            minimize: "Minimizza",
            close_window: "Chiudi finestra",
            help: "Aiuto",
            about: "Informazioni su ApexDisk",
            services: "Servizi",
            hide: "Nascondi ApexDisk",
            hide_others: "Nascondi altre",
            show_all: "Mostra tutto",
            quit: "Esci da ApexDisk",
        },
        "es" => MenuLabels {
            release_notes: "Notas de versión",
            license: "Licencia",
            check_for_updates: "Buscar actualizaciones…",
            checking_for_updates: "Buscando actualizaciones…",
            downloading_update: "Descargando actualización…",
            restart_to_update: "Reiniciar para actualizar",
            website_label: "Notas de versión",
            window: "Ventana",
            minimize: "Minimizar",
            close_window: "Cerrar ventana",
            help: "Ayuda",
            about: "Acerca de ApexDisk",
            services: "Servicios",
            hide: "Ocultar ApexDisk",
            hide_others: "Ocultar otras",
            show_all: "Mostrar todo",
            quit: "Salir de ApexDisk",
        },
        "fr" => MenuLabels {
            release_notes: "Notes de version",
            license: "Licence",
            check_for_updates: "Vérifier les mises à jour…",
            checking_for_updates: "Recherche de mises à jour…",
            downloading_update: "Téléchargement de la mise à jour…",
            restart_to_update: "Redémarrer pour mettre à jour",
            website_label: "Notes de version",
            window: "Fenêtre",
            minimize: "Réduire",
            close_window: "Fermer la fenêtre",
            help: "Aide",
            about: "À propos de ApexDisk",
            services: "Services",
            hide: "Masquer ApexDisk",
            hide_others: "Masquer les autres",
            show_all: "Tout afficher",
            quit: "Quitter ApexDisk",
        },
        "pt" => MenuLabels {
            release_notes: "Notas de versão",
            license: "Licença",
            check_for_updates: "Verificar atualizações…",
            checking_for_updates: "Verificando atualizações…",
            downloading_update: "Baixando atualização…",
            restart_to_update: "Reiniciar para atualizar",
            website_label: "Notas de versão",
            window: "Janela",
            minimize: "Minimizar",
            close_window: "Fechar janela",
            help: "Ajuda",
            about: "Sobre o ApexDisk",
            services: "Serviços",
            hide: "Ocultar ApexDisk",
            hide_others: "Ocultar outras",
            show_all: "Mostrar tudo",
            quit: "Sair do ApexDisk",
        },
        "de" => MenuLabels {
            release_notes: "Versionshinweise",
            license: "Lizenz",
            check_for_updates: "Nach Updates suchen…",
            checking_for_updates: "Suche nach Updates…",
            downloading_update: "Update wird heruntergeladen…",
            restart_to_update: "Neu starten zum Aktualisieren",
            website_label: "Versionshinweise",
            window: "Fenster",
            minimize: "Minimieren",
            close_window: "Fenster schließen",
            help: "Hilfe",
            about: "Über ApexDisk",
            services: "Dienste",
            hide: "ApexDisk ausblenden",
            hide_others: "Andere ausblenden",
            show_all: "Alle einblenden",
            quit: "ApexDisk beenden",
        },
        "ru" => MenuLabels {
            release_notes: "Заметки о выпуске",
            license: "Лицензия",
            check_for_updates: "Проверить обновления…",
            checking_for_updates: "Проверка обновлений…",
            downloading_update: "Загрузка обновления…",
            restart_to_update: "Перезапустить для обновления",
            website_label: "Заметки о выпуске",
            window: "Окно",
            minimize: "Свернуть",
            close_window: "Закрыть окно",
            help: "Справка",
            about: "О программе ApexDisk",
            services: "Службы",
            hide: "Скрыть ApexDisk",
            hide_others: "Скрыть остальные",
            show_all: "Показать все",
            quit: "Выйти из ApexDisk",
        },
        "zh" => MenuLabels {
            release_notes: "发行说明",
            license: "许可证",
            check_for_updates: "检查更新…",
            checking_for_updates: "正在检查更新…",
            downloading_update: "正在下载更新…",
            restart_to_update: "重启以更新",
            website_label: "发行说明",
            window: "窗口",
            minimize: "最小化",
            close_window: "关闭窗口",
            help: "帮助",
            about: "关于 ApexDisk",
            services: "服务",
            hide: "隐藏 ApexDisk",
            hide_others: "隐藏其他",
            show_all: "显示全部",
            quit: "退出 ApexDisk",
        },
        "ja" => MenuLabels {
            release_notes: "リリースノート",
            license: "ライセンス",
            check_for_updates: "アップデートを確認…",
            checking_for_updates: "アップデートを確認中…",
            downloading_update: "アップデートをダウンロード中…",
            restart_to_update: "再起動して更新",
            website_label: "リリースノート",
            window: "ウインドウ",
            minimize: "最小化",
            close_window: "ウィンドウを閉じる",
            help: "ヘルプ",
            about: "ApexDisk について",
            services: "サービス",
            hide: "ApexDisk を隠す",
            hide_others: "ほかを隠す",
            show_all: "すべてを表示",
            quit: "ApexDisk を終了",
        },
        "ar" => MenuLabels {
            release_notes: "ملاحظات الإصدار",
            license: "الرخصة",
            check_for_updates: "التحقق من التحديثات…",
            checking_for_updates: "جارٍ التحقق من التحديثات…",
            downloading_update: "جارٍ تنزيل التحديث…",
            restart_to_update: "إعادة التشغيل للتحديث",
            website_label: "ملاحظات الإصدار",
            window: "نافذة",
            minimize: "تصغير",
            close_window: "إغلاق النافذة",
            help: "المساعدة",
            about: "حول ApexDisk",
            services: "الخدمات",
            hide: "إخفاء ApexDisk",
            hide_others: "إخفاء الآخرين",
            show_all: "إظهار الكل",
            quit: "إنهاء ApexDisk",
        },
        // English is the default
        _ => MenuLabels {
            release_notes: "Release Notes",
            license: "License",
            check_for_updates: "Check for Updates…",
            checking_for_updates: "Checking for Updates…",
            downloading_update: "Downloading Update…",
            restart_to_update: "Restart to Update",
            website_label: "Release Notes",
            window: "Window",
            minimize: "Minimize",
            close_window: "Close Window",
            help: "Help",
            about: "About ApexDisk",
            services: "Services",
            hide: "Hide ApexDisk",
            hide_others: "Hide Others",
            show_all: "Show All",
            quit: "Quit ApexDisk",
        },
    }
}
