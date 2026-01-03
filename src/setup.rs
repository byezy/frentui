//! Terminal and logging setup/teardown

use crate::event::EventHandler;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::io;

/// Initialize file-based logging
/// Creates logs directory if needed and sets up env_logger to write to logs/frentui.log
pub fn init_logging() -> io::Result<()> {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("logs").unwrap_or_else(|_| {
        eprintln!("Warning: Could not create logs directory");
    });
    
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/frentui.log")
        .unwrap_or_else(|_| {
            // Fallback to stderr if file can't be opened
            eprintln!("Warning: Could not open logs/frentui.log, logging to stderr");
            std::process::exit(1);
        });
    
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_secs()
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
    
    log::info!("Starting frentui");
    
    Ok(())
}

/// Initialize terminal and create event handler
/// Sets up raw mode, alternate screen, and mouse capture
pub fn init_terminal() -> io::Result<(Terminal<CrosstermBackend<io::Stdout>>, EventHandler)> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    
    // Create event handler
    let event_handler = EventHandler::new(250); // 250ms tick rate
    
    Ok((terminal, event_handler))
}

/// Restore terminal to its original state
/// Disables raw mode, leaves alternate screen, and disables mouse capture
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}
