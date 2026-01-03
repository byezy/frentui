//! Tests for the Strings module

use frentui::strings;

#[test]
fn test_header_strings() {
    assert!(!strings::header::TITLE.is_empty());
    assert!(!strings::header::DESCRIPTION.is_empty());
    assert!(strings::header::TITLE.contains("frentui"));
}

#[test]
fn test_footer_strings() {
    assert!(!strings::footer::NAVIGATION.is_empty());
    assert!(strings::footer::NAVIGATION.contains("navigate") || strings::footer::NAVIGATION.contains("quit"));
}

#[test]
fn test_working_directory_strings() {
    assert!(!strings::working_directory::TITLE.is_empty());
    assert!(!strings::working_directory::HINT.is_empty());
    assert!(!strings::working_directory::actions::ADD_DIRECTORY.is_empty());
    assert!(!strings::working_directory::actions::REMOVE_DIRECTORY.is_empty());
    assert!(!strings::working_directory::dialog::TITLE.is_empty());
}

#[test]
fn test_match_files_strings() {
    assert!(!strings::match_files::TITLE.is_empty());
    assert!(!strings::match_files::HINT.is_empty());
    assert!(!strings::match_files::actions::ENTER_CUSTOM_PATTERN.is_empty());
    assert!(!strings::match_files::actions::CHOOSE_TEMPLATE.is_empty());
    assert!(!strings::match_files::dialog::TITLE.is_empty());
}

#[test]
fn test_exclusions_strings() {
    assert!(!strings::exclusions::TITLE.is_empty());
    assert!(!strings::exclusions::HINT.is_empty());
    assert!(!strings::exclusions::NONE.is_empty());
    assert!(!strings::exclusions::actions::ENTER_EXCLUSION_PATTERN.is_empty());
    assert!(!strings::exclusions::actions::CHOOSE_TEMPLATE.is_empty());
    assert!(!strings::exclusions::actions::SPECIFY_FILES.is_empty());
    assert!(!strings::exclusions::actions::CLEAR.is_empty());
}

#[test]
fn test_renaming_rule_strings() {
    assert!(!strings::renaming_rule::TITLE.is_empty());
    assert!(!strings::renaming_rule::HINT.is_empty());
    assert!(!strings::renaming_rule::actions::ENTER_CUSTOM_RULE.is_empty());
    assert!(!strings::renaming_rule::actions::CHOOSE_TEMPLATE.is_empty());
}

#[test]
fn test_preview_strings() {
    assert!(!strings::preview::TITLE.is_empty());
    assert!(!strings::preview::HINT.is_empty());
    assert!(!strings::preview::EMPTY.is_empty());
}

#[test]
fn test_validation_strings() {
    assert!(!strings::validation::TITLE.is_empty());
    assert!(!strings::validation::PENDING.is_empty());
    assert!(!strings::validation::ALL_VALID.is_empty());
    assert!(!strings::validation::ALL_VALID_MESSAGE.is_empty());
}

#[test]
fn test_apply_strings() {
    assert!(!strings::apply::TITLE.is_empty());
    assert!(!strings::apply::HINT.is_empty());
    assert!(!strings::apply::STATUS_READY.is_empty());
    assert!(!strings::apply::STATUS_VALIDATION_REQUIRED.is_empty());
}

#[test]
fn test_undo_strings() {
    assert!(!strings::undo::TITLE.is_empty());
    assert!(!strings::undo::HINT.is_empty());
    assert!(!strings::undo::STATUS_AVAILABLE.is_empty());
    assert!(!strings::undo::STATUS_NOT_AVAILABLE.is_empty());
}

#[test]
fn test_dialog_strings() {
    assert!(!strings::dialog::hints::DIRECTORY_SELECTION.is_empty());
    assert!(!strings::dialog::hints::MATCH_PATTERN_INPUT.is_empty());
    assert!(!strings::dialog::hints::EXCLUSION_PATTERN_INPUT.is_empty());
    assert!(!strings::dialog::hints::RENAMING_RULE_INPUT.is_empty());
    assert!(!strings::dialog::actions::SAVE.is_empty());
    assert!(!strings::dialog::actions::CANCEL.is_empty());
}
