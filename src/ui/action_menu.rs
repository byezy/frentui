//! Action menu widget for displaying section actions

use crate::action::Action;
use crate::color::SectionColors;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render a horizontal action menu
pub fn render_action_menu(
    f: &mut Frame,
    area: Rect,
    actions: &[Action],
    selected_idx: Option<usize>,
    app: &crate::app::App,
) {
    if actions.is_empty() {
        return;
    }
    
    let selected = selected_idx.unwrap_or(0);
    
    // Create action items as spans
    let mut spans = Vec::new();
    for (idx, action) in actions.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::styled(" | ", Style::default().fg(SectionColors::HINT)));
        }
        
        let enabled = action.enabled(app);
        let is_selected = idx == selected;
        
        let style = if !enabled {
            Style::default().fg(SectionColors::DISABLED)
        } else if is_selected {
            Style::default()
                .fg(SectionColors::HIGHLIGHT)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(SectionColors::ACTION)
        };
        
        let prefix = if is_selected { "> " } else { "  " };
        spans.push(Span::styled(format!("{}{}", prefix, action.name()), style));
    }
    
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .block(Block::default().borders(Borders::NONE));
    
    f.render_widget(paragraph, area);
}

