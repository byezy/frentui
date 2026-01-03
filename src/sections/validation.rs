//! Validation section

use crate::color::SectionColors;
use crate::section::{Section, SectionId};
use crate::strings;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Validation section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 1,
    note_max: 1,
    content_min: 3, // At least 3 lines for list
    content_max: 15, // Can grow to show more issues (scrollable if more)
    actions_min: 0,
    actions_max: 0,
    border_overhead: 2,
};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
};

pub fn create_validation_section() -> Section {
    Section::new(
        SectionId::Validation,
        strings::validation::TITLE,
        |app| {
            if let Some(ref validation) = app.state.validation {
                if validation.issues.is_empty() {
                    strings::validation::ALL_VALID_MESSAGE.to_string()
                } else {
                    format!("Found {} issue(s). Review and fix before applying.", validation.issues.len())
                }
            } else {
                strings::validation::PENDING.to_string()
            }
        },
        |_| String::new(), // Value shown in list
        |_app| vec![], // No actions - not enterable
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
            
            // Split into note and list areas
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
            
            // Note - always show
            let hint: String = if let Some(ref validation) = app.state.validation {
                if validation.issues.is_empty() {
                    strings::validation::ALL_VALID_MESSAGE.to_string()
                } else {
                    format!("Found {} issue(s). Review and fix before applying.", validation.issues.len())
                }
            } else {
                strings::validation::PENDING.to_string()
            };
            
            let note_paragraph = ratatui::widgets::Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Create table rows from validation issues
            let rows: Vec<Row> = if let Some(ref validation) = app.state.validation {
                if validation.issues.is_empty() {
                    // Style "All valid" message with SUCCESS color
                    vec![Row::new(vec![
                        Cell::from(strings::validation::ALL_VALID)
                            .style(Style::default().fg(SectionColors::SUCCESS)),
                        Cell::from(""),
                        Cell::from(""),
                    ])]
                } else {
                    // Create rows with owned strings and proper styling
                    validation.issues.iter()
                        .map(|(old_path, issue)| {
                            let old_name = old_path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("?")
                                .to_string();
                            
                            // Get new name from state if available
                            let new_name = app.state.list.iter()
                                .position(|p| p == old_path)
                                .and_then(|idx| app.state.new_names.get(idx))
                                .map(|n| n.clone())
                                .unwrap_or_else(|| "?".to_string());
                            
                            let reason = format!("{:?}", issue);
                            
                            // Create row with owned strings
                            // Style the issue/reason column with ERROR color
                            Row::new(vec![
                                Cell::from(old_name),
                                Cell::from(new_name),
                                Cell::from(reason).style(Style::default().fg(SectionColors::ERROR)),
                            ])
                        })
                        .collect()
                }
            } else {
                vec![Row::new([strings::validation::NOT_VALIDATED, "", ""])]
            };
            
            // Create table with header
            let table = Table::new(rows, [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ])
            .header(Row::new(["Current Name", "New Name", "Issue"])
                .style(Style::default().add_modifier(Modifier::BOLD)))
            .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            
            // Update table state selection if needed
            let items_len = app.state.validation
                .as_ref()
                .map(|v| v.issues.len())
                .unwrap_or(0);
            {
                let mut table_state = app.validation_table_state.borrow_mut();
                if let Some(sel) = table_state.selected() {
                    if sel >= items_len && items_len > 0 {
                        table_state.select(Some(0));
                    }
                } else if items_len > 0 {
                    table_state.select(Some(0));
                }
            }
            
            // Render table (this will mutate the state to update offset)
            {
                let mut table_state = app.validation_table_state.borrow_mut();
                f.render_stateful_widget(table, list_area, &mut *table_state);
            }
            
            // Update and render scrollbar if content exceeds visible area
            let content_length = items_len;
            let visible_height = list_area.height.saturating_sub(1) as usize; // Subtract 1 for header
            if content_length > visible_height {
                // Update scrollbar state
                let offset = app.validation_table_state.borrow().offset();
                let mut scrollbar_state = app.validation_scrollbar_state.borrow_mut();
                *scrollbar_state = ScrollbarState::new(content_length)
                    .position(offset)
                    .viewport_content_length(visible_height);
                
                // Render scrollbar on the right side
                let scrollbar = Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight);
                f.render_stateful_widget(scrollbar, list_area, &mut *scrollbar_state);
            }
            
            // Render block border to wrap the entire section (note + list)
            let block = Block::default()
                .title(strings::validation::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false, // Not enterable
        HEIGHTS,
    )
}

