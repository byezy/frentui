//! Event handling for user input

use crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events
#[derive(Clone, Debug)]
pub enum AppEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
}

/// Event handler
pub struct EventHandler {
    rx: mpsc::Receiver<AppEvent>,
    _tx: mpsc::Sender<AppEvent>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();
        
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            let tick_duration = Duration::from_millis(tick_rate);
            
            loop {
                let timeout = tick_duration
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));
                
                // Use catch_unwind to prevent panics from propagating
                // Crossterm can panic on malformed mouse events during polling or reading
                let event_result = std::panic::catch_unwind(|| {
                    match event::poll(timeout) {
                        Ok(true) => event::read(),
                        Ok(false) => Ok(Event::Resize(0, 0)), // Dummy event to indicate no event
                        Err(_) => Ok(Event::Resize(0, 0)), // Error polling - ignore
                    }
                });
                
                match event_result {
                    Ok(Ok(Event::Key(key))) => {
                        if key.kind == KeyEventKind::Press {
                            let _ = tx_clone.send(AppEvent::Key(key));
                        }
                    }
                    Ok(Ok(Event::Mouse(mouse))) => {
                        // Validate mouse coordinates before sending
                        // Mouse coordinates should be 1-based and within reasonable bounds
                        // Filter out invalid events that might cause panics
                        if mouse.row > 0 && mouse.column > 0 && 
                           mouse.row < 10000 && mouse.column < 10000 {
                            let _ = tx_clone.send(AppEvent::Mouse(mouse));
                        }
                    }
                    Ok(Ok(Event::Resize(_, _))) => {
                        // Dummy event from poll timeout - ignore
                    }
                    Ok(Ok(_)) => {}
                    Ok(Err(_)) => {
                        // Ignore parse errors - they can happen with invalid escape sequences
                    }
                    Err(_) => {
                        // Panic was caught - ignore malformed events
                        // This prevents the entire application from crashing
                        // The panic likely occurred during mouse event parsing
                    }
                }
                
                if last_tick.elapsed() >= tick_duration {
                    let _ = tx_clone.send(AppEvent::Tick);
                    last_tick = Instant::now();
                }
            }
        });
        
        Self { rx, _tx: tx }
    }
    
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.rx.recv()
    }
}

