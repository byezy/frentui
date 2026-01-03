//! List pattern selection step

use crate::app::App;
use crate::steps::common::{handle_step_input};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, PreviewDefinition, StepDefinition};
use crate::ui::input::Input;
use crossterm::event::{KeyCode, KeyEvent};

/// Validate if a string is a valid glob pattern
/// Basic validation: checks for common invalid characters and structure
pub(crate) fn is_valid_glob_pattern(pattern: &str) -> bool {
    if pattern.is_empty() {
        return false;
    }
    
    // Basic validation: glob patterns should contain at least one wildcard or be a valid file extension pattern
    // Allow: *, ?, [], {}, and normal characters
    // Reject: patterns with only special regex characters that aren't glob
    // This is a simple check - actual validation happens when trying to use the pattern
    
    // Check for obviously invalid patterns (e.g., only special characters)
    if pattern.chars().all(|c| !c.is_alphanumeric() && c != '.' && c != '*' && c != '?' && c != '[' && c != ']') {
        return false;
    }
    
    // Basic sanity: if it contains * or ?, it's likely a glob pattern
    // If it's just a file extension like ".txt", that's also valid
    // If it contains only alphanumeric and dots, that's valid (e.g., "*.txt")
    true
}

/// Get the step definition for list pattern selection
pub fn get_list_pattern_step() -> StepDefinition {
    use crate::strings::list_pattern;
    
    let menu = MenuDefinition::new(
        list_pattern::MENU_TITLE,
        vec![
            MenuOption::new(list_pattern::MENU_CUSTOM_PATTERN, |app| {
                // Open custom pattern input dialog
                let current_pattern = app.state.list_patterns.first()
                    .cloned()
                    .unwrap_or_else(|| "*.txt".to_string());
                log::info!("Opening custom pattern input dialog with pattern: {}", current_pattern);
                app.input_dialog = Some(Input::with_value("Enter File Pattern", current_pattern));
                MenuAction::None
            }),
            MenuOption::new(list_pattern::MENU_COMMON_PATTERNS, |app| {
                log::warn!("TODO: Show common patterns menu - using default for now");
                // TODO: Show common patterns menu
                app.state.list_patterns = vec!["*.*".to_string()];
                log::info!("Set pattern to: {:?}", app.state.list_patterns);
                // Reload files with new pattern - header and file list will update on next render
                // Don't proceed automatically - let user see the updated file list
                MenuAction::None
            }),
            MenuOption::new(list_pattern::MENU_FILES_FROM, |_app| {
                log::warn!("TODO: Show file input dialog - proceeding without it");
                // TODO: Show file input dialog
                MenuAction::Next
            }),
        ],
        |app| !app.state.list_patterns.is_empty(), // Can move forward if pattern is set
    );
    
    StepDefinition::new(list_pattern::HEADER_TITLE, menu)
        .with_title_hint(|app| {
            app.state.list_patterns.first()
                .map(|p| p.clone())
                .or_else(|| Some("No pattern selected".to_string()))
        })
        .with_preview(
            PreviewDefinition::files_list(|app| app.state.selected_files.clone())
        )
}

pub fn handle_list_pattern_input(app: &mut App, key: KeyEvent) -> bool {
    log::debug!("handle_list_pattern_input: key={:?}, menu_selection={:?}, input_dialog={:?}", 
                key.code, app.menu_selection, app.input_dialog.is_some());
    
    // If input dialog is active, handle input
    if let Some(ref mut input) = app.input_dialog {
        log::debug!("Input dialog is active, handling key in dialog");
        match key.code {
            KeyCode::Enter => {
                // Confirm: set the pattern
                let pattern = input.value().trim().to_string();
                log::info!("List pattern input confirmed: '{}'", pattern);
                if !pattern.is_empty() {
                    // Validate pattern by trying to compile it as a glob pattern
                    if is_valid_glob_pattern(&pattern) {
                        log::info!("Setting list pattern to: {}", pattern);
                        app.state.list_patterns = vec![pattern];
                        app.input_dialog = None; // Close dialog
                        // Reload files with new pattern
                        // Note: This will happen on next render/key input
                    } else {
                        log::warn!("Invalid pattern: {}", pattern);
                        // Keep dialog open, show error (TODO: show error message in UI)
                        // For now, just don't accept the pattern
                    }
                } else {
                    log::warn!("Empty pattern input, not setting pattern");
                    app.input_dialog = None; // Close dialog
                }
                true
            }
            KeyCode::Esc => {
                // Cancel: close dialog
                log::info!("Cancelling list pattern input");
                app.input_dialog = None;
                true
            }
            _ => {
                // Pass key to input handler
                input.handle_key(key)
            }
        }
    } else {
        // Use common input handling for menu
        let definition = get_list_pattern_step();
        handle_step_input(&definition, app, key)
    }
}
