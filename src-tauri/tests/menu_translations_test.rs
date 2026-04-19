//! Tests for `menu_translations::labels_for`.
//!
//! Verifies that all supported languages return non-empty labels for every
//! `MenuLabels` field (including the update-flow labels), that unknown language
//! codes fall back to English, and that each non-English language differs from
//! English for at least one label (so translations aren't silent copies).

use apex_disk_lib::menu_translations;

const SUPPORTED_LANGUAGES: &[&str] = &["en", "it", "es", "fr", "pt", "de", "ru", "zh", "ja", "ar"];

/// Every supported language must return non-empty strings for all label fields.
#[test]
fn all_supported_languages_have_non_empty_labels() {
    for lang in SUPPORTED_LANGUAGES {
        let labels = menu_translations::labels_for(lang);
        assert!(
            !labels.release_notes.is_empty(),
            "{lang}: release_notes empty"
        );
        assert!(!labels.license.is_empty(), "{lang}: license empty");
        assert!(
            !labels.website_label.is_empty(),
            "{lang}: website_label empty"
        );
        assert!(!labels.window.is_empty(), "{lang}: window empty");
        assert!(!labels.minimize.is_empty(), "{lang}: minimize empty");
        assert!(
            !labels.close_window.is_empty(),
            "{lang}: close_window empty"
        );
        assert!(!labels.help.is_empty(), "{lang}: help empty");
        assert!(!labels.about.is_empty(), "{lang}: about empty");
        assert!(!labels.services.is_empty(), "{lang}: services empty");
        assert!(!labels.hide.is_empty(), "{lang}: hide empty");
        assert!(!labels.hide_others.is_empty(), "{lang}: hide_others empty");
        assert!(!labels.show_all.is_empty(), "{lang}: show_all empty");
        assert!(!labels.quit.is_empty(), "{lang}: quit empty");
        assert!(
            !labels.check_for_updates.is_empty(),
            "{lang}: check_for_updates empty"
        );
        assert!(
            !labels.checking_for_updates.is_empty(),
            "{lang}: checking_for_updates empty"
        );
        assert!(
            !labels.downloading_update.is_empty(),
            "{lang}: downloading_update empty"
        );
        assert!(
            !labels.restart_to_update.is_empty(),
            "{lang}: restart_to_update empty"
        );
    }
}

/// Unknown language codes must fall back to English.
#[test]
fn unknown_language_falls_back_to_english() {
    let en = menu_translations::labels_for("en");
    let unknown = menu_translations::labels_for("xx");

    assert_eq!(en.release_notes, unknown.release_notes);
    assert_eq!(en.license, unknown.license);
    assert_eq!(en.about, unknown.about);
    assert_eq!(en.quit, unknown.quit);
    assert_eq!(en.window, unknown.window);
    assert_eq!(en.help, unknown.help);
}

/// Each non-English language must differ from English for at least one label,
/// ensuring that translations are not just copies of English.
#[test]
fn non_english_languages_differ_from_english() {
    let en = menu_translations::labels_for("en");

    for lang in &SUPPORTED_LANGUAGES[1..] {
        let labels = menu_translations::labels_for(lang);
        let any_different = labels.release_notes != en.release_notes
            || labels.license != en.license
            || labels.about != en.about
            || labels.quit != en.quit
            || labels.window != en.window
            || labels.help != en.help;

        assert!(
            any_different,
            "{lang} labels are identical to English — translations may be missing"
        );
    }
}
