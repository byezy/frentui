//! User-facing strings for frentui
//!
//! All user-facing text should be defined here, organized by section/component.
//! Only use string literals for logging/internal purposes.

/// Header section strings
pub mod header {
    pub const TITLE: &str = "frentui";
    pub const DESCRIPTION: &str = "Interactive file renaming tool";
}

/// Footer section strings
pub mod footer {
    pub const NAVIGATION: &str = "↑↓ navigate sections | Enter execute action | ←→ navigate actions";
    pub const SCROLLING_HELP: &str = "↑↓ scroll preview/validation | PageUp/PageDown scroll faster | Home/End jump to start/end";
    pub const QUIT: &str = "'q' quit";
}

/// Working Directory section strings
pub mod working_directory {
    pub const TITLE: &str = "Working directory";
    pub const HINT: &str = "Directories to search for files. Defaults to current working directory.";
    
    pub mod actions {
        pub const ADD_DIRECTORY: &str = "Add directory";
        pub const ADD_CURRENT_WORKING_DIRECTORY: &str = "Add current working directory";
        pub const REMOVE_DIRECTORY: &str = "Remove directory";
    }
    
    pub mod dialog {
        pub const TITLE: &str = "Enter directory path";
        pub const REMOVE_DIRECTORY_TITLE: &str = "Select directories to remove";
    }
}

/// Match Files section strings
pub mod match_files {
    pub const TITLE: &str = "Match Files";
    pub const HINT: &str = "Pattern to match files (e.g., *.txt, *.jpg). Defaults to *.* (all files).";
    
    pub mod actions {
        pub const ENTER_CUSTOM_PATTERN: &str = "Enter custom pattern";
        pub const CHOOSE_TEMPLATE: &str = "Choose from template";
        pub const ADD_FILE: &str = "Add file";
        pub const REMOVE_FILE: &str = "Remove file";
        pub const CLEAR: &str = "Clear";
    }
    
    pub mod dialog {
        pub const TITLE: &str = "Enter file pattern (e.g., *.txt)";
        pub const ADD_FILE_TITLE: &str = "Enter file path";
        pub const REMOVE_FILE_TITLE: &str = "Select files to remove";
    }
}

/// Exclusions section strings
pub mod exclusions {
    pub const TITLE: &str = "Exclude";
    pub const HINT: &str = "Files or patterns to exclude from the file list.";
    pub const NONE: &str = "None";
    
    pub mod actions {
        pub const ENTER_EXCLUSION_PATTERN: &str = "Enter exclusion pattern";
        pub const CHOOSE_TEMPLATE: &str = "Choose from template";
        pub const SPECIFY_FILES: &str = "Specify files";
        pub const CLEAR: &str = "Clear";
    }
    
    pub mod dialog {
        pub const EXCLUSION_PATTERN_TITLE: &str = "Enter exclusion pattern";
        pub const FILE_SELECTION_TITLE: &str = "Select files to exclude";
    }
}

/// Renaming Rule section strings
pub mod renaming_rule {
    pub const TITLE: &str = "Rename Rule";
    pub const HINT: &str = "Freneng DSL pattern for renaming (e.g., %N_v2.%E, %L%N.%E). Defaults to %N.%E (no change).";
    
    pub mod actions {
        pub const ENTER_CUSTOM_RULE: &str = "Enter custom rule";
        pub const CHOOSE_TEMPLATE: &str = "Choose from template";
    }
    
    pub mod dialog {
        pub const TITLE: &str = "Enter renaming rule (freneng DSL)";
    }
}

/// Preview Pane section strings
pub mod preview {
    pub const TITLE: &str = "Preview";
    pub const HINT: &str = "Shows the mapping between current file names and new file names.";
    pub const HINT_AFTER_RENAME: &str = "⚠ Preview is invalid: files have been renamed. Current names shown may not exist.";
    pub const EMPTY: &str = "No files to preview";
    // pub const ARROW: &str = " → ";
}

/// Validation section strings
pub mod validation {
    pub const TITLE: &str = "Validation";
    pub const PENDING: &str = "Validation pending...";
    pub const NOT_VALIDATED: &str = "Not validated yet";
    pub const ALL_VALID: &str = "✓ All valid";
    pub const ALL_VALID_MESSAGE: &str = "All renames are valid and ready to apply.";
    // Note: ISSUES_FOUND uses format! with {} placeholder, so we keep the format string pattern
    // Format: "Found {} issue(s). Review and fix before applying."
}

/// Apply Renaming section strings
pub mod apply {
    pub const TITLE: &str = "Apply";
    pub const HINT: &str = "Apply the renaming operations to the files.";
    pub const STATUS_READY: &str = "Ready to apply";
    pub const STATUS_VALIDATION_REQUIRED: &str = "Validation required";
    
    pub mod actions {
        pub const APPLY_RENAMING: &str = "Apply renaming";
    }
}

/// Undo section strings
pub mod undo {
    pub const TITLE: &str = "Undo Changes";
    pub const HINT: &str = "Undo the last renaming operation.";
    pub const STATUS_AVAILABLE: &str = "Undo available";
    pub const STATUS_NOT_AVAILABLE: &str = "No undo available";
    
    pub mod actions {
        pub const UNDO_RENAMING: &str = "Undo renaming";
    }
}

/// Dialog strings
pub mod dialog {
    pub mod hints {
        pub const DIRECTORY_SELECTION: &str = "Enter the directory path to search for files.";
        pub const MATCH_PATTERN_INPUT: &str = "Enter a file pattern (e.g., *.txt, *.jpg).";
        pub const EXCLUSION_PATTERN_INPUT: &str = "Enter a pattern or file path to exclude.";
        pub const RENAMING_RULE_INPUT: &str = "Enter a renaming rule using freneng DSL (e.g., %N_v2.%E).";
        pub const GENERIC: &str = "Enter value:";
    }
    
    pub mod actions {
        pub const SAVE: &str = "Enter";
        pub const SAVE_LABEL: &str = " to save";
        pub const CANCEL: &str = "Esc";
        pub const CANCEL_LABEL: &str = " to cancel";
        pub const SEPARATOR: &str = " | ";
    }
    
    pub mod template_selection {
        pub const TITLE: &str = "Select template";
    }
}

