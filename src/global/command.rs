use crate::{app::App, util::parse_goto_expression};
use ratatui::{
    Frame,
    widgets::{Clear, Paragraph},
};

use crate::{editor::UIState, widgets::ErrorMessage};

use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;
use tui_input::backend::crossterm::EventHandler;

pub fn command_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(format!(":{}", app.command_input.value()));

    frame.render_widget(Clear, app.command_area);
    frame.render_widget(para, app.command_area);
    let x = app.command_input.visual_cursor();
    frame.set_cursor_position((app.command_area.x + 1 + x as u16, app.command_area.y));
}

fn parse_command(app: &mut App, cmd: &str) {
    if cmd == "q" {
        app.running = false;
    }

    // check if command is an offset
    if let Ok(mut ofs) = parse_goto_expression(cmd) {
        if cmd.starts_with('+') {
            ofs += app.hex_view.offset;
        }
        if ofs < app.file_info.size {
            app.dialog_renderer = None;
            app.goto(ofs);
        } else {
            app.dialog_renderer = Some(command_error_invalid_offset_draw);
        }
    } else {
        app.dialog_renderer = Some(command_error_invalid_draw);
    }
}

pub fn command_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Enter => {
                let v = app.command_input.value_and_reset();
                parse_command(app, &v);
                app.state = UIState::Normal;
            }
            _ => {
                app.command_input.handle_event(event);
            }
        }
    }
    Ok(false)
}

pub fn command_error_invalid_offset_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = ErrorMessage::new();
    dialog.buffer = format!(
        "Invalid offset. Maximum offset for this file: {:X}",
        app.file_info.size - 1
    );
    dialog.render(app, frame);
}

pub fn command_error_invalid_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = ErrorMessage::new();
    dialog.buffer = String::from("Invalid command");
    dialog.render(app, frame);
}
