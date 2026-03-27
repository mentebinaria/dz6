use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Row, Table},
};

use crate::app::App;

pub fn pe_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let pe_header = match app.header_view.pe.as_ref() {
        Some(h) => h,
        None => return,
    };

    // tabs
    // let tabs = Tabs::new(vec!["DOS", "PE/COFF", "Dirs", "Sections", "Imports"])
    //     .style(app.config.theme.main)
    //     .highlight_style(app.config.theme.highlight)
    //     .divider("|")
    //     .padding(" ", " ");

    let mut items = Vec::new();

    items.push(ListItem::new(format!(
        "{}: {:X}",
        "Characteristics", pe_header.coff_header.characteristics
    )));
    items.push(ListItem::new(format!(
        "{}: {:X}",
        "Machine", pe_header.coff_header.machine
    )));
    items.push(ListItem::new(format!(
        "{}: {}",
        "Compilation Timestamp", pe_header.coff_header.time_date_stamp
    )));

    if let Some(opt) = &pe_header.optional_header {
        items.push(ListItem::new(format!(
            "{}: {:X}",
            "Entrypoint", opt.address_of_entry_point
        )));
        items.push(ListItem::new(format!("{}: {:X}", "Magic", opt.magic)));
        items.push(ListItem::new(format!(
            "{}: {}",
            "Minor linker version", opt.minor_linker_version
        )));
        items.push(ListItem::new(format!(
            "{}: {}",
            "Major linker version", opt.major_linker_version
        )));
    }

    let headers_block = Block::new()
        .borders(Borders::all())
        .title(pe_header.summary.clone())
        .title_alignment(Alignment::Center);

    let header_listing = List::new(items)
        .style(app.config.theme.main)
        .highlight_style(app.config.theme.highlight)
        .block(headers_block);

    // section table

    let mut rows = Vec::new();

    for sec in &pe_header.sections {
        let mut cells = Vec::new();
        cells.push(Cell::new(sec.name.as_str()));
        cells.push(Cell::new(format!("{:08X}", sec.virtual_address)));
        cells.push(Cell::new(format!("{:08X}", sec.virtual_size)));
        cells.push(Cell::new(format!("{:08X}", sec.pointer_to_raw_data)));
        cells.push(Cell::new(format!("{:08X}", sec.size_of_raw_data)));
        cells.push(Cell::new(format!("{:08X}", sec.characteristics)));

        rows.push(Row::new(cells));
    }

    let sections_block = Block::new()
        .borders(Borders::all())
        .title("Sections")
        .title_alignment(Alignment::Center);

    let widths = [Constraint::Ratio(1, 6); 6];

    let section_table = Table::new(rows, widths)
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
        .cell_highlight_style(app.config.theme.highlight)
        .block(sections_block);

    // imports
    let mut rows = Vec::new();

    for imp in &pe_header.imports {
        let mut cells = Vec::new();
        cells.push(Cell::new(imp.dll.as_str()));
        cells.push(Cell::new(imp.name.as_str()));
        cells.push(Cell::new(format!("{:08X}", imp.offset)));
        cells.push(Cell::new(format!("{:08X}", imp.ordinal)));
        cells.push(Cell::new(format!("{:08X}", imp.rva)));
        // cells.push(Cell::new(format!("{:08X}", imp.size)));

        rows.push(Row::new(cells));
    }

    let imports_block = Block::new()
        .borders(Borders::all())
        .title("Imports")
        .title_alignment(Alignment::Center);

    let widths = [Constraint::Ratio(1, 5); 5];

    let imports_table = Table::new(rows, widths)
        .column_spacing(1)
        .style(app.config.theme.main)
        .header(
            Row::new(vec![
                "DLL", "Name", "Offset", "Ordinal", "RVA",
                // "Size",
            ])
            .style(Style::new().bold()),
        )
        .cell_highlight_style(app.config.theme.highlight)
        .block(imports_block);

    let layout = Layout::vertical([
        Constraint::Length(9),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]);
    let sub_area = area.layout_vec(&layout);

    frame.render_widget(Clear, area);
    // frame.render_widget(tabs, area);
    frame.render_stateful_widget(
        header_listing,
        sub_area[0],
        &mut app.header_view.header_list_state,
    );
    frame.render_stateful_widget(
        section_table,
        sub_area[1],
        &mut app.header_view.section_table_state,
    );
    frame.render_stateful_widget(
        imports_table,
        sub_area[2],
        &mut app.header_view.imports_table_state,
    );
}
