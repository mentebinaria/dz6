use goblin::{
    elf::{
        header::*,
        program_header::pt_to_str,
        section_header::sht_to_str,
        sym::{bind_to_str, type_to_str, visibility_to_str},
    },
    elf64::{
        program_header::{PF_R, PF_W, PF_X},
        section_header::{
            SHF_ALLOC, SHF_COMPRESSED, SHF_EXCLUDE, SHF_EXECINSTR, SHF_GROUP, SHF_INFO_LINK,
            SHF_LINK_ORDER, SHF_MASKOS, SHF_MASKPROC, SHF_MERGE, SHF_OS_NONCONFORMING, SHF_STRINGS,
            SHF_TLS, SHF_WRITE,
        },
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Cell, Clear, Row, Table, Tabs},
};

use crate::{app::App, util::number_to_str_radix};

fn osabi_to_str(osabi: u8) -> &'static str {
    // https://refspecs.linuxfoundation.org/elf/gabi4+/ch4.eheader.html
    match osabi {
        ELFOSABI_NONE => "UNIX System V",
        ELFOSABI_HPUX => "Hewlett-Packard HP-UX",
        ELFOSABI_NETBSD => "NetBSD",
        ELFOSABI_LINUX => "Linux",
        ELFOSABI_SOLARIS => "Sun Solaris",
        ELFOSABI_AIX => "IBM AIX",
        ELFOSABI_IRIX => "SGI Irix",
        ELFOSABI_FREEBSD => "FreeBSD",
        ELFOSABI_TRU64 => "Compaq TRU64 UNIX",
        ELFOSABI_MODESTO => "Novell Modesto",
        ELFOSABI_OPENBSD => "OpenBSD",
        13 => "OpenVMS",
        14 => "Hewlett-Packard Non-Stop Kernel",
        ELFOSABI_ARM => "ARM",
        ELFOSABI_ARM_AEABI => "ARM EABI",
        ELFOSABI_STANDALONE => "Standalone (embedded) application",
        _ => "Unknown",
    }
}

fn pflags_to_string(flags: u32) -> String {
    let mut ret = String::with_capacity(3);

    ret.push(if flags & PF_R != 0 { 'R' } else { ' ' });
    ret.push(if flags & PF_W != 0 { 'W' } else { ' ' });
    // readelf uses `E`, but I think `X` represents the `executable` bit better
    ret.push(if flags & PF_X != 0 { 'X' } else { ' ' });

    ret
}

fn shflags_to_string(flags: u32) -> String {
    let mut ret = String::with_capacity(2); // two flags are quite common

    // not complete yet -- see /usr/include/llvm-21/llvm/BinaryFormat/ELF.h
    const SHF_GNU_RETAIN: u32 = 0x200000;
    const SHF_GNU_MBIND: u32 = 0x1000000;
    const SHF_X86_64_LARGE: u32 = 0x10000000;

    if flags & SHF_WRITE != 0 {
        ret.push('W');
    }
    if flags & SHF_ALLOC != 0 {
        ret.push('A');
    }
    if flags & SHF_EXECINSTR != 0 {
        ret.push('X');
    }
    if flags & SHF_MERGE != 0 {
        ret.push('M');
    }
    if flags & SHF_STRINGS != 0 {
        ret.push('S');
    }
    if flags & SHF_INFO_LINK != 0 {
        ret.push('I');
    }
    if flags & SHF_LINK_ORDER != 0 {
        ret.push('L');
    }
    if flags & SHF_OS_NONCONFORMING != 0 {
        ret.push('O');
    }
    if flags & SHF_GROUP != 0 {
        ret.push('G');
    }
    if flags & SHF_TLS != 0 {
        ret.push('T');
    }
    if flags & SHF_COMPRESSED != 0 {
        ret.push('C');
    }
    if flags & SHF_GNU_RETAIN != 0 {
        ret.push('G');
    }
    if flags & SHF_MASKOS != 0 {
        ret.push('o');
    }
    if flags & SHF_EXCLUDE != 0 {
        ret.push('E');
    }
    if flags & SHF_GNU_MBIND != 0 {
        ret.push('D');
    }
    if flags & SHF_X86_64_LARGE != 0 {
        ret.push('l');
    }
    if flags & SHF_MASKPROC != 0 {
        ret.push('p');
    }

    ret
}

