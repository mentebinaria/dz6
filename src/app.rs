use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::Path,
};

use arboard::Clipboard;
use mmap_io::{MemoryMappedFile, MmapMode};
use ratatui::{Frame, layout::Rect, widgets::ListState};

use crate::{
    config::*,
    editor::*,
    global::calculator::Calculator,
    hex::{hex_view::HexView, strings::FoundString},
    input_history::InputHistory,
    reader::Reader,
    themes::*,
};

#[derive(Default)]
pub struct FileInfo {
    pub file: Option<File>,
    pub path: String,
    pub is_read_only: bool,
    pub name: String,
    pub r#type: &'static str,
    pub size: usize,
    pub mmap: Option<MemoryMappedFile>,
}

impl FileInfo {
    /// Get memory mapped file buffer.
    ///
    /// This slice appears to have all file, but beware it is just a mapping from it and every
    /// time you access a page that is not mapped it will load from disk to memory by the OS,
    /// which also takes care of unloading it if memory constrained.
    pub fn get_buffer(&mut self) -> &[u8] {
        if let Some(mmap) = self.mmap.as_mut() {
            match mmap.as_slice(0, self.size as u64) {
                Ok(slice) => return slice,
                Err(_) => (), // TODO: panic ? (file was deleted or changed)
            }
        }

        return &[];
    }
}

#[derive(Debug)]
pub struct TextView {
    pub area_height: u16,
    pub lines_to_show: usize,
    pub scroll_offset: (u16, u16), // order is (y, x)
    pub table: &'static encoding_rs::Encoding,
}

pub struct App {
    pub calculator: Calculator,
    pub clipboard: Result<Clipboard, arboard::Error>,
    pub command_area: Rect,
    pub command_input: InputHistory,
    pub config: Config,
    pub dialog_2nd_renderer: Option<fn(&mut App, &mut Frame)>,
    pub dialog_renderer: Option<fn(&mut App, &mut Frame)>,
    pub editor_view: AppView,
    pub file_info: FileInfo,
    pub hex_view: HexView,
    pub list_state: ListState,
    pub log_scroll_offset: (u16, u16),
    pub logs: Vec<String>,
    pub reader: Reader,
    pub running: bool,
    pub state: UIState,
    pub string_regex: String,
    pub strings: Vec<FoundString>,
    pub text_view: TextView,
}

impl App {
    pub fn new() -> Self {
        App {
            calculator: Calculator::default(),
            clipboard: Clipboard::new(),
            command_area: Rect::default(),
            command_input: InputHistory::default(),
            config: Config {
                database: true,
                dim_control_chars: false,
                dim_zeroes: true,
                hex_mode_bytes_per_line: 16,
                hex_mode_non_graphic_char: '.',
                maximum_strings_to_show: 3000,
                minimum_string_length: 4,
                theme: DARK,
                // hex_mode_dword_separator: '-',
                // text_mode_tab_spaces: 4,
            },
            dialog_renderer: None,
            dialog_2nd_renderer: None,
            editor_view: AppView::Hex,
            file_info: FileInfo::default(),
            hex_view: HexView {
                editing_hex: true,
                highlights: HashSet::with_capacity(8),
                ..Default::default()
            },
            list_state: ListState::default(),
            log_scroll_offset: (0, 0),
            logs: Vec::with_capacity(100),
            reader: Reader::new(),
            running: true,
            state: UIState::Normal,
            string_regex: String::new(),
            strings: Vec::new(),
            text_view: TextView {
                area_height: 0,
                lines_to_show: 0,
                scroll_offset: (0, 0),
                table: encoding_rs::UTF_8,
            },
        }
    }

