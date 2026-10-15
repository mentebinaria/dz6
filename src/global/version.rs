use crate::{app::App, util::center_widget};

use ratatui::{
    Frame, symbols,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap},
};

use crate::editor::UIState;

use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;

pub fn dialog_version_draw(app: &mut App, frame: &mut Frame) {
    let s = format!(
        "{} v{} Copyright (C) 2026 Mente Binária - {}
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>        
This program comes with ABSOLUTELY NO WARRANTY. This is free software, and you are welcome to redistribute it under certain conditions.",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_HOMEPAGE"),
    );
    let para = Paragraph::new(s).wrap(Wrap { trim: true });

    // let dialog_area = center_widget(68, 8, frame.area());
    let dialog_area = center_widget(68, 8, frame.area());

    let block = Block::new()
        .borders(Borders::ALL)
        .border_set(symbols::border::DOUBLE)
        .style(app.config.theme.dialog)
        .padding(Padding::horizontal(1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para.block(block), dialog_area);
}

pub fn dialog_version_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            _ => {}
        }
    }

    Ok(false)
}
