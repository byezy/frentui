//! Command-line argument processing for YAZI integration

use crate::app::App;
use std::path::PathBuf;

/// Process command-line arguments (positional paths from YAZI)
/// Adds directories to workdirs and files to match_files, avoiding duplicates
pub fn process_args(app: &mut App, args: Vec<String>) -> std::io::Result<()> {
    if args.is_empty() {
        return Ok(());
    }
    
    log::info!("Processing {} YAZI path(s)", args.len());
    
    for arg in args {
        let path = PathBuf::from(&arg);
        if !path.exists() {
            log::warn!("Path does not exist, skipping: {}", arg);
            continue;
        }
        
        // Canonicalize for proper comparison
        let path_canonical = path.canonicalize().ok();
        let path_to_use = path_canonical.as_ref().unwrap_or(&path);
        
        if path.is_dir() {
            // Add to workdirs if not already present
            let already_exists = if let Some(ref canonical) = path_canonical {
                app.state.workdirs.iter()
                    .any(|w| w.canonicalize().ok().as_ref() == Some(canonical))
            } else {
                app.state.workdirs.contains(&path)
            };
            
            if !already_exists {
                app.state.workdirs.push(path_to_use.clone());
                log::debug!("Added directory to workdirs: {}", path_to_use.display());
            } else {
                log::debug!("Directory already in workdirs: {}", path_to_use.display());
            }
        } else if path.is_file() {
            // Add to match_files if not already present
            let already_exists = if let Some(ref canonical) = path_canonical {
                app.state.match_files.iter()
                    .any(|f| f.canonicalize().ok().as_ref() == Some(canonical))
            } else {
                app.state.match_files.contains(&path)
            };
            
            if !already_exists {
                app.state.match_files.push(path_to_use.clone());
                log::debug!("Added file to match_files: {}", path_to_use.display());
            } else {
                log::debug!("File already in match_files: {}", path_to_use.display());
            }
        } else {
            log::warn!("Path is neither file nor directory, skipping: {}", arg);
        }
    }
    
    Ok(())
}
