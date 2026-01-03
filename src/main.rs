//! frentui - Interactive TUI for batch file renaming
//!
//! This tool provides a single-screen TUI for batch file renaming
//! using the freneng library. All sections are visible simultaneously.
//!
//! ## Architecture
//!
//! **main.rs is intentionally minimal** - it serves only as the entry point and
//! high-level glue code that wires together components. All business logic,
//! rendering, and input handling has been factored out into dedicated modules:
//!
//! - `ui::render` - All UI rendering logic (section layout, dialogs)
//! - `ui::handler` - All key input handling logic
//! - `ui::completion` - Path completion utilities
//! - `sections::*` - Individual section definitions
//! - `app` - Application state and coordination
//! - `state` - State computation and management
//!
//! **DO NOT add business logic to main.rs** - if you need to add functionality,
//! place it in the appropriate module. This keeps the codebase maintainable and
//! prevents regression to a bloated main.rs file.

mod app;
mod args;
mod event;
mod setup;
mod state;
mod section;
mod action;
mod color;
mod strings;
mod templates;
mod ui;
mod sections;

use app::App;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    setup::init_logging()?;
    
    let (mut terminal, event_handler) = setup::init_terminal()?;
    
    let mut app = App::new();
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        args::process_args(&mut app, args)?;
    }
    
    app.initialize().await?;
    
    app.run(&mut terminal, &event_handler).await?;
    
    setup::restore_terminal(&mut terminal)?;
    
    Ok(())
}

