//! Tests for `disk::parse_volume_name`.
//!
//! The volume name is read from `diskutil info` stdout. We test the parser with
//! realistic samples: "Volume Name: ..." lines, the " - Data" suffix stripped for
//! display, and edge cases (empty, no match, first match wins). No subprocess or
//! filesystem is used.

use mac_disk_tree_lib::disk::parse_volume_name;

/// A line "Volume Name: Macintosh HD - Data" must yield "Macintosh HD" (suffix stripped).
#[test]
fn parse_volume_name_data_suffix_stripped() {
    let stdout = r#"
   Device Node:               /dev/disk3s1
   Volume Name:               Macintosh HD - Data
   "#;
    assert_eq!(parse_volume_name(stdout).as_deref(), Some("Macintosh HD"));
}

/// A line "Volume Name: My Volume" with no suffix must yield "My Volume" unchanged.
#[test]
fn parse_volume_name_no_data_suffix() {
    let stdout = r#"
   Volume Name: My Volume
   "#;
    assert_eq!(parse_volume_name(stdout).as_deref(), Some("My Volume"));
}

/// Leading/trailing whitespace around the volume name must be trimmed.
#[test]
fn parse_volume_name_trimmed() {
    let stdout = "   Volume Name:    Startup Disk   \n";
    assert_eq!(parse_volume_name(stdout).as_deref(), Some("Startup Disk"));
}

/// Empty or whitespace-only stdout must return None (no volume name line).
#[test]
fn parse_volume_name_empty_returns_none() {
    assert_eq!(parse_volume_name(""), None);
    assert_eq!(parse_volume_name("   \n  \n"), None);
}

/// Output without a "Volume Name:" line (e.g. only Device Node, Mounted) must return None.
#[test]
fn parse_volume_name_no_volume_name_line_returns_none() {
    let stdout = "   Device Node: /dev/disk1\n   Mounted: Yes\n";
    assert_eq!(parse_volume_name(stdout), None);
}

/// If stdout contains multiple "Volume Name:" lines, the first one is used.
#[test]
fn parse_volume_name_first_match_wins() {
    let stdout = r#"
   Volume Name: First
   Volume Name: Second
   "#;
    assert_eq!(parse_volume_name(stdout).as_deref(), Some("First"));
}
