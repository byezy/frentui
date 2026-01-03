//! Rename pattern selection step

use crate::app::App;
use crate::steps::common::{handle_step_input};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, StepDefinition};
use crate::ui::input::Input;
use crossterm::event::{KeyCode, KeyEvent};

/// Get the step definition for rename pattern
pub fn get_rename_pattern_step() -> StepDefinition {
    let menu = MenuDefinition::new(
        "Options",
        vec![
            MenuOption::new("Enter custom pattern (e.g., \"%N_backup.%E\")", |app| {
                let current_pattern = app.state.rename_pattern.as_ref()
                    .cloned()
                    .unwrap_or_else(|| "%N_backup.%E".to_string());
                log::info!("Opening custom pattern input dialog with pattern: {}", current_pattern);
                app.input_dialog = Some(Input::with_value("Enter Rename Pattern", current_pattern));
                MenuAction::None
            }),
            MenuOption::new("Select from templates", |app| {
                app.template_selection_mode = true;
                app.menu_selection = Some(0);
                MenuAction::None
            }),
        ],
        |app| app.state.rename_pattern.is_some(), // Can move forward if pattern is set
    );
    
    StepDefinition::new("Define Rename Pattern", menu)
        .with_title_hint(|app| {
            app.state.rename_pattern.as_ref()
                .map(|p| format!("Current pattern: {}", p))
        })
}

pub fn handle_rename_pattern_input(app: &mut App, key: KeyEvent) -> bool {
    log::debug!("handle_rename_pattern_input: key={:?}, input_dialog={:?}, template_selection_mode={}, menu_selection={:?}", 
                key.code, app.input_dialog.is_some(), app.template_selection_mode, app.menu_selection);
    
    // If input dialog is active, handle input
    if let Some(ref mut input) = app.input_dialog {
        log::debug!("Input dialog is active, handling key in dialog");
        match key.code {
            KeyCode::Enter => {
                // Confirm: set the pattern
                let pattern = input.value().trim().to_string();
                log::info!("Rename pattern input confirmed: '{}'", pattern);
                if !pattern.is_empty() {
                    log::info!("Setting rename pattern to: {}", pattern);
                    app.state.rename_pattern = Some(pattern);
                } else {
                    log::warn!("Empty pattern input, not setting pattern");
                }
                app.input_dialog = None; // Close dialog
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
    } else if app.template_selection_mode {
        // Template selection mode
        let templates = {
            let registry = app.get_template_registry();
            registry.list_for_field(TemplateField::RenamingRule)
        };
        let template_count = templates.len();
        
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                let current = app.menu_selection.unwrap_or(0);
                let prev = if current == 0 {
                    template_count.saturating_sub(1)
                } else {
                    current - 1
                };
                app.menu_selection = Some(prev);
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let current = app.menu_selection.unwrap_or(0);
                let next = if current >= template_count.saturating_sub(1) {
                    0
                } else {
                    current + 1
                };
                app.menu_selection = Some(next);
                true
            }
            KeyCode::Enter => {
                // Select template
                if let Some(sel) = app.menu_selection {
                    let templates = {
                        let registry = app.get_template_registry();
                        registry.list_for_field(TemplateField::RenamingRule)
                    };
                    if sel < templates.len() {
                        let (_, pattern) = &templates[sel];
                        app.state.rename_pattern = Some((*pattern).clone());
                        app.template_selection_mode = false;
                        app.menu_selection = Some(0); // Reset for main menu
                    }
                }
                true
            }
            KeyCode::Esc => {
                // Cancel template selection
                app.template_selection_mode = false;
                app.menu_selection = Some(0);
                true
            }
            _ => false,
        }
    } else {
        // Use common input handling for menu
        let definition = get_rename_pattern_step();
        handle_step_input(&definition, app, key)
    }
}
