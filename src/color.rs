//! Centralized color definitions for section parts
//!
//! Single source of truth for all color settings used throughout the TUI.
//! All colors are defined as constants in the `SectionColors` struct.
//!
//! ## Color Organization
//!
//! Colors are organized by their usage:
//! - **Text colors**: HINT, VALUE, ACTION, DISABLED, ERROR, SUCCESS, FOOTER, BRANDING
//! - **Border colors**: BORDER, FOCUSED_BORDER
//! - **Highlight colors**: HIGHLIGHT (for selected items)
//!
//! ## Usage
//!
//! ```rust
//! use frentui::color::SectionColors;
//! use ratatui::style::Style;
//!
//! let style = Style::default().fg(SectionColors::HINT);
//! ```
//!
//! ## Color Values
//!
//! - **HINT**: Rgb(150, 150, 150) - Gray for hints/notes
//! - **VALUE**: Yellow - Highlighted values
//! - **BORDER**: Cyan - Section borders
//! - **FOCUSED_BORDER**: Yellow - Focused section borders (with bold modifier)
//! - **HIGHLIGHT**: Magenta - Selected/highlighted items
//! - **ACTION**: White - Action menu items
//! - **DISABLED**: Rgb(150, 150, 150) - Disabled actions (same as HINT)
//! - **ERROR**: Red - Validation errors
//! - **SUCCESS**: Green - Validation success
//! - **FOOTER**: Rgb(100, 100, 100) - Footer text (darker gray)
//! - **BRANDING**: Yellow - Branding/header text

use ratatui::style::Color;

/// Color definitions for section components
///
/// All colors used in the TUI are defined here as constants.
/// This ensures consistency and makes it easy to adjust the color scheme.
pub struct SectionColors;

impl SectionColors {
    /// Color for section hints/notes (improved contrast, darker gray)
    pub const HINT: Color = Color::Rgb(150, 150, 150);
    
    /// Color for section values (highlighted, yellow)
    pub const VALUE: Color = Color::Yellow;
    
    /// Color for section borders
    pub const BORDER: Color = Color::Cyan;
    
    /// Color for highlighted/selected section (when focused)
    pub const HIGHLIGHT: Color = Color::Magenta;
    
    /// Color for focused section border (yellow+bold)
    pub const FOCUSED_BORDER: Color = Color::Yellow;
    
    /// Color for action menu items
    pub const ACTION: Color = Color::White;
    
    /// Color for disabled actions (same as note text for consistency)
    pub const DISABLED: Color = Color::Rgb(150, 150, 150);
    
    /// Color for validation errors
    pub const ERROR: Color = Color::Red;
    
    /// Color for validation success
    pub const SUCCESS: Color = Color::Green;
    
    /// Color for footer text
    pub const FOOTER: Color = Color::Rgb(100, 100, 100);
    
    /// Color for branding text
    pub const BRANDING: Color = Color::Yellow;
}
