use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::io::Result;

use crate::app::App;
use crate::app::Direction;
use crate::editor::UIState;

pub fn select_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc => {
            app.state = UIState::Normal;
            app.hex_view.changed_bytes.clear();
            app.dialog_renderer = None;
            app.hex_view.editing_hex = true;
        }

        KeyCode::Left | KeyCode::Char('h') => {
            let new_offset = app.hex_view.offset.saturating_sub(1);

            // return if at the first offset
            if new_offset == 0 {
                return Ok(true);
            }

            // unset direction if at the selection origin
            if app.hex_view.selection.start == app.hex_view.selection.end {
                app.hex_view.selection.direction = None;
            }

            match app.hex_view.selection.direction {
                None => {
                    app.hex_view.selection.direction = Some(Direction::Left);
                    app.hex_view.selection.start = new_offset;
                }
                Some(Direction::Left) => app.hex_view.selection.start = new_offset,
                Some(Direction::Right) => app.hex_view.selection.end = new_offset - 1,
            }

            app.goto(new_offset);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            let new_offset = app.hex_view.offset + 1;

            // return if at the last offset
            if new_offset >= app.file_info.size {
                return Ok(true);
            }

            // unset direction if at the selection origin
            if app.hex_view.selection.start == app.hex_view.selection.end {
                app.hex_view.selection.direction = None;
            }

            match app.hex_view.selection.direction {
                None => {
                    app.hex_view.selection.direction = Some(Direction::Right);
                    app.hex_view.selection.end = new_offset;
                }
                Some(Direction::Left) => app.hex_view.selection.start = new_offset + 1,
                Some(Direction::Right) => app.hex_view.selection.end = new_offset,
            }

            app.goto(new_offset);
        }
        KeyCode::Enter => {
            app.state = UIState::Normal;
            app.hex_view.editing_hex = true; // just in case it was in ASCII before
        }

        // actions
        // fill with zero
        KeyCode::Char('z') => {
            if app.file_info.is_read_only {
                return Ok(true);
            }

            app.state = UIState::HexEditing;
            let s = format!("{:02X}", 0x00);
            for offset in app.hex_view.selection {
                app.hex_view.changed_bytes.insert(offset, s.clone());
            }
        }
        // fill with NOPs
        KeyCode::Char('n') => {
            if app.file_info.is_read_only {
                return Ok(true);
            }

            app.state = UIState::HexEditing;
            let s = format!("{:02X}", 0x90);
            for offset in app.hex_view.selection {
                app.hex_view.changed_bytes.insert(offset, s.clone());
            }
        }
        // yank
        KeyCode::Char('y') => {
            let mut s = String::new();
            for offset in app.hex_view.selection {
                let b = app.read_u8(offset);
                if let Some(byte) = b {
                    s.push_str(&format!("{:02X}", byte));
                }
            }
            if let Ok(clip) = app.clipboard.as_mut() {
                let _ = clip.set_text(s);
            }
            app.state = UIState::Normal;
        }
        _ => {}
    }
    Ok(false)
}
