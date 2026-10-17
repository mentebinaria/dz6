use ratatui::style::{
    Color::{self, Indexed},
    Style,
};

pub struct Theme {
    pub main: Style,
    pub dimmed: Style,
    pub offsets: Style,
    pub changed_bytes: Style,
    pub highlight: Style,
    pub byte_highlight: Style,
    pub status_bar: Style,
    pub error: Style,
    pub editing: Style,
    pub dialog: Style,
    pub selection: Style,
}

// "old" themes left for compatibility
pub const DARK: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x569cd6))
        .bg(Color::from_u32(0x1e1e1e))
        .bold(),
    main: Style::new()
        .fg(Color::from_u32(0xd4d4d4))
        .bg(Color::from_u32(0x1e1e1e))
        .bold(),
    dimmed: Style::new()
        .fg(Color::from_u32(0x949494))
        .bg(Color::from_u32(0x1e1e1e))
        .bold(),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204))
        .bg(Color::from_u32(0x081e32))
        .bold(),
    changed_bytes: Style::new()
        .fg(Color::Rgb(255, 215, 0))
        .bg(Color::from_u32(0x1e1e1e)),
    highlight: Style::new()
        .fg(Color::Rgb(255, 255, 255))
        .bg(Color::Rgb(38, 79, 120)),
    selection: Style::new()
        .fg(Color::Rgb(255, 255, 255))
        .bg(Color::Rgb(38, 79, 120)),
    byte_highlight: Style::new().fg(Color::White).bg(Color::Red),
    status_bar: Style::new()
        .fg(Color::Rgb(204, 204, 204))
        .bg(Color::from_u32(0x3c3c3c)),
    error: Style::new()
        .fg(Color::Rgb(255, 85, 85))
        .bg(Color::from_u32(0x400000)),
    editing: Style::new()
        .fg(Color::from_u32(0x1e1e1e))
        .bg(Color::Rgb(255, 215, 0))
        .rapid_blink(),
};

pub const LIGHT: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x237893))
        .bg(Color::from_u32(0xffffff))
        .bold(),
    main: Style::new()
        .fg(Color::from_u32(0x000000))
        .bg(Color::from_u32(0xffffff))
        .bold(),
    dimmed: Style::new()
        .fg(Color::from_u32(0xa0a0a0))
        .bg(Color::from_u32(0xffffff))
        .bold(),
    dialog: Style::new()
        .fg(Color::from_u32(0x333333))
        .bg(Color::from_u32(0xe7f3ff))
        .bold(),
    changed_bytes: Style::new()
        .fg(Color::from_u32(0x795e00))
        .bg(Color::from_u32(0xffffff)),
    highlight: Style::new()
        .fg(Color::from_u32(0x000000))
        .bg(Color::from_u32(0xadd6ff)),
    selection: Style::new()
        .fg(Color::from_u32(0x000000))
        .bg(Color::from_u32(0xadd6ff)),
    byte_highlight: Style::new().fg(Color::Black).bg(Color::from_u32(0xffb3b3)),
    status_bar: Style::new()
        .fg(Color::from_u32(0x333333))
        .bg(Color::from_u32(0xf3f3f3)),
    error: Style::new()
        .fg(Color::from_u32(0xe51400))
        .bg(Color::from_u32(0xf2dede)),
    editing: Style::new()
        .fg(Color::from_u32(0xffffff))
        .bg(Color::from_u32(0xffcc00))
        .rapid_blink(),
};

// Vim themes
// to see these colors in Vim, use for example :hi LineNr
// they depend on the terminal config, which is good and bad :)
// Color::Reset means the terminal default for background or foreground
// the comment before each member field refers to Vim's variable name

// these colors differ
const DEFAULT_OFFSETS_DARK: Style = Style::new().fg(Color::Indexed(11)).bg(Color::Reset);
const DEFAULT_OFFSETS_LIGHT: Style = Style::new().fg(Color::Indexed(130)).bg(Color::Reset);

