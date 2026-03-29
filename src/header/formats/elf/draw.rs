use goblin::elf::{
    header::*,
    program_header::pt_to_str,
    section_header::sht_to_str,
    sym::{bind_to_str, type_to_str, visibility_to_str},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Cell, Clear, Row, Table, Tabs},
};

use crate::app::App;

fn osabi_to_str(osabi: u8) -> &'static str {
    // https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
    match osabi {
        ELFOSABI_NONE => "NONE",
        ELFOSABI_HPUX => "HP-UX",
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
        14 => "HP Non-Stop Kernel",
        _ => "UNKNOWN_OSABI",
    }
}

fn draw_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        let mut rows = Vec::new();

        let e_ident = elf.header.e_ident;

        rows.push(Row::new(vec![
            Cell::new("Class"),
            Cell::new(format!("{:08X} ({})", e_ident[4], class_to_str(e_ident[4]))),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Data"),
            Cell::new(format!(
                "{:08X} ({})",
                e_ident[5],
                match e_ident[5] {
                    1 => "LSB/little endian",
                    2 => "MSB/big endian",
                    _ => "Invalid data encoding",
                }
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Version"),
            Cell::new(format!(
                "{:08X} ({})",
                e_ident[6],
                if e_ident[6] == 1 {
                    "current"
                } else {
                    "invalid"
                }
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("OS/ABI"),
            Cell::new(format!("{:08X} ({})", e_ident[7], osabi_to_str(e_ident[7]))),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Type"),
            Cell::new(format!(
                "{:08X} ({})",
                elf.header.e_type,
                et_to_str(elf.header.e_type)
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Machine"),
            Cell::new(format!(
                "{:08X} ({})",
                elf.header.e_machine,
                machine_to_str(elf.header.e_machine)
            )),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Version"),
            Cell::new(format!("{:08X}", elf.header.e_version)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Entrypoint"),
            Cell::new(format!("{:08X}", elf.header.e_entry)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Program header offset"),
            Cell::new(format!("{:08X}", elf.header.e_phoff)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Section header offset"),
            Cell::new(format!("{:08X}", elf.header.e_shoff)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Flags"),
            Cell::new(format!("{:08X}", elf.header.e_flags)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("(This) header size"),
            Cell::new(format!("{:08X}", elf.header.e_ehsize)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Program header size"),
            Cell::new(format!("{:08X}", elf.header.e_phentsize)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Number of program headers"),
            Cell::new(format!("{:08X}", elf.header.e_phnum)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Section headers size"),
            Cell::new(format!("{:08X}", elf.header.e_shentsize)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Number of section headers"),
            Cell::new(format!("{:08X}", elf.header.e_shnum)),
        ]));

        rows.push(Row::new(vec![
            Cell::new("Section header string table index"),
            Cell::new(format!("{:08X}", elf.header.e_shstrndx)),
        ]));

        let widths = [Constraint::Min(8), Constraint::Fill(1), Constraint::Fill(1)];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            header_table,
            area,
            &mut app.header_view.elf_header_table_state,
        );
    }
}

fn draw_program_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        let mut rows = Vec::new();

        let phdrs = &elf.phdrs;

        for phdr in phdrs {
            rows.push(Row::new(vec![
                Cell::new(format!("{}", pt_to_str(phdr.p_type))),
                Cell::new(format!("{:08X}", phdr.p_offset)),
                Cell::new(format!("{:08X}", phdr.p_filesz)),
                Cell::new(format!("{:08X}", phdr.p_vaddr)),
                Cell::new(format!("{:08X}", phdr.p_memsz)),
                Cell::new(format!("{:08X}", phdr.p_paddr)),
                Cell::new(format!("{:X}", phdr.p_flags)),
                Cell::new(format!("{:X}", phdr.p_align)),
            ]));
        }

        let widths = [Constraint::Ratio(1, 8); 8];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(vec![
                "Type", "Offset", "FileSiz", "VirtAddr", "MemSiz", "PhysAddr", "Flags", "Align",
            ]))
            .style(Style::new().bold())
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_widget(header_table, area);
    }
}

fn draw_section_header(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
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

            rows.push(Row::new(vec![
                Cell::new(format!("{:X}", i)),
                name_cell,
                Cell::new(format!("{:08X}", section.sh_name)),
                Cell::new(format!("{}", sht_to_str(section.sh_type))),
                Cell::new(format!("{:08X}", section.sh_flags)),
                Cell::new(format!("{:08X}", section.sh_addr)),
                Cell::new(format!("{:08X}", section.sh_offset)),
                Cell::new(format!("{:08X}", section.sh_size)),
                Cell::new(format!("{:08X}", section.sh_link)),
                Cell::new(format!("{:08X}", section.sh_info)),
                Cell::new(format!("{:08X}", section.sh_addralign)),
                Cell::new(format!("{:08X}", section.sh_entsize)),
            ]));
        }

        let widths = [Constraint::Ratio(1, 8); 8];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(vec![
                "Idx", "Name", "NameIdx", "Type", "Flags", "Addr", "Offset", "Size", "Link",
                "Info", "Align", "EntSize",
            ]))
            .style(Style::new().bold())
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_widget(header_table, area);
    }
}

fn draw_symbols(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        let mut rows = Vec::new();

        for symbol in &elf.symtab {
            rows.push(Row::new(vec![
                Cell::new(format!("{:X}", symbol.st_name)),
                Cell::new(format!("{}", bind_to_str(symbol.st_bind()))),
                Cell::new(format!("{}", type_to_str(symbol.st_type()))),
                Cell::new(format!("{}", visibility_to_str(symbol.st_visibility()))),
                Cell::new(format!("{:08X}", symbol.st_shndx)),
                Cell::new(format!("{:08X}", symbol.st_value)),
                Cell::new(format!("{:08X}", symbol.st_size)),
            ]));
        }

        let widths = [Constraint::Ratio(1, 8); 8];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(vec![
                "Name",
                "Bind",
                "Type",
                "Visibility",
                "SecHdrIdx",
                "Value",
                "Size",
            ]))
            .style(Style::new().bold())
            .cell_highlight_style(app.config.theme.highlight);

        frame.render_widget(header_table, area);
    }
}

pub fn elf_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(vec!["Header", "Program", "Sections", "Symbols"])
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
        0 => draw_header(app, frame, main),
        1 => draw_program_header(app, frame, main),
        2 => draw_section_header(app, frame, main),
        3 => draw_symbols(app, frame, main),
        _ => {}
    }
}
