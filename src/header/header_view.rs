use ratatui::widgets::TableState;

#[derive(Debug, Clone)]
pub struct PEImport {
    pub dll: String,
    pub name: String,
    pub offset: usize,
    pub ordinal: u16,
    pub rva: usize,
    pub _size: usize,
}

#[derive(Debug, Clone)]
pub struct Pe {
    pub dos_header: goblin::pe::header::DosHeader,
    pub coff_header: goblin::pe::header::CoffHeader,
    pub optional_header: Option<goblin::pe::optional_header::OptionalHeader>,
    pub sections: Vec<goblin::pe::section_table::SectionTable>,
    pub imports: Vec<PEImport>,
}

#[derive(Debug, Clone)]
pub struct Elf {
    pub header: goblin::elf::Header,
    pub phdrs: goblin::elf::ProgramHeaders,
    pub sections: goblin::elf::SectionHeaders,
    pub symtab: Vec<goblin::elf::Sym>,
}

#[derive(Debug, Default)]
pub struct PeState {
    pub dos_header_table_state: TableState,
    pub pe_header_table_state: TableState,
    pub sections_table_state: TableState,
    pub imports_table_sate: TableState,
}

#[derive(Debug, Default)]
pub struct ElfState {
    pub elf_header_table_state: TableState,
    pub program_header_table_state: TableState,
    pub sections_table_state: TableState,
    pub symbols_table_state: TableState,
}

#[derive(Default, Debug)]
pub struct HeaderView {
    pub pe: Option<Pe>,
    pub elf: Option<Elf>,
    pub elf_state: ElfState,
    pub pe_state: PeState,
    pub tab_index: usize,
}
