//! Input handling for the TUI
//!
//! This module contains all key input handling logic, including
//! dialog handling, action menu navigation, and section-specific input.

use crate::app::App;
use crate::section;
use crate::ui::completion;
use crate::ui::dialog::{DialogType, TemplateField};
use crate::ui::section_layout;
use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use glob::Pattern;

/// Handle key input for the application
pub async fn handle_key_input(app: &mut App, key: KeyEvent) -> bool {
    // If input dialog is active, handle it
    // We need to handle template selection separately to avoid borrow conflicts
    let is_template_selection = app.input_dialog.as_ref()
        .map(|d| matches!(d.dialog_type, DialogType::TemplateSelection { .. }))
        .unwrap_or(false);
    
    // Handle ESC key early for all dialogs (including template selection and FileSelection)
    if app.input_dialog.is_some() {
        if key.code == KeyCode::Esc {
            app.input_dialog = None;
            return true;
        }
    }
    
    // Check if we're in file selection dialog
    let is_file_selection = app.input_dialog.as_ref()
        .map(|d| matches!(d.dialog_type, DialogType::FileSelection))
        .unwrap_or(false);
    // Check if we're in directory selection dialog
    let is_directory_selection = app.input_dialog.as_ref()
        .map(|d| matches!(d.dialog_type, DialogType::DirectorySelectionForRemoval))
        .unwrap_or(false);
    // Check if we're in match file selection dialog
    let is_match_file_selection = app.input_dialog.as_ref()
        .map(|d| matches!(d.dialog_type, DialogType::MatchFileSelection))
        .unwrap_or(false);
    
    if is_template_selection {
        let state_updated = handle_template_selection(app, key);
        if state_updated {
            // Template was applied - trigger state cascade
            if let Err(e) = app.update_state().await {
                log::error!("Error updating state after template selection: {}", e);
            }
            return true;
        }
    } else if is_file_selection {
        let state_updated = handle_file_selection(app, key);
        if state_updated {
            // Files were selected - trigger state cascade
            if let Err(e) = app.update_state().await {
                log::error!("Error updating state after file selection: {}", e);
            }
            return true;
        }
    } else if is_directory_selection {
        let state_updated = handle_directory_selection(app, key);
        if state_updated {
            // Directories were selected for removal - trigger state cascade
            if let Err(e) = app.update_state().await {
                log::error!("Error updating state after directory selection: {}", e);
            }
            return true;
        }
    } else if is_match_file_selection {
        let state_updated = handle_match_file_selection(app, key);
        if state_updated {
            // Match files were selected for removal - trigger state cascade
            if let Err(e) = app.update_state().await {
                log::error!("Error updating state after match file selection: {}", e);
            }
            return true;
        }
    }
    
    // Handle non-template, non-file-selection, non-directory-selection, non-match-file-selection dialogs
    if app.input_dialog.as_ref()
        .map(|d| !matches!(d.dialog_type, DialogType::TemplateSelection { .. })
                  && !matches!(d.dialog_type, DialogType::FileSelection)
                  && !matches!(d.dialog_type, DialogType::DirectorySelectionForRemoval)
                  && !matches!(d.dialog_type, DialogType::MatchFileSelection))
        .unwrap_or(false)
    {
        // We need to handle this carefully to avoid borrow conflicts
        // Extract what we need, then handle
        
        if let Some(ref mut dialog) = app.input_dialog {
            // handle_dialog_input needs app for state updates, but we already have dialog borrowed
            // We'll handle this inline for now
            match key.code {
                KeyCode::Enter => {
                    let value = dialog.input.value().to_string();
                    let dialog_type = dialog.dialog_type.clone();
                    
                    // Validate and apply based on dialog type
                    let mut should_update_state = false;
                    match &dialog_type {
                        DialogType::DirectorySelection => {
                            let path = std::path::PathBuf::from(&value);
                            if path.exists() && path.is_dir() {
                                // Check if directory is already in the list
                                let path_canonical = path.canonicalize().ok();
                                let already_exists = if let Some(ref canonical) = path_canonical {
                                    app.state.workdirs.iter()
                                        .any(|w| w.canonicalize().ok().as_ref() == Some(canonical))
                                } else {
                                    app.state.workdirs.contains(&path)
                                };
                                
                                if !already_exists {
                                    app.state.workdirs.push(path);
                                    should_update_state = true;
                                } else {
                                    log::warn!("Directory already in list: {}", value);
                                }
                            } else {
                                log::warn!("Invalid directory: {}", value);
                                return true;
                            }
                        }
                        DialogType::MatchPatternInput => {
                            let pattern = value.trim();
                            if pattern.is_empty() {
                                log::warn!("Empty match pattern");
                                return true;
                            }
                            // Validate pattern before accepting
                            if Pattern::new(pattern).is_err() {
                                log::warn!("Invalid glob pattern: {}", pattern);
                                return true; // Don't close dialog, keep editing
                            }
                            app.state.match_pattern = pattern.to_string();
                            app.temp_match_pattern = None; // Clear temp pattern
                            app.temp_list.clear();
                            app.temp_new_names.clear();
                            should_update_state = true;
                        }
                        DialogType::ExclusionPatternInput => {
                            if !value.trim().is_empty() {
                                // Validate pattern before accepting
                                if Pattern::new(value.trim()).is_err() {
                                    log::warn!("Invalid glob pattern: {}", value.trim());
                                    return true; // Don't close dialog, keep editing
                                }
                                app.state.exclude.push(value.trim().to_string());
                                app.temp_exclusion_pattern = None; // Clear temp pattern
                                app.temp_list.clear();
                                app.temp_new_names.clear();
                                should_update_state = true;
                            } else {
                                log::warn!("Empty exclusion pattern");
                                return true;
                            }
                        }
                        DialogType::RenamingRuleInput => {
                            if !value.trim().is_empty() {
                                // Note: Rename rule validation happens in compute_new_names
                                // We'll let it fail there and show "invalid pattern" in preview
                                app.state.rename_rule = value.trim().to_string();
                                app.temp_rename_pattern = None; // Clear temp pattern
                                app.temp_list.clear();
                                app.temp_new_names.clear();
                                should_update_state = true;
                            } else {
                                log::warn!("Empty renaming rule");
                                return true;
                            }
                        }
                        DialogType::FileSelection => {
                            for &file_idx in &dialog.selected_files {
                                if file_idx < app.state.list.len() {
                                    let path = app.state.list[file_idx].display().to_string();
                                    app.state.exclude.push(path);
                                }
                            }
                            should_update_state = true;
                        }
                        DialogType::MatchFileInput => {
                            let path = std::path::PathBuf::from(&value);
                            if path.exists() && path.is_file() {
                                // Check if file is already in the list
                                let path_canonical = path.canonicalize().ok();
                                let already_exists = if let Some(ref canonical) = path_canonical {
                                    app.state.match_files.iter()
                                        .any(|f| f.canonicalize().ok().as_ref() == Some(canonical))
                                } else {
                                    app.state.match_files.contains(&path)
                                };
                                
                                if !already_exists {
                                    app.state.match_files.push(path);
                                    should_update_state = true;
                                } else {
                                    log::warn!("File already in list: {}", value);
                                }
                            } else {
                                log::warn!("Invalid file: {}", value);
                                return true;
                            }
                        }
                        _ => {}
                    }
                    
                    app.input_dialog = None;
                    
                    if should_update_state {
                        if let Err(e) = app.update_state().await {
                            log::error!("Error updating state: {}", e);
                        }
                    }
                    return true;
                }
                KeyCode::Tab => {
                    if matches!(dialog.dialog_type, DialogType::DirectorySelection) {
                        let current_path = dialog.input.get_path_for_completion();
                        if let Some(completed) = completion::complete_path(&current_path) {
                            dialog.input.set_completed_path(completed);
                        }
                        return true;
                    }
                    return false;
                }
                _ => {
                    if dialog.input.handle_key(key) {
                        // If editing match pattern, update live preview
                        if matches!(dialog.dialog_type, DialogType::MatchPatternInput) {
                            let pattern = dialog.input.value().to_string();
                            if !pattern.trim().is_empty() {
                                app.temp_match_pattern = Some(pattern.trim().to_string());
                                // Preview will be updated on next Tick event
                            } else {
                                app.temp_match_pattern = None;
                                app.temp_list.clear();
                                app.temp_new_names.clear();
                            }
                        }
                        // If editing exclusion pattern, update live preview
                        else if matches!(dialog.dialog_type, DialogType::ExclusionPatternInput) {
                            let pattern = dialog.input.value().to_string();
                            if !pattern.trim().is_empty() {
                                app.temp_exclusion_pattern = Some(pattern.trim().to_string());
                                // Preview will be updated on next Tick event
                            } else {
                                app.temp_exclusion_pattern = None;
                                app.temp_list.clear();
                                app.temp_new_names.clear();
                            }
                        }
                        // If editing rename pattern, update live preview
                        else if matches!(dialog.dialog_type, DialogType::RenamingRuleInput) {
                            let rule = dialog.input.value().to_string();
                            if !rule.trim().is_empty() {
                                app.temp_rename_pattern = Some(rule.trim().to_string());
                                // Preview will be updated on next Tick event
                            } else {
                                app.temp_rename_pattern = None;
                                app.temp_list.clear();
                                app.temp_new_names.clear();
                            }
                        }
                        return true;
                    }
                }
            }
        }
    }
    
    // Handle preview pane scrolling when focused
    // Note: We handle this before action menu to allow scrolling in preview
    if app.current_section == section::SectionId::PreviewPane {
        if handle_preview_scrolling(app, key) {
            return true;
        }
    }
    
    // Handle validation section scrolling when focused
    if app.current_section == section::SectionId::Validation {
        if handle_validation_scrolling(app, key) {
            return true;
        }
    }
    
    // Handle working directories list navigation when focused
    if app.current_section == section::SectionId::WorkingDirectory {
        if handle_working_directories_navigation(app, key) {
            return true;
        }
    }
    
    // Handle match files list navigation when focused
    if app.current_section == section::SectionId::MatchFiles {
        if handle_match_files_navigation(app, key) {
            return true;
        }
    }
    
    // Handle action menu navigation first (before section input)
    if handle_action_menu_navigation(app, key).await {
        return true;
    }
    
    // Handle section-specific input
    // Note: We skip this for now due to borrow checker issues
    // Sections primarily use actions, so this is not critical
    false
}

