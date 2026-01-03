//! Working Directory section

use crate::action::{Action, ActionResult};
use crate::color::SectionColors;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::ui::dialog::DialogType;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Working Directory section
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
use ratatui::layout::Rect;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use std::path::PathBuf;

pub fn create_working_directory_section() -> Section {
    Section::new(
        SectionId::WorkingDirectory,
        strings::working_directory::TITLE,
        |_| strings::working_directory::HINT.to_string(),
        |app| {
            if app.state.workdirs.is_empty() {
                "None".to_string()
            } else {
                app.state.workdirs.iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        },
        |app| {
            let mut actions = vec![
                Action::new(
                    strings::working_directory::actions::ADD_DIRECTORY,
                    |_| true,
                    |app| {
                        let current_dir = std::env::current_dir()
                            .unwrap_or_else(|_| PathBuf::from("."))
                            .display()
                            .to_string();
                        app.input_dialog = Some(crate::ui::dialog::Dialog::with_value(
                            DialogType::DirectorySelection,
                            strings::working_directory::dialog::TITLE,
                            current_dir,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
            ];
            
            // Add "Add Current Working Directory" action
            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let cwd_canonical = cwd.canonicalize().ok();
            let cwd_in_list = if let Some(ref cwd_canon) = cwd_canonical {
                app.state.workdirs.iter()
                    .any(|w| w.canonicalize().ok().as_ref() == Some(cwd_canon))
            } else {
                app.state.workdirs.contains(&cwd)
            };
            
            actions.push(Action::new(
                strings::working_directory::actions::ADD_CURRENT_WORKING_DIRECTORY,
                move |_app| !cwd_in_list, // Enabled when CWD is not in list
                |app| {
                    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                    let cwd_canonical = cwd.canonicalize().ok();
                    let already_exists = if let Some(ref cwd_canon) = cwd_canonical {
                        app.state.workdirs.iter()
                            .any(|w| w.canonicalize().ok().as_ref() == Some(cwd_canon))
                    } else {
                        app.state.workdirs.contains(&cwd)
                    };
                    
                    if !already_exists {
                        app.state.workdirs.push(cwd);
                        return ActionResult::StateUpdated;
                    }
                    ActionResult::NoChange
                },
            ));
            
            // Add "Remove Directory" action if there are directories and more than one
            if app.state.workdirs.len() > 1 {
                actions.push(Action::new(
                    strings::working_directory::actions::REMOVE_DIRECTORY,
                    |_app| true, // Always enabled when there's more than one directory
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::DirectorySelectionForRemoval,
                            strings::working_directory::dialog::REMOVE_DIRECTORY_TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ));
            }
            
            actions
        },
        |f, app, area, is_focused| {
            use ratatui::layout::{Constraint, Layout};
            
            let border_style = if is_focused {
                Style::default()
                    .fg(SectionColors::FOCUSED_BORDER)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SectionColors::BORDER)
            };
            
            // Split area into content and actions (both inside the block)
            let inner_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            };
            
            // Use Length(1) when empty or single item to avoid extra blank lines
            // Use Fill(1) when multiple items to allow scrolling
            let list_constraint = if app.state.workdirs.is_empty() || app.state.workdirs.len() == 1 {
                Constraint::Length(1) // Exactly 1 line for "None" or single item
            } else {
                Constraint::Fill(1) // Fill remaining space for multiple items (scrollable)
            };
            
            let chunks = Layout::default()
                .constraints([
                    Constraint::Length(1), // Blank line
                    Constraint::Length(1), // Note
                    Constraint::Length(1), // Blank line
                    list_constraint, // List (1 line if empty, variable if not)
                    Constraint::Length(1), // Blank line
                    Constraint::Length(1), // Actions
                ])
                .split(inner_area);
            
            let _blank1 = chunks[0]; // Blank line - not rendered
            let note_area = chunks[1];
            let _blank2 = chunks[2]; // Blank line - not rendered
            let list_area = chunks[3];
            let _blank3 = chunks[4]; // Blank line - not rendered
            let actions_area = chunks[5];
            
            // Render note (hint)
            let hint = strings::working_directory::HINT;
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Render directories list
            if app.state.workdirs.is_empty() {
                let empty_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled("None", Style::default().fg(SectionColors::VALUE)),
                ]));
                f.render_widget(empty_paragraph, list_area);
            } else if app.state.workdirs.len() == 1 {
                // Single item - render as Paragraph to avoid List widget padding
                let single_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled(
                        app.state.workdirs[0].display().to_string(),
                        Style::default().fg(SectionColors::VALUE),
                    ),
                ]));
                f.render_widget(single_paragraph, list_area);
            } else {
                let items: Vec<ListItem> = app.state.workdirs.iter()
                    .map(|workdir| {
                        ListItem::new(Line::from(vec![
                            Span::styled(
                                workdir.display().to_string(),
                                Style::default().fg(SectionColors::VALUE),
                            ),
                        ]))
                    })
                    .collect();
                
                let list = List::new(items);
                
                // Update list state selection if needed
                let items_len = app.state.workdirs.len();
                {
                    let mut list_state = app.working_directories_list_state.borrow_mut();
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
                    let mut list_state = app.working_directories_list_state.borrow_mut();
                    f.render_stateful_widget(list, list_area, &mut *list_state);
                }
                
                // Update and render scrollbar if content exceeds visible area
                let content_length = items_len;
                let visible_height = list_area.height as usize;
                if content_length > visible_height {
                    // Update scrollbar state
                    let offset = app.working_directories_list_state.borrow().offset();
                    let mut scrollbar_state = app.working_directories_scrollbar_state.borrow_mut();
                    *scrollbar_state = ScrollbarState::new(content_length)
                        .position(offset)
                        .viewport_content_length(visible_height);
                    
                    // Render scrollbar on the right side
                    let scrollbar = Scrollbar::default()
                        .orientation(ScrollbarOrientation::VerticalRight);
                    f.render_stateful_widget(scrollbar, list_area, &mut *scrollbar_state);
                }
            }
            
            // Render action menu inside the block
            let actions = app.sections.iter()
                .find(|s| s.id() == SectionId::WorkingDirectory)
                .map(|s| s.actions(app))
                .unwrap_or_default();
            crate::ui::action_menu::render_action_menu(
                f,
                actions_area,
                &actions,
                if is_focused { app.action_selection } else { None },
                app,
            );
            
            // Render block border last (so it's on top)
            let block = Block::default()
                .title(strings::working_directory::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false, // Input handling will be done via actions
        HEIGHTS,
    )
}

