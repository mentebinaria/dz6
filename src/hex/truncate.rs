use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use std::io::{Result, Write};

use crate::{
    app::App,
    editor::UIState,
    widgets::{Message, MessageType},
};

pub fn dialog_truncate(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from(&format!(
        "Permanently delete from offset {:X} to the end of file? (y/N)",
        app.hex_view.offset.saturating_add(1)
    ));
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}

pub fn dialog_truncate_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('y') => {
                if let Some(f) = &app.file_info.file {
                    f.set_len((app.hex_view.offset + 1) as u64)?;
                    app.reload_file();
                }
            }
            _ => {}
        }
        app.dialog_renderer = None;
        app.state = UIState::Normal;
        app.hex_view.editing_hex = true;
    }
    Ok(false)
}

pub fn dialog_reverse_truncate(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from(&format!(
        "Permanently delete from offset 0 to {:X}? (y/N)",
        app.hex_view.offset.saturating_sub(1)
    ));
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}

pub fn dialog_reverse_truncate_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('y') => {
                let buff = &mut app.file_info.get_buffer().to_vec();
                let new_buff = buff.drain(app.hex_view.offset..);

                if let Some(f) = &mut app.file_info.file {
                    f.write_all(new_buff.as_slice())?;
                    f.set_len(new_buff.len() as u64)?;
                    app.reload_file();
                    app.goto(0);
                }
            }
            _ => {}
        }
        app.dialog_renderer = None;
        app.state = UIState::Normal;
        app.hex_view.editing_hex = true;
    }
    Ok(false)
}
