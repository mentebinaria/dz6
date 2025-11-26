use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Clear, Paragraph, Wrap},
};

use crate::{app::App, config::APP_CACHE_SIZE};

// FIXME: Show the entire file contents in text view. Currently,
// it only shows up to APP_CACHE_SIZE bytes from the file.
pub fn text_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let limit = app.file_info.size.min(APP_CACHE_SIZE);
    let (mut text, _, had_error) = app.text_view.table.decode(&app.buffer[..limit]);

    if had_error {
        text = text
            .chars()
            .map(|c| if c.is_ascii_graphic() { c } else { ' ' })
            .collect();
    }

    app.text_view.lines_to_show = text.lines().count();

    let paragraph = Paragraph::new(text)
        .style(app.config.theme.main)
        .wrap(Wrap { trim: true })
        .scroll(app.text_view.scroll_offset);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}
