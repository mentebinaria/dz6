use ratatui::{
    Frame,
    crossterm::event::KeyModifiers,
    layout::Alignment,
    symbols,
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph},
};

use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;
use tui_input::backend::crossterm::EventHandler;

use crate::{app::App, editor::UIState, util::center_widget};

pub fn dialog_names_draw(app: &mut App, frame: &mut Frame) {
    let re = if !app.hex_view.names_regex.is_empty() {
        regex::RegexBuilder::new(&app.hex_view.names_regex)
            .case_insensitive(true)
            .build()
            .ok()
    } else {
        None
    };

    let mut items = Vec::new();
    let mut count = 0;

    for cmt in &app.hex_view.comment_name_list {
        let is_match = if let Some(ref r) = re {
            r.is_match(&cmt.comment)
        } else {
            true
        };

        if is_match {
            items.push(ListItem::from(format!(
                "{:08X}  {}",
                cmt.offset, cmt.comment
            )));
            count += 1;
        }
    }

    let list = List::new(items)
        .style(app.config.theme.dialog)
        .block(
            Block::bordered()
                .title(format!(" Names ({}) ", count))
                .title_alignment(Alignment::Center)
                .padding(Padding::horizontal(1)),
        )
        .highlight_style(app.config.theme.highlight)
        .repeat_highlight_symbol(true);

    let width = frame.area().width / 2;
    let height = frame.area().height / 2 + 4;
    let dialog_area = center_widget(width, height, frame.area());

    frame.render_widget(Clear, dialog_area);
    frame.render_stateful_widget(list, dialog_area, &mut app.hex_view.names_list_state);
}

pub fn dialog_names_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.hex_view.names_list_state.select_next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.hex_view.names_list_state.select_previous();
            }
            KeyCode::PageDown => {
                app.hex_view.names_list_state.scroll_down_by(30);
            }
            KeyCode::PageUp => {
                app.hex_view.names_list_state.scroll_up_by(30);
            }
            KeyCode::Home => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.hex_view.names_list_state.select_first();
                } else if let Some(n) = app.hex_view.names_list_state.selected() {
                    // we show 30 strings at a time, so this will select
                    // the string at the top of the list
                    let new_index = n.saturating_sub(29);
                    app.hex_view.names_list_state.select(Some(new_index));
                }
            }
            KeyCode::End => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.hex_view.names_list_state.select_last();
                } else if let Some(n) = app.hex_view.names_list_state.selected() {
                    let new_index = n + 29;
                    app.hex_view.names_list_state.select(Some(new_index));
                }
            }
            KeyCode::Enter => {
                if let Some(choice) = app.hex_view.names_list_state.selected() {
                    let re = if !app.hex_view.names_regex.is_empty() {
                        regex::RegexBuilder::new(&app.hex_view.names_regex)
                            .case_insensitive(true)
                            .build()
                            .ok()
                    } else {
                        None
                    };

                    let mut matched_comments = Vec::new();
                    for cmt in &app.hex_view.comment_name_list {
                        let is_match = if let Some(ref r) = re {
                            r.is_match(&cmt.comment)
                        } else {
                            true
                        };
                        if is_match {
                            matched_comments.push(cmt.clone());
                        }
                    }

                    if choice < matched_comments.len() {
                        app.goto(matched_comments[choice].offset);
                    }
                }
                app.state = UIState::Normal;
                app.dialog_renderer = None;
            }
            KeyCode::Char('D') => {
                app.hex_view.comments.clear();
                app.hex_view.comment_name_list.clear();
            }
            KeyCode::Char('f') => {
                app.state = UIState::DialogNamesRegex;
                app.dialog_2nd_renderer = Some(dialog_names_regex_draw);
            }
            KeyCode::Char('o') => {
                app.hex_view.comment_name_list.sort_by_key(|x| x.offset);
            }
            KeyCode::Char('n') => {
                app.hex_view
                    .comment_name_list
                    .sort_by_key(|x| x.comment.clone());
            }
            _ => {}
        }
    }
    Ok(false)
}

pub fn dialog_names_regex_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(app.hex_view.names_regex_input.value());

    let dialog_area = center_widget(frame.area().width / 3, 3, frame.area());

    let block = Block::new()
        .title(" Regex ")
        .borders(Borders::ALL)
        .border_set(symbols::border::PLAIN)
        .style(app.config.theme.main)
        .padding(Padding::horizontal(1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(para.block(block), dialog_area);
    let x = app.hex_view.names_regex_input.visual_cursor();
    frame.set_cursor_position((dialog_area.x + 2 + x as u16, dialog_area.y + 1));
}

pub fn dialog_names_regex_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_2nd_renderer = None;
                app.state = UIState::DialogNames;
            }
            KeyCode::Enter => {
                app.hex_view.names_regex = String::from(app.hex_view.names_regex_input.value());
                app.dialog_2nd_renderer = None;
                app.state = UIState::DialogNames;
                app.hex_view.names_list_state.select(Some(0));
            }
            _ => {
                app.hex_view.names_regex_input.handle_event(event);
            }
        }
    }
    Ok(false)
}
