//! UI rendering functions
//!
//! This module contains all rendering logic for the TUI, including
//! section layout, dialog rendering, and UI composition.

use crate::app::App;
use crate::section::{Section, SectionId, SectionTrait};
use crate::strings;
use crate::color::SectionColors;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use crate::ui::dialog::{Dialog, DialogType};

/// Render the entire UI
/// 
/// Note: This function takes &mut App because template registry access requires mutable reference.
/// The rendering itself doesn't mutate app state, but get_template_registry() does for lazy initialization.
pub fn render_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    
    // First, create layout for header + separator + content + footer
    let main_chunks = Layout::default()
        .constraints([
            Constraint::Length(2),  // Header + separator (2 lines)
            Constraint::Min(0),     // Scrollable content area
            Constraint::Length(2),  // Footer (2 lines)
        ])
        .split(area);
    
    let header_area = main_chunks[0];
    let content_area = main_chunks[1];
    let footer_area = main_chunks[2];
    
    // Render header
    let section_ids = crate::section::SectionId::all();
    if let Some(header_section) = app.sections.iter().find(|s| s.id() == section_ids[0]) {
        SectionTrait::render(header_section, f, app, Rect {
            x: header_area.x,
            y: header_area.y,
            width: header_area.width,
            height: 1,
        }, app.current_section == section_ids[0]);
    }
    
    // Render separator line below branding
    let separator_area = Rect {
        x: header_area.x,
        y: header_area.y + 1,
        width: header_area.width,
        height: 1,
    };
    let separator = Paragraph::new("─".repeat(separator_area.width as usize))
        .style(Style::default().fg(ratatui::style::Color::DarkGray));
    f.render_widget(separator, separator_area);
    
    // Render footer
    if let Some(footer_section) = app.sections.iter().find(|s| s.id() == section_ids[section_ids.len() - 1]) {
        SectionTrait::render(footer_section, f, app, footer_area, false);
    }
    
    // Now render all other sections in the scrollable content area
    // Calculate total height needed
    let section_ids_without_header_footer: Vec<_> = section_ids.iter()
        .skip(1)
        .take(section_ids.len() - 2)
        .collect();
    
    // Calculate dynamic heights for each section based on current state
    use crate::ui::section_layout;
    
    let section_heights: Vec<u16> = section_ids_without_header_footer.iter()
        .filter_map(|id| {
            // Find the section to get its heights
            app.sections.iter()
                .find(|s| s.id() == **id)
                .map(|section| {
                    // Calculate dynamic height based on content
                    section_layout::calculate_section_height(section, app, content_area.height)
                })
        })
        .collect();
    
    // Calculate total height needed
    let total_min_height: u16 = section_heights.iter().sum();
    
    // Calculate positions of all sections to determine if we need to auto-scroll
    let mut section_positions: Vec<(SectionId, u16)> = Vec::new();
    let mut current_y_pos = 0u16;
    for (idx, section_id) in section_ids_without_header_footer.iter().enumerate() {
        if idx < section_heights.len() {
            section_positions.push((**section_id, current_y_pos));
            current_y_pos += section_heights[idx];
        }
    }
    
    // Auto-scroll to keep focused section visible
    if let Some((_, focused_y)) = section_positions.iter().find(|(id, _)| *id == app.current_section) {
        let focused_height = section_ids_without_header_footer.iter()
            .position(|id| **id == app.current_section)
            .and_then(|idx| section_heights.get(idx))
            .copied()
            .unwrap_or(0);
        
        // Calculate where the focused section should be
        let focused_bottom = *focused_y + focused_height;
        let viewport_top = app.screen_scroll as u16;
        let viewport_bottom = viewport_top + content_area.height;
        
        // Adjust scroll if focused section is not fully visible
        if *focused_y < viewport_top {
            // Section is above viewport, scroll up to show it
            app.screen_scroll = *focused_y as usize;
        } else if focused_bottom > viewport_bottom {
            // Section extends below viewport, scroll down to show it
            let new_scroll = focused_bottom.saturating_sub(content_area.height);
            app.screen_scroll = new_scroll as usize;
        }
    }
    
    // Apply scroll offset - calculate which sections are visible
    let max_scroll = total_min_height.saturating_sub(content_area.height);
    let scroll_offset = (app.screen_scroll as u16).min(max_scroll);
    
    // Calculate starting position based on scroll
    let mut current_y = content_area.y as i32 - scroll_offset as i32;
    
    // Render each section with proper scrolling
    for (idx, section_id) in section_ids_without_header_footer.iter().enumerate() {
        if idx < section_heights.len() {
            // Use the calculated dynamic height
            let calculated_height = section_heights[idx];
            
            // Calculate section position
            let section_y = current_y.max(content_area.y as i32) as u16;
            let remaining_height = (content_area.y + content_area.height).saturating_sub(section_y);
            let section_height = calculated_height.min(remaining_height);
            
            // Only render if section is visible
            if section_y < content_area.y + content_area.height && section_height > 0 {
                let section_rect = Rect {
                    x: content_area.x,
                    y: section_y,
                    width: content_area.width,
                    height: section_height,
                };
                
                let section = app.sections.iter()
                    .find(|s| s.id() == **section_id)
                    .expect("Section should exist");
                
                let is_focused = app.current_section == **section_id;
                
                // If dialog is active and this is the focused section, show dialog instead
                if is_focused && app.input_dialog.is_some() {
                    // Check dialog type first to avoid borrow conflicts
                    let is_template = app.input_dialog.as_ref()
                        .map(|d| matches!(d.dialog_type, DialogType::TemplateSelection { .. }))
                        .unwrap_or(false);
                    let is_file_selection = app.input_dialog.as_ref()
                        .map(|d| matches!(d.dialog_type, DialogType::FileSelection))
                        .unwrap_or(false);
                    let is_directory_selection = app.input_dialog.as_ref()
                        .map(|d| matches!(d.dialog_type, DialogType::DirectorySelectionForRemoval))
                        .unwrap_or(false);
                    let is_match_file_selection = app.input_dialog.as_ref()
                        .map(|d| matches!(d.dialog_type, DialogType::MatchFileSelection))
                        .unwrap_or(false);
                    
                    // Now we can safely borrow for rendering
                    if is_template {
                        // For template selection, we need mutable access for get_template_registry
                        // So we extract what we need first
                        let selected_template = app.input_dialog.as_ref().and_then(|d| d.selected_template);
                        render_template_selection_dialog(f, app, selected_template, section_rect);
                    } else if is_file_selection {
                        // For file selection, extract what we need first
                        let selected_file_index = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
                        let selected_files = app.input_dialog.as_ref()
                            .map(|d| d.selected_files.clone())
                            .unwrap_or_default();
                        render_file_selection_dialog(f, app, selected_file_index, selected_files, section_rect);
                    } else if is_directory_selection {
                        // For directory selection, extract what we need first
                        let selected_dir_index = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
                        let selected_dirs = app.input_dialog.as_ref()
                            .map(|d| d.selected_files.clone())
                            .unwrap_or_default();
                        render_directory_selection_dialog(f, app, selected_dir_index, selected_dirs, section_rect);
                    } else if is_match_file_selection {
                        // For match file selection, extract what we need first
                        let selected_file_index = app.input_dialog.as_ref().and_then(|d| d.selected_file_index);
                        let selected_files = app.input_dialog.as_ref()
                            .map(|d| d.selected_files.clone())
                            .unwrap_or_default();
                        render_match_file_selection_dialog(f, app, selected_file_index, selected_files, section_rect);
                    } else {
                        // For action dialogs, we can use immutable reference
                        if let Some(ref dialog) = app.input_dialog {
                            render_action_dialog(f, app, dialog, section, section_rect);
                        }
                    }
                } else {
                    SectionTrait::render(section, f, app, section_rect, is_focused);
                }
            }
            
            // Advance current_y for next section (whether rendered or not)
            current_y += calculated_height as i32;
        }
    }
    
    // Render main screen scrollbar - always visible, non-interactive
    // Update scrollbar state
    let mut scrollbar_state = app.main_scrollbar_state.borrow_mut();
    *scrollbar_state = ScrollbarState::new(total_min_height as usize)
        .position(app.screen_scroll)
        .viewport_content_length(content_area.height as usize);
    
    // Render scrollbar on the right side of content area
    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight);
    f.render_stateful_widget(scrollbar, content_area, &mut *scrollbar_state);
}

