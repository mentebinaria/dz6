use goblin::elf::{
    header::*,
    program_header::pt_to_str,
    section_header::sht_to_str,
    sym::{bind_to_str, type_to_str, visibility_to_str},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Text,
    widgets::{Cell, Clear, Row, Table, Tabs},
};

use crate::{app::App, util::number_to_str_radix};

fn osabi_to_str(osabi: u8) -> &'static str {
    // https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
    match osabi {
        ELFOSABI_NONE => "NONE",
        ELFOSABI_HPUX => "Hewlett-Packard HP-UX",
        ELFOSABI_NETBSD => "NetBSD",
        ELFOSABI_LINUX => "Linux",
        ELFOSABI_SOLARIS => "Sun Solaris",
        ELFOSABI_AIX => "AIX",
        ELFOSABI_IRIX => "IRIX",
        ELFOSABI_FREEBSD => "FreeBSD",
        ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
        ELFOSABI_MODESTO => "Novell Modesto",
        ELFOSABI_OPENBSD => "OpenBSD",
        13 => "OpenVMS",
        14 => "Hewlett-Packard Non-Stop Kernel",
        _ => "UNKNOWN_OSABI",
    }
}

fn draw_elf_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        let e_ident = elf.header.e_ident;
        let rows = [
            Row::new([
                Cell::new("Class"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(e_ident[4], app.config.header_base),
                    class_to_str(e_ident[4])
                )),
            ]),
            Row::new([
                Cell::new("Data"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(e_ident[5], app.config.header_base),
                    match e_ident[5] {
                        1 => "LSB/little endian",
                        2 => "MSB/big endian",
                        _ => "Invalid data encoding",
                    }
                )),
            ]),
            Row::new([
                Cell::new("Version"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(e_ident[6], app.config.header_base),
                    if e_ident[6] == 1 {
                        "current"
                    } else {
                        "invalid"
                    }
                )),
            ]),
            Row::new([
                Cell::new("OS/ABI"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(e_ident[7], app.config.header_base),
                    osabi_to_str(e_ident[7])
                )),
            ]),
            Row::new([
                Cell::new("ABI Version"),
                Cell::new(number_to_str_radix(e_ident[8], app.config.header_base)),
            ]),
            Row::new([
                Cell::new("Type"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(elf.header.e_type, app.config.header_base),
                    et_to_str(elf.header.e_type)
                )),
            ]),
            Row::new([
                Cell::new("Machine"),
                Cell::new(format!(
                    "{} ({})",
                    number_to_str_radix(elf.header.e_machine, app.config.header_base),
                    machine_to_str(elf.header.e_machine)
                )),
            ]),
            Row::new([
                Cell::new("Version"),
                Cell::new(number_to_str_radix(
                    elf.header.e_version,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Entrypoint"),
                Cell::new(number_to_str_radix(
                    elf.header.e_entry,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Program header offset"),
                Cell::new(number_to_str_radix(
                    elf.header.e_phoff,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Section header offset"),
                Cell::new(number_to_str_radix(
                    elf.header.e_shoff,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Flags"),
                Cell::new(number_to_str_radix(
                    elf.header.e_flags,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("(This) header size"),
                Cell::new(number_to_str_radix(
                    elf.header.e_ehsize,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Program header size"),
                Cell::new(number_to_str_radix(
                    elf.header.e_phentsize,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Number of program headers"),
                Cell::new(number_to_str_radix(
                    elf.header.e_phnum,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Section headers size"),
                Cell::new(number_to_str_radix(
                    elf.header.e_shentsize,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Number of section headers"),
                Cell::new(number_to_str_radix(
                    elf.header.e_shnum,
                    app.config.header_base,
                )),
            ]),
            Row::new([
                Cell::new("Section header string table index"),
                Cell::new(number_to_str_radix(
                    elf.header.e_shstrndx,
                    app.config.header_base,
                )),
            ]),
        ];

        let widths = [Constraint::Min(8), Constraint::Fill(1), Constraint::Fill(1)];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.elf_state.elf_header_table_state,
        );
    }
}

fn draw_program_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        let mut rows = Vec::new();
        let phdrs = &elf.phdrs;

        for phdr in phdrs {
            rows.push(Row::new([
                Cell::new(pt_to_str(phdr.p_type).to_string()),
                Cell::new(number_to_str_radix(phdr.p_offset, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_filesz, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_vaddr, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_memsz, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_paddr, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_flags, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_align, app.config.header_base)),
            ]));
        }

        let widths = [Constraint::Ratio(1, 8); 8];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Type", "Offset", "FileSiz", "VirtAddr", "MemSiz", "PhysAddr", "Flags", "Align",
            ]))
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.elf_state.program_header_table_state,
        );
    }
}

fn draw_section_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        if elf.sections.is_empty() {
            let message = Text::from("No sections found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

        let mut rows = Vec::new();

        let strtab = elf.sections.get(elf.header.e_shstrndx as usize);
        let buf = app.file_info.get_buffer();

        for (i, section) in elf.sections.iter().enumerate() {
            let mut name_cell = Cell::default();
            if let Some(strtab) = strtab {
                let bytes: Vec<u8> = buf
                    .iter()
                    .skip(strtab.sh_offset as usize + section.sh_name)
                    .take_while(|b| **b != 0)
                    .copied()
                    .collect();

                let name = String::from_utf8(bytes).unwrap_or_default();
                name_cell = Cell::new(name);
            }

            rows.push(Row::new([
                Cell::new(number_to_str_radix(i, app.config.header_base)),
                name_cell,
                Cell::new(number_to_str_radix(section.sh_name, app.config.header_base)),
                Cell::new(sht_to_str(section.sh_type)),
                Cell::new(number_to_str_radix(
                    section.sh_flags,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(section.sh_addr, app.config.header_base)),
                Cell::new(number_to_str_radix(
                    section.sh_offset,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(section.sh_size, app.config.header_base)),
                Cell::new(number_to_str_radix(section.sh_link, app.config.header_base)),
                Cell::new(number_to_str_radix(section.sh_info, app.config.header_base)),
                Cell::new(number_to_str_radix(
                    section.sh_addralign,
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(
                    section.sh_entsize,
                    app.config.header_base,
                )),
            ]));
        }

        let widths = [Constraint::Ratio(1, 8); 8];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Idx", "Name", "NameIdx", "Type", "Flags", "Addr", "Offset", "Size", "Link",
                "Info", "Align", " EntSize",
            ]))
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.elf_state.sections_table_state,
        );
    }
}

fn draw_symbols(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        if elf.symtab.is_empty() {
            let message = Text::from("No symbols found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

        let mut rows = Vec::new();

        for symbol in &elf.symtab {
            let name = elf
                .strtab
                .get(&symbol.st_name)
                .map(String::as_str)
                .unwrap_or_default();

            rows.push(Row::new([
                Cell::new(name),
                Cell::new(bind_to_str(symbol.st_bind())),
                Cell::new(type_to_str(symbol.st_type())),
                Cell::new(visibility_to_str(symbol.st_visibility())),
                Cell::new(number_to_str_radix(symbol.st_shndx, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_value, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_size, app.config.header_base)),
            ]));
        }

        let widths = [
            Constraint::Fill(1),    // Name
            Constraint::Length(6),  // Bind
            Constraint::Length(6),  // Type
            Constraint::Length(10), // Visibility
            Constraint::Length(8),  // SecHdrIdx
            Constraint::Length(8),  // Value
            Constraint::Length(8),  // Size
        ];
        let symbol_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Name",
                "Bind",
                "Type",
                "Visibility",
                "SecHdrIdx",
                "Value",
                "Size",
            ]))
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            symbol_table,
            area,
            &mut app.header_view.elf_state.symbols_table_state,
        );
    }
}

pub fn elf_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(["ELF", "Segments", "Sections", "Symbols"])
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
        0 => draw_elf_header(app, frame, main),
        1 => draw_program_header(app, frame, main),
        2 => draw_section_header(app, frame, main),
        3 => draw_symbols(app, frame, main),
        _ => {}
    }
}
