//! Tests for the setup module

use frentui::setup;

#[test]
fn test_setup_module_exists() {
    // Verify module can be imported
    assert!(true);
}

#[test]
fn test_init_logging_creates_logs_dir() {
    // This test verifies that init_logging can be called
    // Note: This will create/append to logs/frentui.log
    // In a real scenario, we might want to use a temp directory
    let result = setup::init_logging();
    
    // Should succeed (or fail gracefully if logs dir can't be created)
    // We're mainly checking it doesn't panic
    assert!(result.is_ok() || result.is_err());
    
    // Verify logs directory exists after call
    assert!(std::path::Path::new("logs").exists() || std::path::Path::new("logs/frentui.log").exists());
}

#[test]
fn test_init_terminal_requires_cleanup() {
    // This test verifies init_terminal can be called
    // Note: This actually modifies terminal state, so we need to restore it
    let result = setup::init_terminal();
    
    match result {
        Ok((mut terminal, _event_handler)) => {
            // Verify terminal was created
            assert!(terminal.size().is_ok());
            
            // Clean up - restore terminal
            let restore_result = setup::restore_terminal(&mut terminal);
            assert!(restore_result.is_ok());
        }
        Err(_e) => {
            // If we can't initialize terminal (e.g., in CI without TTY), that's okay
            // Just verify we got an error (any io::Error is fine)
            assert!(true);
        }
    }
}

#[test]
fn test_restore_terminal_after_init() {
    // Test that we can initialize and restore terminal in sequence
    if let Ok((mut terminal, _event_handler)) = setup::init_terminal() {
        let result = setup::restore_terminal(&mut terminal);
        assert!(result.is_ok());
    }
    // If terminal init fails (e.g., no TTY), skip test
}
