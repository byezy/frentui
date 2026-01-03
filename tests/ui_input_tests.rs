//! Tests for the Input UI widget

use frentui::ui::input::Input;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers, KeyEventState};

fn create_key_event(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
    KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

#[test]
fn test_input_new() {
    let input = Input::new("Test Input");
    assert_eq!(input.value(), "");
    assert_eq!(input.get_path_for_completion(), "");
}

#[test]
fn test_input_with_value() {
    let input = Input::with_value("Test Input", "test.txt".to_string());
    assert_eq!(input.value(), "test.txt");
    assert_eq!(input.get_path_for_completion(), "test.txt");
}

#[test]
fn test_input_handle_key_char() {
    let mut input = Input::new("Test");
    let key = create_key_event(KeyCode::Char('a'), KeyModifiers::NONE);
    
    assert!(input.handle_key(key));
    assert_eq!(input.value(), "a");
}

#[test]
fn test_input_handle_key_char_shift() {
    let mut input = Input::new("Test");
    let key = create_key_event(KeyCode::Char('A'), KeyModifiers::SHIFT);
    
    assert!(input.handle_key(key));
    assert_eq!(input.value(), "A");
}

#[test]
fn test_input_handle_key_backspace() {
    let mut input = Input::with_value("Test", "abc".to_string());
    // Set cursor to end
    let end_key = create_key_event(KeyCode::End, KeyModifiers::NONE);
    input.handle_key(end_key);
    
    let backspace = create_key_event(KeyCode::Backspace, KeyModifiers::NONE);
    assert!(input.handle_key(backspace));
    assert_eq!(input.value(), "ab");
}

#[test]
fn test_input_handle_key_delete() {
    let mut input = Input::with_value("Test", "abc".to_string());
    // Cursor is at end by default with with_value (position 3)
    
    let delete = create_key_event(KeyCode::Delete, KeyModifiers::NONE);
    // Delete at end should do nothing
    assert!(!input.handle_key(delete));
    assert_eq!(input.value(), "abc");
    
    // Move cursor left twice to position 1 (before 'b')
    let left = create_key_event(KeyCode::Left, KeyModifiers::NONE);
    input.handle_key(left); // Now at position 2 (before 'c')
    input.handle_key(left); // Now at position 1 (before 'b')
    
    // Now delete should remove 'b'
    assert!(input.handle_key(delete));
    assert_eq!(input.value(), "ac");
}

#[test]
fn test_input_handle_key_arrow_keys() {
    let mut input = Input::with_value("Test", "abc".to_string());
    
    // Move left
    let left = create_key_event(KeyCode::Left, KeyModifiers::NONE);
    assert!(input.handle_key(left));
    
    // Move right
    let right = create_key_event(KeyCode::Right, KeyModifiers::NONE);
    assert!(input.handle_key(right));
}

#[test]
fn test_input_handle_key_home_end() {
    let mut input = Input::with_value("Test", "abc".to_string());
    
    // Go to home
    let home = create_key_event(KeyCode::Home, KeyModifiers::NONE);
    assert!(input.handle_key(home));
    
    // Go to end
    let end = create_key_event(KeyCode::End, KeyModifiers::NONE);
    assert!(input.handle_key(end));
}

#[test]
fn test_input_handle_key_tab() {
    let mut input = Input::new("Test");
    let tab = create_key_event(KeyCode::Tab, KeyModifiers::NONE);
    
    // Tab should return false (handled by caller)
    assert!(!input.handle_key(tab));
}

#[test]
fn test_input_set_completed_path() {
    let mut input = Input::new("Test");
    input.set_completed_path("/home/user/file.txt".to_string());
    
    assert_eq!(input.value(), "/home/user/file.txt");
    assert_eq!(input.get_path_for_completion(), "/home/user/file.txt");
}

#[test]
fn test_input_backspace_at_start() {
    let mut input = Input::new("Test");
    let backspace = create_key_event(KeyCode::Backspace, KeyModifiers::NONE);
    
    // Backspace at start should do nothing
    assert!(!input.handle_key(backspace));
    assert_eq!(input.value(), "");
}

#[test]
fn test_input_left_at_start() {
    let mut input = Input::new("Test");
    let left = create_key_event(KeyCode::Left, KeyModifiers::NONE);
    
    // Left at start should do nothing
    assert!(!input.handle_key(left));
}

#[test]
fn test_input_right_at_end() {
    let mut input = Input::with_value("Test", "abc".to_string());
    let right = create_key_event(KeyCode::Right, KeyModifiers::NONE);
    
    // Right at end should do nothing
    assert!(!input.handle_key(right));
}

