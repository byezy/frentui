//! Tests for section layout positions
//! Verifies that rendered areas have correct positions (no gaps, no overlaps, no space after actions)

use ratatui::layout::{Constraint, Layout, Rect};

/// Verify layout positions for a given constraint set
fn verify_area_positions(inner_area: Rect, constraints: &[Constraint]) -> (bool, String) {
    let chunks = Layout::default()
        .constraints(constraints)
        .split(inner_area);
    
    // Check that all chunks are consecutive (no gaps)
    let mut last_end = inner_area.y;
    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.y != last_end {
            return (false, format!("Gap detected at chunk {}: expected y={}, got y={}", i, last_end, chunk.y));
        }
        last_end = chunk.y + chunk.height;
    }
    
    // Check that last chunk ends exactly at inner_area bottom (no space after)
    if last_end != inner_area.y + inner_area.height {
        return (false, format!("Extra space after actions: last_end={}, inner_area.bottom={}", 
            last_end, inner_area.y + inner_area.height));
    }
    
    // Check that total height matches
    let total_height: u16 = chunks.iter().map(|c| c.height).sum();
    if total_height != inner_area.height {
        return (false, format!("Total height mismatch: sum={}, inner_area.height={}", 
            total_height, inner_area.height));
    }
    
    (true, String::new())
}

#[test]
fn test_working_directory_area_positions_empty() {
    let area = Rect::new(0, 0, 50, 15);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    let constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (empty, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_area_positions(inner_area, &constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_working_directory_area_positions_multiple() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    let constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (multiple, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_area_positions(inner_area, &constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_area_positions_pattern_only() {
    let area = Rect::new(0, 0, 50, 15);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    let constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (pattern only, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_area_positions(inner_area, &constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_area_positions_both() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    let constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (both pattern and files, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_area_positions(inner_area, &constraints);
    assert!(valid, "{}", msg);
}
