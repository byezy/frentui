//! Undo section

use crate::action::{Action, ActionResult};
use crate::color::SectionColors;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Undo section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 1,
    note_max: 1,
    content_min: 1, // Value display (single line)
    content_max: 1,
    actions_min: 1,
    actions_max: 1,
    border_overhead: 2,
};
use ratatui::layout::Rect;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn create_undo_section() -> Section {
    Section::new(
        SectionId::Undo,
        strings::undo::TITLE,
        |_| strings::undo::HINT.to_string(),
        |_| String::new(), // No value display
        |_app| {
            vec![
                Action::new(
                    strings::undo::actions::UNDO_RENAMING,
                    move |app| app.state.can_undo,
                    |_app| {
                        // Mark that we need to execute undo
                        // This will be handled asynchronously in main loop
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
            
            let chunks = Layout::default()
                .constraints([
                    Constraint::Length(1), // Blank line
                    Constraint::Length(1), // Note
                    Constraint::Length(1), // Blank line
                    Constraint::Length(HEIGHTS.content_min), // Value (exact height to avoid extra blank lines)
                    Constraint::Length(1), // Blank line
                    Constraint::Length(1), // Actions
                ])
                .split(inner_area);
            
            let _blank1 = chunks[0]; // Blank line - not rendered
            let note_area = chunks[1];
            let _blank2 = chunks[2]; // Blank line - not rendered
            let value_area = chunks[3];
            let _blank3 = chunks[4]; // Blank line - not rendered
            let actions_area = chunks[5];
            
            // Render note (hint)
            let hint = strings::undo::HINT;
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Render value (status)
            let enabled = app.state.can_undo;
            let status = if enabled {
                strings::undo::STATUS_AVAILABLE
            } else {
                strings::undo::STATUS_NOT_AVAILABLE
            };
            let value_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(
                    status,
                    Style::default().fg(if enabled { SectionColors::SUCCESS } else { SectionColors::DISABLED }),
                ),
            ]));
            f.render_widget(value_paragraph, value_area);
            
            let actions = app.sections.iter()
                .find(|s| s.id() == SectionId::Undo)
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
                .title(strings::undo::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false,
        HEIGHTS,
    )
}

