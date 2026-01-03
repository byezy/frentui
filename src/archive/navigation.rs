//! Navigation and workflow step management

/// Workflow steps in the interactive process
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStep {
    Start,
    SelectDirectory,
    SelectListPattern,
    ReviewFileList,
    SelectRenamePattern,
    ReviewPreview,
    ConfigureApplyOptions,
    ConfirmAndExecute,
    ShowResults,
    Complete,
}

impl WorkflowStep {
    /// Get the next step in the workflow
    #[allow(dead_code)]
    pub fn next(self) -> Option<Self> {
        match self {
            WorkflowStep::Start => Some(WorkflowStep::SelectDirectory),
            WorkflowStep::SelectDirectory => Some(WorkflowStep::SelectListPattern),
            WorkflowStep::SelectListPattern => Some(WorkflowStep::ReviewFileList),
            WorkflowStep::ReviewFileList => Some(WorkflowStep::SelectRenamePattern),
            WorkflowStep::SelectRenamePattern => Some(WorkflowStep::ReviewPreview),
            WorkflowStep::ReviewPreview => Some(WorkflowStep::ConfigureApplyOptions),
            WorkflowStep::ConfigureApplyOptions => Some(WorkflowStep::ConfirmAndExecute),
            WorkflowStep::ConfirmAndExecute => Some(WorkflowStep::ShowResults),
            WorkflowStep::ShowResults => Some(WorkflowStep::Complete),
            WorkflowStep::Complete => None,
        }
    }
    
    /// Get the previous step in the workflow
    #[allow(dead_code)]
    pub fn previous(self) -> Option<Self> {
        match self {
            WorkflowStep::Start => None,
            WorkflowStep::SelectDirectory => Some(WorkflowStep::Start),
            WorkflowStep::SelectListPattern => Some(WorkflowStep::SelectDirectory),
            WorkflowStep::ReviewFileList => Some(WorkflowStep::SelectListPattern),
            WorkflowStep::SelectRenamePattern => Some(WorkflowStep::ReviewFileList),
            WorkflowStep::ReviewPreview => Some(WorkflowStep::SelectRenamePattern),
            WorkflowStep::ConfigureApplyOptions => Some(WorkflowStep::ReviewPreview),
            WorkflowStep::ConfirmAndExecute => Some(WorkflowStep::ConfigureApplyOptions),
            WorkflowStep::ShowResults => Some(WorkflowStep::ConfirmAndExecute),
            WorkflowStep::Complete => Some(WorkflowStep::ShowResults),
        }
    }
    
    /// Get a human-readable name for the step
    #[allow(dead_code)]
    pub fn name(self) -> &'static str {
        match self {
            WorkflowStep::Start => "Start",
            WorkflowStep::SelectDirectory => "Select Directory",
            WorkflowStep::SelectListPattern => "Select Files",
            WorkflowStep::ReviewFileList => "Review File List",
            WorkflowStep::SelectRenamePattern => "Define Pattern",
            WorkflowStep::ReviewPreview => "Review Preview",
            WorkflowStep::ConfigureApplyOptions => "Configure Options",
            WorkflowStep::ConfirmAndExecute => "Confirm & Execute",
            WorkflowStep::ShowResults => "Results",
            WorkflowStep::Complete => "Complete",
        }
    }
    
    /// Get the step number (1-based) for progress display
    #[allow(dead_code)]
    pub fn step_number(self) -> usize {
        match self {
            WorkflowStep::Start => 1,
            WorkflowStep::SelectDirectory => 2,
            WorkflowStep::SelectListPattern => 3,
            WorkflowStep::ReviewFileList => 4,
            WorkflowStep::SelectRenamePattern => 5,
            WorkflowStep::ReviewPreview => 6,
            WorkflowStep::ConfigureApplyOptions => 7,
            WorkflowStep::ConfirmAndExecute => 8,
            WorkflowStep::ShowResults => 9,
            WorkflowStep::Complete => 10,
        }
    }
    
    /// Total number of steps in the workflow
    #[allow(dead_code)]
    pub const TOTAL_STEPS: usize = 10;
}

/// Navigation stack for going back through steps
pub struct WorkflowNavigator {
    step_stack: Vec<WorkflowStep>,
}

/// Validation result for state transitions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionValidation {
    Valid,
    Invalid(String), // Reason why transition is invalid
}

impl WorkflowNavigator {
    pub fn new() -> Self {
        Self {
            step_stack: vec![WorkflowStep::Start],
        }
    }
    
    pub fn current_step(&self) -> WorkflowStep {
        *self.step_stack.last().unwrap_or(&WorkflowStep::Start)
    }
    
    /// Check if a transition from current step to target step is valid
    /// This validates that required state is present before allowing transitions
    pub fn can_transition_to(&self, target: WorkflowStep, state: &crate::state::InteractiveState) -> TransitionValidation {
        let current = self.current_step();
        
        // Validate transitions based on required state
        match target {
            WorkflowStep::ReviewFileList => {
                // Need patterns or files-from to review files
                if state.list_patterns.is_empty() && state.list_files_from.is_none() {
                    return TransitionValidation::Invalid(
                        "No file patterns specified. Please select files first.".to_string()
                    );
                }
            }
            WorkflowStep::ReviewPreview => {
                // Need a rename pattern to generate preview
                if state.rename_pattern.is_none() {
                    return TransitionValidation::Invalid(
                        "No rename pattern specified. Please define a pattern first.".to_string()
                    );
                }
                // Need files to rename
                if state.selected_files.is_empty() {
                    return TransitionValidation::Invalid(
                        "No files selected. Please select files first.".to_string()
                    );
                }
            }
            WorkflowStep::ConfirmAndExecute => {
                // Need preview result to confirm
                if state.preview_result.is_none() {
                    return TransitionValidation::Invalid(
                        "No preview available. Please generate a preview first.".to_string()
                    );
                }
                // Need files
                if state.selected_files.is_empty() {
                    return TransitionValidation::Invalid(
                        "No files selected.".to_string()
                    );
                }
            }
            _ => {
                // Other transitions are always valid (they're just navigation)
            }
        }
        
        TransitionValidation::Valid
    }
    
    pub fn go_next(&mut self) -> Option<WorkflowStep> {
        let current = self.current_step();
        if let Some(next) = current.next() {
            self.step_stack.push(next);
            Some(next)
        } else {
            None
        }
    }
    
    /// Go to next step with validation
    pub fn go_next_validated(&mut self, state: &crate::state::InteractiveState) -> Result<WorkflowStep, String> {
        let current = self.current_step();
        if let Some(next) = current.next() {
            match self.can_transition_to(next, state) {
                TransitionValidation::Valid => {
                    self.step_stack.push(next);
                    Ok(next)
                }
                TransitionValidation::Invalid(reason) => {
                    Err(reason)
                }
            }
        } else {
            Err("Already at the end of the workflow".to_string())
        }
    }
    
    pub fn go_back(&mut self) -> Option<WorkflowStep> {
        if self.step_stack.len() > 1 {
            self.step_stack.pop();
            Some(self.current_step())
        } else {
            None
        }
    }
    
    pub fn jump_to(&mut self, step: WorkflowStep) {
        // Remove steps after the target if it's in the stack
        if let Some(pos) = self.step_stack.iter().position(|&s| s == step) {
            self.step_stack.truncate(pos + 1);
        } else {
            // Add to stack if not present
            self.step_stack.push(step);
        }
    }
}

impl Default for WorkflowNavigator {
    fn default() -> Self {
        Self::new()
    }
}