/// Handle template selection dialog
/// Returns true if a template was applied (state needs updating), false otherwise
fn handle_template_selection(app: &mut App, key: KeyEvent) -> bool {
    let selected_idx = app.input_dialog.as_ref().and_then(|d| d.selected_template);
    let field = if let Some(dialog_ref) = app.input_dialog.as_ref() {
        match &dialog_ref.dialog_type {
            DialogType::TemplateSelection { field } => field.clone(),
            _ => return false,
        }
    } else {
        return false;
    };
    
    let template_count = {
        let registry = app.get_template_registry();
        registry.list_for_field(field.clone()).len()
    };
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = selected_idx.unwrap_or(0);
            let prev = if current == 0 {
                template_count.saturating_sub(1)
            } else {
                current - 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_template = if template_count > 0 {
                    Some(prev)
                } else {
                    None
                };
            }
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = selected_idx.unwrap_or(0);
            let next = if current >= template_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_template = if template_count > 0 {
                    Some(next)
                } else {
                    None
                };
            }
            return true;
        }
        KeyCode::Enter => {
            if let Some(idx) = selected_idx {
                let pattern_opt = {
                    let registry = app.get_template_registry();
                    let templates = registry.list_for_field(field.clone());
                    if idx < templates.len() {
                        Some((*templates[idx].1).clone())
                    } else {
                        None
                    }
                };
                
                if let Some(pattern) = pattern_opt {
                    match field {
                        TemplateField::MatchPattern => {
                            app.state.match_pattern = pattern;
                        }
                        TemplateField::ExclusionPattern => {
                            app.state.exclude.push(pattern);
                        }
                        TemplateField::RenamingRule => {
                            app.state.rename_rule = pattern;
                        }
                    }
                    app.input_dialog = None;
                    // Return true to indicate state was updated and needs cascade
                    return true;
                }
            }
            // No template selected or invalid index - just close dialog
            app.input_dialog = None;
            return false;
        }
        KeyCode::Esc => {
            app.input_dialog = None;
            return false; // No state update needed when canceling
        }
        _ => {}
    }
    
    false
}

