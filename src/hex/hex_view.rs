use std::collections::{HashMap, HashSet};

use ratatui::widgets::{ListState, TableState};
use serde::{Deserialize, Serialize};
use tui_input::Input;

use crate::hex::comment::Comment;

// used in hex view struct to track the cursor position
#[derive(Default, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HexView {
    #[serde(skip)]
    pub ascii_state: TableState,
    pub bookmarks: Vec<usize>,
    #[serde(skip)]
    pub changed_bytes: HashMap<usize, String>,
    #[serde(skip)]
    pub changed_history: Vec<usize>,
    #[serde(skip)]
    pub comment_input: Input, // the input comment widget (tui-input)

    // `comment_name_list` is used to show comments in Names list
    // and also on the conversion from selected item on the list
    // to file offset passed to goto()
    pub comment_name_list: Vec<Comment>,

    // `comments` store the comments internally as it is much easier
    // to handle that with a hash map
    pub comments: HashMap<usize, String>,

    #[serde(skip)]
    pub cursor: Point,
    #[serde(skip)]
    pub editing_hex: bool,
    #[serde(skip)]
    pub highlights: HashSet<u8>, // byte highlight
    #[serde(skip)]
    pub last_visited_offset: usize,
    #[serde(skip)]
    pub names_list_state: ListState,
    #[serde(skip)]
    pub names_regex_input: Input,
    #[serde(skip)]
    pub names_regex: String,
    #[serde(skip)]
    pub offset_state: TableState,
    #[serde(skip)]
    pub offset: usize,
    #[serde(skip)]
    pub search: crate::hex::search::Search,
    #[serde(skip)]
    pub selection: crate::hex::selection::Selection,
    #[serde(skip)]
    pub strings_regex_input: Input,
    #[serde(skip)]
    pub table_state: TableState,
}
