//! Tests for the Dialog UI module

use frentui::ui::dialog::{Dialog, DialogType, TemplateField};

#[test]
fn test_dialog_new() {
    let dialog = Dialog::new(DialogType::DirectorySelection, "Test Dialog");
    assert!(matches!(dialog.dialog_type, DialogType::DirectorySelection));
    assert_eq!(dialog.input.value(), "");
    assert!(dialog.selected_files.is_empty());
}

#[test]
fn test_dialog_with_value() {
    let dialog = Dialog::with_value(
        DialogType::MatchPatternInput,
        "Pattern Input",
        "*.txt".to_string(),
    );
    assert!(matches!(dialog.dialog_type, DialogType::MatchPatternInput));
    assert_eq!(dialog.input.value(), "*.txt");
}

#[test]
fn test_dialog_template_selection_initialized() {
    let dialog = Dialog::new(
        DialogType::TemplateSelection {
            field: TemplateField::MatchPattern,
        },
        "Select Template",
    );
    
    // Template selection should start with first template selected
    assert_eq!(dialog.selected_template, Some(0));
}

#[test]
fn test_dialog_type_variants() {
    let _dir = DialogType::DirectorySelection;
    let _match = DialogType::MatchPatternInput;
    let _exclude = DialogType::ExclusionPatternInput;
    let _rename = DialogType::RenamingRuleInput;
    let _template = DialogType::TemplateSelection {
        field: TemplateField::MatchPattern,
    };
    let _file = DialogType::FileSelection;
}

#[test]
fn test_template_field_variants() {
    let _match = TemplateField::MatchPattern;
    let _exclude = TemplateField::ExclusionPattern;
    let _rename = TemplateField::RenamingRule;
}

