//! Application state management for frentui
//!
//! State structure and computation functions following the flow:
//! workdirs + match → raw_list
//! raw_list + exclude → list
//! list → file_count + parent_count
//! list + rename_rule → new_names
//! new_names → validation

use std::path::PathBuf;
use std::collections::HashSet;
use glob::Pattern;
use freneng::{find_matching_files, find_matching_files_recursive, RenamingEngine, ValidationResult, FileRename};

/// Main application state
pub struct AppState {
    /// Working directories (defaults to [cwd], must be valid)
    pub workdirs: Vec<PathBuf>,
    
    /// File selection pattern (defaults to "*.*", must be valid)
    pub match_pattern: String,
    
    /// Specific files to include (separate from pattern matching)
    pub match_files: Vec<PathBuf>,
    
    /// File set resulting from directory and selection pattern application
    pub raw_list: Vec<PathBuf>,
    
    /// List or pattern defining files to exclude from raw_list
    pub exclude: Vec<String>,
    
    /// Result of exclusions applied to raw list
    pub list: Vec<PathBuf>,
    
    /// List of parents of all files in list
    pub parents: Vec<PathBuf>,
    
    /// List length
    pub file_count: usize,
    
    /// Parents count
    pub parent_count: usize,
    
    /// The renaming rule (freneng DSL, defaults to "%N.%E" - no change)
    pub rename_rule: String,
    
    /// A list of new file names, one for each of the list files
    pub new_names: Vec<String>,
    
    /// The freneng validation results for the renaming process
    pub validation: Option<ValidationResult>,
    
    /// Whether undo is possible (tracks if renames have been applied)
    pub can_undo: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let workdir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            workdirs: vec![workdir],
            match_pattern: "*.*".to_string(),
            match_files: Vec::new(),
            raw_list: Vec::new(),
            exclude: Vec::new(),
            list: Vec::new(),
            parents: Vec::new(),
            file_count: 0,
            parent_count: 0,
            rename_rule: "%N.%E".to_string(),
            new_names: Vec::new(),
            validation: None,
            can_undo: false,
        }
    }
}

