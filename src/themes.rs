use ratatui::style::{Color, Modifier, Style};

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
}

pub const DARK: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x569cd6))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    main: Style::new()
        .fg(Color::from_u32(0xd4d4d4))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    dimmed: Style::new()
        .fg(Color::from_u32(0x949494))
        .bg(Color::from_u32(0x1e1e1e))
        .add_modifier(Modifier::BOLD),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204))
        .bg(Color::from_u32(0x081e32))
        .add_modifier(Modifier::BOLD),
    changed_bytes: Style::new()
        .fg(Color::Rgb(255, 215, 0))
        .bg(Color::from_u32(0x1e1e1e)),
    highlight: Style::new()
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
        .add_modifier(Modifier::RAPID_BLINK),
};

pub const LIGHT: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x237893))
        .bg(Color::from_u32(0xffffff))
        .add_modifier(Modifier::BOLD),

    main: Style::new()
        .fg(Color::from_u32(0x000000))
        .bg(Color::from_u32(0xffffff))
        .add_modifier(Modifier::BOLD),

    dimmed: Style::new()
        .fg(Color::from_u32(0xa0a0a0))
        .bg(Color::from_u32(0xffffff))
        .add_modifier(Modifier::BOLD),

    dialog: Style::new()
        .fg(Color::from_u32(0x333333))
        .bg(Color::from_u32(0xe7f3ff))
        .add_modifier(Modifier::BOLD),

    changed_bytes: Style::new()
        .fg(Color::from_u32(0x795e00))
        .bg(Color::from_u32(0xffffff)),

    highlight: Style::new()
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
        .add_modifier(Modifier::RAPID_BLINK),
};

pub const DEFAULT_LIGHT: Theme = Theme {
    offsets: Style::new()
        .fg(Color::Indexed(130))
        .bg(Color::Black),
    main: Style::new()
        .fg(Color::White)
        .bg(Color::Black),
    dimmed: Style::new()
        .fg(Color::DarkGray)
        .bg(Color::Black),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204)),
        // .bg(Color::from_u32(0x081e32)),
    changed_bytes: Style::new()
        .fg(Color::Yellow)
        .bg(Color::Black),
    highlight: Style::new()
        .fg(Color::Black)
        .bg(Color::White),
    byte_highlight: Style::new()
        .fg(Color::White)
        .bg(Color::LightYellow),
    status_bar: Style::new()
        .fg(Color::White)
        .bg(Color::Black),
    error: Style::new()
        .fg(Color::White)
        .bg(Color::Red),
    editing: Style::new()
        .fg(Color::from_u32(0x1e1e1e))
        .bg(Color::Rgb(255, 215, 0))
        .add_modifier(Modifier::RAPID_BLINK),
};

pub const DEFAULT_DARK: Theme = Theme {
    offsets: Style::new()
        .fg(Color::LightYellow)
        .bg(Color::Black),
    main: Style::new()
        .fg(Color::White)
        .bg(Color::Black),
    dimmed: Style::new()
        .fg(Color::DarkGray)
        .bg(Color::Black),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204)),
        // .bg(Color::from_u32(0x081e32)),
    changed_bytes: Style::new()
        .fg(Color::Yellow)
        .bg(Color::Black),
    highlight: Style::new()
        .fg(Color::Black)
        .bg(Color::White),
    byte_highlight: Style::new()
        .fg(Color::White)
        .bg(Color::LightYellow),
    status_bar: Style::new()
        .fg(Color::White)
        .bg(Color::Black),
    error: Style::new()
        .fg(Color::White)
        .bg(Color::Red),
    editing: Style::new()
        .fg(Color::from_u32(0x1e1e1e))
        .bg(Color::Rgb(255, 215, 0))
        .add_modifier(Modifier::RAPID_BLINK),
};

pub const BLUE: Theme = Theme {
    offsets: Style::new()
        .fg(Color::from_u32(0x5dffff))
        .bg(Color::from_u32(0x001686)),
    main: Style::new()
        .fg(Color::from_u32(0xffd404))
        .bg(Color::from_u32(0x001686)),
    dimmed: Style::new()
        .fg(Color::DarkGray)
        .bg(Color::from_u32(0x001686)),
    dialog: Style::new()
        .fg(Color::Rgb(204, 204, 204)),
        // .bg(Color::from_u32(0x081e32)),
    changed_bytes: Style::new()
        .fg(Color::Yellow)
        .bg(Color::from_u32(0x001686)),
    highlight: Style::new()
        .fg(Color::Black)
        .bg(Color::White),
    byte_highlight: Style::new()
        .fg(Color::from_u32(0x000000))
        .bg(Color::from_u32(0xffd404)),
    status_bar: Style::new()
        .fg(Color::from_u32(0xffd404))
        .bg(Color::from_u32(0x001686)),
    error: Style::new()
        .fg(Color::White)
        .bg(Color::Red),
    editing: Style::new()
        .fg(Color::from_u32(0x1e1e1e))
        .bg(Color::Rgb(255, 215, 0))
        .add_modifier(Modifier::RAPID_BLINK),
};