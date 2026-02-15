use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::io::Result;

use crate::app::App;
use crate::editor::UIState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
    pub direction: Option<Direction>,
}

impl IntoIterator for Selection {
    type Item = usize;
    type IntoIter = std::ops::RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

impl Selection {
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset <= self.end
    }
    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
        self.direction = None;
    }
    pub fn select_left(&mut self, offset: usize) {
        // unset direction if at the selection origin
        if self.start == self.end {
            self.direction = None;
        }

        match self.direction {
            None => {
                self.direction = Some(Direction::Left);
                self.start = offset;
            }
            Some(Direction::Left) => self.start = offset,
            Some(Direction::Right) => self.end = offset.saturating_sub(1),
        }
    }
    pub fn select_right(&mut self, offset: usize) {
        // unset direction if at the selection origin
        if self.start == self.end {
            self.direction = None;
        }

        match self.direction {
            None => {
                self.direction = Some(Direction::Right);
                self.end = offset;
            }
            Some(Direction::Left) => self.start = offset + 1,
            Some(Direction::Right) => self.end = offset,
        }
    }
}

pub fn select_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app.state = UIState::Normal;
            app.dialog_renderer = None;
            app.hex_view.editing_hex = true;
            app.hex_view.selection.clear();
        }

        // Navigation
        KeyCode::Left | KeyCode::Char('h') => {
            let new_offset = app.hex_view.offset.saturating_sub(1);

            app.hex_view.selection.select_left(new_offset);
            app.goto(new_offset);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            let new_offset = app.hex_view.offset + 1;

            // return if at the last offset
            if new_offset >= app.file_info.size {
                return Ok(true);
            }

            app.hex_view.selection.select_right(new_offset);
            app.goto(new_offset);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let new_offset = app
                .hex_view
                .offset
                .saturating_sub(app.config.hex_mode_bytes_per_line);

            // return if at the first offset
            if new_offset == 0 {
                return Ok(true);
            }

            app.hex_view.selection.select_left(new_offset);
            app.goto(new_offset);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let new_offset = app
                .hex_view
                .offset
                .saturating_add(app.config.hex_mode_bytes_per_line)
                .min(app.file_info.size - 1);

            app.hex_view.selection.select_right(new_offset);
            app.goto(new_offset);
        }

        // Actions
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
            app.hex_view.selection.clear();
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
            app.hex_view.selection.clear();
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
            app.hex_view.selection.clear();
        }
        _ => {}
    }

    Ok(false)
}
