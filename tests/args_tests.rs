//! Tests for the args module

use frentui::app::App;
use frentui::args;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_args_module_exists() {
    // Verify module can be imported
    assert!(true);
}

#[test]
fn test_process_args_empty() {
    let mut app = App::new();
    let initial_workdirs = app.state.workdirs.len();
    let initial_match_files = app.state.match_files.len();
    
    let result = args::process_args(&mut app, vec![]);
    assert!(result.is_ok());
    
    // Should not change anything
    assert_eq!(app.state.workdirs.len(), initial_workdirs);
    assert_eq!(app.state.match_files.len(), initial_match_files);
}

#[test]
fn test_process_args_nonexistent_path() {
    let mut app = App::new();
    let initial_workdirs = app.state.workdirs.len();
    let initial_match_files = app.state.match_files.len();
    
    let result = args::process_args(&mut app, vec!["/nonexistent/path/that/does/not/exist".to_string()]);
    assert!(result.is_ok());
    
    // Should not add anything
    assert_eq!(app.state.workdirs.len(), initial_workdirs);
    assert_eq!(app.state.match_files.len(), initial_match_files);
}

#[test]
fn test_process_args_adds_directory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    
    let mut app = App::new();
    let initial_count = app.state.workdirs.len();
    
    let result = args::process_args(&mut app, vec![dir_path.to_string_lossy().to_string()]);
    assert!(result.is_ok());
    
    // Should add the directory
    assert_eq!(app.state.workdirs.len(), initial_count + 1);
    assert!(app.state.workdirs.contains(&dir_path) || 
            app.state.workdirs.iter().any(|w| w.canonicalize().ok() == dir_path.canonicalize().ok()));
}

#[test]
fn test_process_args_adds_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    fs::write(&file_path, "test content").unwrap();
    
    let mut app = App::new();
    let initial_count = app.state.match_files.len();
    
    let result = args::process_args(&mut app, vec![file_path.to_string_lossy().to_string()]);
    assert!(result.is_ok());
    
    // Should add the file
    assert_eq!(app.state.match_files.len(), initial_count + 1);
    assert!(app.state.match_files.contains(&file_path) || 
            app.state.match_files.iter().any(|f| f.canonicalize().ok() == file_path.canonicalize().ok()));
}

#[test]
fn test_process_args_avoids_duplicate_directories() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    
    let mut app = App::new();
    
    // Add directory first time
    let result1 = args::process_args(&mut app, vec![dir_path.to_string_lossy().to_string()]);
    assert!(result1.is_ok());
    let count_after_first = app.state.workdirs.len();
    
    // Try to add same directory again
    let result2 = args::process_args(&mut app, vec![dir_path.to_string_lossy().to_string()]);
    assert!(result2.is_ok());
    
    // Should not add duplicate
    assert_eq!(app.state.workdirs.len(), count_after_first);
}

#[test]
fn test_process_args_avoids_duplicate_files() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    fs::write(&file_path, "test content").unwrap();
    
    let mut app = App::new();
    
    // Add file first time
    let result1 = args::process_args(&mut app, vec![file_path.to_string_lossy().to_string()]);
    assert!(result1.is_ok());
    let count_after_first = app.state.match_files.len();
    
    // Try to add same file again
    let result2 = args::process_args(&mut app, vec![file_path.to_string_lossy().to_string()]);
    assert!(result2.is_ok());
    
    // Should not add duplicate
    assert_eq!(app.state.match_files.len(), count_after_first);
}

#[test]
fn test_process_args_mixed_paths() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    let file_path = temp_dir.path().join("test_file.txt");
    fs::write(&file_path, "test content").unwrap();
    
    let mut app = App::new();
    let initial_workdirs = app.state.workdirs.len();
    let initial_match_files = app.state.match_files.len();
    
    let args = vec![
        dir_path.to_string_lossy().to_string(),
        file_path.to_string_lossy().to_string(),
    ];
    
    let result = args::process_args(&mut app, args);
    assert!(result.is_ok());
    
    // Should add both
    assert_eq!(app.state.workdirs.len(), initial_workdirs + 1);
    assert_eq!(app.state.match_files.len(), initial_match_files + 1);
}
