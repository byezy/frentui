//! Path completion utilities
//!
//! Provides tab completion for file paths in input dialogs.

use std::path::Path;

/// Complete a path using tab completion
pub fn complete_path(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    
    let path = Path::new(input);
    
    // If input ends with a separator, we're looking for items in that directory
    let (base_dir, prefix) = if input.ends_with('/') || input.ends_with('\\') {
        (path, "")
    } else {
        (path.parent().unwrap_or(Path::new(".")), 
         path.file_name().and_then(|n| n.to_str()).unwrap_or(""))
    };
    
    // Try to read the directory
    let entries = match std::fs::read_dir(base_dir) {
        Ok(entries) => entries,
        Err(_) => return None,
    };
    
    // Find matching entries
    let mut matches: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with(prefix) {
                let full_path = if base_dir == Path::new(".") {
                    name.to_string()
                } else {
                    base_dir.join(name).to_string_lossy().to_string()
                };
                
                // Add separator if it's a directory
                let metadata = entry.metadata().ok();
                if metadata.map(|m| m.is_dir()).unwrap_or(false) {
                    matches.push(format!("{}/", full_path));
                } else {
                    matches.push(full_path);
                }
            }
        }
    }
    
    if matches.is_empty() {
        None
    } else if matches.len() == 1 {
        // Single match - complete it
        Some(matches[0].clone())
    } else {
        // Multiple matches - find common prefix
        let common_prefix = find_common_prefix(&matches);
        if common_prefix.len() > input.len() {
            Some(common_prefix)
        } else {
            None
        }
    }
}

/// Find the common prefix of a list of strings
pub fn find_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let first = &strings[0];
    let mut prefix_len = first.len();
    
    for s in strings.iter().skip(1) {
        prefix_len = first
            .chars()
            .zip(s.chars())
            .take_while(|(a, b)| a == b)
            .count()
            .min(prefix_len);
    }
    
    first.chars().take(prefix_len).collect()
}