/// Render action dialog with same layout as parent section
/// Layout: border, blank line, input area, blank line, border
pub fn render_action_dialog(
    f: &mut Frame,
    _app: &App,
    dialog: &Dialog,
    parent_section: &Section,
    area: Rect,
) {
    let border_style = Style::default()
        .fg(SectionColors::FOCUSED_BORDER)
        .add_modifier(Modifier::BOLD);
    
    // Inner area (inside border)
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Layout: blank line, note, blank line, input area, blank line, actions
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1), // Blank line after border
            Constraint::Length(1), // Note
            Constraint::Length(1), // Blank line after note
            Constraint::Min(5),    // Input area (needs space: border(2) + padding(2) + content(1) = 5 min)
            Constraint::Length(1), // Blank line before actions
            Constraint::Length(1), // Actions area
        ])
        .split(inner_area);
    
    let _blank_line_1 = chunks[0];
    let note_area = chunks[1];
    let _blank_line_2 = chunks[2];
    let input_area = chunks[3];
    let _blank_line_3 = chunks[4];
    let actions_area = chunks[5];
    
    // Get action summary/intent based on dialog type
    let action_summary = match &dialog.dialog_type {
        DialogType::DirectorySelection => strings::dialog::hints::DIRECTORY_SELECTION,
        DialogType::MatchPatternInput => strings::dialog::hints::MATCH_PATTERN_INPUT,
        DialogType::ExclusionPatternInput => strings::dialog::hints::EXCLUSION_PATTERN_INPUT,
        DialogType::RenamingRuleInput => strings::dialog::hints::RENAMING_RULE_INPUT,
        _ => strings::dialog::hints::GENERIC,
    };
    
    // Render note
    let note_line = Line::from(vec![
        Span::styled(action_summary, Style::default().fg(SectionColors::HINT)),
    ]);
    let note_paragraph = Paragraph::new(note_line);
    f.render_widget(note_paragraph, note_area);
    
    // Render input widget - use full input area height
    dialog.input.render(f, input_area);
    
    // Render save/cancel instructions
    let save_cancel_text = Line::from(vec![
        Span::styled(strings::dialog::actions::SAVE, Style::default().fg(SectionColors::ACTION).add_modifier(Modifier::BOLD)),
        Span::styled(strings::dialog::actions::SAVE_LABEL, Style::default().fg(SectionColors::HINT)),
        Span::styled(strings::dialog::actions::SEPARATOR, Style::default().fg(SectionColors::HINT)),
        Span::styled(strings::dialog::actions::CANCEL, Style::default().fg(SectionColors::ACTION).add_modifier(Modifier::BOLD)),
        Span::styled(strings::dialog::actions::CANCEL_LABEL, Style::default().fg(SectionColors::HINT)),
    ]);
    let menu_paragraph = Paragraph::new(save_cancel_text);
    f.render_widget(menu_paragraph, actions_area);
    
    // Render block with same title as parent
    let block = Block::default()
        .title(parent_section.title()) // Same title as parent
        .borders(Borders::ALL)
        .border_style(border_style);
    f.render_widget(block, area);
}