/*
The idea would be to mimic `file`:
ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=00221374bf695bce1421f1fdc4b5d8dd24cd4fca, for GNU/Linux 3.2.0, with debug_info, not stripped
 */
// fn draw_summary(app: &mut App, frame: &mut Frame, area: Rect) {
//     if let Some(elf) = &app.header_view.elf {
//         let mut summary = String::from("ELF ");

//         let class = match elf.header.e_ident[4] {
//             1 => "32-bit ",
//             2 => "64-bit ",
//             _ => "<invalid class> ",
//         };

//         let data = match elf.header.e_ident[5] {
//             1 => "LSB ",
//             2 => "MSB ",
//             _ => "<invalid data>, ",
//         };

//         let kind = match elf.header.e_type {
//             1 => "relocatable, ",
//             2 => "executable, ",
//             3 => "shared object, ",
//             4 => "core, ",
//             _ => "<invalid type>, ",
//         };

//         summary.push_str(class);
//         summary.push_str(data);
//         summary.push_str(kind);

//         if let Some(interp) = &elf.interpreter {
//             summary.push_str(format!("Requesting program interpreter: {}", interp).as_str());
//         }

//         frame.render_widget(Paragraph::new(summary), area);
//     }
// }

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
    if let Some(elf) = &mut app.header_view.elf {
        let mut rows = Vec::new();
        let phdrs = &elf.phdrs;

        for (i, phdr) in phdrs.iter().enumerate() {
            rows.push(Row::new([
                Cell::new(number_to_str_radix(i, app.config.header_base)),
                Cell::new(pt_to_str(phdr.p_type).replace("PT_", "")),
                Cell::new(number_to_str_radix(phdr.p_offset, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_filesz, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_vaddr, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_memsz, app.config.header_base)),
                Cell::new(number_to_str_radix(phdr.p_paddr, app.config.header_base)),
                Cell::new(pflags_to_string(phdr.p_flags)),
                Cell::new(number_to_str_radix(phdr.p_align, app.config.header_base)),
            ]));
        }

        let widths = [
            Constraint::Length(4),  // Num
            Constraint::Length(16), // Type
            Constraint::Fill(1),    // Offset
            Constraint::Fill(1),    // FileSiz
            Constraint::Fill(1),    // VirtAddr
            Constraint::Fill(1),    // MemSiz
            Constraint::Fill(1),    // PhysAddr
            Constraint::Fill(1),    // Flags
            Constraint::Fill(1),    // Align
        ];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Num", "Type", "Offset", "FileSiz", "VirtAddr", "MemSiz", "PhysAddr", "Flags",
                "Align",
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
                Cell::new(sht_to_str(section.sh_type).replace("SHT_", "")),
                Cell::new(shflags_to_string(
                    section.sh_flags.try_into().unwrap_or_default(),
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

        // let widths = [Constraint::Ratio(1, 8); 8];

        let widths = [
            Constraint::Length(4), // Num
            Constraint::Min(20),   // Name
            Constraint::Length(7), // NameIdx
            Constraint::Min(15),   // Type
            Constraint::Fill(1),   // Flags
            Constraint::Fill(1),   // Addr
            Constraint::Fill(1),   // Offset
            Constraint::Fill(1),   // Size
            Constraint::Fill(1),   // Size
            Constraint::Fill(1),   // Size
            Constraint::Fill(1),   // Size
        ];

        let header_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Num", "Name", "NameIdx", "Type", "Flags", "Addr", "Offset", "Size", "Link",
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
        if elf.dynsymtab.is_empty() && elf.symtab.is_empty() {
            let message = Text::from("No symbols found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

        let mut rows = Vec::new();

        for (i, symbol) in elf.dynsymtab.iter().enumerate() {
            let name = elf
                .dynstrtab
                .get(&symbol.st_name)
                .map(String::as_str)
                .unwrap_or_default();

            // highlight symbol functions (with offsets style)
            let name_style = if symbol.is_function() {
                app.config.theme.offsets
            } else {
                Style::default()
            };

            // show "UND" when the index is zero, just like readelf does
            let ndx_cell = match symbol.st_shndx {
                0 => Cell::new("UND"),
                0xfff1 => Cell::new("ABS"),
                _ => Cell::new(number_to_str_radix(symbol.st_shndx, app.config.header_base)),
            };

            rows.push(Row::new([
                Cell::new(number_to_str_radix(i, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_value, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_size, app.config.header_base)),
                Cell::new(type_to_str(symbol.st_type())),
                Cell::new(bind_to_str(symbol.st_bind())),
                Cell::new(visibility_to_str(symbol.st_visibility())),
                ndx_cell,
                Cell::new(name).style(name_style),
            ]));
        }

        // syms
        for (i, symbol) in elf.symtab.iter().enumerate() {
            let name = elf
                .strtab
                .get(&symbol.st_name)
                .map(String::as_str)
                .unwrap_or_default();

            // highlight symbol functions (with offsets style)
            let name_style = if symbol.is_function() {
                app.config.theme.offsets
            } else {
                Style::default()
            };

            // show "UND" when the index is zero, just like readelf does
            let ndx_cell = match symbol.st_shndx {
                0 => Cell::new("UND"),
                0xfff1 => Cell::new("ABS"),
                _ => Cell::new(number_to_str_radix(symbol.st_shndx, app.config.header_base)),
            };

            rows.push(Row::new([
                Cell::new(number_to_str_radix(i, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_value, app.config.header_base)),
                Cell::new(number_to_str_radix(symbol.st_size, app.config.header_base)),
                Cell::new(type_to_str(symbol.st_type())),
                Cell::new(bind_to_str(symbol.st_bind())),
                Cell::new(visibility_to_str(symbol.st_visibility())),
                ndx_cell,
                Cell::new(name).style(name_style),
            ]));
        }

        let widths = [
            Constraint::Length(4), // Num
            Constraint::Max(16),   // Value
            Constraint::Length(8), // Size
            Constraint::Length(7), // Type
            Constraint::Length(7), // Bind
            Constraint::Length(8), // Visibility
            Constraint::Length(5), // SecHdrIdx
            Constraint::Fill(1),   // Name
        ];

        let symbol_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new([
                "Num", "Value", "Size", "Type", "Bind", "Vis", "Ndx", "Name",
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

fn draw_relocations(app: &mut App, frame: &mut Frame, area: Rect) {
    if let Some(elf) = &app.header_view.elf {
        if elf.relocs.is_empty() {
            let message = Text::from("No relocations found").centered();

            frame.render_widget(message, area.centered_vertically(Constraint::Ratio(1, 4)));
            return;
        }

        let mut rows = Vec::new();

        for (i, reloc) in elf.relocs.iter().enumerate() {
            rows.push(Row::new([
                Cell::new(number_to_str_radix(i, app.config.header_base)),
                Cell::new(number_to_str_radix(reloc.r_offset, app.config.header_base)),
                Cell::new(number_to_str_radix(reloc.r_type, app.config.header_base)),
                Cell::new(number_to_str_radix(
                    reloc.r_addend.unwrap_or_default(),
                    app.config.header_base,
                )),
                Cell::new(number_to_str_radix(reloc.r_sym, app.config.header_base)),
            ]));
        }

        let widths = [Constraint::Ratio(1, 5); 5];

        let symbol_table = Table::new(rows, widths)
            .column_spacing(1)
            .style(app.config.theme.main)
            .header(Row::new(["Num", "Offset", "Type", "Addend", "Sym"]))
            .style(app.config.theme.main)
            .row_highlight_style(app.config.theme.highlight);

        frame.render_stateful_widget(
            symbol_table,
            area,
            &mut app.header_view.elf_state.relocations_table_state,
        );
    }
}

pub fn elf_draw(app: &mut App, frame: &mut Frame, area: Rect) {
    let tabs = Tabs::new(["ELF", "Program", "Sections", "Symbols", "Relocations"])
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
        4 => draw_relocations(app, frame, main),
        _ => {}
    }
}
