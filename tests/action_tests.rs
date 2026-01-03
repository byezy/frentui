//! Tests for the Action module

use frentui::action::{Action, ActionResult};
use frentui::app::App;

#[test]
fn test_action_new() {
    let action = Action::new(
        "Test Action",
        |_| true,
        |_| ActionResult::NoChange,
    );
    
    assert_eq!(action.name(), "Test Action");
}

#[test]
fn test_action_always_enabled() {
    let action = Action::always_enabled(
        "Always Enabled",
        |_| ActionResult::NoChange,
    );
    
    let app = App::new();
    assert!(action.enabled(&app));
}

#[test]
fn test_action_enabled_conditional() {
    let enabled = true;
    let action = Action::new(
        "Conditional",
        move |_| enabled,
        |_| ActionResult::NoChange,
    );
    
    let app = App::new();
    assert!(action.enabled(&app));
    
    // enabled = false;
    // Note: closure captures the value, so this won't change the behavior
    // This is just testing the structure
}

#[test]
fn test_action_result_variants() {
    assert_eq!(ActionResult::NoChange, ActionResult::NoChange);
    assert_eq!(ActionResult::StateUpdated, ActionResult::StateUpdated);
    assert_eq!(ActionResult::DialogOpened, ActionResult::DialogOpened);
    assert_eq!(ActionResult::Cancelled, ActionResult::Cancelled);
    
    assert_ne!(ActionResult::NoChange, ActionResult::StateUpdated);
}

#[test]
fn test_action_run() {
    let mut app = App::new();
    let action = Action::always_enabled(
        "Test",
        |_| ActionResult::StateUpdated,
    );
    
    let result = action.run(&mut app);
    assert_eq!(result, ActionResult::StateUpdated);
}

#[test]
fn test_action_name() {
    let action = Action::always_enabled(
        "My Action",
        |_| ActionResult::NoChange,
    );
    
    assert_eq!(action.name(), "My Action");
}

