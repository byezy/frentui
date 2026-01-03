//! Start screen step

use crate::app::App;
use crate::ui::layout::StepLayout;
use ratatui::{Frame, layout::Rect};
use ratatui::{
    style::Color,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_start(f: &mut Frame, _app: &App, step_area: Rect) {
    let layout = StepLayout::without_content(step_area);
    
    // 1. Header with brief description
    let description = "This tool will guide you through the file renaming process step by step.";
    StepLayout::render_header(f, layout.header, "frentui - Interactive File Renaming", Some(description));
    
    // 2. Input area - show instructions content
    let instructions = Block::default()
        .borders(Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(Color::Cyan))
        .padding(ratatui::widgets::Padding {
            left: 1,
            right: 1,
            top: 1,
            bottom: 1,
        });
    
    let text = "You can navigate using:\n\
                • Arrow keys or h/l to move\n\
                • Enter to select\n\
                • q or Esc to quit\n\
                • h to go back\n\n\
                Press any key to continue...";
    
    let paragraph = Paragraph::new(text)
        .block(instructions)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, layout.input);
}

pub fn handle_start_input(app: &mut App, key: crossterm::event::KeyEvent) -> bool {
    use crossterm::event::KeyCode;
    
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
            true
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.go_next();
            true
        }
        _ => false,
    }
}
