use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;
use std::time::Duration;

use crate::app::App;
use crate::commands;
use crate::editor::{AppView, UIState};
use crate::global;
use crate::hex;
use crate::text;

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

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        let event = event::read()?;
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match app.state {
                    UIState::Normal => {
                        global::events::handle_global_events(app, key)?;
                        match app.editor_view {
                            AppView::Hex => hex::events::hex_mode_events(app, key)?,
                            AppView::Text => text::events::text_mode_events(app, key)?,
                        }
                    }
                    UIState::DialogHelp => handle_dialog_error_events(app, key)?,
                    UIState::DialogEncoding => {
                        text::dialog_encoding::dialog_encoding_events(app, key)?
                    }
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
                    UIState::DialogNamesRegex => {
                        hex::names::dialog_names_regex_events(app, &event)?
                    }
                    UIState::DialogCalculator => {
                        global::calculator::dialog_calculator_events(app, &event)?
                    }
                };
            }
            _ => {}
        }
    }
    Ok(false)
}
