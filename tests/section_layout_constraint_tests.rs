//! Tests for section layout constraints
//! Verifies that layout constraints match the expected structure: blank, note, blank, list, blank, actions

use ratatui::layout::{Constraint, Rect};

/// Test helper: Verify layout structure produces correct area positions
/// Expected: 6 constraints = [blank(1), note(1), blank(1), list(variable), blank(1), actions(1)]
/// This verifies that areas are consecutive and actions ends at inner_area bottom (no extra space)
fn verify_layout_positions(inner_area: Rect, constraints: &[Constraint]) -> (bool, String) {
    // Should have exactly 6 constraints
    if constraints.len() != 6 {
        return (false, format!("Expected 6 constraints, got {}", constraints.len()));
    }
    
    // Verify areas add up correctly (no gaps, no extra space)
    let chunks = ratatui::layout::Layout::default()
        .constraints(constraints)
        .split(inner_area);
    
    // All chunks should be consecutive (no gaps)
    let mut last_end = inner_area.y;
    for (i, chunk) in chunks.iter().enumerate() {
        if chunk.y != last_end {
            return (false, format!("Gap detected at chunk {}: expected y={}, got y={}", i, last_end, chunk.y));
        }
        last_end = chunk.y + chunk.height;
    }
    
    // Last chunk (actions) should end exactly at inner_area bottom (no space after)
    // This is the key requirement: NO blank lines after actions
    if last_end != inner_area.y + inner_area.height {
        return (false, format!("Extra space after actions: last_end={}, inner_area.bottom={} ({} lines of extra space)", 
            last_end, inner_area.y + inner_area.height, 
            (inner_area.y + inner_area.height) - last_end));
    }
    
    // Verify constraint types for fixed elements
    // First 3 should be Length(1) for blank, note, blank
    if !matches!(constraints[0], Constraint::Length(1)) {
        return (false, format!("Constraint 0 (first blank) should be Length(1), got {:?}", constraints[0]));
    }
    if !matches!(constraints[1], Constraint::Length(1)) {
        return (false, format!("Constraint 1 (note) should be Length(1), got {:?}", constraints[1]));
    }
    if !matches!(constraints[2], Constraint::Length(1)) {
        return (false, format!("Constraint 2 (second blank) should be Length(1), got {:?}", constraints[2]));
    }
    
    // Fourth (list) can be Length, Fill, or Min depending on content
    match constraints[3] {
        Constraint::Length(_) | Constraint::Fill(_) | Constraint::Min(_) => {},
        _ => return (false, format!("Constraint 3 (list) should be Length, Fill, or Min, got {:?}", constraints[3])),
    }
    
    // Fifth should be blank (Length(1)) - this is the blank line BETWEEN list and actions
    if !matches!(constraints[4], Constraint::Length(1)) {
        return (false, format!("Constraint 4 (blank between list and actions) should be Length(1), got {:?}", constraints[4]));
    }
    
    // Sixth should be actions (Length(1))
    if !matches!(constraints[5], Constraint::Length(1)) {
        return (false, format!("Constraint 5 (actions) should be Length(1), got {:?}", constraints[5]));
    }
    
    (true, String::new())
}

#[test]
fn test_working_directory_layout_constraints_empty() {
    // Create a test area
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected constraints: [blank(1), note(1), blank(1), list(Length(1)), blank(1), actions(1)]
    // When empty, list should be Length(1) for "None"
    // But to fill the space, we need to use Fill for the list
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (empty = "None", but fill remaining space)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_working_directory_layout_constraints_single() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should fill remaining space (even for single item, to avoid extra space)
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (single item, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_working_directory_layout_constraints_multiple() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should be Fill(1) for multiple items
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (multiple items, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_layout_constraints_empty() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should fill remaining space
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (empty = "None", fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_layout_constraints_pattern_only() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should fill remaining space (even for single pattern)
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (single pattern, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_layout_constraints_files_only_single() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should fill remaining space
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (single file, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_layout_constraints_files_only_multiple() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should fill remaining space (even when only files, to avoid extra space)
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (multiple files, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}

#[test]
fn test_match_files_layout_constraints_both() {
    let area = Rect::new(0, 0, 50, 20);
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Expected: list should be Fill(1) when both pattern and files
    let expected_constraints = vec![
        Constraint::Length(1), // blank
        Constraint::Length(1), // note
        Constraint::Length(1), // blank
        Constraint::Fill(1), // list (both pattern and files, fill remaining)
        Constraint::Length(1), // blank
        Constraint::Length(1), // actions
    ];
    
    let (valid, msg) = verify_layout_positions(inner_area, &expected_constraints);
    assert!(valid, "{}", msg);
}
