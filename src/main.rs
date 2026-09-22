mod app;
mod commands;
mod config;
mod database;
mod draw;
mod editor;
mod events;
mod global;
mod header;
mod hex;
mod initfile;
mod input_history;
mod logging;
mod reader;
mod ruler;
mod text;
mod themes;
mod util;
mod widgets;

use std::{io, process::ExitCode};

use clap::Parser;
use ratatui::{DefaultTerminal, crossterm::event};
use tracing::{debug, info, warn};

use app::App;
use logging::Logging;

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

fn main() -> ExitCode {
    let args = Args::parse();
    let logging = logging::init();

    // this goes before ratatui::init(), whose hook wraps ours, so the terminal
    // is restored first and then we flush the log
    install_panic_hook(logging);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        file = %args.file,
        readonly = args.readonly,
        "starting"
    );

    let cursor_offset = match util::parse_offset(&args.offset) {
        Ok(offset) => offset,
        Err(error) => {
            warn!(offset = %args.offset, %error, "invalid offset, starting at 0");
            0
        }
    };

    let mut app = App::new();

    if let Err(error) = app.load_file(&args.file, cursor_offset, args.readonly) {
        eprintln!("{}: {}", args.file, error);
        return ExitCode::FAILURE;
    }

    app.list_state.select_first();
    app.read_initfile();

    let mut terminal = ratatui::init();
    let outcome = run(&mut app, &mut terminal);
    ratatui::restore();

    // the TUI is gone, so we can print what the log window couldn't show
    logging.flush();

    if let Err(error) = outcome {
        eprintln!("dz6: {}", error);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Draw, read one event, handle it, until the user quits or something fails
fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while app.running {
        terminal.draw(|frame| {
            update_page_size(app, frame.area().height);
            app.screen = frame.area();
            draw::draw(frame, app);
        })?;

        let event = event::read()?;
        events::handle_events(app, event)?;
    }

    debug!("quitting");

    Ok(())
}

/// Flushes the log after a panic. The hook set by ratatui::init() restores the
/// terminal and the default one prints the panic, so our messages come last
fn install_panic_hook(logging: Logging) {
    let previous = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        previous(info);
        logging.flush();
    }));
}

/// Page size is dynamically calculated as:
/// frame height - (command line + status line + header) * bytes per line
pub fn update_page_size(app: &mut App, height: u16) {
    // Prevent panic on underflow with small screen sizes
    // Currently, we can't have them because of widgets such as Calculator,
    // but we might add support for such small screen sizes in the future
    let page_size = if height.checked_sub(3).is_some() {
        (height - 3) as usize * app.config.hex_mode_bytes_per_line
    } else {
        app.config.hex_mode_bytes_per_line
    };

    if page_size != app.reader.page_current_size {
        debug!(
            height,
            from = app.reader.page_current_size,
            to = page_size,
            "page size changed"
        );

        app.reader.page_current_size = page_size;
        app.reader.page_end = app.reader.page_start + page_size.wrapping_sub(1);
    }
}

#[macro_export]
macro_rules! beep {
    () => {
        print!("\x07")
    };
}
