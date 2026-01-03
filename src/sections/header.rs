//! Header section - displays app branding and version

use crate::color::SectionColors;
use crate::section::{Section, SectionId};
use crate::strings;
use crate::ui::section_layout::SectionHeights;

/// Height configuration for Header section
/// Defined within this module as per design requirement
pub const HEIGHTS: SectionHeights = SectionHeights {
    note_min: 0,
    note_max: 0,
    content_min: 1,
    content_max: 1,
    actions_min: 0,
    actions_max: 0,
    border_overhead: 0,
};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::Paragraph,
};

pub fn create_header_section() -> Section {
    Section::new(
        SectionId::Header,
        strings::header::TITLE,
        |_| strings::header::DESCRIPTION.to_string(),
        |_| String::new(), // No value display
        |_| vec![], // No actions
        |f, _app, area, _is_focused| {
            let version = env!("CARGO_PKG_VERSION");
            let branding_text = format!("frentui v{}", version);
            
            let paragraph = Paragraph::new(Span::styled(
                branding_text,
                Style::default()
                    .fg(SectionColors::BRANDING)
                    .add_modifier(Modifier::BOLD)
            ));
            f.render_widget(paragraph, area);
        },
        |_app, _key| false, // No input handling
        HEIGHTS,
    )
}