/// Render template selection dialog
pub fn render_template_selection_dialog(
    f: &mut Frame,
    app: &mut App,
    selected_template: Option<usize>,
    area: Rect,
) {
    use ratatui::widgets::{List, ListItem};
    use crate::ui::dialog::{DialogType, TemplateField};
    
    let border_style = Style::default()
        .fg(SectionColors::FOCUSED_BORDER)
        .add_modifier(Modifier::BOLD);
    
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Get the field type to determine which templates to show
    let field = app.input_dialog.as_ref()
        .and_then(|d| match &d.dialog_type {
            DialogType::TemplateSelection { field } => Some(field.clone()),
            _ => None,
        })
        .unwrap_or(TemplateField::RenamingRule); // Default to rename rules if unknown
    
    let templates = {
        let registry = app.get_template_registry();
        registry.list_for_field(field)
    };
    
    // Create list items
    let items: Vec<ListItem> = templates.iter()
        .enumerate()
        .map(|(idx, (name, pattern))| {
            let is_selected = selected_template == Some(idx);
            let prefix = if is_selected { "> " } else { "  " };
            let line = Line::from(vec![
                Span::styled(
                    format!("{}{}", prefix, name),
                    if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
                Span::styled(
                    format!(" -> {}", pattern),
                    Style::default().fg(SectionColors::HINT),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();
    
    let block = Block::default()
        .title("Select Template (↑↓ to navigate, Enter to select, Esc to cancel)")
        .borders(Borders::ALL)
        .border_style(border_style);
    
    let list = List::new(items)
        .block(block);
    
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(selected_template);
    
    f.render_stateful_widget(list, inner_area, &mut list_state);
}

/// Render file selection dialog
pub fn render_file_selection_dialog(
    f: &mut Frame,
    app: &mut App,
    selected_file_index: Option<usize>,
    selected_files: Vec<usize>,
    area: Rect,
) {
    use ratatui::widgets::{List, ListItem};
    
    let border_style = Style::default()
        .fg(SectionColors::FOCUSED_BORDER)
        .add_modifier(Modifier::BOLD);
    
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Get files from state.list
    let files = &app.state.list;
    
    // Create list items with checkboxes
    let items: Vec<ListItem> = files.iter()
        .enumerate()
        .map(|(idx, path)| {
            let is_selected = selected_file_index == Some(idx);
            let is_marked = selected_files.contains(&idx);
            let checkbox = if is_marked { "x " } else { "  " };
            let prefix = if is_selected { "> " } else { "  " };
            
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");
            
            let line = Line::from(vec![
                Span::styled(
                    format!("{}{}", prefix, checkbox),
                    if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
                Span::styled(
                    file_name,
                    if is_marked {
                        Style::default()
                            .fg(ratatui::style::Color::Green)
                            .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
                    } else if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
            ]);
            ListItem::new(line)
        })
        .collect();
    
    let selected_count = selected_files.len();
    let title = if selected_count > 0 {
        format!("Select Files (↑↓ navigate, Space toggle, Enter confirm, Esc cancel) - {} selected", selected_count)
    } else {
        "Select Files (↑↓ navigate, Space toggle, Enter confirm, Esc cancel)".to_string()
    };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);
    
    let list = List::new(items)
        .block(block);
    
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(selected_file_index);
    
    f.render_stateful_widget(list, inner_area, &mut list_state);
}

/// Render directory selection dialog
pub fn render_directory_selection_dialog(
    f: &mut Frame,
    app: &mut App,
    selected_dir_index: Option<usize>,
    selected_dirs: Vec<usize>,
    area: Rect,
) {
    use ratatui::widgets::{List, ListItem};
    
    let border_style = Style::default()
        .fg(SectionColors::FOCUSED_BORDER)
        .add_modifier(Modifier::BOLD);
    
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Get directories from state.workdirs
    let dirs = &app.state.workdirs;
    
    // Create list items with checkboxes
    let items: Vec<ListItem> = dirs.iter()
        .enumerate()
        .map(|(idx, path)| {
            let is_selected = selected_dir_index == Some(idx);
            let is_marked = selected_dirs.contains(&idx);
            let checkbox = if is_marked { "x " } else { "  " };
            let prefix = if is_selected { "> " } else { "  " };
            
            let dir_path = path.display().to_string();
            
            let line = Line::from(vec![
                Span::styled(
                    format!("{}{}", prefix, checkbox),
                    if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
                Span::styled(
                    dir_path,
                    if is_marked {
                        Style::default()
                            .fg(ratatui::style::Color::Green)
                            .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
                    } else if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
            ]);
            ListItem::new(line)
        })
        .collect();
    
    let selected_count = selected_dirs.len();
    let title = if selected_count > 0 {
        format!("Select Directories (↑↓ navigate, Space toggle, Enter confirm, Esc cancel) - {} selected", selected_count)
    } else {
        "Select Directories (↑↓ navigate, Space toggle, Enter confirm, Esc cancel)".to_string()
    };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);
    
    let list = List::new(items)
        .block(block);
    
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(selected_dir_index);
    
    f.render_stateful_widget(list, inner_area, &mut list_state);
}

/// Render match file selection dialog
pub fn render_match_file_selection_dialog(
    f: &mut Frame,
    app: &mut App,
    selected_file_index: Option<usize>,
    selected_files: Vec<usize>,
    area: Rect,
) {
    use ratatui::widgets::{List, ListItem};
    
    let border_style = Style::default()
        .fg(SectionColors::FOCUSED_BORDER)
        .add_modifier(Modifier::BOLD);
    
    let inner_area = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
    
    // Get files from state.match_files
    let files = &app.state.match_files;
    
    // Create list items with checkboxes
    let items: Vec<ListItem> = files.iter()
        .enumerate()
        .map(|(idx, path)| {
            let is_selected = selected_file_index == Some(idx);
            let is_marked = selected_files.contains(&idx);
            let checkbox = if is_marked { "x " } else { "  " };
            let prefix = if is_selected { "> " } else { "  " };
            
            let file_path = path.display().to_string();
            
            let line = Line::from(vec![
                Span::styled(
                    format!("{}{}", prefix, checkbox),
                    if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
                Span::styled(
                    file_path,
                    if is_marked {
                        Style::default()
                            .fg(ratatui::style::Color::Green)
                            .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
                    } else if is_selected {
                        Style::default()
                            .fg(SectionColors::FOCUSED_BORDER)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(SectionColors::ACTION)
                    }
                ),
            ]);
            ListItem::new(line)
        })
        .collect();
    
    let selected_count = selected_files.len();
    let title = if selected_count > 0 {
        format!("Select Files (↑↓ navigate, Space toggle, Enter confirm, Esc cancel) - {} selected", selected_count)
    } else {
        "Select Files (↑↓ navigate, Space toggle, Enter confirm, Esc cancel)".to_string()
    };
    
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);
    
    let list = List::new(items)
        .block(block);
    
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(selected_file_index);
    
    f.render_stateful_widget(list, inner_area, &mut list_state);
}
