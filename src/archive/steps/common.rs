//! Common step rendering and input handling utilities

use crate::app::App;
use crate::steps::definition::{MenuAction, StepDefinition};
use crate::ui::layout::StepLayout;
use crate::ui::menu::Menu;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

/// Render a step using its definition
pub fn render_step(definition: &StepDefinition, f: &mut Frame, app: &App, step_area: Rect) {
    // Determine layout based on what's needed
    let has_hint = definition.hint.as_ref().map_or(false, |h| !h.is_empty());
    let has_preview = definition.preview.is_some();
    let preview_note = definition
        .preview
        .as_ref()
        .and_then(|p| p.note.as_ref().map(|s| s.as_str()));
    
    let layout = if has_preview {
        StepLayout::new_with_preview(
            step_area,
            definition.hint.as_deref(),
            Some(""), // Preview content will be rendered separately
            preview_note,
        )
    } else if has_hint {
        StepLayout::new_with_note(step_area, definition.hint.as_deref())
    } else {
        StepLayout::new(step_area)
    };
    
    // 1. Render header with title and optional title hint
    let title_hint = (definition.title_hint)(app);
    StepLayout::render_header(f, layout.header, &definition.title, title_hint.as_deref());
    
    // 2. Render hint if present
    if let (Some(note_area), Some(hint)) = (layout.note, &definition.hint) {
        if !hint.is_empty() {
            StepLayout::render_note(f, note_area, hint);
        }
    }
    
    // 3. Render input (either input dialog or menu)
    if let Some(ref input) = app.input_dialog {
        input.render(f, layout.input);
        
        // Show instructions in content area when input dialog is active
        // (This is step-specific, so we'll handle it in individual steps if needed)
    } else {
        // Render menu
        let menu_items = definition.menu.all_items(app);
        
        // Initialize menu selection if needed
        let menu_selection = app.menu_selection.unwrap_or(0);
        let item_count = menu_items.len();
        let safe_selection = if item_count > 0 {
            Some(menu_selection.min(item_count.saturating_sub(1)))
        } else {
            None
        };
        
        let mut menu = Menu::new(&definition.menu.title, menu_items);
        if let Some(sel) = safe_selection {
            menu.state.select(Some(sel));
        }
        menu.render(f, layout.input);
    }
    
    // 4. Render preview if present
    if let (Some(preview_area), Some(preview_def)) = (layout.preview, &definition.preview) {
        use crate::steps::definition::PreviewContent;
        use ratatui::widgets::{Block, Borders, List, ListItem, Row, Table, TableState};
        use ratatui::style::Color;
        use ratatui::layout::Constraint;
        
        match (preview_def.content)(app) {
            PreviewContent::Text(text) => {
                StepLayout::render_preview(f, preview_area, &text);
            }
            PreviewContent::FilesList(files) => {
                let file_items: Vec<ListItem> = files
                    .iter()
                    .take(50) // Limit to 50 files for performance
                    .map(|path| {
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?");
                        ListItem::new(name)
                    })
                    .collect();
                
                let file_list = List::new(file_items)
                    .block(
                        Block::default()
                            .title("Files")
                            .borders(Borders::ALL)
                            .border_style(ratatui::style::Style::default().fg(Color::Cyan))
                            .padding(ratatui::widgets::Padding {
                                left: 1,
                                right: 1,
                                top: 1,
                                bottom: 1,
                            })
                    );
                f.render_widget(file_list, preview_area);
            }
            PreviewContent::FrenengPreview(preview) => {
                // Render freneng preview as a table
                let rows: Vec<Row> = preview.renames
                    .iter()
                    .take(50) // Limit to 50 for performance
                    .map(|rename| {
                        let old_name = rename
                            .old_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("?");
                        let new_name = &rename.new_name;
                        
                        Row::new(vec![
                            old_name.to_string(),
                            "→".to_string(),
                            new_name.clone(),
                        ])
                    })
                    .collect();
                
                let mut table_state = TableState::default();
                if !rows.is_empty() {
                    table_state.select(Some(0));
                }
                
                let table = Table::new(
                    rows,
                    [
                        Constraint::Percentage(45),
                        Constraint::Percentage(10),
                        Constraint::Percentage(45),
                    ],
                )
                .header(
                    Row::new(vec!["Old Name", "", "New Name"])
                        .style(ratatui::style::Style::default().fg(Color::Yellow))
                )
                .block(
                    Block::default()
                        .title("Rename Preview")
                        .borders(Borders::ALL)
                        .border_style(ratatui::style::Style::default().fg(Color::Cyan))
                        .padding(ratatui::widgets::Padding {
                            left: 1,
                            right: 1,
                            top: 1,
                            bottom: 1,
                        }),
                )
                .column_spacing(1);
                
                f.render_stateful_widget(table, preview_area, &mut table_state);
                
                // Show warnings if any
                if !preview.warnings.is_empty() {
                    let warnings_text = format!("Warnings: {}", preview.warnings.join("; "));
                    if let Some(preview_note_area) = layout.preview_note {
                        StepLayout::render_preview_note(f, preview_note_area, &warnings_text);
                    }
                }
            }
        }
        
        if let (Some(preview_note_area), Some(note)) = (layout.preview_note, &preview_def.note) {
            StepLayout::render_preview_note(f, preview_note_area, note);
        }
    }
    
    // 5. Render custom content if present
    if let Some(ref renderer) = definition.content_renderer {
        renderer(f, app, layout.content);
    }
}

/// Handle input for a step using its definition
pub fn handle_step_input(
    definition: &StepDefinition,
    app: &mut App,
    key: KeyEvent,
) -> bool {
    // If input dialog is active, handle input (this is step-specific)
    // We'll need to handle this in individual steps for now
    // TODO: Make input dialog handling more generic
    
    // Normal menu handling
    let menu_items_count = definition.menu.item_count(app);
    
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            let current = app.menu_selection.unwrap_or(0);
            let prev = if current == 0 {
                menu_items_count.saturating_sub(1)
            } else {
                current - 1
            };
            app.menu_selection = Some(prev);
            true
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let current = app.menu_selection.unwrap_or(0);
            let next = if current >= menu_items_count.saturating_sub(1) {
                0
            } else {
                current + 1
            };
            app.menu_selection = Some(next);
            true
        }
        KeyCode::Enter => {
            let selection = app.menu_selection.unwrap_or(0);
            let action = definition.menu.execute_action(app, selection);
            
            // Apply the action
            match action {
                MenuAction::Next => {
                    app.go_next();
                }
                MenuAction::Back => {
                    app.go_back();
                }
                MenuAction::JumpTo(step) => {
                    app.jump_to(step);
                }
                MenuAction::Quit => {
                    app.quit();
                }
                MenuAction::None => {
                    // Action was handled by the callback
                }
            }
            true
        }
        KeyCode::Char('h') | KeyCode::Left => {
            app.go_back();
            true
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
            true
        }
        _ => false,
    }
}