/// Handle file selection dialog
/// Returns true if files were selected and state needs updating, false otherwise
fn handle_file_selection(app: &mut App, key: KeyEvent) -> bool {
    let selected_idx = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
    let file_count = app.state.list.len();
    
    if file_count == 0 {
        // No files to select
        return false;
    }
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = selected_idx.unwrap_or(0);
            let prev = if current == 0 {
                file_count.saturating_sub(1)
            } else {
                current - 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(prev);
            }
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = selected_idx.unwrap_or(0);
            let next = if current >= file_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(next);
            }
            return true;
        }
        KeyCode::Char(' ') => {
            // Toggle selection of current file
            if let Some(idx) = selected_idx {
                if let Some(ref mut dialog) = app.input_dialog {
                    if dialog.selected_files.contains(&idx) {
                        // Remove from selection
                        dialog.selected_files.retain(|&i| i != idx);
                    } else {
                        // Add to selection
                        dialog.selected_files.push(idx);
                    }
                }
            }
            return true;
        }
        KeyCode::Enter => {
            // Confirm selection - process selected files and close dialog
            if let Some(ref mut dialog) = app.input_dialog {
                if !dialog.selected_files.is_empty() {
                    // Add selected files to exclude list
                    for &file_idx in &dialog.selected_files {
                        if file_idx < file_count {
                            let path = app.state.list[file_idx].display().to_string();
                            app.state.exclude.push(path);
                        }
                    }
                    app.input_dialog = None;
                    // Return true to indicate state was updated and needs cascade
                    return true;
                }
            }
            // No files selected - just close dialog
            app.input_dialog = None;
            return false;
        }
        KeyCode::Esc => {
            app.input_dialog = None;
            return false; // No state update needed when canceling
        }
        _ => {}
    }
    
    false
}

