use std::fmt::{Display, LowerHex};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Row, Table},
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

    let block = Block::new().borders(Borders::all());

    // section table

    let mut rows = Vec::new();

    for sec in &pe.sections {
        let mut cells = Vec::new();
        if let Ok(name) = sec.name() {
            // items.push(ListItem::new(name.to_string()));
            cells.push(Cell::new(name));
        }
        cells.push(Cell::new(sec.virtual_address.to_string()));
        cells.push(Cell::new(sec.virtual_size.to_string()));
        cells.push(Cell::new(sec.pointer_to_raw_data.to_string()));
        cells.push(Cell::new(sec.size_of_raw_data.to_string()));
        cells.push(Cell::new(sec.characteristics.to_string()));

        rows.push(Row::new(cells));
    }

    let widths = [Constraint::Length(15); 6];
    let table = Table::new(rows, widths)
        .column_spacing(1)
        .style(app.config.theme.main)
        .header(
            Row::new(vec![
                "Name",
                "VirtualAddress",
                "VirtualSize",
                "PtrToRawData",
                "SizeOfRawData",
                "Characteristics",
            ])
            .style(Style::new().bold()),
        )
        .row_highlight_style(Style::new().reversed())
        .column_highlight_style(Style::new().red())
        .cell_highlight_style(Style::new().blue());

    let list = List::new(items)
        .style(app.config.theme.main)
        .highlight_style(app.config.theme.highlight)
        .block(block);

    frame.render_widget(Clear, area);

    let layout = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let area1 = area.layout_vec(&layout);

    frame.render_stateful_widget(list, area1[0], &mut app.header_view.list_state);

    frame.render_widget(table, area1[1]);

    Ok(())
}

pub fn header_contents_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    if app.file_info.r#type == "PE" {
        let _ = header_contents_pe_draw(app, frame, area);
    }
}
