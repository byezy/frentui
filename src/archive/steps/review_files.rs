//! Review file list step

use crate::app::App;
use crate::navigation::WorkflowStep;
use crate::steps::common::{handle_step_input, render_step};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, PreviewDefinition, StepDefinition};
use ratatui::{
    layout::Rect,
    Frame,
};

/// Get the step definition for review files
pub fn get_review_files_step() -> StepDefinition {
    let menu = MenuDefinition::new(
        "Actions",
        vec![
            MenuOption::new("Continue to pattern selection", |_app| {
                MenuAction::Next
            }),
            MenuOption::new("Change list pattern", |app| {
                app.jump_to(WorkflowStep::SelectListPattern);
                MenuAction::None
            }),
            MenuOption::new("Change directory", |app| {
                app.jump_to(WorkflowStep::SelectDirectory);
                MenuAction::None
            }),
        ],
        |app| !app.state.selected_files.is_empty(), // Can move forward if files are selected
    );
    
    StepDefinition::new("Review File List", menu)
        .with_title_hint(|app| {
            let file_count = app.state.selected_files.len();
            Some(format!("Found {} file(s) to process", file_count))
        })
        .with_preview(
            PreviewDefinition::files_list(|app| app.state.selected_files.clone())
        )
}

pub fn render_review_files(f: &mut Frame, app: &mut App, step_area: Rect) {
    log::debug!("Rendering review_files step: {} files selected, menu_selection: {:?}", 
                app.state.selected_files.len(), app.menu_selection);
    
    let definition = get_review_files_step();
    render_step(&definition, f, app, step_area);
}

pub async fn load_files(app: &mut App) -> Result<(), String> {
    use freneng::find_matching_files_recursive;
    
    log::info!("load_files called: patterns={:?}, recursive={}, files_from={:?}", 
               app.state.list_patterns, app.state.list_recursive, app.state.list_files_from);
    
    if let Some(files_from) = &app.state.list_files_from {
        log::warn!("--files-from not yet implemented: {}", files_from);
        // TODO: Read from file
        return Err("--files-from not yet implemented".to_string());
    }
    
    if app.state.list_patterns.is_empty() {
        log::error!("No patterns specified for file loading");
        return Err("No patterns specified".to_string());
    }
    
    let mut all_files = Vec::new();
    for pattern in &app.state.list_patterns {
        log::debug!("Finding files with pattern: {}, recursive: {}", pattern, app.state.list_recursive);
        let files = find_matching_files_recursive(pattern, app.state.list_recursive)
            .await
            .map_err(|e| {
                log::error!("Error finding files with pattern {}: {}", pattern, e);
                format!("Error finding files: {}", e)
            })?;
        log::debug!("Found {} files for pattern: {}", files.len(), pattern);
        all_files.extend(files);
    }
    
    log::info!("Total files loaded: {}", all_files.len());
    app.state.selected_files = all_files;
    Ok(())
}

pub fn handle_review_files_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    log::debug!("handle_review_files_input: key={:?}, menu_selection={:?}, files_count={}", 
                key.code, app.menu_selection, app.state.selected_files.len());
    
    let definition = get_review_files_step();
    handle_step_input(&definition, app, key)
}
