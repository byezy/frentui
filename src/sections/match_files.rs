//! Match Files section

use crate::action::{Action, ActionResult};
use crate::color::SectionColors;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::ui::dialog::DialogType;
use crate::ui::section_layout::SectionHeights;
use ratatui::layout::Rect;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

/// Height configuration for Match Files section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 1,
    note_max: 1,
    content_min: 3, // List display (min 3 lines)
    content_max: 15, // List display (max 15 lines, scrollable if more)
    actions_min: 1,
    actions_max: 1,
    border_overhead: 2,
};

pub fn create_match_files_section() -> Section {
    Section::new(
        SectionId::MatchFiles,
        strings::match_files::TITLE,
        |_| strings::match_files::HINT.to_string(),
        |app| {
            // Show pattern and specific files
            let mut items = Vec::new();
            if !app.state.match_pattern.is_empty() {
                items.push(format!("Pattern: {}", app.state.match_pattern));
            }
            for file in &app.state.match_files {
                items.push(format!("File: {}", file.display()));
            }
            if items.is_empty() {
                "None".to_string()
            } else {
                items.join(", ")
            }
        },
        |app| {
            let mut actions = vec![
                Action::new(
                    strings::match_files::actions::ENTER_CUSTOM_PATTERN,
                    |_| true,
                    |app| {
                        let current_pattern = app.state.match_pattern.clone();
                        app.input_dialog = Some(crate::ui::dialog::Dialog::with_value(
                            DialogType::MatchPatternInput,
                            strings::match_files::dialog::TITLE,
                            current_pattern,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::match_files::actions::CHOOSE_TEMPLATE,
                    |_| true,
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::TemplateSelection {
                                field: crate::ui::dialog::TemplateField::MatchPattern,
                            },
                            strings::dialog::template_selection::TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::match_files::actions::ADD_FILE,
                    |_| true,
                    |app| {
                        let current_dir = std::env::current_dir()
                            .unwrap_or_else(|_| std::path::PathBuf::from("."))
                            .display()
                            .to_string();
                        app.input_dialog = Some(crate::ui::dialog::Dialog::with_value(
                            DialogType::MatchFileInput,
                            strings::match_files::dialog::ADD_FILE_TITLE,
                            current_dir,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
            ];
            
            // Add "Remove File" action if there are specific files
            if !app.state.match_files.is_empty() {
                actions.push(Action::new(
                    strings::match_files::actions::REMOVE_FILE,
                    |_app| true,
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::MatchFileSelection,
                            strings::match_files::dialog::REMOVE_FILE_TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ));
            }
            
            // Add "Clear" action if pattern is not default or there are specific files
            if app.state.match_pattern != "*.*" || !app.state.match_files.is_empty() {
                actions.push(Action::new(
                    strings::match_files::actions::CLEAR,
                    |_app| true,
                    |app| {
                        app.state.match_pattern = "*.*".to_string();
                        app.state.match_files.clear();
                        ActionResult::StateUpdated
                    },
                ));
            }
            
            actions
        },
        |f, app, area, is_focused| {
            let border_style = if is_focused {
                Style::default()
                    .fg(SectionColors::FOCUSED_BORDER)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SectionColors::BORDER)
            };
            
            let inner_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            };
            
            // Calculate how many items we'll have
            let has_pattern = !app.state.match_pattern.is_empty();
            let has_files = !app.state.match_files.is_empty();
            let item_count = if has_pattern { 1 } else { 0 } + if has_files { app.state.match_files.len() } else { 0 };
            
            // Use Length(1) when empty or single item to avoid extra blank lines
            // Use Fill(1) when multiple items to allow scrolling
            let list_constraint = if item_count == 0 {
                ratatui::layout::Constraint::Length(1) // Exactly 1 line for "None"
            } else if item_count == 1 {
                ratatui::layout::Constraint::Length(1) // Exactly 1 line for single item
            } else {
                ratatui::layout::Constraint::Fill(1) // Fill remaining space for multiple items (scrollable)
            };
            
            let chunks = ratatui::layout::Layout::default()
                .constraints([
                    ratatui::layout::Constraint::Length(1), // Blank line
                    ratatui::layout::Constraint::Length(1), // Note
                    ratatui::layout::Constraint::Length(1), // Blank line
                    list_constraint, // List (1 line if empty/single, variable if multiple)
                    ratatui::layout::Constraint::Length(1), // Blank line
                    ratatui::layout::Constraint::Length(1), // Actions
                ])
                .split(inner_area);
            
            let _blank1 = chunks[0]; // Blank line - not rendered
            let note_area = chunks[1];
            let _blank2 = chunks[2]; // Blank line - not rendered
            let list_area = chunks[3];
            let _blank3 = chunks[4]; // Blank line - not rendered
            let actions_area = chunks[5];
            
            // Render note
            let hint = strings::match_files::HINT;
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Render list (pattern + specific files)
            let has_pattern = !app.state.match_pattern.is_empty();
            let has_files = !app.state.match_files.is_empty();
            let show_prefixes = has_pattern && has_files; // Only show prefixes when both are present
            
            // Count total items
            let item_count = (if has_pattern { 1 } else { 0 }) + app.state.match_files.len();
            
            if item_count == 0 {
                let empty_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled("None", Style::default().fg(SectionColors::VALUE)),
                ]));
                f.render_widget(empty_paragraph, list_area);
            } else if item_count == 1 {
                // Single item - render as Paragraph to avoid List widget padding
                let single_text = if has_pattern && !has_files {
                    // Only pattern
                    app.state.match_pattern.clone()
                } else if !has_pattern && has_files {
                    // Only one file
                    app.state.match_files[0].display().to_string()
                } else {
                    // Shouldn't happen, but fallback
                    "".to_string()
                };
                let single_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled(single_text, Style::default().fg(SectionColors::VALUE)),
                ]));
                f.render_widget(single_paragraph, list_area);
            } else {
                // Multiple items - build List
                let mut items: Vec<ListItem> = Vec::new();
                
                // Add pattern if set
                if has_pattern {
                    let pattern_text = if show_prefixes {
                        format!("Pattern: {}", app.state.match_pattern)
                    } else {
                        app.state.match_pattern.clone()
                    };
                    items.push(ListItem::new(Line::from(vec![
                        Span::styled(
                            pattern_text,
                            Style::default().fg(SectionColors::VALUE),
                        ),
                    ])));
                }
                
                // Add specific files
                for file in &app.state.match_files {
                    let file_text = if show_prefixes {
                        format!("File: {}", file.display())
                    } else {
                        file.display().to_string()
                    };
                    items.push(ListItem::new(Line::from(vec![
                        Span::styled(
                            file_text,
                            Style::default().fg(SectionColors::VALUE),
                        ),
                    ])));
                }
                // Get items length before moving items into List
                let items_len = items.len();
                let list = List::new(items);
                
                // Update list state selection if needed
                {
                    let mut list_state = app.match_files_list_state.borrow_mut();
                    if let Some(sel) = list_state.selected() {
                        if sel >= items_len && items_len > 0 {
                            list_state.select(Some(0));
                        }
                    } else if items_len > 0 {
                        list_state.select(Some(0));
                    }
                }
                
                // Render list
                {
                    let mut list_state = app.match_files_list_state.borrow_mut();
                    f.render_stateful_widget(list, list_area, &mut *list_state);
                }
                
                // Update and render scrollbar if content exceeds visible area
                let content_length = items_len;
                let visible_height = list_area.height as usize;
                if content_length > visible_height {
                    // Update scrollbar state
                    let offset = app.match_files_list_state.borrow().offset();
                    let mut scrollbar_state = app.match_files_scrollbar_state.borrow_mut();
                    *scrollbar_state = ScrollbarState::new(content_length)
                        .position(offset)
                        .viewport_content_length(visible_height);
                    
                    // Render scrollbar on the right side
                    let scrollbar = Scrollbar::default()
                        .orientation(ScrollbarOrientation::VerticalRight);
                    f.render_stateful_widget(scrollbar, list_area, &mut *scrollbar_state);
                }
            }
            
            // Render actions (no blank line below)
            let actions = {
                let section = app.sections.iter()
                    .find(|s| s.id() == SectionId::MatchFiles)
                    .expect("MatchFiles section should exist");
                section.actions(app)
            };
            crate::ui::action_menu::render_action_menu(
                f,
                actions_area,
                &actions,
                if is_focused { app.action_selection } else { None },
                app,
            );
            
            // Render block border last (so it's on top)
            let block = Block::default()
                .title(strings::match_files::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false, // Input handling will be done via actions
        HEIGHTS,
    )
}