impl AppState {
    /// Create a new state with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Compute raw_list from workdirs + match_pattern
    /// This is async because freneng uses async file operations
    /// Iterates over all directories in workdirs and combines results
    pub async fn compute_raw_list(&mut self) -> Result<(), String> {
        let original_dir = std::env::current_dir().map_err(|e| format!("Failed to get current dir: {}", e))?;
        
        // Check if pattern is recursive (contains **)
        let is_recursive = self.match_pattern.contains("**");
        
        let mut all_files = Vec::new();
        let mut errors = Vec::new();
        
        // Iterate over all working directories
        for workdir in &self.workdirs {
            // Change to workdir for pattern matching
            if let Err(e) = std::env::set_current_dir(workdir) {
                errors.push(format!("Failed to change to workdir {}: {}", workdir.display(), e));
                continue;
            }
            
            // Find matching files (recursive or not based on pattern)
            let result = if is_recursive {
                find_matching_files_recursive(&self.match_pattern, true).await
            } else {
                find_matching_files(&self.match_pattern).await
            };
            
            // Restore original directory
            let _ = std::env::set_current_dir(&original_dir);
            
            match result {
                Ok(files) => {
                    // Make paths absolute and add to collection
                    for p in files {
                        let abs_path = if p.is_absolute() {
                            p
                        } else {
                            workdir.join(p)
                        };
                        all_files.push(abs_path);
                    }
                }
                Err(e) => {
                    errors.push(format!("Failed to find matching files in {}: {}", workdir.display(), e));
                }
            }
        }
        
        // Combine pattern-matched files with specific files
        // Use HashSet to deduplicate
        let mut file_set = HashSet::new();
        
        // Add pattern-matched files
        for file in all_files {
            if let Ok(canonical) = file.canonicalize() {
                file_set.insert(canonical);
            } else {
                file_set.insert(file);
            }
        }
        
        // Add specific files
        for file in &self.match_files {
            if let Ok(canonical) = file.canonicalize() {
                file_set.insert(canonical);
            } else {
                file_set.insert(file.clone());
            }
        }
        
        // Convert back to Vec and sort for consistency
        self.raw_list = file_set.into_iter().collect();
        self.raw_list.sort();
        
        // If we have any files, use them; otherwise return error if all directories failed
        if !self.raw_list.is_empty() || errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Failed to find files in any directory: {}", errors.join("; ")))
        }
    }
    
    /// Apply exclusions to raw_list to get list
    /// Exclusion patterns match against filenames only, regardless of parent directory
    pub fn apply_exclusions(&mut self) {
        if self.exclude.is_empty() {
            self.list = self.raw_list.clone();
            return;
        }
        
        let mut excluded_paths = HashSet::new();
        
        for exclude_pattern in &self.exclude {
            // Check if it's a file path (exact match)
            if let Ok(path) = PathBuf::from(exclude_pattern).canonicalize() {
                excluded_paths.insert(path);
            }
            
            // Expand brace patterns (e.g., *.{jpg,png} -> [*.jpg, *.png])
            let expanded_patterns = expand_brace_pattern(exclude_pattern);
            
            // Use proper glob pattern matching on filenames only
            // Patterns apply to all files regardless of parent directory
            for pattern in expanded_patterns {
                if let Ok(glob_pattern) = Pattern::new(&pattern) {
                    for file in &self.raw_list {
                        if let Some(file_name) = file.file_name().and_then(|n| n.to_str()) {
                            // Match pattern against filename only (not full path)
                            if glob_pattern.matches(file_name) {
                                excluded_paths.insert(file.clone());
                            }
                        }
                    }
                } else {
                    // Invalid glob pattern - fallback to simple string match on filename
                    for file in &self.raw_list {
                        if let Some(file_name) = file.file_name().and_then(|n| n.to_str()) {
                            if file_name == pattern || file_name.contains(&pattern) {
                                excluded_paths.insert(file.clone());
                            }
                        }
                    }
                }
            }
        }
        
        self.list = self.raw_list.iter()
            .filter(|p| !excluded_paths.contains(*p))
            .cloned()
            .collect();
    }
    
    /// Compute parents and counts from list
    pub fn compute_parents(&mut self) {
        let mut parent_set = HashSet::new();
        
        for file in &self.list {
            if let Some(parent) = file.parent() {
                parent_set.insert(parent.to_path_buf());
            }
        }
        
        self.parents = parent_set.into_iter().collect();
        self.parents.sort();
        
        self.file_count = self.list.len();
        self.parent_count = self.parents.len();
    }
    
    /// Compute new_names from list + rename_rule
    /// This is async because freneng uses async operations
    pub async fn compute_new_names(&mut self) -> Result<(), String> {
        if self.list.is_empty() {
            self.new_names = Vec::new();
            return Ok(());
        }
        
        let engine = RenamingEngine;
        let preview_result = engine.generate_preview(&self.list, &self.rename_rule).await
            .map_err(|e| format!("Failed to generate preview: {}", e))?;
        
        self.new_names = preview_result.renames.iter()
            .map(|r| r.new_name.clone())
            .collect();
        
        Ok(())
    }
    
    /// Compute validation from new_names
    /// This is async because freneng uses async operations
    pub async fn compute_validation(&mut self) {
        if self.list.is_empty() || self.new_names.is_empty() {
            self.validation = None;
            return;
        }
        
        // Build FileRename list for validation
        let renames: Vec<FileRename> = self.list.iter()
            .zip(self.new_names.iter())
            .filter_map(|(old_path, new_name)| {
                old_path.parent().map(|parent| FileRename {
                    old_path: old_path.clone(),
                    new_path: parent.join(new_name),
                    new_name: new_name.clone(),
                })
            })
            .collect();
        
        let engine = RenamingEngine;
        self.validation = Some(engine.validate(&renames, false).await);
    }
    
    /// Update all derived state fields
    /// This recomputes everything in the correct order
    pub async fn update_state(&mut self) -> Result<(), String> {
        // 1. workdirs + match → raw_list
        self.compute_raw_list().await?;
        
        // 2. raw_list + exclude → list
        self.apply_exclusions();
        
        // 3. list → file_count + parent_count
        self.compute_parents();
        
        // 4. list + rename_rule → new_names
        self.compute_new_names().await?;
        
        // 5. new_names → validation
        self.compute_validation().await;
        
        Ok(())
    }
    
    /// Check if validation is OK (no issues)
    pub fn validation_ok(&self) -> bool {
        self.validation.as_ref()
            .map(|v| v.issues.is_empty())
            .unwrap_or(false)
    }
    
    /// Check if there are any actual changes (any old name != new name)
    pub fn has_changes(&self) -> bool {
        if self.list.is_empty() || self.new_names.is_empty() {
            return false;
        }
        
        // Check if any file would actually be renamed (old name != new name)
        self.list.iter()
            .zip(self.new_names.iter())
            .any(|(old_path, new_name)| {
                if let Some(old_name) = old_path.file_name().and_then(|n| n.to_str()) {
                    old_name != new_name.as_str()
                } else {
                    false
                }
            })
    }
}

/// Expands brace patterns like `*.{jpg,png}` into multiple patterns: `["*.jpg", "*.png"]`
/// If no braces are found, returns a vector with the original pattern
fn expand_brace_pattern(pattern: &str) -> Vec<String> {
    // Find the brace pattern {a,b,c}
    if let Some(start) = pattern.find('{') {
        if let Some(end) = pattern.rfind('}') {
            if end > start {
                let prefix = &pattern[..start];
                let suffix = &pattern[end + 1..];
                let brace_content = &pattern[start + 1..end];
                
                // Split by comma and expand
                let mut expanded = Vec::new();
                for item in brace_content.split(',') {
                    let item = item.trim();
                    if !item.is_empty() {
                        expanded.push(format!("{}{}{}", prefix, item, suffix));
                    }
                }
                
                if !expanded.is_empty() {
                    return expanded;
                }
            }
        }
    }
    
    // No braces found, return original pattern
    vec![pattern.to_string()]
}
