//! Confirm and execute step

use crate::app::App;
use crate::navigation::WorkflowStep;
use crate::steps::common::{handle_step_input, render_step};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, StepDefinition};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Get the step definition for confirm
pub fn get_confirm_step() -> StepDefinition {
    let menu = MenuDefinition::new(
        "Actions",
        vec![
            MenuOption::new("Execute rename", |_app| {
                MenuAction::Next
            }),
            MenuOption::new("Change options", |app| {
                app.jump_to(WorkflowStep::ConfigureApplyOptions);
                MenuAction::None
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
        |_app| true, // Can always move forward
    );
    
    StepDefinition::new("Confirm Rename", menu)
        .with_title_hint(|app| {
            let file_count = app.state.selected_files.len();
            let pattern = app.state.rename_pattern.as_ref()
                .map(|p| p.as_str())
                .unwrap_or("(none)");
            Some(format!("{} files will be renamed using pattern: {}", file_count, pattern))
        })
        .with_content_renderer(|f, app, area| {
            let mut options = Vec::new();
            if app.state.apply_overwrite {
                options.push("overwrite");
            }
            if app.state.apply_yes {
                options.push("skip confirmation");
            }
            if app.state.apply_interactive {
                options.push("interactive");
            }
            let options_str = if options.is_empty() {
                "(none)".to_string()
            } else {
                options.join(", ")
            };
            
            let summary = Block::default()
                .title("Summary")
                .borders(Borders::ALL)
                .padding(ratatui::widgets::Padding {
                    left: 1,
                    right: 1,
                    top: 1,
                    bottom: 1,
                });
            
            let summary_text = format!("Options: {}", options_str);
            let paragraph = Paragraph::new(summary_text).block(summary);
            f.render_widget(paragraph, area);
        })
}

pub fn render_confirm(f: &mut Frame, app: &mut App, step_area: Rect) {
    let definition = get_confirm_step();
    render_step(&definition, f, app, step_area);
}

pub async fn execute_rename(app: &mut App) -> Result<(), String> {
    use freneng::perform_renames;
    
    let preview = app.state.preview_result.take()
        .ok_or("No preview available".to_string())?;
    
    let renames = preview.renames;
    let result = perform_renames(&renames, app.state.apply_overwrite)
        .await
        .map_err(|e| format!("Error performing renames: {}", e))?;
    
    app.state.apply_result = Some(crate::state::ApplyResult {
        successful: result.successful.len(),
        skipped: result.skipped.len(),
        errors: result.errors.len(),
    });
    
    Ok(())
}

pub fn handle_confirm_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let definition = get_confirm_step();
    handle_step_input(&definition, app, key)
}
