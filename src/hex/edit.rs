use ratatui::crossterm::event::KeyModifiers;

use crate::app::App;

use crate::editor::UIState;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::io::Result;

pub fn fill_with(app: &mut App, with: u8, advance: bool) {
    let s = format!("{:02X}", with);
    app.hex_view.changed_bytes.insert(app.hex_view.offset, s);
    app.hex_view.changed_history.push(app.hex_view.offset);
    if advance {
        app.goto(app.hex_view.offset + 1);
    }
}

pub fn edit_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.state = UIState::Normal;
            // app.hex_view.changed_bytes.clear();
            app.dialog_renderer = None;
            app.hex_view.editing_hex = true;
        }

        KeyCode::Left | KeyCode::Backspace => {
            if app.hex_view.offset > 0 {
                app.goto(app.hex_view.offset - 1);
            }
        }
        KeyCode::Right => {
            app.goto(app.hex_view.offset + 1);
        }
        KeyCode::Up => {
            if app.hex_view.offset >= app.config.hex_mode_bytes_per_line {
                app.goto(app.hex_view.offset - app.config.hex_mode_bytes_per_line);
            }
        }
        KeyCode::Down => {
            app.goto(app.hex_view.offset + app.config.hex_mode_bytes_per_line);
        }

        KeyCode::Tab => {
            app.hex_view.editing_hex = !app.hex_view.editing_hex;
        }

        KeyCode::Char(c) => {
            if app.hex_view.editing_hex {
                if c.is_ascii_hexdigit() && !key.modifiers.contains(KeyModifiers::CONTROL) {
                    // If the hashmap contains the key, it means the user has typed
                    // one character
                    if app
                        .hex_view
                        .changed_bytes
                        .contains_key(&app.hex_view.offset)
                    {
                        // Get the current value and check if it has two characters, meaning
                        // the user navigated back to an already changed offset and will change
                        // it again
                        let value = app
                            .hex_view
                            .changed_bytes
                            .get_mut(&app.hex_view.offset)
                            .unwrap(); // It should be safe as we checked for .contains_key()
                        if value.len() == 2 {
                            // There are two characters there already, restart the process
                            // by replacing the value using the same key
                            app.hex_view
                                .changed_bytes
                                .insert(app.hex_view.offset, c.to_ascii_uppercase().to_string());
                            // Update history for undo command
                            app.hex_view.changed_history.push(app.hex_view.offset);
                        } else {
                            // If the number of characters there is not two, concatenate
                            // what's in there with whatever the user typed
                            (*value).push(c.to_ascii_uppercase());
                            app.hex_view.changed_history.push(app.hex_view.offset);
                            app.goto(app.hex_view.offset + 1);
                        }
                    } else {
                        // First char was typed, just add it to the hashmap
                        app.hex_view
                            .changed_bytes
                            .insert(app.hex_view.offset, c.to_ascii_uppercase().to_string());
                    }
                } else if c == 'z' {
                    // zero out bytes
                    fill_with(app, 0x00, true);
                } else if c == 'n' {
                    // NOP bytes
                    fill_with(app, 0x90, true);
                } else if c == 'a' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    let ofs = app.hex_view.offset;
                    if let Some(s) = app.hex_view.changed_bytes.get(&ofs) {
                        if let Ok(b) = u8::from_str_radix(s, 16) {
                            fill_with(app, b.wrapping_add(1), false);
                        }
                    } else if let Some(b) = app.read_u8(ofs) {
                        fill_with(app, b.wrapping_add(1), false);
                    }
                } else if c == 'x' && key.modifiers.contains(KeyModifiers::CONTROL) {
                    let ofs = app.hex_view.offset;
                    if let Some(s) = app.hex_view.changed_bytes.get(&ofs) {
                        if let Ok(b) = u8::from_str_radix(s, 16) {
                            fill_with(app, b.wrapping_sub(1), false);
                        }
                    } else if let Some(b) = app.read_u8(ofs) {
                        fill_with(app, b.wrapping_sub(1), false);
                    }
                } else if c == 'T' {
                    // truncate the file
                    if let Some(f) = &app.file_info.file {
                        f.set_len((app.hex_view.offset + 1) as u64).unwrap();
                        app.reload_file();
                        app.state = UIState::Normal;
                        app.hex_view.editing_hex = true;
                    }
                }
            } else {
                fill_with(app, c as u8, true);
            }
        }
        _ => {}
    }
    Ok(false)
}
