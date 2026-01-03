//! Configure apply options step

use crate::app::App;
use crate::steps::common::{handle_step_input, render_step};
use crate::steps::definition::{MenuAction, MenuDefinition, MenuOption, StepDefinition};
use ratatui::{layout::Rect, Frame};

/// Get the step definition for apply options
pub fn get_apply_options_step() -> StepDefinition {
    let menu = MenuDefinition::new(
        "Options",
        vec![
            MenuOption::with_dynamic_name(
                |app| {
                    let overwrite_status = if app.state.apply_overwrite { "Yes" } else { "No" };
                    format!("Overwrite existing files: {}", overwrite_status)
                },
                |app| {
                    // Toggle overwrite
                    app.state.apply_overwrite = !app.state.apply_overwrite;
                    MenuAction::None
                },
            ),
            MenuOption::with_dynamic_name(
                |app| {
                    let yes_status = if app.state.apply_yes { "Yes" } else { "No" };
                    format!("Skip confirmation: {}", yes_status)
                },
                |app| {
                    // Toggle yes
                    app.state.apply_yes = !app.state.apply_yes;
                    MenuAction::None
                },
            ),
            MenuOption::with_dynamic_name(
                |app| {
                    let interactive_status = if app.state.apply_interactive { "Yes" } else { "No" };
                    format!("Interactive rename mode: {}", interactive_status)
                },
                |app| {
                    // Toggle interactive
                    app.state.apply_interactive = !app.state.apply_interactive;
                    MenuAction::None
                },
            ),
            MenuOption::new("Execute rename", |_app| {
                MenuAction::Next
            }),
        ],
        |_app| true, // Can always move forward
    );
    
    StepDefinition::new("Configure Options", menu)
        .with_title_hint(|_app| Some("Configure rename operation options".to_string()))
}

pub fn render_apply_options(f: &mut Frame, app: &mut App, step_area: Rect) {
    let definition = get_apply_options_step();
    render_step(&definition, f, app, step_area);
}

pub fn handle_apply_options_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    let definition = get_apply_options_step();
    handle_step_input(&definition, app, key)
}
