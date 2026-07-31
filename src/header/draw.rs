use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    text::Text,
};

use crate::{app::App, header::formats};

pub fn header_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    if app.file_info.r#type == "PE" {
        formats::pe::draw::pe_draw(app, frame, area);
    } else if app.file_info.r#type == "ELF" {
        formats::elf::draw::elf_draw(app, frame, area);
    } else {
        let message = Text::from("No header view available for this file format").centered();

        frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
    }
}
