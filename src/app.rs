use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::Path,
};

use arboard::Clipboard;
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
}

#[derive(Debug)]
pub struct TextView {
    pub area_height: u16,
    pub lines_to_show: usize,
    pub scroll_offset: (u16, u16), // order is (y, x)
    pub table: &'static encoding_rs::Encoding,
}

pub struct App {
    pub buffer: [u8; APP_CACHE_SIZE],
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
            buffer: [0u8; APP_CACHE_SIZE],
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
        self.file_info.r#type = match self.buffer[0] {
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

        // try to open the file with write permissions. Fallback to read-only otherise.
        if !read_only && let Ok(file) = OpenOptions::new().read(true).write(true).open(path) {
            self.file_info.file = Some(file);
        } else {
            let file = OpenOptions::new().read(true).open(path)?;
            self.file_info.file = Some(file);
            self.file_info.is_read_only = true;
        }

        if let Some(f) = self.file_info.file.as_mut() {
            self.file_info.size = meta.len() as usize;
            self.reader.cache_blocks = self.file_info.size.div_ceil(APP_CACHE_SIZE);
            self.reader.pages = self.file_info.size.div_ceil(APP_PAGE_SIZE);
            self.reader.page_last = self.reader.pages.saturating_sub(1);
            let _bytes_read = f.read(&mut self.buffer)?;
            self.id_file();
            self.log(format!(
                "filesize: {} (0x{:x})",
                self.file_info.size, self.file_info.size
            ));
        }
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

    /// read one cache page from the file to the cache
    pub fn read_chunk_from_file(&mut self, nblock: usize) -> io::Result<()> {
        if self.file_info.file.is_none() {
            return Err(std::io::Error::other("file not open"));
        }

        if let Some(f) = &mut self.file_info.file {
            f.seek(SeekFrom::Start((nblock * APP_CACHE_SIZE) as u64))?;
            let _ = f.read(&mut self.buffer)?;
            self.reader.cache_block_number = nblock;
            self.log(format!("read_chunk_from_file({nblock})"));
        }
        Ok(())
    }

    /// write what's cached to the buffer, but not to the file yet
    pub fn write_to_buffer(&mut self, changed: HashMap<usize, String>) {
        let ofs = self.hex_view.offset;
        let mut total_written = 0usize;

        for (k, v) in &changed {
            self.read_chunk_for_offset(*k);
            if let Ok(b) = u8::from_str_radix(v, 16) {
                let buf_ofs = k % APP_CACHE_SIZE;
                self.buffer[buf_ofs] = b;
                total_written += 1;
            }
        }

        App::log(
            self,
            format!("{} bytes written to buffer: {:?}", total_written, changed),
        );

        // restore state
        self.goto(ofs);
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
        let ofs = offset % APP_CACHE_SIZE;

        if offset >= self.file_info.size {
            return None;
        }

        Some(self.buffer[ofs])
    }

    pub fn read_i8(&mut self, offset: usize) -> Option<i8> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset >= self.file_info.size {
            return None;
        }

        Some(self.buffer[ofs] as i8)
    }

    pub fn read_u16(&mut self, offset: usize) -> Option<u16> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 1 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];

        Some(u16::from_le_bytes([b1, b2]))
    }

    pub fn read_i16(&mut self, offset: usize) -> Option<i16> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 1 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];

        Some(i16::from_le_bytes([b1, b2]))
    }

    pub fn read_u32(&mut self, offset: usize) -> Option<u32> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 3 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];
        let b3 = self.buffer[ofs + 2];
        let b4 = self.buffer[ofs + 3];

        Some(u32::from_le_bytes([b1, b2, b3, b4]))
    }

    pub fn read_i32(&mut self, offset: usize) -> Option<i32> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 3 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];
        let b3 = self.buffer[ofs + 2];
        let b4 = self.buffer[ofs + 3];

        Some(i32::from_le_bytes([b1, b2, b3, b4]))
    }

    pub fn read_u64(&mut self, offset: usize) -> Option<u64> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 7 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];
        let b3 = self.buffer[ofs + 2];
        let b4 = self.buffer[ofs + 3];
        let b5 = self.buffer[ofs + 4];
        let b6 = self.buffer[ofs + 5];
        let b7 = self.buffer[ofs + 6];
        let b8 = self.buffer[ofs + 7];

        Some(u64::from_le_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
    }

    pub fn read_i64(&mut self, offset: usize) -> Option<i64> {
        let ofs = offset % APP_CACHE_SIZE;

        if offset + 7 >= self.file_info.size {
            return None;
        }

        let b1 = self.buffer[ofs];
        let b2 = self.buffer[ofs + 1];
        let b3 = self.buffer[ofs + 2];
        let b4 = self.buffer[ofs + 3];
        let b5 = self.buffer[ofs + 4];
        let b6 = self.buffer[ofs + 5];
        let b7 = self.buffer[ofs + 6];
        let b8 = self.buffer[ofs + 7];

        Some(i64::from_le_bytes([b1, b2, b3, b4, b5, b6, b7, b8]))
    }

    pub fn read_chunk_for_offset(&mut self, offset: usize) {
        let nblock = offset / APP_CACHE_SIZE;

        if offset > self.reader.cache_end {
            self.read_chunk_from_file(nblock).unwrap();
            self.reader.cache_start += nblock * APP_CACHE_SIZE;
            self.reader.cache_end += nblock * APP_CACHE_SIZE;
        } else if offset < self.reader.cache_start {
            self.read_chunk_from_file(nblock).unwrap();
            self.reader.cache_start -= APP_CACHE_SIZE;
            self.reader.cache_end -= APP_CACHE_SIZE;
        } else if offset == 0 {
            self.reader.cache_start = 0;
            self.reader.cache_end = APP_CACHE_SIZE - 1;
            self.reader.page_start = 0;
            self.reader.page_end = APP_PAGE_SIZE - 1;
        }
    }
}
