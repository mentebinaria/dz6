use std::collections::{HashMap, HashSet};

use ratatui::widgets::{ListState, TableState};
use tui_input::Input;

use crate::hex::{blocks::ColoredBlock, comment::Comment};

// used in hex view struct to track the cursor position
#[derive(Default, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Default)]
pub struct HexView {
    pub ascii_state: TableState,
    // blocks are ByteBlock structs -- ranges with different colors
    pub blocks: Vec<ColoredBlock>,
    pub bookmarks: Vec<usize>,
    pub changed_bytes: HashMap<usize, String>,
    pub changed_history: Vec<usize>,
    pub comment_input: Input, // the input comment widget (tui-input)

    // `comment_name_list` is used to show comments in Names list
    // and also on the conversion from selected item on the list
    // to file offset passed to goto()
    pub comment_name_list: Vec<Comment>,

    // `comments` store the comments internally as it is much easier
    // to handle that with a hash map
    pub comments: HashMap<usize, String>,

    pub cursor: Point,
    pub editing_hex: bool,
    pub highlights: HashSet<u8>, // byte highlight
    pub last_visited_offset: usize,
    pub names_list_state: ListState,
    pub names_regex_input: Input,
    pub names_regex: String,
    pub offset_state: TableState,
    pub offset: usize,
    pub search: crate::hex::search::Search,
    pub selection: crate::hex::selection::Selection,
    pub strings_regex_input: Input,
    pub table_state: TableState,
}
