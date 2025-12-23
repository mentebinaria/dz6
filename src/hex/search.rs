use crate::widgets::{Message, MessageType};
use crate::{app::App, editor::UIState};
use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::Paragraph;
use std::io::Result;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

#[derive(Default, Debug)]
pub struct Search {
    pub input_text: Input,
    pub mode: SearchMode,
    pub input_hex: Input,
}

#[derive(Default, Debug, PartialEq)]
pub enum SearchMode {
    #[default]
    Utf8,
    // UTF_16,
    // UTF_16_LE,
    Hex,
}

impl SearchMode {
    pub fn next(&mut self) {
        if *self == SearchMode::Utf8 {
            *self = SearchMode::Hex;
        } else {
            *self = SearchMode::Utf8
        }
    }
}

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
    let buffer = app.file_info.get_buffer();
    for (i, win) in buffer[app.hex_view.offset + 1..].windows(siz).enumerate() {
        if win == text {
            return Some(app.hex_view.offset + i + 1);
        }
    }

    None
}

// string
// hex
pub fn dialog_search_draw(app: &mut App, frame: &mut Frame) {
    let x;
    let para;

    match app.hex_view.search.mode {
        SearchMode::Utf8 => {
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
                SearchMode::Utf8 => {
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
                    SearchMode::Utf8 => {
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
                    SearchMode::Utf8 => app.hex_view.search.input_text.handle_event(event),
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
                    SearchMode::Utf8 => app.hex_view.search.input_text.handle_event(event),
                    SearchMode::Hex => app.hex_view.search.input_hex.handle_event(event),
                };
            }
        }
    }
    Ok(false)
}

pub fn dialog_search_error_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from("Pattern not found");
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}
