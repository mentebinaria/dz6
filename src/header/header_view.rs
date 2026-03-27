use ratatui::widgets::{ListState, TableState};

#[derive(Debug, Clone)]
pub struct PECoffHeader {
    pub characteristics: u16,
    pub machine: u16,
    pub time_date_stamp: u32,
}

#[derive(Debug, Clone)]
pub struct PEStandardFields {
    pub address_of_entry_point: u32,
    pub magic: u16,
    pub minor_linker_version: u8,
    pub major_linker_version: u8,
}

#[derive(Debug, Clone)]
pub struct PESection {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub pointer_to_raw_data: u32,
    pub size_of_raw_data: u32,
    pub characteristics: u32,
}

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
    pub summary: String,
    pub coff_header: PECoffHeader,
    pub optional_header: Option<PEStandardFields>,
    pub sections: Vec<PESection>,
    pub number_of_sections: usize,
    pub imports: Vec<PEImport>,
    pub number_of_imports: usize,
}

#[derive(Debug, Clone)]
pub struct Elf {
    pub header: goblin::elf::Header,
    pub phdrs: Vec<goblin::elf::ProgramHeader>,
    pub header_table_state: TableState,
    // pub program_header: Option<PEStandardFields>,
    // pub section_header: Vec<PESection>,
    // pub symbols: usize,
}

#[derive(Default, Debug)]
pub struct HeaderView {
    pub header_list_state: ListState,
    pub section_table_state: TableState,
    pub imports_table_state: TableState,
    pub entrypoint: u32,
    pub pe: Option<Pe>,
    pub elf: Option<Elf>,
    pub elf_header_table_state: TableState,
    pub tab_index: usize,
}
