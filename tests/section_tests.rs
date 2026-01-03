//! Tests for the Section module

use frentui::section::{SectionId, SectionTrait};
use frentui::app::App;

#[test]
fn test_section_id_all() {
    let all = SectionId::all();
    assert!(!all.is_empty());
    assert_eq!(all[0], SectionId::Header);
    assert_eq!(all[all.len() - 1], SectionId::Footer);
}

#[test]
fn test_section_id_next() {
    assert_eq!(SectionId::Header.next(), Some(SectionId::WorkingDirectory));
    assert_eq!(SectionId::WorkingDirectory.next(), Some(SectionId::MatchFiles));
    
    // Footer should have no next
    assert_eq!(SectionId::Footer.next(), None);
}

#[test]
fn test_section_id_previous() {
    assert_eq!(SectionId::WorkingDirectory.previous(), Some(SectionId::Header));
    assert_eq!(SectionId::MatchFiles.previous(), Some(SectionId::WorkingDirectory));
    
    // Header should have no previous
    assert_eq!(SectionId::Header.previous(), None);
}

#[test]
fn test_section_id_equality() {
    assert_eq!(SectionId::Header, SectionId::Header);
    assert_ne!(SectionId::Header, SectionId::WorkingDirectory);
}

#[test]
fn test_section_trait_methods() {
    use frentui::section::SectionTrait;
    let app = App::new();
    
    // Find a section and test its methods
    if let Some(section) = app.sections.iter().find(|s| s.id() == SectionId::Header) {
        assert_eq!(section.id(), SectionId::Header);
        assert!(!section.title().is_empty());
        
        // let hint = section.hint(&app);
        // let value = section.value(&app);
        let actions = section.actions(&app);
        
        // Header should have no actions
        assert_eq!(actions.len(), 0);
    } else {
        panic!("Header section should exist");
    }
}

#[test]
fn test_section_working_directory_has_actions() {
    let app = App::new();
    
    if let Some(section) = app.sections.iter().find(|s| s.id() == SectionId::WorkingDirectory) {
        let actions = section.actions(&app);
        // Working directory should have actions
        assert!(!actions.is_empty());
    } else {
        panic!("WorkingDirectory section should exist");
    }
}

