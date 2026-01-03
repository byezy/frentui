//! Renaming Rule section

use crate::action::{Action, ActionResult};
use crate::color::SectionColors;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::ui::dialog::DialogType;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Renaming Rule section
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

pub fn create_renaming_rule_section() -> Section {
    Section::new(
        SectionId::RenamingRule,
        strings::renaming_rule::TITLE,
        |_| strings::renaming_rule::HINT.to_string(),
        |app| app.state.rename_rule.clone(),
        |_app| {
            vec![
                Action::new(
                    strings::renaming_rule::actions::ENTER_CUSTOM_RULE,
                    |_| true,
                    |app| {
                        let current_rule = app.state.rename_rule.clone();
                        app.input_dialog = Some(crate::ui::dialog::Dialog::with_value(
                            DialogType::RenamingRuleInput,
                            strings::renaming_rule::dialog::TITLE,
                            current_rule,
                        ));
                        ActionResult::DialogOpened
                    },
                ),
                Action::new(
                    strings::renaming_rule::actions::CHOOSE_TEMPLATE,
                    |_| true,
                    |app| {
                        app.input_dialog = Some(crate::ui::dialog::Dialog::new(
                            DialogType::TemplateSelection {
                                field: crate::ui::dialog::TemplateField::RenamingRule,
                            },
                            strings::dialog::template_selection::TITLE,
                        ));
                        ActionResult::DialogOpened
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
            let hint = strings::renaming_rule::HINT;
            let note_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(hint, Style::default().fg(SectionColors::HINT)),
            ]));
            f.render_widget(note_paragraph, note_area);
            
            // Render value (current rename rule)
            let value = app.state.rename_rule.clone();
            let value_paragraph = Paragraph::new(Line::from(vec![
                Span::styled(value, Style::default().fg(SectionColors::VALUE)),
            ]));
            f.render_widget(value_paragraph, value_area);
            
            let actions = app.sections.iter()
                .find(|s| s.id() == SectionId::RenamingRule)
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
                .title(strings::renaming_rule::TITLE)
                .borders(Borders::ALL)
                .border_style(border_style);
            f.render_widget(block, area);
        },
        |_app, _key| false,
        HEIGHTS,
    )
}