/// Handle directory selection dialog
/// Returns true if directories were selected and state needs updating, false otherwise
fn handle_directory_selection(app: &mut App, key: KeyEvent) -> bool {
    let selected_idx = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
    let dir_count = app.state.workdirs.len();
    
    if dir_count == 0 {
        // No directories to select
        return false;
    }
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = selected_idx.unwrap_or(0);
            let prev = if current == 0 {
                dir_count.saturating_sub(1)
            } else {
                current - 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(prev);
            }
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = selected_idx.unwrap_or(0);
            let next = if current >= dir_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(next);
            }
            return true;
        }
        KeyCode::Char(' ') => {
            // Toggle selection of current directory
            if let Some(idx) = selected_idx {
                if let Some(ref mut dialog) = app.input_dialog {
                    if dialog.selected_files.contains(&idx) {
                        // Remove from selection
                        dialog.selected_files.retain(|&i| i != idx);
                    } else {
                        // Add to selection
                        dialog.selected_files.push(idx);
                    }
                }
            }
            return true;
        }
        KeyCode::Enter => {
            // Confirm selection - remove selected directories and close dialog
            if let Some(ref mut dialog) = app.input_dialog {
                if !dialog.selected_files.is_empty() {
                    // Remove selected directories (in reverse order to maintain indices)
                    let mut indices_to_remove: Vec<usize> = dialog.selected_files.clone();
                    indices_to_remove.sort_unstable();
                    indices_to_remove.reverse();
                    
                    // Ensure at least one directory remains
                    if indices_to_remove.len() < dir_count {
                        for &dir_idx in &indices_to_remove {
                            if dir_idx < dir_count {
                                app.state.workdirs.remove(dir_idx);
                            }
                        }
                        app.input_dialog = None;
                        // Return true to indicate state was updated and needs cascade
                        return true;
                    } else {
                        // Can't remove all directories - just close dialog
                        log::warn!("Cannot remove all directories");
                        app.input_dialog = None;
                        return false;
                    }
                }
            }
            // No directories selected - just close dialog
            app.input_dialog = None;
            return false;
        }
        KeyCode::Esc => {
            app.input_dialog = None;
            return false; // No state update needed when canceling
        }
        _ => {}
    }
    
    false
}

