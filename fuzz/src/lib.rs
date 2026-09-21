#[path = "../../src/app.rs"] pub mod app;
#[path = "../../src/commands.rs"] pub mod commands;
#[path = "../../src/config.rs"] pub mod config;
#[path = "../../src/database.rs"] pub mod database;
#[path = "../../src/draw.rs"] pub mod draw;
#[path = "../../src/editor.rs"] pub mod editor;
#[path = "../../src/events.rs"] pub mod events;
#[path = "../../src/global/mod.rs"] pub mod global;
#[path = "../../src/header/mod.rs"] pub mod header;
#[path = "../../src/hex/mod.rs"] pub mod hex;
#[path = "../../src/initfile.rs"] pub mod initfile;
#[path = "../../src/input_history.rs"] pub mod input_history;
#[path = "../../src/reader.rs"] pub mod reader;
#[path = "../../src/ruler.rs"] pub mod ruler;
#[path = "../../src/text/mod.rs"] pub mod text;
#[path = "../../src/themes.rs"] pub mod themes;
#[path = "../../src/util.rs"] pub mod util;
#[path = "../../src/widgets.rs"] pub mod widgets;

#[macro_export]
macro_rules! beep {
    () => {
        print!("\x07")
    };
}
