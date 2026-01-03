//! Main application structure

use crate::state::AppState;
use crate::section::{Section, SectionId, SectionTrait};
use crate::sections::create_all_sections;
use crate::event::{AppEvent, EventHandler};
use crate::ui;
use glob::Pattern;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::event::{KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use std::io;

/// Main application structure
pub struct App {
    pub state: AppState,
    pub sections: Vec<Section>,
    pub current_section: SectionId,
    pub should_quit: bool,
    /// Current action menu selection index (for sections with actions)
    pub action_selection: Option<usize>,
    /// Current input dialog state (None = no input dialog active)
    pub input_dialog: Option<crate::ui::dialog::Dialog>,
    /// Template registry for pattern templates
    pub template_registry: Option<crate::templates::TemplateRegistry>,
    /// Screen scroll offset for when content exceeds screen height
    pub screen_scroll: usize,
    /// Preview pane TableState for scrolling (wrapped in RefCell for interior mutability during rendering)
    pub preview_table_state: std::cell::RefCell<ratatui::widgets::TableState>,
    /// Validation section TableState for scrolling (wrapped in RefCell for interior mutability during rendering)
    pub validation_table_state: std::cell::RefCell<ratatui::widgets::TableState>,
    /// Exclusions section ListState for scrolling (wrapped in RefCell for interior mutability during rendering)
    pub exclusions_list_state: std::cell::RefCell<ratatui::widgets::ListState>,
    /// Match files section ListState for scrolling (wrapped in RefCell for interior mutability during rendering)
    pub match_files_list_state: std::cell::RefCell<ratatui::widgets::ListState>,
    /// Working directories section ListState for scrolling (wrapped in RefCell for interior mutability during rendering)
    pub working_directories_list_state: std::cell::RefCell<ratatui::widgets::ListState>,
    /// Match files section ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub match_files_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// Working directories section ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub working_directories_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// Preview pane ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub preview_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// Validation section ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub validation_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// Exclusions section ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub exclusions_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// Main screen ScrollbarState for visual scroll indicator (wrapped in RefCell for interior mutability during rendering)
    pub main_scrollbar_state: std::cell::RefCell<ratatui::widgets::ScrollbarState>,
    /// History of rename operations for undo
    pub rename_history: Option<freneng::history::History>,
    /// Temporary match pattern being edited (for live preview)
    pub temp_match_pattern: Option<String>,
    /// Temporary exclusion pattern being edited (for live preview)
    pub temp_exclusion_pattern: Option<String>,
    /// Temporary rename rule being edited (for live preview)
    pub temp_rename_pattern: Option<String>,
    /// Temporary file list for live preview (when editing match/exclusion/rename patterns)
    pub temp_list: Vec<std::path::PathBuf>,
    /// Temporary new names for live preview (when editing match/exclusion/rename patterns)
    pub temp_new_names: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let sections = create_all_sections();
        // Find first focusable section (skip Header and Footer)
        let first_focusable = SectionId::all()
            .into_iter()
            .find(|&id| !matches!(id, SectionId::Header | SectionId::Footer))
            .unwrap_or(SectionId::WorkingDirectory);
        
        Self {
            state: AppState::new(),
            sections,
            current_section: first_focusable,
            should_quit: false,
            action_selection: Some(0),
            input_dialog: None,
            template_registry: None,
            screen_scroll: 0,
            preview_table_state: {
                let mut state = ratatui::widgets::TableState::default();
                state.select(Some(0));
                std::cell::RefCell::new(state)
            },
            validation_table_state: std::cell::RefCell::new(ratatui::widgets::TableState::default()),
            exclusions_list_state: std::cell::RefCell::new(ratatui::widgets::ListState::default()),
            match_files_list_state: std::cell::RefCell::new(ratatui::widgets::ListState::default()),
            match_files_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            working_directories_list_state: std::cell::RefCell::new(ratatui::widgets::ListState::default()),
            working_directories_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            preview_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            validation_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            exclusions_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            main_scrollbar_state: std::cell::RefCell::new(ratatui::widgets::ScrollbarState::new(0)),
            rename_history: None,
            temp_match_pattern: None,
            temp_exclusion_pattern: None,
            temp_rename_pattern: None,
            temp_list: Vec::new(),
            temp_new_names: Vec::new(),
        }
    }
    
    pub fn get_template_registry(&mut self) -> &mut crate::templates::TemplateRegistry {
        if self.template_registry.is_none() {
            self.template_registry = Some(crate::templates::TemplateRegistry::new());
        }
        self.template_registry.as_mut().unwrap()
    }
    
    /// Get the current section index
    pub fn current_section_idx(&self) -> usize {
        self.sections.iter()
            .position(|s| s.id() == self.current_section)
            .expect("Current section should always exist")
    }
    
    /// Check if a section can receive focus
    /// A section can receive focus if it has enabled actions or scrollable content
    /// Header and Footer are never focusable (they're informational only)
    pub fn section_can_focus(&self, section_id: SectionId) -> bool {
        // Header and Footer are never focusable
        if matches!(section_id, SectionId::Header | SectionId::Footer) {
            return false;
        }
        
        // Find the section
        if let Some(section) = self.sections.iter().find(|s| s.id() == section_id) {
            // Check if it has enabled actions
            let actions = section.actions(self);
            let has_enabled_actions = actions.iter().any(|a| a.enabled(self));
            
            // Check if it has scrollable content
            let has_scrollable_content = match section_id {
                SectionId::PreviewPane => {
                    // Preview is scrollable if it has files
                    !self.state.list.is_empty() && !self.state.new_names.is_empty()
                }
                SectionId::Validation => {
                    // Validation is scrollable if it has issues
                    self.state.validation
                        .as_ref()
                        .map(|v| !v.issues.is_empty())
                        .unwrap_or(false)
                }
                _ => false, // Other sections don't have scrollable content
            };
            
            has_enabled_actions || has_scrollable_content
        } else {
            false
        }
    }
    
    /// Navigate to next section (only skip Header and Footer)
    /// Stays at current section if already at the last focusable section (no wrap-around)
    pub fn go_next_section(&mut self) {
        // Navigate through all sections in order, only skipping Header and Footer
        loop {
            if let Some(next) = self.current_section.next() {
                // Only skip Header and Footer, navigate to all other sections
                if matches!(next, SectionId::Header | SectionId::Footer) {
                    // Skip Header/Footer and try next
                    self.current_section = next;
                } else {
                    // Navigate to this section
                    self.current_section = next;
                    self.action_selection = Some(0);
                    return;
                }
            } else {
                // Reached end, stay at current section (no wrap-around)
                return;
            }
        }
    }
    
    /// Navigate to previous section (only skip Header and Footer)
    /// Stays at current section if already at the first focusable section (no wrap-around)
    pub fn go_previous_section(&mut self) {
        // Navigate through all sections in reverse order, only skipping Header and Footer
        loop {
            if let Some(prev) = self.current_section.previous() {
                // Only skip Header and Footer, navigate to all other sections
                if matches!(prev, SectionId::Header | SectionId::Footer) {
                    // Skip Header/Footer and try previous
                    self.current_section = prev;
                } else {
                    // Navigate to this section
                    self.current_section = prev;
                    self.action_selection = Some(0);
                    return;
                }
            } else {
                // Reached beginning, stay at current section (no wrap-around)
                return;
            }
        }
    }
    
    /// Get actions for current section
    pub fn current_actions(&self) -> Vec<crate::action::Action> {
        let idx = self.current_section_idx();
        self.sections[idx].actions(self)
    }
    
    /// Update state (recomputes all derived fields)
    pub async fn update_state(&mut self) -> Result<(), String> {
        self.state.update_state().await
    }
    
    /// Compute temporary preview for live editing (e.g., when typing match pattern)
    /// This updates temp_list and temp_new_names based on temp_match_pattern
    pub async fn update_temp_preview(&mut self) {
        if let Some(ref temp_pattern) = self.temp_match_pattern {
            // Validate pattern first (basic glob validation)
            if Pattern::new(temp_pattern).is_err() {
                // Invalid pattern - show "invalid pattern" for current files as placeholder
                let count = self.state.list.len().max(1);
                self.temp_list = self.state.list.clone();
                self.temp_new_names = (0..count)
                    .map(|_| "invalid pattern".to_string())
                    .collect();
                return;
            }
            
            // Try to compute preview with temp pattern
            // We need to compute raw_list with temp pattern, then apply exclusions, then compute new names
            let original_pattern = self.state.match_pattern.clone();
            let original_list = self.state.list.clone();
            let original_raw_list = self.state.raw_list.clone();
            self.state.match_pattern = temp_pattern.clone();
            
            // Compute raw_list with temp pattern
            if let Err(_) = self.state.compute_raw_list().await {
                // Error computing - show "invalid pattern" for original list
                self.temp_list = original_list.clone();
                self.temp_new_names = original_list.iter()
                    .map(|_| "invalid pattern".to_string())
                    .collect();
                self.state.match_pattern = original_pattern;
                self.state.list = original_list;
                self.state.raw_list = original_raw_list;
                return;
            }
            
            // Apply exclusions
            self.state.apply_exclusions();
            
            // Store the new list for preview
            self.temp_list = self.state.list.clone();
            
            // Compute new names for the temp list
            if let Err(_) = self.state.compute_new_names().await {
                // Error computing - show "invalid pattern"
                self.temp_new_names = self.temp_list.iter()
                    .map(|_| "invalid pattern".to_string())
                    .collect();
            } else {
                // Success - use the computed new names
                self.temp_new_names = self.state.new_names.clone();
            }
            
            // Restore original pattern and lists
            self.state.match_pattern = original_pattern;
            self.state.list = original_list;
            self.state.raw_list = original_raw_list;
            // Don't call update_state here - we just want the preview, not full state update
        } else {
            self.temp_list.clear();
            self.temp_new_names.clear();
        }
    }
    
    /// Compute temporary preview for live editing exclusion pattern
    /// This updates temp_list and temp_new_names based on temp_exclusion_pattern
    pub async fn update_temp_preview_exclusion(&mut self) {
        if let Some(ref temp_pattern) = self.temp_exclusion_pattern {
            // Validate pattern first (basic glob validation)
            if Pattern::new(temp_pattern).is_err() {
                // Invalid pattern - show "invalid pattern" for current files as placeholder
                let count = self.state.raw_list.len().max(1);
                self.temp_list = self.state.raw_list.clone();
                self.temp_new_names = (0..count)
                    .map(|_| "invalid pattern".to_string())
                    .collect();
                return;
            }
            
            // Try to compute preview with temp exclusion pattern
            // We need to temporarily add the pattern to exclude list, apply exclusions, then compute new names
            let original_exclude = self.state.exclude.clone();
            let original_list = self.state.list.clone();
            let original_raw_list = self.state.raw_list.clone();
            
            // Add temp pattern to exclude list
            self.state.exclude.push(temp_pattern.clone());
            
            // Apply exclusions (this works on raw_list and filters it into list)
            self.state.apply_exclusions();
            
            // Store the new list for preview
            self.temp_list = self.state.list.clone();
            
            // Compute new names for the temp list
            if let Err(_) = self.state.compute_new_names().await {
                // Error computing - show "invalid pattern"
                self.temp_new_names = self.temp_list.iter()
                    .map(|_| "invalid pattern".to_string())
                    .collect();
            } else {
                // Success - use the computed new names
                self.temp_new_names = self.state.new_names.clone();
            }
            
            // Restore original exclude list and list
            self.state.exclude = original_exclude;
            self.state.list = original_list;
            self.state.raw_list = original_raw_list;
            // Don't call update_state here - we just want the preview, not full state update
        } else {
            self.temp_list.clear();
            self.temp_new_names.clear();
        }
    }
    
    /// Compute temporary preview for live editing rename pattern
    /// This updates temp_new_names based on temp_rename_pattern
    pub async fn update_temp_preview_rename(&mut self) {
        if let Some(ref temp_rule) = self.temp_rename_pattern {
            // Try to compute preview with temp rename rule
            // We need to temporarily set the rename_rule, then compute new names
            let original_rule = self.state.rename_rule.clone();
            let original_new_names = self.state.new_names.clone();
            
            self.state.rename_rule = temp_rule.clone();
            
            // Compute new names for the current list
            if let Err(_) = self.state.compute_new_names().await {
                // Error computing - show "invalid pattern" for all files
                let count = self.state.list.len().max(1);
                self.temp_list = self.state.list.clone();
                self.temp_new_names = (0..count)
                    .map(|_| "invalid pattern".to_string())
                    .collect();
            } else {
                // Success - use the computed new names
                self.temp_list = self.state.list.clone();
                self.temp_new_names = self.state.new_names.clone();
            }
            
            // Restore original rule and new names
            self.state.rename_rule = original_rule;
            self.state.new_names = original_new_names;
            // Don't call update_state here - we just want the preview, not full state update
        } else {
            self.temp_list.clear();
            self.temp_new_names.clear();
        }
    }
    
    /// Execute apply renaming action
    pub async fn execute_apply(&mut self) -> Result<(), String> {
        if !self.state.validation_ok() {
            return Err("Validation failed".to_string());
        }
        
        // Build FileRename list from state
        let renames: Vec<freneng::FileRename> = self.state.list.iter()
            .zip(self.state.new_names.iter())
            .filter_map(|(old_path, new_name)| {
                old_path.parent().map(|parent| freneng::FileRename {
                    old_path: old_path.clone(),
                    new_path: parent.join(new_name),
                    new_name: new_name.clone(),
                })
            })
            .collect();
        
        // Execute renames
        let result = freneng::perform_renames(&renames, false).await
            .map_err(|e| format!("Failed to perform renames: {}", e))?;
        
        // Store history for undo
        if !result.successful.is_empty() {
            self.rename_history = Some(freneng::history::History {
                timestamp: chrono::Local::now(),
                actions: result.successful,
            });
            self.state.can_undo = true;
        }
        
        // Don't call update_state() here - it would recompute the file list and apply
        // the rename rule again to the already-renamed files, causing double application.
        // The preview will show stale data (handled with warning), and the user can
        // manually trigger a state update by changing any setting if needed.
        
        Ok(())
    }
    
    /// Execute undo renaming action
    pub async fn execute_undo(&mut self) -> Result<(), String> {
        let history = self.rename_history.as_ref()
            .ok_or_else(|| "No history available".to_string())?;
        
        let engine = freneng::RenamingEngine;
        let (safe_actions, conflicts) = engine.check_undo(history).await;
        
        if !conflicts.is_empty() {
            return Err(format!("Cannot undo: {}", conflicts.join(", ")));
        }
        
        if safe_actions.is_empty() {
            return Err("No safe actions to undo".to_string());
        }
        
        // Execute undo
        let count = engine.apply_undo(safe_actions).await
            .map_err(|e| format!("Failed to undo: {}", e))?;
        
        // Clear history after successful undo
        self.rename_history = None;
        self.state.can_undo = false;
        
        // Update state
        self.update_state().await?;
        
        log::info!("Undid {} rename operations", count);
        Ok(())
    }
    
    /// Run the main event loop
    /// Handles rendering, events (Tick, Mouse, Key), and continues until should_quit is true
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        event_handler: &EventHandler,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                ui::render::render_ui(f, self);
            })?;
            
            match event_handler.next().map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                AppEvent::Tick => {
                    // Update temp preview if user is editing match pattern
                    if self.temp_match_pattern.is_some() {
                        self.update_temp_preview().await;
                    }
                    // Update temp preview if user is editing exclusion pattern
                    if self.temp_exclusion_pattern.is_some() {
                        self.update_temp_preview_exclusion().await;
                    }
                    // Update temp preview if user is editing rename pattern
                    if self.temp_rename_pattern.is_some() {
                        self.update_temp_preview_rename().await;
                    }
                }
                AppEvent::Mouse(mouse) => {
                    // Get terminal size for coordinate calculations
                    let terminal_size = terminal.size()?;
                    let terminal_rect = ratatui::layout::Rect {
                        x: 0,
                        y: 0,
                        width: terminal_size.width,
                        height: terminal_size.height,
                    };
                    
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            // Mouse click to enter sections
                            ui::handler::handle_mouse_click(self, mouse, terminal_rect);
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            // Mouse drag for scrollbars
                            ui::handler::handle_mouse_drag(self, mouse, terminal_rect);
                        }
                        MouseEventKind::ScrollUp => {
                            // Mouse wheel scroll - check if over preview/validation table
                            if self.current_section == SectionId::PreviewPane {
                                // Scroll preview table
                                let item_count = self.state.list.len().min(self.state.new_names.len());
                                if item_count > 0 {
                                    let mut table_state = self.preview_table_state.borrow_mut();
                                    // Ensure there's a selection
                                    if table_state.selected().is_none() {
                                        table_state.select(Some(0));
                                    }
                                    table_state.select_previous();
                                }
                            } else if self.current_section == SectionId::Validation {
                                // Scroll validation table
                                let item_count = self.state.validation
                                    .as_ref()
                                    .map(|v| v.issues.len())
                                    .unwrap_or(0);
                                if item_count > 0 {
                                    let mut table_state = self.validation_table_state.borrow_mut();
                                    // Ensure there's a selection
                                    if table_state.selected().is_none() {
                                        table_state.select(Some(0));
                                    }
                                    table_state.select_previous();
                                }
                            } else {
                                // Scroll main screen
                                self.screen_scroll = self.screen_scroll.saturating_sub(3);
                            }
                        }
                        MouseEventKind::ScrollDown => {
                            // Mouse wheel scroll - check if over preview/validation table
                            if self.current_section == SectionId::PreviewPane {
                                // Scroll preview table
                                let item_count = self.state.list.len().min(self.state.new_names.len());
                                if item_count > 0 {
                                    let mut table_state = self.preview_table_state.borrow_mut();
                                    // Ensure there's a selection
                                    if table_state.selected().is_none() {
                                        table_state.select(Some(0));
                                    }
                                    table_state.select_next();
                                }
                            } else if self.current_section == SectionId::Validation {
                                // Scroll validation table
                                let item_count = self.state.validation
                                    .as_ref()
                                    .map(|v| v.issues.len())
                                    .unwrap_or(0);
                                if item_count > 0 {
                                    let mut table_state = self.validation_table_state.borrow_mut();
                                    // Ensure there's a selection
                                    if table_state.selected().is_none() {
                                        table_state.select(Some(0));
                                    }
                                    table_state.select_next();
                                }
                            } else {
                                // Scroll main screen
                                self.screen_scroll = self.screen_scroll.saturating_add(3);
                            }
                        }
                        _ => {}
                    }
                }
                AppEvent::Key(key) => {
                    // Handle Ctrl+C to quit (works even in raw mode)
                    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                        log::info!("Ctrl+C pressed, quitting");
                        self.quit();
                        continue;
                    }
                    
                    // If input dialog is active, handle it
                    if self.input_dialog.is_some() {
                        log::debug!("Input dialog is active, handling key");
                        if ui::handler::handle_key_input(self, key).await {
                            // Key was handled
                        }
                    } else {
                        // Global shortcuts only when no input dialog
                        match key.code {
                            KeyCode::Char('q') => {
                                self.quit();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                // Check for shift modifier for table scrolling
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    // Shift+Up/Down for table scrolling
                                    if self.current_section == SectionId::PreviewPane {
                                        if ui::handler::handle_preview_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    } else if self.current_section == SectionId::Validation {
                                        if ui::handler::handle_validation_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    }
                                }
                                // Otherwise, always navigate sections
                                self.go_previous_section();
                                // Auto-scroll will be handled in render function
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                // Check for shift modifier for table scrolling
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    // Shift+Up/Down for table scrolling
                                    if self.current_section == SectionId::PreviewPane {
                                        if ui::handler::handle_preview_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    } else if self.current_section == SectionId::Validation {
                                        if ui::handler::handle_validation_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    }
                                }
                                // Otherwise, always navigate sections
                                self.go_next_section();
                                // Auto-scroll will be handled in render function
                            }
                            KeyCode::PageUp | KeyCode::PageDown | KeyCode::Home | KeyCode::End => {
                                // Check for shift modifier for table scrolling
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    // Shift+PageUp/PageDown/Home/End for table scrolling
                                    if self.current_section == SectionId::PreviewPane {
                                        if ui::handler::handle_preview_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    } else if self.current_section == SectionId::Validation {
                                        if ui::handler::handle_validation_scrolling(self, key) {
                                            continue; // Table scrolling handled
                                        }
                                    }
                                }
                                // For PageUp/PageDown without shift, handle main screen scrolling
                                match key.code {
                                    KeyCode::PageUp => {
                                        self.screen_scroll = self.screen_scroll.saturating_sub(10);
                                    }
                                    KeyCode::PageDown => {
                                        self.screen_scroll = self.screen_scroll.saturating_add(10);
                                    }
                                    _ => {}
                                }
                            }
                            KeyCode::Left | KeyCode::Char('h') => {
                                // Navigate actions left
                                let actions = self.current_actions();
                                if !actions.is_empty() {
                                    let current = self.action_selection.unwrap_or(0);
                                    let prev = if current == 0 {
                                        actions.len().saturating_sub(1)
                                    } else {
                                        current - 1
                                    };
                                    self.action_selection = Some(prev);
                                }
                            }
                            KeyCode::Right | KeyCode::Char('l') => {
                                // Navigate actions right
                                let actions = self.current_actions();
                                if !actions.is_empty() {
                                    let current = self.action_selection.unwrap_or(0);
                                    let next = if current >= actions.len().saturating_sub(1) {
                                        0
                                    } else {
                                        current + 1
                                    };
                                    self.action_selection = Some(next);
                                }
                            }
                            _ => {
                                // Handle other keys based on current section
                                if ui::handler::handle_key_input(self, key).await {
                                    // Key was handled, continue
                                }
                            }
                        }
                    }
                }
            }
            
            if self.should_quit {
                break;
            }
        }
        
        Ok(())
    }
    
    /// Initialize application state after creation
    /// Computes initial file list, new names, and validation
    pub async fn initialize(&mut self) -> io::Result<()> {
        // Initial state update - listing and validation occur automatically at startup
        // This computes: workdirs + match → raw_list → list → new_names → validation
        match self.update_state().await {
            Ok(()) => {
                log::info!("Initial state update completed successfully");
                log::debug!("Listed {} files, generated {} new names", 
                          self.state.list.len(), 
                          self.state.new_names.len());
                if let Some(ref validation) = self.state.validation {
                    log::debug!("Validation completed with {} issues", validation.issues.len());
                }
                Ok(())
            }
            Err(e) => {
                log::error!("Error updating initial state: {}", e);
                // Continue anyway - user can still interact with the UI
                // Convert String error to io::Error
                Err(io::Error::new(io::ErrorKind::Other, e))
            }
        }
    }
    
    pub fn quit(&mut self) {
        log::info!("Quitting application");
        self.should_quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
