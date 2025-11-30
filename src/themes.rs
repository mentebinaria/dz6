use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub main: Style,
    pub dimmed: Style,
    pub offsets: Style,
    pub changed_bytes: Style,
    pub highlight: Style,
    pub byte_highlight: Style,
    pub topbar: Style,
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
    topbar: Style::new()
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

    topbar: Style::new()
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