/// Handle match file selection dialog
/// Returns true if files were selected and state needs updating, false otherwise
fn handle_match_file_selection(app: &mut App, key: KeyEvent) -> bool {
    let selected_idx = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
    let file_count = app.state.match_files.len();
    
    if file_count == 0 {
        // No files to select
        return false;
    }
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = selected_idx.unwrap_or(0);
            let prev = if current == 0 {
                file_count.saturating_sub(1)
            } else {
                current - 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(prev);
            }
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = selected_idx.unwrap_or(0);
            let next = if current >= file_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            if let Some(ref mut dialog) = app.input_dialog {
                dialog.selected_file_index = Some(next);
            }
            return true;
        }
        KeyCode::Char(' ') => {
            // Toggle selection of current file
            if let Some(idx) = selected_idx {
                if let Some(ref mut dialog) = app.input_dialog {
                    if dialog.selected_files.contains(&idx) {
                        // Remove from selection
                        dialog.selected_files.retain(|&i| i != idx);
                    } else {
                        // Add to selection
                        dialog.selected_files.push(idx);
                    }
                }
            }
            return true;
        }
        KeyCode::Enter => {
            // Confirm selection - remove selected files and close dialog
            if let Some(ref mut dialog) = app.input_dialog {
                if !dialog.selected_files.is_empty() {
                    // Remove selected files (in reverse order to maintain indices)
                    let mut indices_to_remove: Vec<usize> = dialog.selected_files.clone();
                    indices_to_remove.sort_unstable();
                    indices_to_remove.reverse();
                    
                    for &file_idx in &indices_to_remove {
                        if file_idx < file_count {
                            app.state.match_files.remove(file_idx);
                        }
                    }
                    app.input_dialog = None;
                    // Return true to indicate state was updated and needs cascade
                    return true;
                }
            }
            // No files selected - just close dialog
            app.input_dialog = None;
            return false;
        }
        KeyCode::Esc => {
            app.input_dialog = None;
            return false; // No state update needed when canceling
        }
        _ => {}
    }
    
    false
}

/// Handle preview pane scrolling using TableState
/// Only handles shift+arrows or shift+page up/down
pub fn handle_preview_scrolling(app: &mut App, key: KeyEvent) -> bool {
    let item_count = app.state.list.len().min(app.state.new_names.len());
    if item_count == 0 {
        return false;
    }
    
    // Only handle if shift is pressed (for shift+arrows or shift+page up/down)
    use crossterm::event::KeyModifiers;
    if !key.modifiers.contains(KeyModifiers::SHIFT) {
        return false;
    }
    
    let mut table_state = app.preview_table_state.borrow_mut();
    
    // Ensure there's a valid selection
    if table_state.selected().is_none() && item_count > 0 {
        table_state.select(Some(0));
    }
    
    match key.code {
        KeyCode::Up => {
            table_state.select_previous();
            return true;
        }
        KeyCode::Down => {
            table_state.select_next();
            return true;
        }
        KeyCode::PageUp => {
            // scroll_up_by moves selection up, but we need to ensure it works
            let current = table_state.selected().unwrap_or(0);
            let new_pos = current.saturating_sub(10);
            table_state.select(Some(new_pos));
            return true;
        }
        KeyCode::PageDown => {
            // scroll_down_by moves selection down, but we need to ensure it works
            let current = table_state.selected().unwrap_or(0);
            let new_pos = (current + 10).min(item_count.saturating_sub(1));
            table_state.select(Some(new_pos));
            return true;
        }
        KeyCode::Home => {
            table_state.select_first();
            return true;
        }
        KeyCode::End => {
            table_state.select_last();
            return true;
        }
        _ => {}
    }
    
    false
}

/// Handle validation section scrolling using TableState
/// Only handles shift+arrows or shift+page up/down
/// Handle working directories list navigation when focused
pub fn handle_working_directories_navigation(app: &mut App, key: KeyEvent) -> bool {
    let item_count = app.state.workdirs.len();
    if item_count == 0 {
        return false;
    }
    
    let mut list_state = app.working_directories_list_state.borrow_mut();
    
    // Ensure there's a valid selection
    if list_state.selected().is_none() && item_count > 0 {
        list_state.select(Some(0));
    }
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = list_state.selected().unwrap_or(0);
            let prev = if current == 0 {
                item_count.saturating_sub(1)
            } else {
                current - 1
            };
            list_state.select(Some(prev));
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = list_state.selected().unwrap_or(0);
            let next = if current >= item_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            list_state.select(Some(next));
            return true;
        }
        _ => {}
    }
    
    false
}

