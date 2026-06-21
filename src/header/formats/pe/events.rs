use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

const NUMBER_OF_TABS: usize = 5;

fn tab_next(app: &mut App) {
    if app.header_view.tab_index < NUMBER_OF_TABS - 1 {
        app.header_view.tab_index = app.header_view.tab_index.saturating_add(1);
    } else {
        app.header_view.tab_index = 0;
    }
}

fn tab_prev(app: &mut App) {
    if app.header_view.tab_index == 0 {
        app.header_view.tab_index = NUMBER_OF_TABS - 1;
    } else {
        app.header_view.tab_index -= 1;
    }
}

fn tab_dos_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .pe_state
                .dos_header_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .pe_state
                    .dos_header_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view
                    .pe_state
                    .dos_header_table_state
                    .select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.pe_state.dos_header_table_state.selected() {
                if idx == 0 {
                    app.header_view.pe_state.dos_header_table_state.select(None);
                } else {
                    app.header_view
                        .pe_state
                        .dos_header_table_state
                        .select_previous();
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

fn tab_pe_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .pe_state
                .pe_header_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .pe_state
                    .pe_header_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.pe_state.pe_header_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.pe_state.pe_header_table_state.selected() {
                if idx == 0 {
                    app.header_view.pe_state.pe_header_table_state.select(None);
                } else {
                    app.header_view
                        .pe_state
                        .pe_header_table_state
                        .select_previous();
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
                .pe_state
                .sections_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .pe_state
                    .sections_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.pe_state.sections_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.pe_state.sections_table_state.selected() {
                if idx == 0 {
                    app.header_view.pe_state.sections_table_state.select(None);
                } else {
                    app.header_view
                        .pe_state
                        .sections_table_state
                        .select_previous();
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

fn tab_imports_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .pe_state
                .imports_table_sate
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .pe_state
                    .imports_table_sate
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.pe_state.imports_table_sate.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.pe_state.imports_table_sate.selected() {
                if idx == 0 {
                    app.header_view.pe_state.imports_table_sate.select(None);
                } else {
                    app.header_view
                        .pe_state
                        .imports_table_sate
                        .select_previous();
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

pub fn view_header_pe_events(app: &mut App, key: KeyEvent) -> Result<bool> {
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
        0 => tab_dos_header_events(app, key),
        1 => tab_pe_header_events(app, key),
        2 => tab_sections_events(app, key),
        3 => tab_imports_events(app, key),
        _ => Ok(false),
    }
}
