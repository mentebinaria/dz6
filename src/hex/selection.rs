use ratatui::crossterm::event::{KeyCode, KeyEvent};
use std::io::Result;

use crate::app::App;
use crate::editor::UIState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    LeftOrUp,
    RightOrDown,
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
    pub fn select_left_or_up(&mut self, step: usize) {
        match self.direction {
            None => {
                self.direction = Some(Direction::LeftOrUp);
                self.start = self.start.saturating_sub(step);
            }
            Some(Direction::LeftOrUp) => self.start = self.start.saturating_sub(step),
            Some(Direction::RightOrDown) => self.end = self.end.saturating_sub(step),
        }

        if self.start == self.end {
            self.direction = None;
        }
    }
    pub fn select_right_or_down(&mut self, offset_max: usize, step: usize) {
        match self.direction {
            None => {
                self.direction = Some(Direction::RightOrDown);
                self.end = (self.start + step).min(offset_max);
            }
            Some(Direction::LeftOrUp) => self.start = (self.start + step).min(offset_max),
            Some(Direction::RightOrDown) => self.end = (self.end + step).min(offset_max),
        }
        if self.start == self.end {
            self.direction = None;
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

            app.hex_view.selection.select_left_or_up(1);
            app.goto(new_offset);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            let new_offset = app.hex_view.offset + 1;

            // return if at the last offset
            if new_offset >= app.file_info.size {
                return Ok(true);
            }

            app.hex_view.selection.select_right_or_down(app.file_info.size, 1);
            app.goto(new_offset);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let new_offset = app
                .hex_view
                .offset
                .saturating_sub(app.config.hex_mode_bytes_per_line);

            if app.hex_view.selection.direction == Some(Direction::RightOrDown) && new_offset < app.hex_view.selection.start {
                return Ok(true);
            }

            app.hex_view
                .selection
                .select_left_or_up(app.config.hex_mode_bytes_per_line);
            app.goto(new_offset);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let new_offset = app
                .hex_view
                .offset
                .saturating_add(app.config.hex_mode_bytes_per_line)
                .min(app.file_info.size - 1);

            if app.hex_view.selection.direction == Some(Direction::LeftOrUp) && new_offset > app.hex_view.selection.end {
                return Ok(true);
            }

            app.hex_view
                .selection
                .select_right_or_down(app.file_info.size, app.config.hex_mode_bytes_per_line);
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
