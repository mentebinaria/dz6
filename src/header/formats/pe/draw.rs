use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Cell, Clear, Row, Table, Tabs},
};

use crate::app::App;

fn draw_dos_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let mut rows = Vec::new();

        rows.push(Row::new(vec![
            Cell::new("Signature"),
            Cell::new(format!("{:X}", pe.dos_header.signature)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("BytesOnLastPage"),
            Cell::new(format!("{:X}", pe.dos_header.bytes_on_last_page)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("PagesInFile"),
            Cell::new(format!("{:X}", pe.dos_header.pages_in_file)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Relocations"),
            Cell::new(format!("{:X}", pe.dos_header.relocations)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("SizeOfHeaderInParagraphs"),
            Cell::new(format!("{:X}", pe.dos_header.size_of_header_in_paragraphs)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("MinimumExtraParagraphsNeeded"),
            Cell::new(format!(
                "{:X}",
                pe.dos_header.minimum_extra_paragraphs_needed
            )),
        ]));
        rows.push(Row::new(vec![
            Cell::new("MaximumExtraParagraphsNeeded"),
            Cell::new(format!(
                "{:X}",
                pe.dos_header.maximum_extra_paragraphs_needed
            )),
        ]));
        rows.push(Row::new(vec![
            Cell::new("InitialRelativeSS"),
            Cell::new(format!("{:X}", pe.dos_header.initial_relative_ss)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("InitialSP"),
            Cell::new(format!("{:X}", pe.dos_header.initial_sp)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Checksum"),
            Cell::new(format!("{:X}", pe.dos_header.checksum)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("InitialIP"),
            Cell::new(format!("{:X}", pe.dos_header.initial_ip)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("InitialRelativeCS"),
            Cell::new(format!("{:X}", pe.dos_header.initial_relative_cs)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("FileAddressOfRelocationTable"),
            Cell::new(format!(
                "{:X}",
                pe.dos_header.file_address_of_relocation_table
            )),
        ]));
        rows.push(Row::new(vec![
            Cell::new("OverlayNumber"),
            Cell::new(format!("{:X}", pe.dos_header.overlay_number)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Reserved"),
            Cell::new(format!("{:?}", pe.dos_header.reserved)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("OemId"),
            Cell::new(format!("{:X}", pe.dos_header.oem_id)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("OemInfo"),
            Cell::new(format!("{:X}", pe.dos_header.oem_info)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Reserved2"),
            Cell::new(format!("{:?}", pe.dos_header.reserved2)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("PEHeaderOffset"),
            Cell::new(format!("{:X}", pe.dos_header.pe_pointer)),
        ]));

        let widths = [Constraint::Min(16), Constraint::Fill(1)];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.pe_state.dos_header_table_state,
        );
    }
}

fn draw_pe_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let mut rows = Vec::new();

        let machine = pe.coff_header.machine;
        rows.push(Row::new(vec![
            Cell::new("Machine"),
            Cell::new(format!(
                "{:X} ({})",
                machine,
                get_pe_machine_string(machine)
            )),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Characteristics"),
            Cell::new(format!("{:X}", pe.coff_header.characteristics)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Compilation Timestamp"),
            Cell::new(format!("{:X}", pe.coff_header.time_date_stamp)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("Number of sections"),
            Cell::new(
                pe.optional_header
                    .unwrap()
                    .windows_fields
                    .number_of_rva_and_sizes
                    .to_string(),
            ),
        ]));

        if let Some(opt) = &pe.optional_header {
            rows.push(Row::new(vec![
                Cell::new("Entrypoint"),
                Cell::new(format!("{:X}", opt.standard_fields.address_of_entry_point)),
            ]));
            rows.push(Row::new(vec![
                Cell::new("Magic"),
                Cell::new(format!("{:X}", opt.standard_fields.magic)),
            ]));
            rows.push(Row::new(vec![
                Cell::new("Minor linker version"),
                Cell::new(opt.standard_fields.minor_linker_version.to_string()),
            ]));
            rows.push(Row::new(vec![
                Cell::new("Major linker version"),
                Cell::new(opt.standard_fields.major_linker_version.to_string()),
            ]));
        }

        let widths = [Constraint::Min(16), Constraint::Fill(1)];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_widget(header_table, area);
    }
}

fn draw_sections(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let mut rows = Vec::new();

        for sec in &pe.sections {
            let mut cells = Vec::new();
            cells.push(Cell::new(sec.name().unwrap_or("")));
            cells.push(Cell::new(format!("{:08X}", sec.virtual_address)));
            cells.push(Cell::new(format!("{:08X}", sec.virtual_size)));
            cells.push(Cell::new(format!("{:08X}", sec.pointer_to_raw_data)));
            cells.push(Cell::new(format!("{:08X}", sec.size_of_raw_data)));
            cells.push(Cell::new(format!("{:08X}", sec.characteristics)));
            rows.push(Row::new(cells));
        }

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
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            section_table,
            area,
            &mut app.header_view.pe_state.sections_table_state,
        );
    }
}

fn draw_imports(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let mut rows = Vec::new();

        for imp in &pe.imports {
            let mut cells = Vec::new();
            cells.push(Cell::new(imp.dll.as_str()));
            cells.push(Cell::new(imp.name.as_str()));
            cells.push(Cell::new(format!("{:08X}", imp.offset)));
            cells.push(Cell::new(format!("{:08X}", imp.ordinal)));
            cells.push(Cell::new(format!("{:08X}", imp.rva)));
            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 5); 5];

        let imports_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(
                Row::new(vec!["DLL", "Name", "Offset", "Ordinal", "RVA"])
                    .style(Style::new().bold()),
            )
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            imports_table,
            area,
            &mut app.header_view.pe_state.imports_table_sate,
        );
    }
}

fn draw_overlay(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let overlay_start = pe
            .sections
            .iter()
            .map(|sec| sec.pointer_to_raw_data.saturating_add(sec.size_of_raw_data) as usize)
            .max()
            .unwrap_or(pe.dos_header.pe_pointer as usize);

        let overlay_size = app.file_info.size.saturating_sub(overlay_start);

        let mut rows = Vec::new();

        rows.push(Row::new(vec![
            Cell::new("OverlayStart"),
            Cell::new(format!("{:08X}", overlay_start)),
        ]));
        rows.push(Row::new(vec![
            Cell::new("OverlaySize"),
            Cell::new(format!("{:08X}", overlay_size)),
        ]));

        let widths = [Constraint::Min(16), Constraint::Fill(1)];

        let overlay_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_widget(overlay_table, area);
    }
}

pub fn pe_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(vec![
        "DOS Header",
        "PE Header",
        "Sections",
        "Imports",
        "Overlay",
    ])
    .style(app.config.theme.main)
    .highlight_style(app.config.theme.highlight)
    .divider("|")
    .padding(" ", " ")
    .select(app.header_view.tab_index);

    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
    let [top, main] = area.layout(&layout);

    frame.render_widget(Clear, area);
    frame.render_widget(tabs, top);

    match app.header_view.tab_index {
        0 => draw_dos_header(app, frame, main),
        1 => draw_pe_header(app, frame, main),
        2 => draw_sections(app, frame, main),
        3 => draw_imports(app, frame, main),
        4 => draw_overlay(app, frame, main),
        _ => {}
    }
}

fn get_pe_machine_string(mach: u16) -> &'static str {
    match mach {
        goblin::pe::header::COFF_MACHINE_ARM => "ARM",
        goblin::pe::header::COFF_MACHINE_ARM64 => "AARCH64",
        goblin::pe::header::COFF_MACHINE_X86 => "x86",
        goblin::pe::header::COFF_MACHINE_X86_64 => "x86-64",
        _ => "",
    }
}