    /// this function tries to identify a file type; this is a boilerplate implementation.
    fn id_file(&mut self) {
        let buffer = self.file_info.get_buffer();
        self.file_info.r#type = match buffer[0] {
            0x7f => "ELF",
            0xca | 0xcf => "Mach-O",
            0x4d => "PE",
            _ => "",
        }
    }

    /// load a file
    pub fn load_file(
        &mut self,
        filepath: &str,
        initial_offset: usize,
        read_only: bool,
    ) -> io::Result<()> {
        let path = Path::new(&filepath);

        if let Some(f) = path.file_name()
            && let Some(fname) = f.to_str()
        {
            self.file_info.name = String::from(fname);
            self.file_info.path = String::from(filepath);
        }

        let meta = path.metadata()?;

        // We try to open file readwrite to use this later for saving
        if !read_only && let Ok(file) = OpenOptions::new().read(true).write(true).open(path) {
            self.file_info.file = Some(file);
        } else {
            self.file_info.is_read_only = true;
        }

        // We map it on memory readonly as changed to mapped memory also changes it on disk
        if let Ok(mmap) = MemoryMappedFile::builder(path)
            .mode(MmapMode::ReadOnly)
            .open()
        {
            self.file_info.mmap = Some(mmap);
        } else {
            return Err(std::io::Error::other("could not open file"));
        }

        self.file_info.size = meta.len() as usize;

        self.id_file();
        self.log(format!(
            "filesize: {} (0x{:x})",
            self.file_info.size, self.file_info.size
        ));

        if initial_offset != 0 {
            self.goto(0);
        }
        self.goto(initial_offset);

        // try to load a database for this file, but continue otherwise
        if self.config.database {
            let _ = self.load_database();
        }
        Ok(())
    }

    pub fn reload_file(&mut self) {
        let fp = self.file_info.path.clone();
        self.load_file(&fp, self.hex_view.offset, self.file_info.is_read_only)
            .expect("could not reload the file");
    }

    /// write what's cached to the actual file
    pub fn write_to_file(&mut self) -> io::Result<()> {
        if self.file_info.file.is_none() {
            return Err(io::Error::other("file not open"));
        }

        let mut total_written = 0;

        if let Some(f) = &mut self.file_info.file {
            // changed bytes are not necessairily contiguous, so we
            // loop through them when writing to file
            for (k, v) in &self.hex_view.changed_bytes {
                f.seek(SeekFrom::Start(*k as u64))?;
                if let Ok(b) = u8::from_str_radix(v, 16) {
                    let buf = vec![b];
                    let written = f.write(&buf)?;
                    if written != 1 {
                        return Err(io::Error::other("could not write to file"));
                    }
                    total_written += written;
                }
            }
        }

        App::log(self, format!("{} bytes written to file", total_written));
        self.hex_view.changed_bytes.clear();
        Ok(())
    }

    pub fn read_u8(&mut self, offset: usize) -> Option<u8> {
        if offset >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        Some(buffer[offset])
    }

    pub fn read_i8(&mut self, offset: usize) -> Option<i8> {
        if offset >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        Some(buffer[offset] as i8)
    }

    pub fn read_u16(&mut self, offset: usize) -> Option<u16> {
        if offset + 1 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];

        Some(u16::from_le_bytes([b1, b2]))
    }

    pub fn read_i16(&mut self, offset: usize) -> Option<i16> {
        if offset + 1 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];

        Some(i16::from_le_bytes([b1, b2]))
    }

    pub fn read_u32(&mut self, offset: usize) -> Option<u32> {
        if offset + 3 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];
        let b3 = buffer[offset + 2];
        let b4 = buffer[offset + 3];

        Some(u32::from_le_bytes([b1, b2, b3, b4]))
    }

    pub fn read_i32(&mut self, offset: usize) -> Option<i32> {
        if offset + 3 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];
        let b3 = buffer[offset + 2];
        let b4 = buffer[offset + 3];

        Some(i32::from_le_bytes([b1, b2, b3, b4]))
    }

    pub fn read_u64(&mut self, offset: usize) -> Option<u64> {
        if offset + 7 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];
        let b3 = buffer[offset + 2];
        let b4 = buffer[offset + 3];
        let b5 = buffer[offset + 4];
        let b6 = buffer[offset + 5];
        let b7 = buffer[offset + 6];
        let b8 = buffer[offset + 7];

        Some(u64::from_le_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
    }

    pub fn read_i64(&mut self, offset: usize) -> Option<i64> {
        if offset + 7 >= self.file_info.size {
            return None;
        }

        let buffer = self.file_info.get_buffer();
        let b1 = buffer[offset];
        let b2 = buffer[offset + 1];
        let b3 = buffer[offset + 2];
        let b4 = buffer[offset + 3];
        let b5 = buffer[offset + 4];
        let b6 = buffer[offset + 5];
        let b7 = buffer[offset + 6];
        let b8 = buffer[offset + 7];

        Some(i64::from_le_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
    }
}
