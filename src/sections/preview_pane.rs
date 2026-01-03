//! Preview Pane section - shows file mappings

use crate::color::SectionColors;
use crate::section::{Section, SectionId};
use crate::strings;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Preview Pane section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 1,
    note_max: 1,
    content_min: 3, // At least 3 lines for list
    content_max: 20, // Can grow to show more files (scrollable if more)
    actions_min: 0,
    actions_max: 0,
    border_overhead: 2,
};
use ratatui::{
    // layout::Constraint,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
};

pub fn create_preview_pane_section() -> Section {
    Section::new(
        SectionId::PreviewPane,
        strings::preview::TITLE,
        |_| strings::preview::HINT.to_string(),
        |_| String::new(), // Value is shown in the list
        |_app| vec![], // No actions
        |f, app, area, is_focused| {
            use ratatui::layout::{Constraint, Layout, Rect};
            
            let border_style = if is_focused {
                Style::default()
                    .fg(SectionColors::FOCUSED_BORDER)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(SectionColors::BORDER)
            };
            
            // Split into note and list areas (inside block)
            let inner_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width.saturating_sub(2),
                height: area.height.saturating_sub(2),
            };
            
            let chunks = Layout::default()
                .constraints([
                    Constraint::Length(1), // Blank line
                    Constraint::Length(1), // Note
                    Constraint::Length(1), // Blank line
                    Constraint::Min(1),    // Table (Value)
                ])
                .split(inner_area);
            
            let _blank1 = chunks[0]; // Blank line - not rendered
            let note_area = chunks[1];
            let _blank2 = chunks[2]; // Blank line - not rendered
            let list_area = chunks[3];
            
            // Render note (hint)
            // Show warning if renames have been applied (can_undo is true)
            let (hint, hint_style) = if app.state.can_undo {
                (
                    strings::preview::HINT_AFTER_RENAME,
                    Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                (
                    strings::preview::HINT,
                    Style::default().fg(SectionColors::HINT),
                )
            };
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, hint_style),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Use temp_list and temp_new_names if available (live preview when editing match pattern)
            let (list_to_use, new_names_to_use) = if !app.temp_list.is_empty() && !app.temp_new_names.is_empty() 
                && app.temp_list.len() == app.temp_new_names.len() {
                (&app.temp_list, &app.temp_new_names)
            } else {
                (&app.state.list, &app.state.new_names)
            };
            
            // Create table rows from list and new_names
            let rows: Vec<Row> = if list_to_use.is_empty() || new_names_to_use.is_empty() {
                // Show empty state
                vec![Row::new([strings::preview::EMPTY, ""])]
            } else {
                // Use the minimum length to ensure we don't panic if lists are mismatched
                let min_len = list_to_use.len().min(new_names_to_use.len());
                list_to_use.iter()
                    .take(min_len)
                    .zip(new_names_to_use.iter().take(min_len))
                    .map(|(old_path, new_name)| {
                        // Show "does not exist" for all entries if:
                        // 1. Renames have been applied (can_undo is true)
                        // 2. We're showing the actual state (not temp preview)
                        // 3. The old file doesn't exist (it was renamed)
                        let is_using_temp = !app.temp_list.is_empty();
                        let old_name = if app.state.can_undo && !is_using_temp && !old_path.exists() {
                            // After renaming, all old files that don't exist should show "does not exist"
                            "does not exist"
                        } else {
                            old_path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("?")
                        };
                        Row::new([old_name, new_name.as_str()])
                    })
                    .collect()
            };
            
            // Create table with header
            // Header stays the same even after renaming (no "(stale)" indicator)
            let current_name_header = "Current Name";
            let header_style = Style::default().add_modifier(Modifier::BOLD);
            let table = Table::new(rows, [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .header(Row::new([current_name_header, "New Name"])
                .style(header_style))
            .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            
            // Update table state selection if needed
            let items_len = list_to_use.len().min(new_names_to_use.len());
            {
                let mut table_state = app.preview_table_state.borrow_mut();
                if let Some(sel) = table_state.selected() {
                    if sel >= items_len && items_len > 0 {
                        table_state.select(Some(0));
                    }
                } else if items_len > 0 && !app.state.list.is_empty() {
                    table_state.select(Some(0));
                }
            }
            
            // Render table (this will mutate the state to update offset)
            {
                let mut table_state = app.preview_table_state.borrow_mut();
                f.render_stateful_widget(table, list_area, &mut *table_state);
            }
            
            // Update and render scrollbar if content exceeds visible area
            let content_length = items_len;
            let visible_height = list_area.height.saturating_sub(1) as usize; // Subtract 1 for header
            if content_length > visible_height {
                // Update scrollbar state
                let offset = app.preview_table_state.borrow().offset();
                let mut scrollbar_state = app.preview_scrollbar_state.borrow_mut();
                *scrollbar_state = ScrollbarState::new(content_length)
                    .position(offset)
                    .viewport_content_length(visible_height);
                
                // Render scrollbar on the right side
                let scrollbar = Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight);
                f.render_stateful_widget(scrollbar, list_area, &mut *scrollbar_state);
            }
            
            // Render block border last (so it's on top) - wraps the entire section (note + list)
            let block = Block::default()
                .title(strings::preview::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false, // Scrolling will be handled at app level
        HEIGHTS,
    )
}

