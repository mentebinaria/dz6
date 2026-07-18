use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
};

use crate::{app::App, editor::UIState, widgets::ListChoice};

use std::io::Result;

const ENCODING_LIST: [&str; 7] = [
    "UTF-8",
    "CP949 (EUC-KR)",
    "CP936 (GBK)",
    "ISO-8859-1",
    "ISO-8859-2",
    "UTF-16-LE",
    "UTF-16-BE",
];

pub fn dialog_encoding_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = ListChoice::new();
    dialog.set_title(" Select encoding ".to_string());
    dialog.choices = ENCODING_LIST.iter().map(|s| s.to_string()).collect();
    dialog.render(app, frame);
}

pub fn dialog_encoding_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        // quit
        KeyCode::Esc => {
            app.state = UIState::Normal;
            app.dialog_renderer = None;
        }
        // switch
        KeyCode::Enter => {
            // app.text_mode.table = app.list_state.selected()
            let sel = app.list_state.selected().unwrap_or(0);
            app.text_view.table = match sel {
                0 => encoding_rs::UTF_8,
                1 => encoding_rs::EUC_KR,
                2 => encoding_rs::GBK,
                3 => encoding_rs::WINDOWS_1252,
                4 => encoding_rs::ISO_8859_2,
                5 => encoding_rs::UTF_16LE,
                6 => encoding_rs::UTF_16BE,
                _ => encoding_rs::UTF_8,
            };

            app.state = UIState::Normal;
            app.dialog_renderer = None;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.list_state.selected() == Some(ENCODING_LIST.len() - 1) {
                app.list_state.select_first();
            } else {
                app.list_state.select_next();
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            if app.list_state.selected() == Some(0) {
                app.list_state.select_last();
            } else {
                app.list_state.select_previous();
            }
        }
        KeyCode::PageUp | KeyCode::Home => {
            app.list_state.select_first();
        }
        KeyCode::PageDown | KeyCode::End => {
            app.list_state.select_last();
        }
        _ => {}
    }
    Ok(false)
}


