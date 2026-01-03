//! Step definition structures for consistent step creation

use crate::app::App;
use crate::navigation::WorkflowStep;
use ratatui::{layout::Rect, Frame};

/// Action that can be taken when a menu option is selected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    /// No action (stay on current step)
    None,
    /// Navigate to next step
    Next,
    /// Navigate to previous step
    Back,
    /// Jump to a specific step
    JumpTo(WorkflowStep),
    /// Quit the application
    Quit,
}

/// A menu option with a name and action callback
pub struct MenuOption {
    pub name_getter: Box<dyn Fn(&App) -> String>,
    pub action: Box<dyn Fn(&mut App) -> MenuAction>,
}

impl MenuOption {
    /// Create a menu option with a static name
    pub fn new(name: impl Into<String>, action: impl Fn(&mut App) -> MenuAction + 'static) -> Self {
        let name_str = name.into();
        Self {
            name_getter: Box::new(move |_| name_str.clone()),
            action: Box::new(action),
        }
    }
    
    /// Create a menu option with a dynamic name (closure)
    pub fn with_dynamic_name(
        name_getter: impl Fn(&App) -> String + 'static,
        action: impl Fn(&mut App) -> MenuAction + 'static,
    ) -> Self {
        Self {
            name_getter: Box::new(name_getter),
            action: Box::new(action),
        }
    }
    
    /// Get the current name for this option
    pub fn name(&self, app: &App) -> String {
        (self.name_getter)(app)
    }
}

/// Menu definition for a step
pub struct MenuDefinition {
    pub title: String,
    pub options: Vec<MenuOption>,
    /// Whether to show built-in navigation entries (forward/back/quit)
    /// These will be automatically appended at the bottom
    pub show_navigation: bool,
    /// Whether forward navigation is allowed (checked dynamically)
    pub can_move_forward: Box<dyn Fn(&App) -> bool>,
}

impl MenuDefinition {
    pub fn new(
        title: impl Into<String>,
        options: Vec<MenuOption>,
        can_move_forward: impl Fn(&App) -> bool + 'static,
    ) -> Self {
        Self {
            title: title.into(),
            options,
            show_navigation: true,
            can_move_forward: Box::new(can_move_forward),
        }
    }

    /// Create a menu without built-in navigation entries
    pub fn without_navigation(
        title: impl Into<String>,
        options: Vec<MenuOption>,
        can_move_forward: impl Fn(&App) -> bool + 'static,
    ) -> Self {
        Self {
            title: title.into(),
            options,
            show_navigation: false,
            can_move_forward: Box::new(can_move_forward),
        }
    }

    /// Get all menu items including navigation entries
    pub fn all_items(&self, app: &App) -> Vec<String> {
        let mut items: Vec<String> = self.options.iter().map(|opt| opt.name(app)).collect();
        
        if self.show_navigation {
            // Add forward navigation if allowed
            if (self.can_move_forward)(app) {
                items.push("Go forward".to_string());
            }
            items.push("Go back".to_string());
            items.push("Quit".to_string());
        }
        
        items
    }

    /// Get total number of menu items (including navigation)
    pub fn item_count(&self, app: &App) -> usize {
        self.all_items(app).len()
    }

    /// Execute action for a menu item by index
    pub fn execute_action(&self, app: &mut App, index: usize) -> MenuAction {
        let option_count = self.options.len();
        let can_forward = (self.can_move_forward)(app);
        
        if !self.show_navigation {
            // No navigation entries, just execute option
            if index < option_count {
                (self.options[index].action)(app)
            } else {
                MenuAction::None
            }
        } else {
            // Check if it's a custom option or navigation entry
            if index < option_count {
                // Custom option
                (self.options[index].action)(app)
            } else {
                // Navigation entry
                let nav_index = index - option_count;
                if can_forward {
                    match nav_index {
                        0 => MenuAction::Next,  // Go forward
                        1 => MenuAction::Back, // Go back
                        2 => MenuAction::Quit, // Quit
                        _ => MenuAction::None,
                    }
                } else {
                    match nav_index {
                        0 => MenuAction::Back, // Go back
                        1 => MenuAction::Quit, // Quit
                        _ => MenuAction::None,
                    }
                }
            }
        }
    }
}

/// Preview content type
pub enum PreviewContent<'a> {
    /// Simple text preview
    Text(String),
    /// Files list preview
    FilesList(Vec<std::path::PathBuf>),
    /// Freneng preview result (renames, warnings, etc.) - using reference to avoid Clone requirement
    FrenengPreview(&'a freneng::EnginePreviewResult),
}

/// Preview definition for optional preview sections
pub struct PreviewDefinition {
    pub content: Box<dyn for<'a> Fn(&'a App) -> PreviewContent<'a>>,
    pub note: Option<String>,
}

impl PreviewDefinition {
    /// Create a text-based preview
    pub fn text(content: impl Fn(&App) -> String + 'static) -> Self {
        Self {
            content: Box::new(move |app| PreviewContent::Text(content(app))),
            note: None,
        }
    }

    /// Create a files list preview
    pub fn files_list(files: impl Fn(&App) -> Vec<std::path::PathBuf> + 'static) -> Self {
        Self {
            content: Box::new(move |app| PreviewContent::FilesList(files(app))),
            note: None,
        }
    }

    /// Create a freneng preview (from EnginePreviewResult reference)
    pub fn freneng_preview(preview: impl for<'a> Fn(&'a App) -> Option<&'a freneng::EnginePreviewResult> + 'static) -> Self {
        Self {
            content: Box::new(move |app| {
                preview(app)
                    .map(PreviewContent::FrenengPreview)
                    .unwrap_or(PreviewContent::Text("No preview available".to_string()))
            }),
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Step definition structure
pub struct StepDefinition {
    pub title: String,
    pub title_hint: Box<dyn Fn(&App) -> Option<String>>,
    pub hint: Option<String>,
    pub menu: MenuDefinition,
    pub preview: Option<PreviewDefinition>,
    /// Custom content renderer (for file lists, etc.)
    pub content_renderer: Option<Box<dyn Fn(&mut Frame, &App, Rect)>>,
}

impl StepDefinition {
    pub fn new(
        title: impl Into<String>,
        menu: MenuDefinition,
    ) -> Self {
        Self {
            title: title.into(),
            title_hint: Box::new(|_| None),
            hint: None,
            menu,
            preview: None,
            content_renderer: None,
        }
    }

    pub fn with_title_hint(
        mut self,
        title_hint: impl Fn(&App) -> Option<String> + 'static,
    ) -> Self {
        self.title_hint = Box::new(title_hint);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_preview(mut self, preview: PreviewDefinition) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn with_content_renderer(
        mut self,
        renderer: impl Fn(&mut Frame, &App, Rect) + 'static,
    ) -> Self {
        self.content_renderer = Some(Box::new(renderer));
        self
    }
}

/// Trait for steps that can be rendered and handle input
pub trait StepHandler {
    /// Get the step definition
    fn definition(&self) -> &StepDefinition;
    
    /// Render the step
    fn render(&self, f: &mut Frame, app: &App, step_area: Rect);
    
    /// Handle input for the step
    fn handle_input(&self, app: &mut App, key: crossterm::event::KeyEvent) -> bool;
}

