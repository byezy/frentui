//! Directory selection step

use crate::app::App;
use crate::steps::common::{handle_step_input};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, StepDefinition};
use crate::ui::input::Input;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::{Path, PathBuf};

/// Get the step definition for directory selection
pub fn get_directory_step() -> StepDefinition {
    use crate::strings::directory as strings;
    
    let menu = MenuDefinition::new(
        strings::MENU_TITLE,
        vec![
            MenuOption::new(strings::MENU_USE_CURRENT, |app| {
                // Use current directory - set to actual CWD but stay on this step
                if let Ok(cwd) = std::env::current_dir() {
                    app.state.working_directory = cwd;
                }
                MenuAction::None
            }),
            MenuOption::new(strings::MENU_CHANGE, |app| {
                // Open directory input dialog
                let current_dir = app.state.working_directory.to_string_lossy().to_string();
                log::info!("Opening directory input dialog with current dir: {}", current_dir);
                app.input_dialog = Some(Input::with_value("Enter Directory Path", current_dir));
                MenuAction::None
            }),
        ],
        |_app| true, // Can always move forward from directory step
    );
    
    StepDefinition::new(strings::HEADER_TITLE, menu)
        .with_title_hint(|app| {
            Some(app.state.working_directory.to_string_lossy().to_string())
        })
        .with_hint(strings::NOTE)
}

pub fn handle_directory_input(app: &mut App, key: KeyEvent) -> bool {
    log::debug!("handle_directory_input: key={:?}, input_dialog={:?}, menu_selection={:?}", 
                key.code, app.input_dialog.is_some(), app.menu_selection);
    
    // If input dialog is active, handle input
    if let Some(ref mut input) = app.input_dialog {
        log::debug!("Input dialog is active, handling key in dialog");
        match key.code {
            KeyCode::Enter => {
                // Confirm: set the directory
                let path_str = input.value().trim();
                log::info!("Directory input confirmed: '{}'", path_str);
                if !path_str.is_empty() {
                    let path = PathBuf::from(path_str);
                    if path.exists() && path.is_dir() {
                        log::info!("Setting working directory to: {:?}", path);
                        app.state.working_directory = path;
                        app.input_dialog = None; // Close dialog
                    } else {
                        log::warn!("Invalid directory path: {:?} (doesn't exist or not a directory)", path);
                        // Invalid path - could show error, but for now just close
                        app.input_dialog = None;
                    }
                } else {
                    log::warn!("Empty directory input, closing dialog");
                    // Empty input - close dialog
                    app.input_dialog = None;
                }
                true
            }
            KeyCode::Tab => {
                // Tab completion
                let current_path = input.get_path_for_completion();
                if let Some(completed) = complete_path(&current_path) {
                    input.set_completed_path(completed);
                }
                true
            }
            KeyCode::Esc => {
                // Cancel: close dialog
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
        let definition = get_directory_step();
        handle_step_input(&definition, app, key)
    }
}

/// Complete a path using tab completion
fn complete_path(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    
    let path = Path::new(input);
    
    // If input ends with a separator, we're looking for items in that directory
    let (base_dir, prefix) = if input.ends_with('/') || input.ends_with('\\') {
        (path, "")
    } else {
        (path.parent().unwrap_or(Path::new(".")), 
         path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
    };
    
    // Try to read the directory
    let entries = match std::fs::read_dir(base_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };
    
    // Find matching entries
    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with(prefix) {
                let full_path = if base_dir == Path::new(".") {
                    name.to_string()
                } else {
                    base_dir.join(name).to_string_lossy().to_string()
                };
                
                // Add separator if it's a directory
                let metadata = entry.metadata().ok();
                if metadata.map(|m| m.is_dir()).unwrap_or(false) {
                    matches.push(format!("{}/", full_path));
                } else {
                    matches.push(full_path);
                }
            }
        }
    }
    
    if matches.is_empty() {
        None
    } else if matches.len() == 1 {
        // Single match - complete it
        Some(matches[0].clone())
    } else {
        // Multiple matches - find common prefix
        let common_prefix = find_common_prefix(&matches);
        if common_prefix.len() > input.len() {
            Some(common_prefix)
        } else {
            None
        }
    }
}

/// Find the common prefix of a list of strings
fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first = &strings[0];
    let mut prefix_len = first.len();
    
    for s in strings.iter().skip(1) {
        prefix_len = first
            .chars()
            .zip(s.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(prefix_len);
    }
    
    first.chars().take(prefix_len).collect()
}
