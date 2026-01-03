//! Tests for the Event module

use frentui::event::{AppEvent, EventHandler};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[test]
fn test_event_handler_new() {
    let _handler = EventHandler::new(100);
    // Handler should be created successfully
    // We can't easily test the thread, but we can verify it doesn't panic
    assert!(true); // Placeholder - handler creation is the test
}

#[test]
fn test_app_event_debug() {
    let event = AppEvent::Tick;
    let debug_str = format!("{:?}", event);
    assert!(debug_str.contains("Tick"));
}

#[test]
fn test_app_event_key() {
    let key_event = KeyEvent {
        code: KeyCode::Char('a'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };
    let app_event = AppEvent::Key(key_event);
    
    // Verify we can create the event
    match app_event {
        AppEvent::Key(_) => assert!(true),
        AppEvent::Tick => panic!("Expected Key event"),
        AppEvent::Mouse(_) => panic!("Expected Key event"),
    }
}

// Note: Testing EventHandler::next() is difficult because it blocks waiting for events
// and requires a terminal. These tests focus on what can be tested without terminal access.

