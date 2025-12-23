use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Clear, Paragraph, Wrap},
};

use crate::app::App;

// FIXME: Show the entire file contents in text view. Currently,
// it only shows up to APP_CACHE_SIZE bytes from the file.
pub fn text_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let buffer = app.file_info.get_buffer();
    let limit = (area.height * area.width) as usize;
    let (mut text, _, had_error) = app
        .text_view
        .table
        .decode(&buffer[app.reader.page_start..app.reader.page_start + limit]);

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
