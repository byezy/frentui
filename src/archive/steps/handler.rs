//! Unified step handler that uses StepDefinition

use crate::app::App;
use crate::navigation::WorkflowStep;
use crate::steps::common::render_step;
use crate::steps::definition::StepDefinition;
use crate::steps::{
    apply_options, confirm, step_set_directory, step_set_list, step_set_rename, results,
    review_files, review_preview, step_start,
};
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

/// Get step definition for a workflow step
pub fn get_step_definition(step: WorkflowStep) -> StepDefinition {
    match step {
        WorkflowStep::Start => {
            // Start step doesn't use StepDefinition (it's special)
            // Return a dummy one - we'll handle it separately
            unreachable!("Start step handled separately")
        }
        WorkflowStep::SelectDirectory => step_set_directory::get_directory_step(),
        WorkflowStep::SelectListPattern => step_set_list::get_list_pattern_step(),
        WorkflowStep::ReviewFileList => review_files::get_review_files_step(),
        WorkflowStep::SelectRenamePattern => step_set_rename::get_rename_pattern_step(),
        WorkflowStep::ReviewPreview => review_preview::get_review_preview_step(),
        WorkflowStep::ConfigureApplyOptions => apply_options::get_apply_options_step(),
        WorkflowStep::ConfirmAndExecute => confirm::get_confirm_step(),
        WorkflowStep::ShowResults | WorkflowStep::Complete => results::get_results_step(),
    }
}

/// Render a step using its definition
pub fn render_step_by_definition(f: &mut Frame, app: &mut App, step: WorkflowStep, step_area: Rect) {
    match step {
        WorkflowStep::Start => {
            // Start step is special - handle separately
            step_start::render_start(f, app, step_area);
        }
        _ => {
            let definition = get_step_definition(step);
            render_step(&definition, f, app, step_area);
        }
    }
}

/// Handle input for a step using its definition
pub fn handle_step_input_by_definition(
    app: &mut App,
    step: WorkflowStep,
    key: KeyEvent,
) -> bool {
    match step {
        WorkflowStep::Start => {
            step_start::handle_start_input(app, key)
        }
        WorkflowStep::SelectDirectory => {
            step_set_directory::handle_directory_input(app, key)
        }
        WorkflowStep::SelectListPattern => {
            step_set_list::handle_list_pattern_input(app, key)
        }
        WorkflowStep::ReviewFileList => {
            review_files::handle_review_files_input(app, key)
        }
        WorkflowStep::SelectRenamePattern => {
            step_set_rename::handle_rename_pattern_input(app, key)
        }
        WorkflowStep::ReviewPreview => {
            review_preview::handle_review_preview_input(app, key)
        }
        WorkflowStep::ConfigureApplyOptions => {
            apply_options::handle_apply_options_input(app, key)
        }
        WorkflowStep::ConfirmAndExecute => {
            confirm::handle_confirm_input(app, key)
        }
        WorkflowStep::ShowResults | WorkflowStep::Complete => {
            results::handle_results_input(app, key)
        }
    }
}

