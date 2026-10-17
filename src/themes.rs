use ratatui::style::{Color, Style};

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

// "old" themes left for compatiblity
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
pub const DEFAULT_DARK: Theme = Theme {
    // LineNr
    offsets: Style::new().fg(Color::Indexed(11)).bg(Color::Reset),
    // Normal
    main: Style::new().fg(Color::Reset).bg(Color::Reset),
    // set to DarkGray because Vim's Comment is too bright
    dimmed: Style::new().fg(Color::DarkGray).bg(Color::Reset),
    // Normal
    dialog: Style::new().fg(Color::Indexed(11)).bg(Color::Reset),
    // Constant
    changed_bytes: Style::new().fg(Color::Indexed(13)).bg(Color::Reset),
    // CursorColumn
    highlight: Style::new().fg(Color::Reset).bg(Color::Reset).reversed(),
    // Visual
    selection: Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(248)),
    // Search
    byte_highlight: Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(11)),
    // StatusLine
    status_bar: Style::new()
        .fg(Color::Reset)
        .bg(Color::Reset)
        .reversed()
        .bold(),
    // ErrorMsg
    error: Style::new().fg(Color::Indexed(15)).bg(Color::Indexed(1)),
    // DiffChange
    editing: Style::new()
        .fg(Color::Reset)
        .bg(Color::Indexed(5))
        .rapid_blink(),
};

pub const DEFAULT_LIGHT: Theme = Theme {
    // LineNr = Brown
    offsets: Style::new().fg(Color::Indexed(130)).bg(Color::Reset),
    // Normal
    main: Style::new().fg(Color::Reset).bg(Color::Reset),
    // set to DarkGray because Vim's Comment is too bright
    dimmed: Style::new().fg(Color::DarkGray).bg(Color::Reset),
    // Normal
    dialog: Style::new().fg(Color::Indexed(130)).bg(Color::Reset),
    // Constant = Magenta
    changed_bytes: Style::new().fg(Color::Magenta).bg(Color::Reset),
    // CursorColumn
    highlight: Style::new().fg(Color::Reset).bg(Color::Reset).reversed(),
    // Visual
    selection: Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(248)),
    // Search
    byte_highlight: Style::new().fg(Color::Indexed(0)).bg(Color::Indexed(11)),
    // StatusLine
    status_bar: Style::new()
        .fg(Color::Reset)
        .bg(Color::Reset)
        .reversed()
        .bold(),
    // ErrorMsg
    error: Style::new().fg(Color::Indexed(15)).bg(Color::Indexed(1)),
    // DiffChange
    editing: Style::new()
        .fg(Color::Reset)
        .bg(Color::LightMagenta)
        .rapid_blink(),
};

pub const DARKBLUE: Theme = Theme {
    // LineNr = Brown
    offsets: Style::new().fg(Color::Indexed(118)).bg(Color::Indexed(17)),
    // Normal
    main: Style::new().fg(Color::Indexed(252)).bg(Color::Indexed(17)),
    // set to DarkGray because Vim's Comment is too bright
    dimmed: Style::new().fg(Color::Indexed(102)).bg(Color::Indexed(17)),
    // Normal
    dialog: Style::new().fg(Color::Indexed(118)).bg(Color::Indexed(17)),
    // Constant
    changed_bytes: Style::new().fg(Color::Indexed(217)).bg(Color::Indexed(17)),
    // CursorColumn
    highlight: Style::new()
        .fg(Color::from_u32(0xffffff))
        .bg(Color::from_u32(0x8489ff)),
    // Search
    byte_highlight: Style::new().fg(Color::Indexed(123)).bg(Color::Indexed(27)),
    // Normal
    status_bar: Style::new().fg(Color::Reset).bg(Color::Indexed(17)),
    // ErrorMsg
    error: Style::new().fg(Color::Indexed(15)).bg(Color::Indexed(1)),
    // DiffChange
    editing: Style::new()
        .fg(Color::Reset)
        .bg(Color::LightMagenta)
        .rapid_blink(),

    selection: Style::new()
        .fg(Color::from_u32(0xffffff))
        .bg(Color::from_u32(0x8489ff)),
};
