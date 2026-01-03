//! Tests for the Completion UI module

use frentui::ui::completion;

#[test]
fn test_complete_path_empty() {
    let result = completion::complete_path("");
    assert!(result.is_none());
}

#[test]
fn test_find_common_prefix_empty() {
    let result = completion::find_common_prefix(&[]);
    assert_eq!(result, "");
}

#[test]
fn test_find_common_prefix_single() {
    let strings = vec!["test.txt".to_string()];
    let result = completion::find_common_prefix(&strings);
    assert_eq!(result, "test.txt");
}

#[test]
fn test_find_common_prefix_multiple() {
    let strings = vec![
        "test1.txt".to_string(),
        "test2.txt".to_string(),
        "test3.txt".to_string(),
    ];
    let result = completion::find_common_prefix(&strings);
    assert_eq!(result, "test");
}

#[test]
fn test_find_common_prefix_no_common() {
    let strings = vec![
        "abc.txt".to_string(),
        "xyz.txt".to_string(),
    ];
    let result = completion::find_common_prefix(&strings);
    assert_eq!(result, "");
}

#[test]
fn test_find_common_prefix_partial() {
    let strings = vec![
        "test_file1.txt".to_string(),
        "test_file2.txt".to_string(),
        "test_other.txt".to_string(),
    ];
    let result = completion::find_common_prefix(&strings);
    assert_eq!(result, "test_");
}

