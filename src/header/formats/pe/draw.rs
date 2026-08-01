use chrono::{DateTime, Utc};

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

use crate::{app::App, util::number_to_str_radix};

fn draw_dos_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(pe) = &app.header_view.pe {
        let rows = [
            Row::new([
                Cell::new("Signature"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.signature,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("BytesOnLastPage"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.bytes_on_last_page,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("PagesInFile"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.pages_in_file,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Relocations"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.relocations,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("SizeOfHeaderInParagraphs"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.size_of_header_in_paragraphs,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("MinimumExtraParagraphsNeeded"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.minimum_extra_paragraphs_needed,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("MaximumExtraParagraphsNeeded"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.maximum_extra_paragraphs_needed,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("InitialRelativeSS"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.initial_relative_ss,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("InitialSP"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.initial_sp,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Checksum"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.checksum,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("InitialIP"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.initial_ip,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("InitialRelativeCS"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.initial_relative_cs,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("FileAddressOfRelocationTable"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.file_address_of_relocation_table,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("OverlayNumber"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.overlay_number,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Reserved"),
                Cell::new(format!("{:?}", pe.dos_header.reserved)),
            ]),
            Row::new([
                Cell::new("OemId"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.oem_id,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("OemInfo"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.oem_info,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Reserved2"),
                Cell::new(format!("{:?}", pe.dos_header.reserved2)),
            ]),
            Row::new([
                Cell::new("PEHeaderOffset"),
                Cell::new(number_to_str_radix(
                    pe.dos_header.pe_pointer,
                    app.config.header_base,
                )),
            ]),
        ];

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
        let machine = pe.coff_header.machine;
        let number_of_sections = pe.coff_header.number_of_sections;
        let time_date_stamp = pe.coff_header.time_date_stamp;
        let dt: DateTime<Utc> =
            DateTime::from_timestamp(time_date_stamp.into(), 0).expect("invalid");

        let rows = [
            Row::new([
                Cell::new("Machine"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(machine, app.config.header_base),
                    machine_to_str(machine)
                )),
            ]),
            Row::new([
                Cell::new("NumberOfSections"),
                Cell::new(number_to_str_radix(
                    number_of_sections,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("TimeDateStamp"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(time_date_stamp, app.config.header_base),
                    dt
                )),
            ]),
            Row::new([
                Cell::new("PointerToSymbolTable"),
                Cell::new(number_to_str_radix(
                    pe.coff_header.pointer_to_symbol_table,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("NumberOfSymbols"),
                Cell::new(number_to_str_radix(
                    pe.coff_header.number_of_symbol_table,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("SizeOfOptionalHeader"),
                Cell::new(number_to_str_radix(
                    pe.coff_header.size_of_optional_header,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Characteristics"),
                Cell::new(number_to_str_radix(
                    pe.coff_header.characteristics,
                    app.config.header_base,
                )),
            ]),
        ];

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

        rows.push(Row::new([
            Cell::new("Magic"),
            Cell::new(format!(
                "{} ({})",
                number_to_str_radix(magic, app.config.header_base),
                magic_str
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MajorLinkerVersion"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.major_linker_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MinorLinkerVersion"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.minor_linker_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfCode"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.size_of_code,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfInitializedData"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.size_of_initialized_data,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfUninitializedData"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.size_of_uninitialized_data,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("AddressOfEntryPoint"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.address_of_entry_point,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("BaseOfCode"),
            Cell::new(number_to_str_radix(
                opt.standard_fields.base_of_code,
                app.config.header_base,
            )),
        ]));

        if magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC {
            rows.push(Row::new([
                Cell::new("BaseOfCode"),
                Cell::new(number_to_str_radix(
                    opt.standard_fields.base_of_data,
                    app.config.header_base,
                )),
            ]));
        }

        // Windows-specific fields
        rows.push(Row::new([
            Cell::new("ImageBase"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.image_base,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SectionAlignment"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.section_alignment,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("FileAlignment"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.file_alignment,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MajorOperatingSystemVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.major_operating_system_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MinorOperatingSystemVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.minor_operating_system_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MajorImageVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.major_image_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MinorImageVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.minor_image_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MajorSubsystemVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.major_subsystem_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("MinorSubsystemVersion"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.minor_subsystem_version,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("Win32VersionValue"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.win32_version_value,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfImage"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_image,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfHeaders"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_headers,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("CheckSum"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.check_sum,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("Subsystem"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.subsystem,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("DllCharacteristics"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.dll_characteristics,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfStackReserve"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_stack_reserve,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfStackCommit"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_stack_commit,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfHeapReserve"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_heap_reserve,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("SizeOfHeapCommit"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.size_of_heap_commit,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("LoaderFlags"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.loader_flags,
                app.config.header_base,
            )),
        ]));

        rows.push(Row::new([
            Cell::new("NumberOfRvaAndSizes"),
            Cell::new(number_to_str_radix(
                opt.windows_fields.number_of_rva_and_sizes,
                app.config.header_base,
            )),
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

            let cells = [
                Cell::new(kind),
                Cell::new(number_to_str_radix(
                    dd.virtual_address,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(dd.size, app.config.header_base)),
            ];

            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 3); 3];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(["Type", "VirtualAddress", "Size"]).style(Style::new().bold()))
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
            let cells = [
                Cell::new(sec.name().unwrap_or("")),
                Cell::new(number_to_str_radix(
                    sec.virtual_address,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(
                    sec.virtual_size,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(
                    sec.pointer_to_raw_data,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(
                    sec.size_of_raw_data,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(
                    sec.characteristics,
                    app.config.header_base,
                )),
            ];

            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 6); 6];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(
                Row::new([
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
            table,
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
            let cells = [
                Cell::new(imp.dll.as_str()),
                Cell::new(imp.name.as_str()),
                Cell::new(number_to_str_radix(imp.offset, app.config.header_base)),
                Cell::new(number_to_str_radix(imp.ordinal, app.config.header_base)),
                Cell::new(number_to_str_radix(imp.rva, app.config.header_base)),
            ];

            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 5); 5];

        let imports_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(
                Row::new(["DLL", "Name", "Offset", "Ordinal", "RVA"]).style(Style::new().bold()),
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
            let cells = [
                Cell::new(exp.name.as_str()),
                Cell::new(number_to_str_radix(exp.offset, app.config.header_base)),
                Cell::new(number_to_str_radix(exp.rva, app.config.header_base)),
                Cell::new(number_to_str_radix(exp.size, app.config.header_base)),
            ];

            rows.push(Row::new(cells));
        }

        let widths = [Constraint::Ratio(1, 4); 4];

        let imports_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(["Name", "Offset", "RVA", "Size"]).style(Style::new().bold()))
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

        let rows = [
            Row::new([
                Cell::new("OverlayStart"),
                Cell::new(number_to_str_radix(overlay_start, app.config.header_base)),
            ]),
            Row::new([
                Cell::new("OverlaySize"),
                Cell::new(number_to_str_radix(overlay_size, app.config.header_base)),
            ]),
        ];

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
    let tabs = Tabs::new([
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
