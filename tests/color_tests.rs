//! Tests for the Color module

use frentui::color::SectionColors;
use ratatui::style::Color;

#[test]
fn test_section_colors_defined() {
    // Test that all color constants are defined and accessible
    let _ = SectionColors::HINT;
    let _ = SectionColors::VALUE;
    let _ = SectionColors::BORDER;
    let _ = SectionColors::HIGHLIGHT;
    let _ = SectionColors::FOCUSED_BORDER;
    let _ = SectionColors::ACTION;
    let _ = SectionColors::DISABLED;
    let _ = SectionColors::ERROR;
    let _ = SectionColors::SUCCESS;
    let _ = SectionColors::FOOTER;
    let _ = SectionColors::BRANDING;
}

#[test]
fn test_section_colors_values() {
    // Test that colors have expected values
    assert_eq!(SectionColors::VALUE, Color::Yellow);
    assert_eq!(SectionColors::BORDER, Color::Cyan);
    assert_eq!(SectionColors::HIGHLIGHT, Color::Magenta);
    assert_eq!(SectionColors::FOCUSED_BORDER, Color::Yellow);
    assert_eq!(SectionColors::ACTION, Color::White);
    assert_eq!(SectionColors::ERROR, Color::Red);
    assert_eq!(SectionColors::SUCCESS, Color::Green);
    assert_eq!(SectionColors::BRANDING, Color::Yellow);
}

#[test]
fn test_section_colors_hint_and_disabled_same() {
    // Hint and disabled should use the same color for consistency
    assert_eq!(SectionColors::HINT, SectionColors::DISABLED);
}

