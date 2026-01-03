//! Section layout and height calculation
//!
//! This module provides dynamic height calculation for sections based on their content.
//! Each section defines min/max heights for its parts (note, value, list, actions).

use crate::app::App;
use crate::section::{SectionId, SectionTrait};

/// Height requirements for a section's parts
#[derive(Debug, Clone, Copy)]
pub struct SectionHeights {
    /// Minimum height for note/hint area (lines)
    pub note_min: u16,
    /// Maximum height for note/hint area (lines)
    pub note_max: u16,
    /// Minimum height for value/content area (lines)
    pub content_min: u16,
    /// Maximum height for value/content area (lines)
    pub content_max: u16,
    /// Minimum height for actions area (lines)
    pub actions_min: u16,
    /// Maximum height for actions area (lines)
    pub actions_max: u16,
    /// Border overhead (top + bottom borders)
    pub border_overhead: u16,
}

impl SectionHeights {
    /// Calculate minimum total height for the section
    /// Includes: border + blank + note + blank + value + blank (if actions) + actions
    pub fn min_total(&self) -> u16 {
        // Border overhead (2) + blank (1) + note + blank (1) + content + blank (1 if actions) + actions
        // Sections with actions: border_overhead + 3 blanks + note_min + content_min + actions_min
        // Sections without actions: border_overhead + 2 blanks + note_min + content_min
        let blank_count = if self.actions_min > 0 { 3 } else { 2 };
        self.border_overhead + blank_count + self.note_min + self.content_min + self.actions_min
    }
    
    /// Calculate maximum total height for the section
    /// Includes: border + blank + note + blank + value + blank (if actions) + actions
    pub fn max_total(&self) -> u16 {
        // Border overhead (2) + blank (1) + note + blank (1) + content + blank (1 if actions) + actions
        // Sections with actions: border_overhead + 3 blanks + note_max + content_max + actions_max
        // Sections without actions: border_overhead + 2 blanks + note_max + content_max
        let blank_count = if self.actions_max > 0 { 3 } else { 2 };
        self.border_overhead + blank_count + self.note_max + self.content_max + self.actions_max
    }
}

/// Calculate dynamic height for a section based on current state
/// Takes the section itself to get heights from its own definition
pub fn calculate_section_height(section: &crate::section::Section, app: &App, available_height: u16) -> u16 {
    let heights = section.heights();
    let min_total = heights.min_total();
    let max_total = heights.max_total();
    
    // Calculate content-based height
    let section_id = section.id();
    let content_height = match section_id {
        SectionId::PreviewPane => {
            // Height based on number of files (min 3, max 20)
            // If more files exist, section will be at max height and scrollable
            // Layout: border + blank + note + blank + table (no actions, so no blank before actions)
            let file_count = app.state.list.len().min(app.state.new_names.len());
            let table_height = (file_count as u16 + 1).max(4).min(heights.content_max + 1); // +1 for header
            // border (2) + blank (1) + note (1) + blank (1) + table (no blank before actions)
            heights.border_overhead + 2 + heights.note_min + table_height
        }
        SectionId::Validation => {
            // Height based on number of validation issues (min 3, max 15)
            // If more issues exist, section will be at max height and scrollable
            // Layout: border + blank + note + blank + table (no actions, so no blank before actions)
            let issue_count = app.state.validation
                .as_ref()
                .map(|v| v.issues.len())
                .unwrap_or(0);
            let table_height = if issue_count == 0 {
                4 // Show "All valid" message + header
            } else {
                (issue_count as u16 + 1).min(heights.content_max + 1).max(4) // +1 for header
            };
            // border (2) + blank (1) + note (1) + blank (1) + table (no blank before actions)
            heights.border_overhead + 2 + heights.note_min + table_height
        }
        SectionId::Exclusions => {
            // Height based on number of exclusions
            // Layout: border + blank + note + blank + list + blank + actions
            let exclusion_count = app.state.exclude.len();
            let list_height = if exclusion_count == 0 || exclusion_count == 1 {
                1 // Show "None" message or single item as Paragraph
            } else {
                // Multiple items - use Fill, so height should be at least content_min for scrolling
                heights.content_min
            };
            // border (2) + blank (1) + note (1) + blank (1) + list + blank (1) + actions (1)
            heights.border_overhead + 3 + heights.note_min + list_height + heights.actions_min
        }
        SectionId::WorkingDirectory => {
            // Height based on number of directories
            // Layout: border + blank + note + blank + list + blank + actions
            let directory_count = app.state.workdirs.len();
            let list_height = if directory_count == 0 || directory_count == 1 {
                1 // Show "None" message or single item as Paragraph
            } else {
                // Multiple items - use Fill, so height should be at least content_min for scrolling
                heights.content_min
            };
            // border (2) + blank (1) + note (1) + blank (1) + list + blank (1) + actions (1)
            heights.border_overhead + 3 + heights.note_min + list_height + heights.actions_min
        }
        SectionId::MatchFiles => {
            // Height based on pattern + number of specific files
            // Layout: border + blank + note + blank + list + blank + actions
            let pattern_count = if app.state.match_pattern.is_empty() { 0 } else { 1 };
            let file_count = app.state.match_files.len();
            let item_count = pattern_count + file_count;
            let list_height = if item_count == 0 || item_count == 1 {
                1 // Show "None" message or single item as Paragraph
            } else {
                // Multiple items - use Fill, so height should be at least content_min for scrolling
                heights.content_min
            };
            // border (2) + blank (1) + note (1) + blank (1) + list + blank (1) + actions (1)
            heights.border_overhead + 3 + heights.note_min + list_height + heights.actions_min
        }
        _ => {
            // Fixed height for other sections
            // Layout: border + blank + note + blank + value + blank + actions
            min_total
        }
    };
    
    // Clamp to max and respect available height
    // For dynamic sections, we don't enforce min_total - the content-based calculation is accurate
    // Only use min_total as a fallback for fixed-height sections (which use min_total as content_height)
    let final_height = if content_height == min_total {
        // Fixed-height section - use min_total
        min_total
    } else {
        // Dynamic section - use content-based calculation (may be less than min_total)
        content_height
    };
    
    final_height
        .min(max_total)
        .min(available_height)
}


