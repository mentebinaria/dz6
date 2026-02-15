use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::{app::App, editor::UIState, hex::search::SearchMode};

pub fn status_bar_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    // Bookmarks
    let mut bookmarks_string = String::new();
    for i in 1..app.hex_view.bookmarks.len() {
        bookmarks_string.push_str(&i.to_string());
    }

    if !app.hex_view.bookmarks.is_empty() {
        bookmarks_string.push('★');
    }

    for i in 0..8 {
        if app.hex_view.bookmarks.get(i).is_none() {
            bookmarks_string.push('-');
        }
    }

    let mode = match app.state {
        UIState::Normal => "NORMAL",
        UIState::HexEditing => "REPLACE",
        UIState::HexSelection => "SELECT",
        UIState::DialogSearch => {
            if app.hex_view.search.mode == SearchMode::Hex {
                "SEARCH/HEX"
            } else {
                "SEARCH/UTF-8"
            }
        }
        UIState::Command => "COMMAND",
        _ => "",
    };

    let filename = app.file_info.name.clone();
    let percent = if app.file_info.size == 0 {
        0.
    } else {
        (app.hex_view.offset as f64 / app.file_info.size as f64 * 100.0).round()
    };

    let read_only = if app.file_info.is_read_only {
        "[RO]"
    } else {
        ""
    };

    let status_bar_info_left = Paragraph::new(format!("{} {}", filename, read_only))
        .style(app.config.theme.topbar)
        .alignment(Alignment::Left);
    frame.render_widget(status_bar_info_left, area);

    let status_bar_info_right = Paragraph::new(format!(
        "{} {} {} {:08X} {}%",
        mode, bookmarks_string, app.file_info.r#type, app.hex_view.offset, percent
    ))
    .style(app.config.theme.topbar)
    .alignment(Alignment::Right);
    frame.render_widget(status_bar_info_right, area);
}
