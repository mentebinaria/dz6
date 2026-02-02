use crate::{app::App, commands::Commands, editor::UIState, hex};

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io::Result;

pub fn hex_mode_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    // this local function goes to the next/previous other byte
    // it is called when the user sends either 'o' or 'O'
    fn goto_other_byte(app: &mut App, delta: isize) {
        let mut ofs = app.hex_view.offset;
        let current_byte = app.read_u8(ofs).unwrap();

        while ofs < app.file_info.size {
            if let Some(b) = app.read_u8(ofs)
                && b != current_byte
            {
                app.goto(ofs);
                break;
            }
            ofs = ofs.saturating_add_signed(delta);
            // this is needed because it can start at 0
            // but it cannot be zero afterwards
            // without it, `O` doesn't work at offset 0
            if ofs == 0 {
                app.goto(0);
                break;
            }
        }
    }

    // it is important to call goto as it looks for the offset in the
    // cache and, in case it is not there, it reads the needed block, and
    // also checks and updates offset position, cursor position, etc.
    match key.code {
        // move left
        KeyCode::Left => {
            if app.hex_view.offset > 0 {
                app.goto(app.hex_view.offset - 1);
            }
        }
        KeyCode::Char('h') => {
            if key.modifiers.contains(KeyModifiers::ALT) {
                if let Some(b) = app.read_u8(app.hex_view.offset) {
                    if app.hex_view.highlights.contains(&b) {
                        app.hex_view.highlights.remove(&b);
                    } else {
                        app.hex_view.highlights.insert(b);
                    }
                }
            } else if app.hex_view.offset > 0 {
                app.goto(app.hex_view.offset - 1);
            }
        }
        // move right
        KeyCode::Right | KeyCode::Char('l') if !key.modifiers.contains(KeyModifiers::ALT) => {
            app.goto(app.hex_view.offset + 1);
        }
        // move up
        KeyCode::Up | KeyCode::Char('k') => {
            if app.hex_view.offset >= app.config.hex_mode_bytes_per_line {
                app.goto(app.hex_view.offset - app.config.hex_mode_bytes_per_line);
            }
        }
        // move down
        KeyCode::Down | KeyCode::Char('j') => {
            app.goto(app.hex_view.offset + app.config.hex_mode_bytes_per_line);
        }
        // BOL
        KeyCode::Char('g') => app.goto(0),
        KeyCode::Home => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.goto(0);
            } else {
                app.goto(app.hex_view.offset - app.hex_view.cursor.x);
            }
        }
        // EOF
        KeyCode::Char('G') => app.goto(app.file_info.size - 1),

        // EOL
        KeyCode::End | KeyCode::Char('$') => {
            // `Ctrl+End` goes to EOF too
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                app.goto(app.file_info.size - 1);
            } else {
                // `End` or `$` alone go to EOL
                if app.hex_view.offset + app.config.hex_mode_bytes_per_line > app.file_info.size {
                    app.goto(app.file_info.size - 1);
                } else {
                    app.goto(
                        app.hex_view.offset + app.config.hex_mode_bytes_per_line
                            - app.hex_view.cursor.x
                            - 1,
                    );
                }
            }
        }
        // go down one page
        KeyCode::PageDown => {
            app.goto(app.hex_view.offset + app.reader.page_current_size);
        }
        // go up one page
        KeyCode::PageUp => {
            if app.hex_view.offset > app.reader.page_current_size {
                app.goto(app.hex_view.offset - app.reader.page_current_size);
            } else {
                app.goto(0);
            }
        }
        // go to last visited offset
        KeyCode::Backspace => {
            app.goto(app.hex_view.last_visited_offset);
        }
        // add a bookmark
        KeyCode::Char('+') => {
            if app.hex_view.bookmarks.len() < 8 {
                app.hex_view.bookmarks.push(app.hex_view.offset);
            }
        }
        // remove last added bookmark
        KeyCode::Char('-') => {
            if !app.hex_view.bookmarks.is_empty() {
                if key.modifiers.contains(KeyModifiers::ALT) {
                    // Alt + - removes the bookmark over an offset
                    if let Some(&ofs) = app.hex_view.bookmarks.last()
                        && ofs == app.hex_view.offset
                    {
                        app.hex_view
                            .bookmarks
                            .remove(app.hex_view.bookmarks.len() - 1);
                    }
                } else {
                    // Go to last bookmark
                    if let Some(&ofs) = app.hex_view.bookmarks.last() {
                        app.goto(ofs);
                    }
                }
            }
        }
        // goto bookmarks
        KeyCode::Char(c) if ('1'..='8').contains(&c) => {
            if key.modifiers.contains(KeyModifiers::ALT) {
                // subtracts 0x30 to convert it to integer
                let n = (c as u8 - b'0') as usize;
                if let Some(&ofs) = app.hex_view.bookmarks.get(n - 1) {
                    // if there's a value there, go to it
                    app.goto(ofs);
                }
            }
        }
        // clear bookmarks
        KeyCode::Char('0') => {
            if key.modifiers.contains(KeyModifiers::ALT) {
                app.hex_view.bookmarks.clear();
            } else {
                app.goto(app.hex_view.offset - app.hex_view.cursor.x)
            }
        }
        // advance (w|d|q)word
        KeyCode::Char('w') => app.goto(app.hex_view.offset + 2),
        KeyCode::Char('W') => app.goto(app.hex_view.offset.saturating_sub(2)),
        KeyCode::Char('d') => app.goto(app.hex_view.offset + 4),
        KeyCode::Char('D') => app.goto(app.hex_view.offset.saturating_sub(4)),
        KeyCode::Char('q') => app.goto(app.hex_view.offset + 8),
        KeyCode::Char('Q') => app.goto(app.hex_view.offset.saturating_sub(8)),

        // next other byte
        KeyCode::Char('o') => goto_other_byte(app, 1),
        KeyCode::Char('O') => goto_other_byte(app, -1),

        // zero out byte
        KeyCode::Char('z') => {
            if !app.file_info.is_read_only && app.hex_view.offset < app.file_info.size {
                app.state = UIState::HexEditing;
                app.hex_view.changed_bytes.clear();
                hex::edit::fill_with(app, 0x00, true);
            }
        }

        // increment byte under the cursor
        KeyCode::Char('a') => {
            if !app.file_info.is_read_only
                && app.hex_view.offset < app.file_info.size
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                app.state = UIState::HexEditing;
                app.hex_view.changed_bytes.clear();
                let ofs = app.hex_view.offset;
                if let Some(s) = app.hex_view.changed_bytes.get(&ofs) {
                    if let Ok(b) = u8::from_str_radix(s, 16) {
                        hex::edit::fill_with(app, b.wrapping_add(1), false);
                    }
                } else if let Some(b) = app.read_u8(ofs) {
                    hex::edit::fill_with(app, b.wrapping_add(1), false);
                }
            }
        }

        // decrement byte under the cursor
        KeyCode::Char('x') => {
            if !app.file_info.is_read_only
                && app.hex_view.offset < app.file_info.size
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                app.state = UIState::HexEditing;
                app.hex_view.changed_bytes.clear();
                let ofs = app.hex_view.offset;
                if let Some(s) = app.hex_view.changed_bytes.get(&ofs) {
                    if let Ok(b) = u8::from_str_radix(s, 16) {
                        hex::edit::fill_with(app, b.wrapping_sub(1), false);
                    }
                } else if let Some(b) = app.read_u8(ofs) {
                    hex::edit::fill_with(app, b.wrapping_sub(1), false);
                }
            }
        }

        // help
        KeyCode::F(1) => {
            app.state = UIState::DialogHelp;
            app.dialog_renderer = Some(hex::help::dialog_help_draw);
        }
        // reaplce
        KeyCode::Char('r') => {
            if app.file_info.is_read_only {
                print!("\x07"); // beep
            } else if app.hex_view.offset < app.file_info.size {
                app.state = UIState::HexEditing;
            }
        }
        // strings list
        KeyCode::Char('s') => {
            Commands::strings(app);
        }
        // search
        KeyCode::Char('/') => {
            app.state = UIState::DialogSearch;
            app.dialog_renderer = Some(hex::search::dialog_search_draw);
        }
        // names and search next
        KeyCode::Char('n') => {
            // names
            if key.modifiers.contains(KeyModifiers::ALT) {
                app.state = UIState::DialogNames;
                app.dialog_renderer = Some(hex::names::dialog_names_draw);
                if app.hex_view.names_list_state.selected().is_none() {
                    app.hex_view.names_list_state.select_first();
                }
            } else {
                // search next
                let mut ofs = None;
                if app.state == UIState::Normal {
                    if app.hex_view.search.mode == hex::search::SearchMode::Utf8
                        && !app.hex_view.search.input_text.value().is_empty()
                    {
                        ofs = crate::hex::search::search(
                            app,
                            &app.hex_view.search.input_text.value().to_string(),
                        )
                    } else if app.hex_view.search.mode == hex::search::SearchMode::Hex
                        && !app.hex_view.search.input_hex.value().is_empty()
                    {
                        let hex_string = app.hex_view.search.input_hex.value().to_string();

                        if let Some(by) = hex::search::hex_string_to_u8(&hex_string) {
                            ofs = crate::hex::search::search(app, &by)
                        }
                    }

                    if let Some(ofs) = ofs {
                        app.goto(ofs);
                    }
                }
            }
        }
        // comment
        KeyCode::Char(';') => {
            if app.file_info.size > 0 {
                app.state = UIState::DialogComment;
                app.dialog_renderer = Some(hex::comment::dialog_comment_draw);
            }
        }
        // selection
        KeyCode::Char('v') => {
            if app.file_info.size > 0 {
                app.state = UIState::HexSelection;
                app.hex_view.selection.start = app.hex_view.offset;
                app.hex_view.selection.end = app.hex_view.offset;
            }
        }
        // undo
        KeyCode::Char('u') => {
            if let Some(ofs) = app.hex_view.changed_history.pop() {
                let _ = app.hex_view.changed_bytes.remove(&ofs);
            }
        }
        _ => {}
    }
    Ok(false)
}
