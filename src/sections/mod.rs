//! Section implementations

pub mod header;
pub mod working_directory;
pub mod match_files;
pub mod exclusions;
pub mod renaming_rule;
pub mod preview_pane;
pub mod validation;
pub mod apply;
pub mod undo;
pub mod footer;

use crate::section::Section;

pub fn create_all_sections() -> Vec<Section> {
    vec![
        header::create_header_section(),
        working_directory::create_working_directory_section(),
        match_files::create_match_files_section(),
        exclusions::create_exclusions_section(),
        renaming_rule::create_renaming_rule_section(),
        preview_pane::create_preview_pane_section(),
        validation::create_validation_section(),
        apply::create_apply_section(),
        undo::create_undo_section(),
        footer::create_footer_section(),
    ]
}

