use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, editor::UIState, text};

pub fn text_mode_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up => {
            if app.text_view.scroll_offset.0 > 0 {
                app.text_view.scroll_offset.0 -= 1;
            }
        }
        KeyCode::Down => {
            // We calculate the last line number being shown and compare
            // with the total number of lines, which is always updated
            // in crate::draw::draw_text_contents()
            let last_line_shown: usize =
                (app.text_view.scroll_offset.0 + app.text_view.area_height).into();
            if last_line_shown < app.text_view.lines_to_show {
                app.text_view.scroll_offset.0 += 1;
                app.text_view.lines_to_show += 1;
            }

            App::log(app, format!("{:#?}", app.text_view));
        }
        KeyCode::PageUp => {
            if app.hex_view.offset < app.reader.page_current_size {
                app.goto(0);
            } else {
                app.goto(app.hex_view.offset - app.reader.page_current_size);
            }
        }
        KeyCode::PageDown => {
            app.goto(app.hex_view.offset + app.reader.page_current_size);
        }
        KeyCode::Left => {
            if app.text_view.scroll_offset.1 > 0 {
                app.text_view.scroll_offset.1 -= 1;
            }
        }
        KeyCode::Right => {
            app.text_view.scroll_offset.1 += 1;
        }
        KeyCode::Home => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Ctrl+Home goes to the beginning of first line
                app.text_view.scroll_offset = (0, 0);
            } else {
                // Home goes to the beginning of the current line
                app.text_view.scroll_offset.1 = 0;
            }
        }
        KeyCode::End => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Ctrl+End goes to the beginning of the last line
                // if last line is not shown yet
                if app.text_view.lines_to_show > app.text_view.area_height as usize {
                    let delta = app.text_view.lines_to_show - app.text_view.area_height as usize;
                    app.text_view.scroll_offset = (delta as u16, 0)
                }
            }
            // End should go to the end of the current line,
            // but I probably need the length of the biggest line
            // to set text_mode.scroll_offset.1 there
        }
        KeyCode::Char('e') => {
            app.state = UIState::DialogEncoding;
            app.dialog_renderer = Some(text::dialog_encoding::dialog_encoding_draw);
        }
        _ => {}
    }
    Ok(false)
}
