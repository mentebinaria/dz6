use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, editor::AppView};

const NUMBER_OF_TABS: usize = 4;

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

fn tab_elf_header_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_state
                .elf_header_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_state
                    .elf_header_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view
                    .elf_state
                    .elf_header_table_state
                    .select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_state.elf_header_table_state.selected() {
                if idx == 0 {
                    app.header_view
                        .elf_state
                        .elf_header_table_state
                        .select(None);
                } else {
                    app.header_view
                        .elf_state
                        .elf_header_table_state
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

fn tab_program_headers_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_state
                .program_header_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_state
                    .program_header_table_state
                    .select_cell(Some((0, 5)));
            } else {
                app.header_view
                    .elf_state
                    .program_header_table_state
                    .select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app
                .header_view
                .elf_state
                .program_header_table_state
                .selected()
            {
                if idx == 0 {
                    app.header_view
                        .elf_state
                        .program_header_table_state
                        .select(None);
                } else {
                    app.header_view
                        .elf_state
                        .program_header_table_state
                        .select_previous();
                }
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app
                .header_view
                .elf_state
                .program_header_table_state
                .selected()
                .is_none()
            {
                tab_prev(app);
            } else {
                app.header_view
                    .elf_state
                    .program_header_table_state
                    .select_previous_column();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if app
                .header_view
                .elf_state
                .program_header_table_state
                .selected()
                .is_none()
            {
                tab_next(app);
            } else {
                app.header_view
                    .elf_state
                    .program_header_table_state
                    .select_next_column();
            }
        }
        // follow
        // TODO: follow only when a PhysAddr field is selected
        KeyCode::Char('f') => {
            if let Some(idx) = app
                .header_view
                .elf_state
                .program_header_table_state
                .selected()
            {
                // if we're here, the ELF should be valid (hopefully)
                let elf = app.header_view.elf.as_ref().unwrap();
                if let Some(phdr) = elf.phdrs.get(idx) {
                    let ofs = phdr.p_offset;
                    app.goto(ofs as usize);
                    app.editor_view = AppView::Hex;
                }
            }
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
                .elf_state
                .sections_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_state
                    .sections_table_state
                    .select_cell(Some((0, 1)));
            } else {
                app.header_view.elf_state.sections_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_state.sections_table_state.selected() {
                if idx == 0 {
                    app.header_view.elf_state.sections_table_state.select(None);
                } else {
                    app.header_view
                        .elf_state
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

fn tab_symbols_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            if app
                .header_view
                .elf_state
                .symbols_table_state
                .selected_cell()
                .is_none()
            {
                app.header_view
                    .elf_state
                    .symbols_table_state
                    .select_cell(Some((0, 0)));
            } else {
                app.header_view.elf_state.symbols_table_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(idx) = app.header_view.elf_state.symbols_table_state.selected() {
                if idx == 0 {
                    app.header_view.elf_state.symbols_table_state.select(None);
                } else {
                    app.header_view
                        .elf_state
                        .symbols_table_state
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

pub fn view_header_elf_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Tab => {
            tab_next(app);
        }
        KeyCode::BackTab => {
            tab_prev(app);
        }
        _ => (),
    }

    match app.header_view.tab_index {
        0 => tab_elf_header_events(app, key),
        1 => tab_program_headers_events(app, key),
        2 => tab_sections_events(app, key),
        3 => tab_symbols_events(app, key),
        _ => Ok(false),
    }
}
