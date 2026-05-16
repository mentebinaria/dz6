use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

fn tab_next(app: &mut App) {
    if app.header_view.tab_index < 3 {
        app.header_view.tab_index = app.header_view.tab_index.saturating_add(1);
    } else {
        app.header_view.tab_index = 0;
    }
}

fn tab_prev(app: &mut App) {
    if app.header_view.tab_index == 0 {
        app.header_view.tab_index = 3;
    } else {
        app.header_view.tab_index -= 1;
    }
}

fn tab_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_header_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_header_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.elf_header_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_header_table_state.selected() {
                if idx == 0 {
                    app.header_view.elf_header_table_state.select(None);
                } else {
                    app.header_view.elf_header_table_state.select_previous();
                }
            }
        }
        KeyCode::Char('G') => {
            // goto
        }
        _ => {}
    }
    Ok(false)
}

fn tab_phdrs_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_phrs_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_phrs_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.elf_phrs_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_phrs_table_state.selected() {
                if idx == 0 {
                    app.header_view.elf_phrs_table_state.select(None);
                } else {
                    app.header_view.elf_phrs_table_state.select_previous();
                }
            }
        }
        KeyCode::Char('G') => {
            // goto
        }
        _ => {}
    }
    Ok(false)
}

fn tab_sections_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_sections_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_sections_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.elf_sections_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_sections_table_state.selected() {
                if idx == 0 {
                    app.header_view.elf_sections_table_state.select(None);
                } else {
                    app.header_view.elf_sections_table_state.select_previous();
                }
            }
        }
        KeyCode::Char('G') => {
            // goto
        }
        _ => {}
    }
    Ok(false)
}

pub fn header_elf_view_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
            tab_next(app);
        }
        KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
            tab_prev(app);
        }
        _ => (),
    }

    match app.header_view.tab_index {
        0 => tab_header_events(app, key),
        1 => tab_phdrs_events(app, key),
        2 => tab_sections_events(app, key),
        _ => Ok(false),
    }
}
