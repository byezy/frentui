//! Section definition and traits
//!
//! Sections are the building blocks of the single-screen TUI.
//! Each section has a title, hint, value display, and optional actions.

use crate::action::ActionList;
use crate::app::App;
use ratatui::{layout::Rect, Frame};

/// Unique identifier for each section
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionId {
    Header,
    WorkingDirectory,
    MatchFiles,
    Exclusions,
    RenamingRule,
    PreviewPane,
    Validation,
    ApplyRenaming,
    Undo,
    Footer,
}

impl SectionId {
    /// Get all sections in display order
    pub fn all() -> Vec<SectionId> {
        vec![
            SectionId::Header,
            SectionId::WorkingDirectory,
            SectionId::MatchFiles,
            SectionId::Exclusions,
            SectionId::RenamingRule,
            SectionId::PreviewPane,
            SectionId::Validation,
            SectionId::ApplyRenaming,
            SectionId::Undo,
            SectionId::Footer,
        ]
    }
    
    /// Get the next section in order
    pub fn next(self) -> Option<SectionId> {
        let all = Self::all();
        let pos = all.iter().position(|&s| s == self)?;
        if pos + 1 < all.len() {
            Some(all[pos + 1])
        } else {
            None
        }
    }
    
    /// Get the previous section in order
    pub fn previous(self) -> Option<SectionId> {
        let all = Self::all();
        let pos = all.iter().position(|&s| s == self)?;
        if pos > 0 {
            Some(all[pos - 1])
        } else {
            None
        }
    }
}

use crate::ui::section_layout::SectionHeights;

/// Trait that all sections must implement
pub trait SectionTrait {
    /// Get the section ID
    fn id(&self) -> SectionId;
    
    /// Get the section title
    fn title(&self) -> &str;
    
    /// Get the section hint (may be empty)
    #[allow(dead_code)] // Used via trait implementation
    fn hint(&self, app: &App) -> String;
    
    /// Get the section value to display (may be empty or multi-line)
    #[allow(dead_code)] // Used via trait implementation
    fn value(&self, app: &App) -> String;
    
    /// Get the list of actions for this section
    fn actions(&self, app: &App) -> ActionList;
    
    /// Render the section
    fn render(&self, f: &mut Frame, app: &App, area: Rect, is_focused: bool);
    
    /// Handle input for this section (returns true if handled)
    #[allow(dead_code)] // Used via trait implementation
    fn handle_input(&self, app: &mut App, key: crossterm::event::KeyEvent) -> bool;
    
    /// Get height requirements for this section
    /// Each section module should define its own height properties
    fn heights(&self) -> SectionHeights;
}

/// Section definition structure
pub struct Section {
    pub id: SectionId,
    pub title: String,
    #[allow(dead_code)] // Used internally in trait implementation
    pub hint_getter: Box<dyn Fn(&App) -> String>,
    #[allow(dead_code)] // Used internally in trait implementation
    pub value_getter: Box<dyn Fn(&App) -> String>,
    pub actions_getter: Box<dyn Fn(&App) -> ActionList>,
    pub renderer: Box<dyn Fn(&mut Frame, &App, Rect, bool)>,
    #[allow(dead_code)] // Used internally in trait implementation
    pub input_handler: Box<dyn Fn(&mut App, crossterm::event::KeyEvent) -> bool>,
    pub heights: SectionHeights,
}

impl Section {
    pub fn new(
        id: SectionId,
        title: impl Into<String>,
        hint_getter: impl Fn(&App) -> String + 'static,
        value_getter: impl Fn(&App) -> String + 'static,
        actions_getter: impl Fn(&App) -> ActionList + 'static,
        renderer: impl Fn(&mut Frame, &App, Rect, bool) + 'static,
        input_handler: impl Fn(&mut App, crossterm::event::KeyEvent) -> bool + 'static,
        heights: SectionHeights,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            hint_getter: Box::new(hint_getter),
            value_getter: Box::new(value_getter),
            actions_getter: Box::new(actions_getter),
            renderer: Box::new(renderer),
            input_handler: Box::new(input_handler),
            heights,
        }
    }
}

impl SectionTrait for Section {
    fn id(&self) -> SectionId {
        self.id
    }
    
    fn title(&self) -> &str {
        &self.title
    }
    
    fn hint(&self, app: &App) -> String {
        (self.hint_getter)(app)
    }
    
    fn value(&self, app: &App) -> String {
        (self.value_getter)(app)
    }
    
    fn actions(&self, app: &App) -> ActionList {
        (self.actions_getter)(app)
    }
    
    fn render(&self, f: &mut Frame, app: &App, area: Rect, is_focused: bool) {
        (self.renderer)(f, app, area, is_focused);
    }
    
    fn handle_input(&self, app: &mut App, key: crossterm::event::KeyEvent) -> bool {
        (self.input_handler)(app, key)
    }
    
    fn heights(&self) -> SectionHeights {
        self.heights
    }
}

