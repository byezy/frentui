//! Review preview step

use crate::app::App;
use crate::navigation::WorkflowStep;
use crate::steps::common::{handle_step_input, render_step};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, PreviewDefinition, StepDefinition};
use ratatui::layout::Rect;
use ratatui::Frame;

/// Get the step definition for review preview
pub fn get_review_preview_step() -> StepDefinition {
    let menu = MenuDefinition::new(
        "Actions",
        vec![
            MenuOption::new("Continue to options", |_app| {
                MenuAction::Next
            }),
            MenuOption::new("Change pattern", |app| {
                app.jump_to(WorkflowStep::SelectRenamePattern);
                MenuAction::None
            }),
            MenuOption::new("Change file selection", |app| {
                app.jump_to(WorkflowStep::ReviewFileList);
                MenuAction::None
            }),
        ],
        |app| app.state.preview_result.is_some(), // Can move forward if preview exists
    );
    
    StepDefinition::new("Review Preview", menu)
        .with_title_hint(|app| {
            let preview_count = app.state.preview_result.as_ref()
                .map(|p| p.renames.len())
                .unwrap_or(0);
            Some(format!("{} file(s) will be renamed", preview_count))
        })
        .with_preview(
            PreviewDefinition::freneng_preview(|app| app.state.preview_result.as_ref())
        )
}

pub fn render_review_preview(f: &mut Frame, app: &mut App, step_area: Rect) {
    let definition = get_review_preview_step();
    render_step(&definition, f, app, step_area);
}

pub async fn generate_preview(app: &mut App) -> Result<(), String> {
    use freneng::RenamingEngine;
    
    let engine = RenamingEngine;
    
    if app.state.selected_files.is_empty() {
        return Err("No files selected".to_string());
    }
    
    let pattern = app.state.rename_pattern.as_ref()
        .ok_or("No rename pattern specified".to_string())?;
    
    let preview = engine.generate_preview(&app.state.selected_files, pattern)
        .await
        .map_err(|e| format!("Error generating preview: {}", e))?;
    
    app.state.preview_result = Some(preview);
    Ok(())
}

pub fn handle_review_preview_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let definition = get_review_preview_step();
    handle_step_input(&definition, app, key)
}
