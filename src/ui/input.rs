//! Text input widget for ratatui

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

/// Text input widget
pub struct Input {
    title: String,
    value: String,
    cursor_pos: usize,
}

impl Input {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: String::new(),
            cursor_pos: 0,
        }
    }
    
    pub fn with_value(title: impl Into<String>, value: String) -> Self {
        let cursor_pos = value.len();
        Self {
            title: title.into(),
            value,
            cursor_pos,
        }
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }
    
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> bool {
        use crossterm::event::{KeyCode, KeyModifiers};
        
        match key.code {
            KeyCode::Char(c) => {
                // Accept characters with no modifiers, or with Shift (for capital letters)
                if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
                    self.value.insert(self.cursor_pos, c);
                    self.cursor_pos += 1;
                    true
                } else {
                    false
                }
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    self.value.remove(self.cursor_pos);
                    true
                } else {
                    false
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.value.len() {
                    self.value.remove(self.cursor_pos);
                    true
                } else {
                    false
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    true
                } else {
                    false
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.value.len() {
                    self.cursor_pos += 1;
                    true
                } else {
                    false
                }
            }
            KeyCode::Home => {
                self.cursor_pos = 0;
                true
            }
            KeyCode::End => {
                self.cursor_pos = self.value.len();
                true
            }
            KeyCode::Tab => {
                // Tab completion - handled by caller
                false
            }
            _ => false,
        }
    }
    
    /// Get the current path for tab completion
    pub fn get_path_for_completion(&self) -> String {
        self.value.clone()
    }
    
    /// Set the completed path
    pub fn set_completed_path(&mut self, path: String) {
        self.value = path;
        self.cursor_pos = self.value.len();
    }
    
    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .padding(Padding {
                left: 1,
                right: 1,
                top: 1,
                bottom: 1,
            });
        
        // Show value with cursor
        let display_value = if self.value.is_empty() {
            " ".to_string() // Show at least one space so cursor is visible
        } else {
            self.value.clone()
        };
        
        let paragraph = Paragraph::new(display_value.as_str())
            .block(block)
            .style(Style::default().fg(Color::Yellow));
        
        f.render_widget(paragraph, area);
        
        // Render cursor (accounting for padding: 1 char left + border)
        let cursor_x = area.x + 1 + 1 + (self.cursor_pos.min(self.value.len()) as u16);
        let cursor_y = area.y + 1 + 1; // 1 for border, 1 for top padding
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

