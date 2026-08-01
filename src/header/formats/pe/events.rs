use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, editor::AppView};

const NUMBER_OF_TABS: usize = 8;

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
        KeyCode::Down | KeyCode::Char('j') => app
            .header_view
            .pe_state
            .dos_header_table_state
            .select_next(),
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .dos_header_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // go to MZ header (always 0) or to PE offset header
        KeyCode::Char('f') => {
            if let Some(idx) = app.header_view.pe_state.dos_header_table_state.selected() {
                let pe = app.header_view.pe.as_ref().unwrap();

                let offset = match idx {
                    // Signature
                    0 => Some(0),
                    // PEHeaderOffset
                    18 => Some(pe.dos_header.pe_pointer),
                    _ => None,
                };

                if let Some(offset) = offset {
                    app.goto(offset as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tab_coff_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => app
            .header_view
            .pe_state
            .coff_header_table_state
            .select_next(),
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .coff_header_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // go to entrypoint
        KeyCode::Char('f') => {
            if let Some(idx) = app.header_view.pe_state.coff_header_table_state.selected()
                && idx == 13
            {
                let pe = app.header_view.pe.as_ref().unwrap();
                let offset = pe
                    .optional_header
                    .unwrap()
                    .standard_fields
                    .address_of_entry_point;
                app.goto(offset as usize);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tab_optional_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => app
            .header_view
            .pe_state
            .optional_header_table_state
            .select_next(),
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .optional_header_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // go to entrypoint
        KeyCode::Char('f') => {
            if let Some(idx) = app
                .header_view
                .pe_state
                .optional_header_table_state
                .selected()
                && idx == 6
            {
                let pe = app.header_view.pe.as_ref().unwrap();
                let offset = pe
                    .optional_header
                    .unwrap()
                    .standard_fields
                    .address_of_entry_point;
                app.goto(offset as usize);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tab_data_dirs_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.pe_state.data_dirs_table_state.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .data_dirs_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        _ => {}
    }
    Ok(false)
}

fn tab_sections_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.pe_state.sections_table_state.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .sections_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // go to section PtrToRawData
        KeyCode::Char('f') => {
            if let Some(idx) = app.header_view.pe_state.sections_table_state.selected() {
                let pe = app.header_view.pe.as_ref().unwrap();

                if let Some(sec) = pe.sections.get(idx) {
                    let offset = sec.pointer_to_raw_data;
                    app.goto(offset as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tab_imports_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.pe_state.imports_table_sate.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .imports_table_sate
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        _ => {}
    }
    Ok(false)
}

fn tab_exports_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.pe_state.exports_table_sate.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .exports_table_sate
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        _ => {}
    }
    Ok(false)
}

fn tab_overlay_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.pe_state.overlay_table_sate.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .pe_state
            .overlay_table_sate
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        _ => {}
    }
    Ok(false)
}

pub fn view_header_pe_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // change base
        KeyCode::Char('b') => {
            app.config.header_base = if app.config.header_base == 10 { 16 } else { 10 };
            Ok(true)
        }
        // global keybindings to change tabs quickly
        KeyCode::Char(c) if ('1'..='8').contains(&c) => {
            app.header_view.tab_index = c.to_string().parse::<usize>().unwrap().wrapping_sub(1);
            Ok(true)
        }
        // otherwise call the tab event handlers based on tab index
        _ => match app.header_view.tab_index {
            0 => tab_dos_header_events(app, key),
            1 => tab_coff_header_events(app, key),
            2 => tab_optional_header_events(app, key),
            3 => tab_data_dirs_events(app, key),
            4 => tab_sections_events(app, key),
            5 => tab_imports_events(app, key),
            6 => tab_exports_events(app, key),
            7 => tab_overlay_events(app, key),
            _ => Ok(false),
        },
    }
}
