use std::io::Result;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, editor::AppView};

pub fn header_view_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.header_view.list_state.select_previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.header_view.list_state.select_next();
        }
        KeyCode::Enter => {
            if let Some(idx) = app.header_view.list_state.selected() {
                if idx == 3 {
                    app.goto(app.header_view.entrypoint as usize);
                    app.editor_view = AppView::Hex;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}
