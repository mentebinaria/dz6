use std::io::Result;

use ratatui::crossterm::event::KeyEvent;

use crate::{app::App, header::formats};

pub fn header_view_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    match app.file_info.r#type {
        "ELF" => formats::elf::events::header_elf_view_events(app, key),
        "PE" => formats::pe::events::header_pe_view_events(app, key),
        _ => Ok(false),
    }
}
