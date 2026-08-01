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
        KeyCode::Down | KeyCode::Char('j') => app
            .header_view
            .elf_state
            .elf_header_table_state
            .select_next(),
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .elf_state
            .elf_header_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        _ => {}
    }
    Ok(false)
}

fn tab_program_headers_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => app
            .header_view
            .elf_state
            .program_header_table_state
            .select_next(),
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .elf_state
            .program_header_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // follow program header offset in hex view
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
            app.header_view.elf_state.sections_table_state.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .elf_state
            .sections_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // follow section offset in hex view
        KeyCode::Char('f') => {
            if let Some(idx) = app.header_view.elf_state.sections_table_state.selected() {
                // if we're here, the ELF should be valid (hopefully)
                let elf = app.header_view.elf.as_ref().unwrap();
                if let Some(sec) = elf.sections.get(idx) {
                    let ofs = sec.sh_offset;
                    app.goto(ofs as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn tab_symbols_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.elf_state.symbols_table_state.select_next()
        }
        KeyCode::Up | KeyCode::Char('k') => app
            .header_view
            .elf_state
            .symbols_table_state
            .select_previous(),
        KeyCode::Left | KeyCode::Char('h') => tab_prev(app),
        KeyCode::Right | KeyCode::Char('l') => tab_next(app),
        // follow symbol value in hex view
        KeyCode::Char('f') => {
            if let Some(idx) = app.header_view.elf_state.symbols_table_state.selected() {
                // if we're here, the ELF should be valid (hopefully)
                let elf = app.header_view.elf.as_ref().unwrap();
                if let Some(sym) = elf.symtab.get(idx) {
                    let ofs = sym.st_value;
                    app.goto(ofs as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

pub fn view_header_elf_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // change base
        KeyCode::Char('b') => {
            app.config.header_base = if app.config.header_base == 10 { 16 } else { 10 };
            Ok(true)
        }
        // global keybindings to change tabs quickly
        KeyCode::Char(c) if ('1'..='4').contains(&c) => {
            app.header_view.tab_index = c.to_string().parse::<usize>().unwrap().wrapping_sub(1);
            Ok(true)
        }
        // otherwise call the tab event handlers based on tab index
        _ => match app.header_view.tab_index {
            0 => tab_elf_header_events(app, key),
            1 => tab_program_headers_events(app, key),
            2 => tab_sections_events(app, key),
            3 => tab_symbols_events(app, key),
            _ => Ok(false),
        },
    }
}
