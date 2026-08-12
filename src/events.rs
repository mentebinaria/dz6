use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;

use crate::app::App;
use crate::editor::{AppView, UIState};
use crate::global;
use crate::hex;
use crate::text;
use crate::{commands, header};

pub fn handle_dialog_error_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.dialog_renderer = None;
            app.state = UIState::Normal;
        }
        _ => {}
    }
    Ok(false)
}

pub fn handle_events(app: &mut App, event: Event) -> Result<bool> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            match app.state {
                UIState::Normal | UIState::Error => {
                    global::events::handle_global_events(app, key)?;
                    match app.editor_view {
                        AppView::Hex => hex::events::hex_mode_events(app, key)?,
                        AppView::Text => text::events::text_mode_events(app, key)?,
                        AppView::Header => header::events::header_view_events(app, key)?,
                    }
                }
                UIState::DialogHelp => handle_dialog_error_events(app, key)?,
                UIState::DialogEncoding => text::dialog_encoding::dialog_encoding_events(app, key)?,
                UIState::DialogSearch => hex::search::dialog_search_events(app, &event)?,
                UIState::Command => commands::command_events(app, &event)?,
                UIState::HexEditing => hex::edit::edit_events(app, key)?,
                UIState::HexSelection => hex::selection::select_events(app, key)?,
                UIState::DialogStrings => hex::strings::dialog_strings_events(app, key)?,
                UIState::DialogStringsRegex => {
                    hex::strings::dialog_strings_regex_events(app, &event)?
                }
                UIState::DialogLog => global::log::dialog_log_events(app, key)?,
                UIState::DialogComment => hex::comment::dialog_comment_events(app, &event)?,
                UIState::DialogNames => hex::names::dialog_names_events(app, &event)?,
                UIState::DialogNamesRegex => hex::names::dialog_names_regex_events(app, &event)?,
                UIState::DialogCalculator => {
                    global::calculator::dialog_calculator_events(app, &event)?
                }
                UIState::DialogTruncate => hex::truncate::dialog_truncate_events(app, &event)?,
                UIState::DialogReverseTruncate => {
                    hex::truncate::dialog_reverse_truncate_events(app, &event)?
                }
            };
        }
        Event::Resize(width, _height) if app.config.hex_mode_bytes_per_line_auto => {
            let max = ((width - 9) / 4) as usize;
            app.config.hex_mode_bytes_per_line = max - 1;
        }
        _ => {}
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use crate::update_page_size;

    use super::*;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    fn test_app() -> App {
        let mut app = App::new();
        app.load_file("test_data/test.bin", 0, false).unwrap();
        update_page_size(&mut app, 5); // set height to 2 so we can test page up/down behavior
        app
    }

    #[test]
    fn test_normal_mode_movements() {
        use KeyCode::*;
        let mut app = test_app();
        let end = app.file_info.size - 1;
        let line_end = app.config.hex_mode_bytes_per_line - 1;
        let page_size = app.reader.page_current_size;

        // (name, initial offset, key code, (expected cursor x, expected cursor y), new_offset)
        let cases = [
            // right
            ("right", 0, Right, (1, 0), 1),
            ("right at edge", line_end, Right, (0, 1), line_end + 1),
            ("right at end", end, Right, (line_end, 1), end),
            // left
            ("left", 6, Left, (5, 0), 5),
            ("left at zero", 0, Left, (0, 0), 0),
            ("left at edge", line_end + 1, Left, (line_end, 0), line_end),
            // down
            ("down", 0, Down, (0, 1), line_end + 1),
            ("down at edge", line_end + 1, Down, (0, 1), page_size),
            ("down at end", end, Down, (15, 1), end),
            // up
            ("up", line_end + 3, Up, (2, 0), 2),
            ("up at egde", page_size + 3, Up, (3, 0), line_end + 4),
            ("up at start", 0, Up, (0, 0), 0),
            // page up
            ("page up", page_size + 7, PageUp, (7, 0), 7),
            (
                "page up 2nd line",
                page_size + line_end + 3,
                PageUp,
                (2, 0), // page up from 2nd line should go to first line
                line_end + 3,
            ),
            ("page up at start", 0, PageUp, (0, 0), 0),
            // page down
            ("page down", 4, PageDown, (4, 1), page_size + 4),
            ("page down end", end, PageDown, (15, 1), end),
        ];

        for case in cases {
            let (name, initial_offset, key_code, expected_point, expected_offset) = case;
            app.goto(initial_offset); // this goto is also being tested in a way
            let event = Event::Key(KeyEvent::new(key_code, KeyModifiers::NONE));
            handle_events(&mut app, event).unwrap();

            let cursor_pos = (app.hex_view.cursor.x, app.hex_view.cursor.y);
            assert_eq!(cursor_pos, expected_point, "Test case '{name}' failed");
            let new_offset = app.hex_view.offset;
            assert_eq!(new_offset, expected_offset, "Test case '{name}' failed");
        }
    }
}
