use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Cell, Clear, Row, Table, Tabs},
};

use goblin::pe::optional_header::IMAGE_NT_OPTIONAL_HDR32_MAGIC;
use goblin::pe::optional_header::IMAGE_NT_OPTIONAL_HDR64_MAGIC;
use goblin::pe::optional_header::IMAGE_ROM_OPTIONAL_HDR_MAGIC;
use goblin::pe::{data_directories::DataDirectoryType::*, header::machine_to_str};

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
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.pe_state.dos_header_table_state,
        );
    }
}

fn draw_coff_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let mut rows = Vec::new();

        let machine = pe.coff_header.machine;
        rows.push(Row::new(vec![
            Cell::new("Machine"),
            Cell::new(format!("{:X} ({})", machine, machine_to_str(machine))),
        ]));

        let number_of_sections = pe.coff_header.number_of_sections;
        rows.push(Row::new(vec![
            Cell::new("NumberOfSections"),
            Cell::new(format!("{:X} ({})", number_of_sections, number_of_sections)),
        ]));

        let timestamp = pe.coff_header.time_date_stamp;
        rows.push(Row::new(vec![
            Cell::new("TimeDateStamp"),
            Cell::new(format!("{:X} ({})", timestamp, timestamp)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("PointerToSymbolTable"),
            Cell::new(format!("{:X}", pe.coff_header.pointer_to_symbol_table)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("NumberOfSymbols"),
            Cell::new(format!("{:X}", pe.coff_header.number_of_symbol_table)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfOptionalHeader"),
            Cell::new(format!("{:X}", pe.coff_header.size_of_optional_header)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Characteristics"),
            Cell::new(format!("{:X}", pe.coff_header.characteristics)),
        ]));

        let widths = [Constraint::Min(16), Constraint::Fill(1)];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            table,
            area,
            &mut app.header_view.pe_state.coff_header_table_state,
        );
    }
}

fn draw_optional_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe
        && let Some(opt) = &pe.optional_header
    {
        let mut rows = Vec::new();

        let magic = opt.standard_fields.magic;
        let magic_str = match magic {
            IMAGE_NT_OPTIONAL_HDR32_MAGIC => "PE32",
            IMAGE_NT_OPTIONAL_HDR64_MAGIC => "PE32+",
            IMAGE_ROM_OPTIONAL_HDR_MAGIC => "ROM",
            _ => "Unknown",
        };
        rows.push(Row::new(vec![
            Cell::new("Magic"),
            Cell::new(format!("{:X} ({})", magic, magic_str)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MajorLinkerVersion"),
            Cell::new(format!("{:X}", opt.standard_fields.major_linker_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MinorLinkerVersion"),
            Cell::new(format!("{:X}", opt.standard_fields.minor_linker_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfCode"),
            Cell::new(format!("{:X}", opt.standard_fields.size_of_code)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfInitializedData"),
            Cell::new(format!(
                "{:X}",
                opt.standard_fields.size_of_initialized_data
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfUninitializedData"),
            Cell::new(format!(
                "{:X}",
                opt.standard_fields.size_of_uninitialized_data
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("AddressOfEntryPoint"),
            Cell::new(format!("{:X}", opt.standard_fields.address_of_entry_point)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("BaseOfCode"),
            Cell::new(format!("{:X}", opt.standard_fields.base_of_code)),
        ]));

        if magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC {
            rows.push(Row::new(vec![
                Cell::new("BaseOfCode"),
                Cell::new(format!("{:X}", opt.standard_fields.base_of_data)),
            ]));
        }

        // Windows-specific fields
        rows.push(Row::new(vec![
            Cell::new("ImageBase"),
            Cell::new(format!("{:X}", opt.windows_fields.image_base)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SectionAlignment"),
            Cell::new(format!("{:X}", opt.windows_fields.section_alignment)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("FileAlignment"),
            Cell::new(format!("{:X}", opt.windows_fields.file_alignment)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MajorOperatingSystemVersion"),
            Cell::new(format!(
                "{:X}",
                opt.windows_fields.major_operating_system_version
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MinorOperatingSystemVersion"),
            Cell::new(format!(
                "{:X}",
                opt.windows_fields.minor_operating_system_version
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MajorImageVersion"),
            Cell::new(format!("{:X}", opt.windows_fields.major_image_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MinorImageVersion"),
            Cell::new(format!("{:X}", opt.windows_fields.minor_image_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MajorSubsystemVersion"),
            Cell::new(format!("{:X}", opt.windows_fields.major_subsystem_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("MinorSubsystemVersion"),
            Cell::new(format!("{:X}", opt.windows_fields.minor_subsystem_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Win32VersionValue"),
            Cell::new(format!("{:X}", opt.windows_fields.win32_version_value)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfImage"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_image)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfHeaders"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_headers)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("CheckSum"),
            Cell::new(format!("{:X}", opt.windows_fields.check_sum)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Subsystem"),
            Cell::new(format!("{:X}", opt.windows_fields.subsystem)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("DllCharacteristics"),
            Cell::new(format!("{:X}", opt.windows_fields.dll_characteristics)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfStackReserve"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_stack_reserve)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfStackCommit"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_stack_commit)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfHeapReserve"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_heap_reserve)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("SizeOfHeapCommit"),
            Cell::new(format!("{:X}", opt.windows_fields.size_of_heap_commit)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("LoaderFlags"),
            Cell::new(format!("{:X}", opt.windows_fields.loader_flags)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("NumberOfRvaAndSizes"),
            Cell::new(format!("{:X}", opt.windows_fields.number_of_rva_and_sizes)),
        ]));

        let widths = [Constraint::Min(16), Constraint::Fill(1)];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            table,
            area,
            &mut app.header_view.pe_state.optional_header_table_state,
        );
    }
}

fn draw_data_directories(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe
        && let Some(opt) = pe.optional_header
    {
        let mut rows = Vec::new();

        for (dt, dd) in opt.data_directories.dirs() {
            let mut cells = Vec::with_capacity(3);

            let kind = match dt {
                ExportTable => "Export Table",
                ImportTable => "Import Table",
                ResourceTable => "Resource Table",
                ExceptionTable => "Exception Table",
                CertificateTable => "Certificate Table",
                BaseRelocationTable => "Base Relocation Table",
                DebugTable => "Debug",
                Architecture => "Architecture",
                GlobalPtr => "Global Ptr",
                TlsTable => "TLS Table",
                LoadConfigTable => "Load Config Table",
                BoundImportTable => "Bound Import",
                ImportAddressTable => "IAT",
                DelayImportDescriptor => "Delay Import Descriptor",
                ClrRuntimeHeader => "CLR Runtime Header",
            };

            cells.push(Cell::new(kind));
            cells.push(Cell::new(format!("{:X}", dd.virtual_address)));
            cells.push(Cell::new(format!("{:X}", dd.size)));
            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 3); 3];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(vec!["Type", "VirtualAddress", "Size"]).style(Style::new().bold()))
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            table,
            area,
            &mut app.header_view.pe_state.data_dirs_table_state,
        );
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
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            section_table,
            area,
            &mut app.header_view.pe_state.sections_table_state,
        );
    }
}

fn draw_imports(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        if pe.imports.is_empty() {
            let message = Text::from("No imports found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

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
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            imports_table,
            area,
            &mut app.header_view.pe_state.imports_table_sate,
        );
    }
}

fn draw_exports(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        if pe.exports.is_empty() {
            let message = Text::from("No exports found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

        let mut rows = Vec::new();

        for exp in &pe.exports {
            let mut cells = Vec::new();
            cells.push(Cell::new(exp.name.as_str()));
            cells.push(Cell::new(format!("{:08X}", exp.offset)));
            cells.push(Cell::new(format!("{:08X}", exp.rva)));
            cells.push(Cell::new(format!("{:08X}", exp.size)));
            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 4); 4];

        let imports_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(vec!["Name", "Offset", "RVA", "Size"]).style(Style::new().bold()))
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            imports_table,
            area,
            &mut app.header_view.pe_state.exports_table_sate,
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
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            overlay_table,
            area,
            &mut app.header_view.pe_state.overlay_table_sate,
        );
    }
}

pub fn pe_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(vec![
        "DOS",
        "COFF",
        "Optional",
        "Data Dirs",
        "Sections",
        "Imports",
        "Exports",
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
        1 => draw_coff_header(app, frame, main),
        2 => draw_optional_header(app, frame, main),
        3 => draw_data_directories(app, frame, main),
        4 => draw_sections(app, frame, main),
        5 => draw_imports(app, frame, main),
        6 => draw_exports(app, frame, main),
        7 => draw_overlay(app, frame, main),
        _ => {}
    }
}
