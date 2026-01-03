//! Exclusions section

use crate::action::{Action, ActionResult};
use crate::color::SectionColors;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::ui::dialog::DialogType;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Exclusions section
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

pub fn create_exclusions_section() -> Section {
    Section::new(
        SectionId::Exclusions,
        strings::exclusions::TITLE,
        |_| strings::exclusions::HINT.to_string(),
        |app| {
            if app.state.exclude.is_empty() {
                strings::exclusions::NONE.to_string()
            } else {
                app.state.exclude.join(", ")
            }
        },
        |_app| {
            vec![
                Action::new(
                    strings::exclusions::actions::ENTER_EXCLUSION_PATTERN,
                    |_| true,
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::ExclusionPatternInput,
                            strings::exclusions::dialog::EXCLUSION_PATTERN_TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::exclusions::actions::CHOOSE_TEMPLATE,
                    |_| true,
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::TemplateSelection {
                                field: crate::ui::dialog::TemplateField::ExclusionPattern,
                            },
                            strings::dialog::template_selection::TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::exclusions::actions::SPECIFY_FILES,
                    |app| !app.state.list.is_empty(),
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::FileSelection,
                            strings::exclusions::dialog::FILE_SELECTION_TITLE,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::exclusions::actions::CLEAR,
                    |app| !app.state.exclude.is_empty(),
                    |app| {
                        app.state.exclude.clear();
                        ActionResult::StateUpdated
                    },
                ),
            ]
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
            
            let inner_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            };
            
            // Use Length(1) when empty or single item to avoid extra blank lines
            // Use Fill(1) when multiple items to allow scrolling
            let list_constraint = if app.state.exclude.is_empty() || app.state.exclude.len() == 1 {
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
            let hint = strings::exclusions::HINT;
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Render exclusions list
            if app.state.exclude.is_empty() {
                let empty_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled(strings::exclusions::NONE, Style::default().fg(SectionColors::VALUE)),
                ]));
                f.render_widget(empty_paragraph, list_area);
            } else if app.state.exclude.len() == 1 {
                // Single item - render as Paragraph to avoid List widget padding
                let workdirs_canonical: Vec<_> = app.state.workdirs.iter()
                    .filter_map(|w| w.canonicalize().ok())
                    .collect();
                let exclusion = &app.state.exclude[0];
                let display_text = if let Ok(path) = std::path::PathBuf::from(exclusion).canonicalize() {
                    // It's a file path - check if it's within any workdir
                    let is_in_workdir = workdirs_canonical.iter()
                        .any(|workdir| path.starts_with(workdir));
                    if is_in_workdir {
                        // Within a workdir - show filename only
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| exclusion.clone())
                    } else {
                        // Outside all workdirs - show full path
                        path.display().to_string()
                    }
                } else {
                    // It's a pattern, not a file path - show as-is
                    exclusion.clone()
                };
                let single_paragraph = Paragraph::new(Line::from(vec![
                    Span::styled(display_text, Style::default().fg(SectionColors::VALUE)),
                ]));
                f.render_widget(single_paragraph, list_area);
            } else {
                // Format paths: show full path if not in any workdir, filename only if in a workdir
                let workdirs_canonical: Vec<_> = app.state.workdirs.iter()
                    .filter_map(|w| w.canonicalize().ok())
                    .collect();
                let items: Vec<ListItem> = app.state.exclude.iter()
                    .map(|exclusion| {
                        // Check if it's a file path (try to parse as PathBuf and canonicalize)
                        let display_text = if let Ok(path) = std::path::PathBuf::from(exclusion).canonicalize() {
                            // It's a file path - check if it's within any workdir
                            let is_in_workdir = workdirs_canonical.iter()
                                .any(|workdir| path.starts_with(workdir));
                            if is_in_workdir {
                                // Within a workdir - show filename only
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| exclusion.clone())
                            } else {
                                // Outside all workdirs - show full path
                                path.display().to_string()
                            }
                        } else {
                            // It's a pattern, not a file path - show as-is
                            exclusion.clone()
                        };
                        ListItem::new(Line::from(vec![
                            Span::styled(display_text, Style::default().fg(SectionColors::VALUE)),
                        ]))
                    })
                    .collect();
                
                let list = List::new(items);
                
                // Update list state selection if needed
                let items_len = app.state.exclude.len();
                {
                    let mut list_state = app.exclusions_list_state.borrow_mut();
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
                    let mut list_state = app.exclusions_list_state.borrow_mut();
                    f.render_stateful_widget(list, list_area, &mut *list_state);
                }
                
                // Update and render scrollbar if content exceeds visible area
                let content_length = items_len;
                let visible_height = list_area.height as usize;
                if content_length > visible_height {
                    // Update scrollbar state
                    let offset = app.exclusions_list_state.borrow().offset();
                    let mut scrollbar_state = app.exclusions_scrollbar_state.borrow_mut();
                    *scrollbar_state = ScrollbarState::new(content_length)
                        .position(offset)
                        .viewport_content_length(visible_height);
                    
                    // Render scrollbar on the right side
                    let scrollbar = Scrollbar::default()
                        .orientation(ScrollbarOrientation::VerticalRight);
                    f.render_stateful_widget(scrollbar, list_area, &mut *scrollbar_state);
                }
            }
            
            let actions = app.sections.iter()
                .find(|s| s.id() == SectionId::Exclusions)
                .map(|s| s.actions(app))
                .unwrap_or_default();
            crate::ui::action_menu::render_action_menu(
                f,
                actions_area,
                &actions,
                if is_focused { app.action_selection } else { None },
                app,
            );
            
            let block = Block::default()
                .title(strings::exclusions::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false,
        HEIGHTS,
    )
}

