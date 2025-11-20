use crate::app::SearchMode;
use crate::widgets::ErrorMessage;
use crate::{app::App, config::APP_CACHE_SIZE, editor::UIState};
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::Paragraph;
use std::io::Result;
use tui_input::backend::crossterm::EventHandler;

pub fn hex_string_to_u8(hex_string: &str) -> Option<Vec<u8>> {
    if hex_string.is_empty() || !hex_string.len().is_multiple_of(2) {
        return None;
    }
    let bytes = hex::decode(hex_string).unwrap();
    Some(bytes)
}

pub fn search<T: AsRef<[u8]>>(app: &mut App, needle: T) -> Option<usize> {
    let text = needle.as_ref();
    let siz = text.len();
    let nblock = app.reader.cache_block_number;
    for block in nblock..=app.reader.cache_blocks {
        for (i, win) in app.buffer.windows(siz).enumerate() {
            let ofs = i + app.reader.cache_block_number * APP_CACHE_SIZE;
            if win == text && ofs > app.hex_view.offset {
                return Some(ofs);
            }
        }
        let _ = app.read_chunk_from_file(block);
    }
    // restore previous block to buffer
    let _ = app.read_chunk_from_file(nblock);
    None
}

// string
// hex
pub fn dialog_search_draw(app: &mut App, frame: &mut Frame) {
    let x;
    let para;

    match app.hex_view.search.mode {
        SearchMode::Ascii => {
            para = Paragraph::new(format!("/{}", app.hex_view.search.input_text.value()));
            x = app.hex_view.search.input_text.visual_cursor();
        }
        SearchMode::Hex => {
            para = Paragraph::new(format!("/{}", app.hex_view.search.input_hex.value()));
            x = app.hex_view.search.input_hex.visual_cursor();
        }
    };

    frame.render_widget(para, app.command_area);
    frame.set_cursor_position((app.command_area.x + 1 + x as u16, app.command_area.y));
}

pub fn dialog_search_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            // if input is empty, backspace works like Esc; otherwise it's handled by tui-input
            KeyCode::Backspace => match app.hex_view.search.mode {
                SearchMode::Ascii => {
                    if app.hex_view.search.input_text.value().is_empty() {
                        app.dialog_renderer = None;
                        app.state = UIState::Normal;
                    } else {
                        app.hex_view.search.input_text.handle_event(event);
                    }
                }
                SearchMode::Hex => {
                    if app.hex_view.search.input_hex.value().is_empty() {
                        app.dialog_renderer = None;
                        app.state = UIState::Normal;
                    } else {
                        app.hex_view.search.input_hex.handle_event(event);
                    }
                }
            },
            KeyCode::Enter => {
                match app.hex_view.search.mode {
                    SearchMode::Ascii => {
                        let text = app.hex_view.search.input_text.value().to_string();
                        app.state = UIState::Normal;

                        if text.is_empty() {
                            app.dialog_renderer = None;
                            return Ok(false);
                        }

                        if let Some(ofs) = search(app, &text) {
                            app.goto(ofs);
                            app.dialog_renderer = None;
                        } else {
                            app.dialog_renderer = Some(dialog_search_error_draw);
                        }
                    }
                    SearchMode::Hex => {
                        let hex_string = app.hex_view.search.input_hex.value().to_string();

                        if let Some(bytes) = hex_string_to_u8(&hex_string) {
                            if let Some(ofs) = search(app, &bytes) {
                                app.goto(ofs);
                                app.dialog_renderer = None;
                            } else {
                                app.dialog_renderer = Some(dialog_search_error_draw);
                            }
                            app.state = UIState::Normal;
                        }
                    }
                };
            }
            KeyCode::Tab => {
                app.hex_view.search.mode.next();
            }

            KeyCode::Char(c) => {
                match app.hex_view.search.mode {
                    SearchMode::Ascii => app.hex_view.search.input_text.handle_event(event),
                    SearchMode::Hex => {
                        if c.is_ascii_hexdigit() {
                            app.hex_view.search.input_hex.handle_event(event)
                        } else {
                            None
                        }
                    }
                };
            }
            _ => {
                match app.hex_view.search.mode {
                    SearchMode::Ascii => app.hex_view.search.input_text.handle_event(event),
                    SearchMode::Hex => app.hex_view.search.input_hex.handle_event(event),
                };
            }
        }
    }
    Ok(false)
}

pub fn dialog_search_error_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = ErrorMessage::new();
    dialog.buffer = String::from("Pattern not found");
    dialog.render(app, frame);
}