/// Handle match files list navigation when focused
pub fn handle_match_files_navigation(app: &mut App, key: KeyEvent) -> bool {
    // Count items: pattern (if set) + specific files
    let item_count = if app.state.match_pattern.is_empty() {
        app.state.match_files.len()
    } else {
        1 + app.state.match_files.len()
    };
    
    if item_count == 0 {
        return false;
    }
    
    let mut list_state = app.match_files_list_state.borrow_mut();
    
    // Ensure there's a valid selection
    if list_state.selected().is_none() && item_count > 0 {
        list_state.select(Some(0));
    }
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = list_state.selected().unwrap_or(0);
            let prev = if current == 0 {
                item_count.saturating_sub(1)
            } else {
                current - 1
            };
            list_state.select(Some(prev));
            return true;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = list_state.selected().unwrap_or(0);
            let next = if current >= item_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            list_state.select(Some(next));
            return true;
        }
        _ => {}
    }
    
    false
}

pub fn handle_validation_scrolling(app: &mut App, key: KeyEvent) -> bool {
    let item_count = app.state.validation
        .as_ref()
        .map(|v| v.issues.len())
        .unwrap_or(0);
    if item_count == 0 {
        return false;
    }
    
    // Only handle if shift is pressed (for shift+arrows or shift+page up/down)
    use crossterm::event::KeyModifiers;
    if !key.modifiers.contains(KeyModifiers::SHIFT) {
        return false;
    }
    
    let mut table_state = app.validation_table_state.borrow_mut();
    
    // Ensure there's a valid selection
    if table_state.selected().is_none() && item_count > 0 {
        table_state.select(Some(0));
    }
    
    match key.code {
        KeyCode::Up => {
            table_state.select_previous();
            return true;
        }
        KeyCode::Down => {
            table_state.select_next();
            return true;
        }
        KeyCode::PageUp => {
            // scroll_up_by moves selection up, but we need to ensure it works
            let current = table_state.selected().unwrap_or(0);
            let new_pos = current.saturating_sub(10);
            table_state.select(Some(new_pos));
            return true;
        }
        KeyCode::PageDown => {
            // scroll_down_by moves selection down, but we need to ensure it works
            let current = table_state.selected().unwrap_or(0);
            let new_pos = (current + 10).min(item_count.saturating_sub(1));
            table_state.select(Some(new_pos));
            return true;
        }
        KeyCode::Home => {
            table_state.select_first();
            return true;
        }
        KeyCode::End => {
            table_state.select_last();
            return true;
        }
        _ => {}
    }
    
    false
}

