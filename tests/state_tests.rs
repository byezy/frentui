//! Tests for the State module

use frentui::state::AppState;
use std::path::PathBuf;

#[test]
fn test_app_state_default() {
    let state = AppState::default();
    
    // Check default values
    assert!(!state.workdirs.is_empty());
    assert!(!state.workdirs[0].as_os_str().is_empty());
    assert_eq!(state.match_pattern, "*.*");
    assert!(state.raw_list.is_empty());
    assert!(state.exclude.is_empty());
    assert!(state.list.is_empty());
    assert!(state.parents.is_empty());
    assert_eq!(state.file_count, 0);
    assert_eq!(state.parent_count, 0);
    assert_eq!(state.rename_rule, "%N.%E");
    assert!(state.new_names.is_empty());
    assert!(state.validation.is_none());
    assert!(!state.can_undo);
}

#[test]
fn test_app_state_new() {
    let state = AppState::new();
    assert_eq!(state.match_pattern, "*.*");
    assert_eq!(state.rename_rule, "%N.%E");
}

#[test]
fn test_app_state_working_directory() {
    let state = AppState::default();
    // Should default to current directory or "."
    assert!(!state.workdirs.is_empty());
    assert!(!state.workdirs[0].as_os_str().is_empty());
}

#[test]
fn test_app_state_match_pattern() {
    let mut state = AppState::default();
    state.match_pattern = "*.txt".to_string();
    assert_eq!(state.match_pattern, "*.txt");
}

#[test]
fn test_app_state_exclude() {
    let mut state = AppState::default();
    state.exclude.push("*.tmp".to_string());
    state.exclude.push("test.txt".to_string());
    assert_eq!(state.exclude.len(), 2);
    assert_eq!(state.exclude[0], "*.tmp");
    assert_eq!(state.exclude[1], "test.txt");
}

#[test]
fn test_app_state_rename_rule() {
    let mut state = AppState::default();
    assert_eq!(state.rename_rule, "%N.%E");
    
    state.rename_rule = "%N_v2.%E".to_string();
    assert_eq!(state.rename_rule, "%N_v2.%E");
}

#[test]
fn test_app_state_validation_ok() {
    let state = AppState::default();
    // No validation means not OK
    assert!(!state.validation_ok());
}

#[test]
fn test_app_state_compute_parents() {
    let mut state = AppState::default();
    
    // Add some test files
    state.list.push(PathBuf::from("/tmp/file1.txt"));
    state.list.push(PathBuf::from("/tmp/file2.txt"));
    state.list.push(PathBuf::from("/home/user/file3.txt"));
    
    state.compute_parents();
    
    assert_eq!(state.file_count, 3);
    assert_eq!(state.parent_count, 2); // /tmp and /home/user
    assert!(state.parents.contains(&PathBuf::from("/tmp")));
    assert!(state.parents.contains(&PathBuf::from("/home/user")));
}

#[test]
fn test_app_state_apply_exclusions_empty() {
    let mut state = AppState::default();
    state.raw_list.push(PathBuf::from("file1.txt"));
    state.raw_list.push(PathBuf::from("file2.txt"));
    
    state.apply_exclusions();
    
    // With no exclusions, list should match raw_list
    assert_eq!(state.list.len(), 2);
}

#[test]
fn test_app_state_apply_exclusions_with_pattern() {
    let mut state = AppState::default();
    state.raw_list.push(PathBuf::from("file1.txt"));
    state.raw_list.push(PathBuf::from("file2.tmp"));
    state.exclude.push("*.tmp".to_string());
    
    state.apply_exclusions();
    
    // Should exclude *.tmp files
    assert_eq!(state.list.len(), 1);
    assert_eq!(state.list[0].file_name().unwrap().to_str().unwrap(), "file1.txt");
}
