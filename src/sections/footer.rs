//! Footer section - displays exit instructions

use crate::color::SectionColors;
use crate::section::{Section, SectionId};
use crate::strings;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Footer section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 0,
    note_max: 0,
    content_min: 2, // Two lines: navigation/scrolling help, and quit
    content_max: 2,
    actions_min: 0,
    actions_max: 0,
    border_overhead: 0,
};
use ratatui::{
    style::Style,
    widgets::Paragraph,
};

pub fn create_footer_section() -> Section {
    Section::new(
        SectionId::Footer,
        "",
        |_| String::new(),
        |_| format!("{}\n{}", format!("{} | {}", strings::footer::NAVIGATION, strings::footer::SCROLLING_HELP), strings::footer::QUIT),
        |_| vec![], // No actions
        |f, _app, area, _is_focused| {
            use ratatui::layout::Alignment;
            use ratatui::text::Line;
            
            // Footer is two lines:
            // Line 1: Navigation and scrolling help
            // Line 2: Quit instruction
            let lines = vec![
                Line::from(format!("{} | {}", strings::footer::NAVIGATION, strings::footer::SCROLLING_HELP)),
                Line::from(strings::footer::QUIT),
            ];
            let paragraph = Paragraph::new(lines)
                .style(Style::default().fg(SectionColors::FOOTER))
                .alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        },
        |_app, _key| false, // No input handling
        HEIGHTS,
    )
}

