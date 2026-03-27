use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, editor::AppView};

pub fn header_pe_view_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.section_table_state.selected() {
                if idx == 0 {
                    app.header_view.section_table_state.select(None);
                    app.header_view.section_table_state.select_column(None);
                    app.header_view.header_list_state.select_last();
                } else {
                    app.header_view.section_table_state.select_previous();
                }
            } else if app.header_view.header_list_state.selected().is_some() {
                app.header_view.header_list_state.select_previous();
            } else if let Some(idx) = app.header_view.imports_table_state.selected() {
                if idx == 0 {
                    app.header_view.imports_table_state.select(None);
                    app.header_view.imports_table_state.select_column(None);
                    app.header_view.section_table_state.select_last();
                } else {
                    app.header_view.imports_table_state.select_previous();
                }
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(idx) = app.header_view.header_list_state.selected() {
                if idx == 6 {
                    app.header_view.header_list_state.select(None);
                    app.header_view
                        .section_table_state
                        .select_cell(Some((0, 0)));
                } else {
                    app.header_view.header_list_state.select_next();
                }
            } else if let Some(idx) = app.header_view.section_table_state.selected() {
                if idx + 1 == app.header_view.pe.as_ref().unwrap().number_of_sections {
                    app.header_view.section_table_state.select(None);
                    app.header_view
                        .imports_table_state
                        .select_cell(Some((0, 1)));
                } else {
                    app.header_view.section_table_state.select_next();
                }
            } else if app.header_view.imports_table_state.selected().is_some() {
                app.header_view.imports_table_state.select_next();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app.header_view.section_table_state.selected().is_some() {
                app.header_view.section_table_state.select_previous_column();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if app.header_view.section_table_state.selected().is_some() {
                app.header_view.section_table_state.select_next_column();
            }
        }
        KeyCode::Char('G') => {
            if let Some(idx) = app.header_view.header_list_state.selected() {
                if idx == 3 {
                    app.goto(app.header_view.entrypoint as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        KeyCode::Tab => {
            if app.header_view.tab_index < 3 {
                app.header_view.tab_index += 1;
            } else {
                app.header_view.tab_index = 0;
            }
        }
        KeyCode::BackTab => {
            if app.header_view.tab_index > 0 {
                app.header_view.tab_index -= 1;
            } else {
                app.header_view.tab_index = 3;
            }
        }
        _ => {}
    }
    Ok(false)
}
