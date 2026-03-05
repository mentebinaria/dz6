use std::fmt::{Display, LowerHex};

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Clear, List, ListItem},
};

use crate::app::App;

use goblin::error;
use goblin::pe::PE;

fn add_to_list_hex<T: LowerHex>(items: &mut Vec<ListItem>, label: &str, number: T) {
    items.push(ListItem::new(format!("{:}: {:#x}", label, number)));
}

fn add_to_list_dec<T: Display>(items: &mut Vec<ListItem>, label: &str, number: T) {
    items.push(ListItem::new(format!("{:}: {:}", label, number)));
}

fn header_contents_pe_draw(app: &mut App, frame: &mut Frame, area: Rect) -> error::Result<()> {
    let buffer = app.file_info.get_buffer();
    let pe = PE::parse(&buffer)?;
    let mut items = Vec::new();

    add_to_list_hex(
        &mut items,
        "Characteristics",
        pe.header.coff_header.characteristics,
    );
    add_to_list_hex(&mut items, "Machine", pe.header.coff_header.machine);
    add_to_list_hex(
        &mut items,
        "Compilation Timestamp",
        pe.header.coff_header.time_date_stamp,
    );

    if let Some(opt) = pe.header.optional_header {
        let ep_rva = opt.standard_fields.address_of_entry_point;
        for sec in &pe.sections {
            if ep_rva >= sec.virtual_address && ep_rva < sec.virtual_address + sec.virtual_size {
                //TODO check over/underflow
                app.header_view.entrypoint = ep_rva - sec.virtual_address + sec.pointer_to_raw_data;
            }
        } 
        
        add_to_list_hex(
            &mut items,
            "Entrypoint",
            opt.standard_fields.address_of_entry_point,
        );
        add_to_list_hex(&mut items, "Magic", opt.standard_fields.magic);
        add_to_list_dec(
            &mut items,
            "Minor linker version",
            opt.standard_fields.minor_linker_version,
        );
        add_to_list_dec(
            &mut items,
            "Major linker version",
            opt.standard_fields.major_linker_version,
        );
    }

    for sec in &pe.sections {
        if let Ok(name) = sec.name() {
            items.push(ListItem::new(name.to_string()));
        }
    }

    for lib in pe.libraries {
        items.push(ListItem::new(lib));
    }

    let list = List::new(items)
        .style(app.config.theme.main)
        .highlight_style(app.config.theme.highlight)
        .block(Block::new().borders(Borders::all()));

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut app.header_view.list_state);

    Ok(())
}

pub fn header_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    if app.file_info.r#type == "PE" {
        let _ = header_contents_pe_draw(app, frame, area);
    }
}