// these are common between dark and light
const DEFAULT_MAIN: Style = Style::new().fg(Color::Reset).bg(Color::Reset);
const DEFAULT_DIMMED: Style = Style::new().fg(Color::DarkGray).bg(Color::Reset);
const DEFAULT_CHANGED_BYTES: Style = Style::new().fg(Indexed(12)).bg(Color::Reset);
const DEFAULT_HIGHLIGHT: Style = DEFAULT_MAIN.reversed();
const DEFAULT_SELECTION: Style = Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(248));
const DEFAULT_BYTE_HIGHLIGHT: Style = Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(11));
const DEFAULT_STATUS_BAR: Style = DEFAULT_MAIN.reversed().bold();
const DEFAULT_ERROR: Style = Style::new().fg(Color::Indexed(15)).bg(Color::Indexed(1));
const DEFAULT_EDITING: Style = DEFAULT_CHANGED_BYTES.reversed().rapid_blink();

pub const DEFAULT_DARK: Theme = Theme {
    // LineNr
    offsets: DEFAULT_OFFSETS_DARK,
    // Normal
    main: DEFAULT_MAIN,
    // set to DarkGray because Vim's Comment is too bright
    dimmed: DEFAULT_DIMMED,
    // Normal
    dialog: DEFAULT_OFFSETS_DARK,
    // Changed
    changed_bytes: DEFAULT_CHANGED_BYTES,
    // CursorColumn
    highlight: DEFAULT_HIGHLIGHT,
    // Visual
    selection: DEFAULT_SELECTION,
    // Search
    byte_highlight: DEFAULT_BYTE_HIGHLIGHT,
    // StatusLine
    status_bar: DEFAULT_STATUS_BAR,
    // ErrorMsg
    error: DEFAULT_ERROR,
    // changed_bytes, but reversed
    editing: DEFAULT_EDITING,
};

pub const DEFAULT_LIGHT: Theme = Theme {
    // LineNr = Brown
    offsets: DEFAULT_OFFSETS_LIGHT,
    // Normal
    main: DEFAULT_MAIN,
    // set to DarkGray because Vim's Comment is too bright
    dimmed: DEFAULT_DIMMED,
    // Normal
    dialog: DEFAULT_OFFSETS_LIGHT,
    // Changed
    changed_bytes: DEFAULT_CHANGED_BYTES,
    // CursorColumn
    highlight: DEFAULT_HIGHLIGHT,
    // Visual
    selection: DEFAULT_SELECTION,
    // Search
    byte_highlight: DEFAULT_BYTE_HIGHLIGHT,
    // StatusLine
    status_bar: DEFAULT_STATUS_BAR,
    // ErrorMsg
    error: DEFAULT_ERROR,
    // changed_bytes, but reversed
    editing: DEFAULT_EDITING,
};

pub const DARKBLUE: Theme = Theme {
    // LineNr
    offsets: Style::new().fg(Color::Indexed(118)).bg(Color::Indexed(17)),
    // Normal
    main: Style::new().fg(Color::Indexed(252)).bg(Color::Indexed(17)),
    // set to DarkGray because Vim's Comment is too bright
    dimmed: Style::new().fg(Color::Indexed(102)).bg(Color::Indexed(17)),
    // Normal
    dialog: Style::new().fg(Color::Indexed(118)).bg(Color::Indexed(17)),
    // Changed
    changed_bytes: Style::new().fg(Color::Indexed(12)).bg(Color::Indexed(17)),
    // CursorColumn
    highlight: Style::new().fg(Color::Indexed(252)).bg(Color::Indexed(59)),
    // Visual
    selection: Style::new().fg(Color::Indexed(231)).bg(Color::Indexed(105)),
    // Search
    byte_highlight: Style::new()
        .fg(Color::Indexed(123))
        .bg(Color::Indexed(27))
        .reversed(),
    // StatusLine
    status_bar: Style::new()
        .fg(Color::Indexed(252))
        .bg(Color::Indexed(17))
        .reversed(),
    // ErrorMsg
    error: DEFAULT_ERROR,
    // changed_bytes, but reversed
    editing: Style::new()
        .fg(Color::Indexed(12))
        .bg(Color::Indexed(17))
        .reversed(),
};
