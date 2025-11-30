use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::app::App;

pub fn ruler_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let ruler = format!(
        "          {}",
        (0..app.config.hex_mode_bytes_per_line)
            .map(|i| format!("{:02X}", i))
            .collect::<Vec<_>>()
            .join(" ")
    );

    let ruler_para = Paragraph::new(ruler).style(app.config.theme.offsets);
    frame.render_widget(ruler_para, area);
}
