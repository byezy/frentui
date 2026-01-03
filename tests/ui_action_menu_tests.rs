//! Tests for the Action Menu UI module

use frentui::action::{Action, ActionResult};

#[test]
fn test_action_menu_actions_exist() {
    // Verify we can create actions for testing
    let action = Action::always_enabled(
        "Test Action",
        |_| ActionResult::NoChange,
    );
    
    assert_eq!(action.name(), "Test Action");
}
