mod app;
mod commands;
mod config;
mod database;
mod draw;
mod editor;
mod events;
mod global;
mod hex;
mod initfile;
mod input_history;
mod reader;
mod ruler;
mod text;
mod themes;
mod util;
mod widgets;

use std::process;

use clap::Parser;

use app::App;

/// vim-like hexadecimal editor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to open
    file: String,

    /// Initial cursor offset (hex default; `t` suffix = decimal)
    #[arg(short, long, default_value = "0")]
    offset: String,

    /// Set read-only mode
    #[arg(short, long)]
    readonly: bool,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();
    let cursor_offset = util::parse_offset(&args.offset).unwrap_or_default();

    app.load_file(&args.file, cursor_offset, args.readonly)
        .unwrap_or_else(|e| {
            eprintln!("{}: {}", args.file, e);
            process::exit(1);
        });

    app.list_state.select_first();

    // read init file ignoring errors
    let _ = app.read_initfile();

    let mut terminal = ratatui::init();

    while app.running {
        terminal
            .draw(|f| {
                // Page size is dynamic calculated as:
                // frame height - (command line + status line + header) * bytes per line
                let page_size = (f.area().height - 3) as usize * app.config.hex_mode_bytes_per_line;
                if page_size != app.reader.page_current_size {
                    app.reader.page_current_size = page_size;
                    app.reader.page_end = app.reader.page_start + page_size - 1;
                }
                draw::draw(f, &mut app)
            })
            .expect("failed to draw frame");

        events::handle_events(&mut app).expect("unable to read events");
    }

    ratatui::restore();
}