/// Handle action menu navigation
async fn handle_action_menu_navigation(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let actions = app.current_actions();
            if !actions.is_empty() {
                let current = app.action_selection.unwrap_or(0);
                let prev = if current == 0 {
                    actions.len().saturating_sub(1)
                } else {
                    current - 1
                };
                app.action_selection = Some(prev);
                return true;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let actions = app.current_actions();
            if !actions.is_empty() {
                let current = app.action_selection.unwrap_or(0);
                let next = if current >= actions.len().saturating_sub(1) {
                    0
                } else {
                    current + 1
                };
                app.action_selection = Some(next);
                return true;
            }
        }
        KeyCode::Enter => {
            let section_id = app.current_section;
            let action_result = {
                let actions = app.current_actions();
                if actions.is_empty() {
                    return false;
                }
                
                let selection = app.action_selection.unwrap_or(0);
                if selection < actions.len() {
                    let action = &actions[selection];
                    if action.enabled(app) {
                        action.run(app)
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            };
            
            // Handle action result
            match action_result {
                crate::action::ActionResult::StateUpdated => {
                    // Check if this is Apply or Undo action
                    if section_id == section::SectionId::ApplyRenaming {
                        if let Err(e) = app.execute_apply().await {
                            log::error!("Error applying renames: {}", e);
                        }
                    } else if section_id == section::SectionId::Undo {
                        if let Err(e) = app.execute_undo().await {
                            log::error!("Error undoing renames: {}", e);
                        }
                    } else {
                        // Regular state update
                        if let Err(e) = app.update_state().await {
                            log::error!("Error updating state: {}", e);
                        }
                    }
                }
                crate::action::ActionResult::DialogOpened => {
                    // Dialog is already opened by action
                }
                _ => {}
            }
            return true;
        }
        _ => {}
    }
    
    false
}

/// Handle mouse click to enter sections
pub fn handle_mouse_click(app: &mut App, mouse: MouseEvent, terminal_size: Rect) {
    use crate::section::SectionId;
    
    // Calculate layout similar to render_ui
    // Header is at y=0, height=1
    // Separator at y=1, height=1
    // Content area starts at y=2
    // Footer is at bottom, height=2
    
    let header_height = 2; // Header + separator
    let footer_height = 2;
    let content_area_y = header_height;
    let content_area_height = terminal_size.height.saturating_sub(header_height + footer_height);
    
    let mouse_y = mouse.row;
    let _mouse_x = mouse.column;
    
    // Check if click is in content area
    if mouse_y < content_area_y || mouse_y >= content_area_y + content_area_height {
        return; // Click outside content area
    }
    
    // Calculate which section was clicked
    let section_ids = SectionId::all();
    let section_ids_without_header_footer: Vec<_> = section_ids.iter()
        .skip(1)
        .take(section_ids.len() - 2)
        .collect();
    
    // Calculate section heights and positions
    let section_heights: Vec<u16> = section_ids_without_header_footer.iter()
        .filter_map(|id| {
            app.sections.iter()
                .find(|s| { use crate::section::SectionTrait; s.id() == **id })
                .map(|section| {
                    section_layout::calculate_section_height(section, app, content_area_height)
                })
        })
        .collect();
    
    // Calculate scroll offset
    let total_min_height: u16 = section_heights.iter().sum();
    let max_scroll = total_min_height.saturating_sub(content_area_height);
    let scroll_offset = (app.screen_scroll as u16).min(max_scroll);
    
    // Find which section contains the click
    let mut current_y = content_area_y as i32 - scroll_offset as i32;
    for (idx, section_id) in section_ids_without_header_footer.iter().enumerate() {
        if idx < section_heights.len() {
            let calculated_height = section_heights[idx];
            let section_y = current_y.max(content_area_y as i32) as u16;
            let section_bottom = section_y + calculated_height;
            
            // Check if click is within this section
            if mouse_y >= section_y && mouse_y < section_bottom {
                // Check if section can be focused
                if app.section_can_focus(**section_id) {
                    app.current_section = **section_id;
                    app.action_selection = Some(0);
                }
                return;
            }
            
            current_y += calculated_height as i32;
        }
    }
}

/// Handle mouse drag for scrollbars
pub fn handle_mouse_drag(app: &mut App, mouse: MouseEvent, terminal_size: Rect) {
    // For main screen scrollbar, calculate scroll position based on drag
    let header_height = 2;
    let footer_height = 2;
    let content_area_y = header_height;
    let content_area_height = terminal_size.height.saturating_sub(header_height + footer_height);
    
    let mouse_y = mouse.row;
    let mouse_x = mouse.column;
    
    // Check if drag is in content area and near right edge (scrollbar area)
    if mouse_y < content_area_y || mouse_y >= content_area_y + content_area_height {
        return;
    }
    
    // Check if near right edge (within last 2 columns for scrollbar)
    if mouse_x < terminal_size.width.saturating_sub(2) {
        return;
    }
    
    // Calculate section heights to get total scrollable height
    use crate::section::SectionId;
    let section_ids = SectionId::all();
    let section_ids_without_header_footer: Vec<_> = section_ids.iter()
        .skip(1)
        .take(section_ids.len() - 2)
        .collect();
    
    let section_heights: Vec<u16> = section_ids_without_header_footer.iter()
        .filter_map(|id| {
            app.sections.iter()
                .find(|s| { use crate::section::SectionTrait; s.id() == **id })
                .map(|section| {
                    section_layout::calculate_section_height(section, app, content_area_height)
                })
        })
        .collect();
    
    let total_min_height: u16 = section_heights.iter().sum();
    let max_scroll = total_min_height.saturating_sub(content_area_height);
    
    if max_scroll == 0 {
        return; // Nothing to scroll
    }
    
    // Calculate scroll position based on mouse Y position
    let relative_y = (mouse_y as i32 - content_area_y as i32).max(0) as u16;
    let scroll_ratio = relative_y as f32 / content_area_height as f32;
    let new_scroll = (scroll_ratio * max_scroll as f32) as usize;
    app.screen_scroll = new_scroll.min(max_scroll as usize);
}
