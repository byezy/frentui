//! Action definitions for section actions
//!
//! Actions are menu items that can be executed within a section.
//! Each action has a name, enabled state, and execution callback.

use crate::app::App;

/// An action that can be executed in a section
pub struct Action {
    /// Display name of the action
    pub name: String,
    /// Function to check if action is enabled
    pub is_enabled: Box<dyn Fn(&App) -> bool>,
    /// Function to execute the action
    pub execute: Box<dyn Fn(&mut App) -> ActionResult>,
}

/// Result of executing an action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    /// Action completed, no state change needed
    #[allow(dead_code)]
    NoChange,
    /// Action completed, state was updated (triggers UI refresh)
    StateUpdated,
    /// Action opened a dialog (section should show dialog UI)
    DialogOpened,
    /// Action cancelled or failed
    #[allow(dead_code)]
    Cancelled,
}

impl Action {
    /// Create a new action
    pub fn new(
        name: impl Into<String>,
        is_enabled: impl Fn(&App) -> bool + 'static,
        execute: impl Fn(&mut App) -> ActionResult + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            is_enabled: Box::new(is_enabled),
            execute: Box::new(execute),
        }
    }
    
    /// Create an action that's always enabled
    #[allow(dead_code)]
    pub fn always_enabled(
        name: impl Into<String>,
        execute: impl Fn(&mut App) -> ActionResult + 'static,
    ) -> Self {
        Self::new(name, |_| true, execute)
    }
    
    /// Check if this action is currently enabled
    pub fn enabled(&self, app: &App) -> bool {
        (self.is_enabled)(app)
    }
    
    /// Execute this action
    pub fn run(&self, app: &mut App) -> ActionResult {
        (self.execute)(app)
    }
    
    /// Get the action name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Helper to create a list of actions for a section
pub type ActionList = Vec<Action>;

