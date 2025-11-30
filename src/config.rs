use crate::themes::*;

// cache block size
pub const APP_CACHE_SIZE: usize = 4096;

// page size (amount of bytes)
pub const APP_PAGE_SIZE: usize = APP_CACHE_SIZE / 4;

pub struct Config {
    pub dim_control_chars: bool,
    pub dim_zeroes: bool,
    pub hex_mode_bytes_per_line: usize,
    pub hex_mode_non_graphic_char: char,
    pub maximum_strings_to_show: usize,
    pub minimum_string_length: usize,
    pub theme: Theme,
    // pub hex_mode_dword_separator: char,
    // pub text_mode_tab_spaces: usize,
}
