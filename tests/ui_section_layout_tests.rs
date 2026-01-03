//! Tests for the Section Layout module

use frentui::app::App;
use frentui::section::{SectionId, SectionTrait};
use frentui::ui::section_layout;

#[test]
fn test_section_heights_defined() {
    // Test that all sections have height definitions
    let app = App::new();
    
    for section in &app.sections {
        let heights = section.heights();
        
        // Min should be <= max
        assert!(heights.min_total() <= heights.max_total());
        
        // All parts should have valid min/max
        assert!(heights.note_min <= heights.note_max);
        assert!(heights.content_min <= heights.content_max);
        assert!(heights.actions_min <= heights.actions_max);
    }
}

#[test]
fn test_section_min_height() {
    // Test that min heights are reasonable
    let app = App::new();
    
    let preview_section = app.sections.iter().find(|s| s.id() == SectionId::PreviewPane).unwrap();
    let validation_section = app.sections.iter().find(|s| s.id() == SectionId::Validation).unwrap();
    let working_dir_section = app.sections.iter().find(|s| s.id() == SectionId::WorkingDirectory).unwrap();
    
    let preview_min = preview_section.heights().min_total();
    let validation_min = validation_section.heights().min_total();
    let working_dir_min = working_dir_section.heights().min_total();
    
    // Preview and validation should have larger min heights
    assert!(preview_min >= 3);
    assert!(validation_min >= 3);
    assert!(working_dir_min >= 3);
}

#[test]
fn test_calculate_section_height_preview() {
    let app = App::new();
    
    // Test with empty state
    let preview_section = app.sections.iter().find(|s| s.id() == SectionId::PreviewPane).unwrap();
    let height_empty = section_layout::calculate_section_height(
        preview_section,
        &app,
        100
    );
    assert!(height_empty >= 3);
    
    // Test with files (simulate by setting state)
    // Note: We can't easily set state.list without making it public or adding test helpers
    // This test verifies the function exists and works with default state
    assert!(height_empty > 0);
}

#[test]
fn test_calculate_section_height_validation() {
    let app = App::new();
    
    // Test with no validation
    let validation_section = app.sections.iter().find(|s| s.id() == SectionId::Validation).unwrap();
    let height_no_validation = section_layout::calculate_section_height(
        validation_section,
        &app,
        100
    );
    assert!(height_no_validation >= 3);
}

#[test]
fn test_calculate_section_height_fixed_sections() {
    let app = App::new();
    
    // WorkingDirectory is now dynamic-height (like Exclusions), not fixed
    // Test that it calculates height based on number of directories
    let working_dir_section = app.sections.iter().find(|s| s.id() == SectionId::WorkingDirectory).unwrap();
    let working_dir_height = section_layout::calculate_section_height(
        working_dir_section,
        &app,
        100
    );
    
    // WorkingDirectory should have dynamic height based on content
    // With default state (1 workdir): border (2) + blank (1) + note (1) + blank (1) + list (1) + blank (1) + actions (1) = 8
    // With multiple workdirs: border (2) + blank (1) + note (1) + blank (1) + list (3) + blank (1) + actions (1) = 10
    assert!(working_dir_height >= 8);
    assert!(working_dir_height <= 10); // Should be 8 with 1 item, 10 with multiple
    
    // MatchFiles is now also dynamic-height (like Exclusions and WorkingDirectory)
    let match_files_section = app.sections.iter().find(|s| s.id() == SectionId::MatchFiles).unwrap();
    let match_files_height = section_layout::calculate_section_height(
        match_files_section,
        &app,
        100
    );
    
    // MatchFiles should have dynamic height based on content
    // With default state (empty): border (2) + blank (1) + note (1) + blank (1) + list (1) + blank (1) + actions (1) = 8
    // With multiple items: border (2) + blank (1) + note (1) + blank (1) + list (3) + blank (1) + actions (1) = 10
    assert!(match_files_height >= 8);
    assert!(match_files_height <= 10); // Should be 8 with empty/1 item, 10 with multiple
    
    // Both should be dynamic-height sections
    // They may differ if one has 1 item and the other is empty, but both should be 8
    assert!(working_dir_height == 8 || working_dir_height == 10);
    assert!(match_files_height == 8 || match_files_height == 10);
}

#[test]
fn test_section_heights_respect_available_height() {
    let app = App::new();
    
    // Test that calculated height respects available height
    let preview_section = app.sections.iter().find(|s| s.id() == SectionId::PreviewPane).unwrap();
    let available = 10;
    let height = section_layout::calculate_section_height(
        preview_section,
        &app,
        available
    );
    
    assert!(height <= available);
}

