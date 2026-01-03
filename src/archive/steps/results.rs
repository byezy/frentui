//! Show results step

use crate::app::App;
use crate::steps::common::{handle_step_input, render_step};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, StepDefinition};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Get the step definition for results
pub fn get_results_step() -> StepDefinition {
    let menu = MenuDefinition::without_navigation(
        "Actions",
        vec![
            MenuOption::new("Start new workflow", |app| {
                // Start new workflow - reset state
                app.state = crate::state::InteractiveState::default();
                app.navigator = crate::navigation::WorkflowNavigator::new();
                app.menu_selection = Some(0);
                app.go_next(); // Go to start
                MenuAction::None
            }),
            MenuOption::new("Exit", |app| {
                app.quit();
                MenuAction::None
            }),
        ],
        |_app| false, // Can't move forward from results
    );
    
    StepDefinition::new("Rename Complete", menu)
        .with_title_hint(|app| {
            let results = app.state.apply_result.as_ref();
            if let Some(result) = results {
                Some(format!(
                    "Successfully renamed: {} files | Skipped: {} files | Errors: {} files",
                    result.successful, result.skipped, result.errors
                ))
            } else {
                Some("No results available".to_string())
            }
        })
        .with_content_renderer(|f, app, area| {
            if let Some(result) = app.state.apply_result.as_ref() {
                let summary = Block::default()
                    .title("Details")
                    .borders(Borders::ALL)
                    .border_style(ratatui::style::Style::default().fg(Color::Green))
                    .padding(ratatui::widgets::Padding {
                        left: 1,
                        right: 1,
                        top: 1,
                        bottom: 1,
                    });
                
                let summary_text = format!(
                    "Successfully renamed: {} files\nSkipped: {} files\nErrors: {} files",
                    result.successful, result.skipped, result.errors
                );
                
                let paragraph = Paragraph::new(summary_text).block(summary);
                f.render_widget(paragraph, area);
            }
        })
}

pub fn render_results(f: &mut Frame, app: &mut App, step_area: Rect) {
    let definition = get_results_step();
    render_step(&definition, f, app, step_area);
}

pub fn handle_results_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let definition = get_results_step();
    handle_step_input(&definition, app, key)
}
