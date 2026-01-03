//! Dialog types and utilities

use crate::ui::input::Input;

/// Type of dialog currently open
#[derive(Debug, Clone)]
pub enum DialogType {
    /// Directory selection dialog
    DirectorySelection,
    /// Pattern input dialog (for match files)
    MatchPatternInput,
    /// Exclusion pattern input dialog
    ExclusionPatternInput,
    /// Renaming rule input dialog
    RenamingRuleInput,
    /// Template selection dialog
    TemplateSelection {
        /// Field to apply template to
        field: TemplateField,
    },
    /// File selection dialog (for exclusions)
    FileSelection,
    /// Directory selection dialog (for removal)
    DirectorySelectionForRemoval,
    /// File path input dialog (for adding specific files to match)
    MatchFileInput,
    /// File selection dialog (for removing specific files from match)
    MatchFileSelection,
}

/// Field that a template can be applied to
#[derive(Debug, Clone)]
pub enum TemplateField {
    MatchPattern,
    ExclusionPattern,
    RenamingRule,
}

/// Dialog state
pub struct Dialog {
    pub dialog_type: DialogType,
    pub input: Input,
    /// For file selection: selected file indices (files marked with 'x')
    pub selected_files: Vec<usize>,
    /// For file selection: currently highlighted file index
    pub selected_file_index: Option<usize>,
    /// For template selection: selected template index
    pub selected_template: Option<usize>,
}

impl Dialog {
    pub fn new(dialog_type: DialogType, title: impl Into<String>) -> Self {
        let selected_template = if matches!(dialog_type, DialogType::TemplateSelection { .. }) {
            Some(0) // Start with first template selected
        } else {
            None
        };
        let selected_file_index = if matches!(dialog_type, DialogType::FileSelection | DialogType::DirectorySelectionForRemoval | DialogType::MatchFileSelection) {
            Some(0) // Start with first item selected
        } else {
            None
        };
        Self {
            dialog_type,
            input: Input::new(title),
            selected_files: Vec::new(),
            selected_file_index,
            selected_template,
        }
    }
    
    pub fn with_value(dialog_type: DialogType, title: impl Into<String>, value: String) -> Self {
        Self {
            dialog_type,
            input: Input::with_value(title, value),
            selected_files: Vec::new(),
            selected_file_index: None,
            selected_template: None,
        }
    }
}

