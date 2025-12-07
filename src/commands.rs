use crate::{app::App, util::parse_offset, widgets::MessageType};
use ratatui::{
    Frame,
    widgets::{Clear, Paragraph},
};

use crate::{editor::UIState, widgets::Message};

use clap::{Parser, Subcommand};
use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;
use tui_input::backend::crossterm::EventHandler;

pub struct Commands;

#[derive(Subcommand, Debug)]
enum Command {
    Q,
    W,
    Wq,
    X,
    Cmt {
        offset: String,
        comment: String,
    },
    Set {
        option: String,
        value: Option<String>,
    },
}

#[derive(Parser, Debug)]
struct CommandLine {
    #[clap(subcommand)]
    command: Option<Command>,
}

fn try_goto(app: &mut App, offset: &str) {
    if let Ok(mut ofs) = parse_offset(offset) {
        if offset.starts_with('+') {
            ofs += app.hex_view.offset;
        }
        if ofs < app.file_info.size {
            app.dialog_renderer = None;
            app.goto(ofs);
        } else {
            app.dialog_renderer = Some(command_error_invalid_offset_draw);
        }
    } else {
        app.dialog_renderer = Some(command_error_invalid_draw);
    }
}

pub fn parse_command(app: &mut App, cmdline: &str) {
    if cmdline.is_empty() {
        app.dialog_renderer = None;
        return;
    }

    let args = shell_words::split(cmdline).unwrap_or_default();
    let mut argv: Vec<&str> = Vec::with_capacity(args.len() + 1);
    argv.push("dz6");

    for s in args.iter() {
        argv.push(s.as_str());
    }

    match CommandLine::try_parse_from(argv) {
        Ok(cli) => match cli.command {
            // quit
            Some(Command::Q) => app.running = false,
            // write to file
            Some(Command::W) => {
                let _ = app.write_to_file();
                if app.config.database {
                    let _ = app.save_database();
                }
                app.dialog_renderer = None;
            }
            // write and quit
            Some(Command::Wq) | Some(Command::X) => {
                let _ = app.write_to_file();
                if app.config.database {
                    let _ = app.save_database();
                }
                app.dialog_renderer = None;
                app.running = false;
            }
            // comment <offset> <comment>
            Some(Command::Cmt { offset, comment }) => {
                if let Ok(mut ofs) = parse_offset(&offset) {
                    if offset.starts_with('+') {
                        ofs = ofs.saturating_add(app.hex_view.offset);
                    }
                    if ofs < app.file_info.size {
                        Commands::comment(app, ofs, comment);
                        app.dialog_renderer = None;
                    } else {
                        app.dialog_renderer = Some(command_error_invalid_offset_draw);
                    }
                } else {
                    app.dialog_renderer = Some(command_error_invalid_draw);
                }
                app.state = UIState::Normal;
            }
            // set
            Some(Command::Set { option, value }) => match option.as_str() {
                // bytes per line
                "byteline" => {
                    if let Some(val) = value
                        && let Ok(bpl) = val.parse::<usize>()
                    {
                        if bpl > 0 {
                            app.config.hex_mode_bytes_per_line = bpl.min(48);
                        }
                    }
                    app.dialog_renderer = None;
                }
                // control / non-graphic bytes
                "ctrlchar" => {
                    if let Some(val) = value
                        && val.len() == 1
                    {
                        let c = val.chars().next().unwrap_or('.');
                        app.config.hex_mode_non_graphic_char = c;
                    }
                    app.dialog_renderer = None;
                }
                // save database files <filename>.dz6
                "db" => {
                    app.config.database = true;
                    app.dialog_renderer = None;
                }
                "nodb" => {
                    app.config.database = false;
                    app.dialog_renderer = None;
                }
                // dim (gray out) control bytes
                "dimctrl" => {
                    app.config.dim_control_chars = true;
                    app.dialog_renderer = None;
                }
                // dim null bytes only
                "dimzero" => {
                    app.config.dim_control_chars = false;
                    app.config.dim_zeroes = true;
                    app.dialog_renderer = None;
                }
                "nodim" => {
                    app.config.dim_control_chars = false;
                    app.config.dim_zeroes = false;
                    app.dialog_renderer = None;
                }
                // theme
                "theme" => {
                    if let Some(val) = value {
                        match val.as_str() {
                            "dark" => app.config.theme = crate::themes::DARK,
                            "light" => app.config.theme = crate::themes::LIGHT,
                            _ => (),
                        }
                    }
                    app.dialog_renderer = None;
                }
                _ => {
                    app.dialog_renderer = None;
                }
            },
            None => {
                try_goto(app, cmdline);
            }
        },
        Err(_) => {
            // goto as :offset
            try_goto(app, cmdline);
        }
    }
    app.state = UIState::Normal;
}

// command bar

pub fn command_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(format!(":{}", app.command_input.value()));

    frame.render_widget(Clear, app.command_area);
    frame.render_widget(para, app.command_area);
    let x = app.command_input.visual_cursor();
    frame.set_cursor_position((app.command_area.x + 1 + x as u16, app.command_area.y));
}

pub fn command_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Enter => {
                let v = app.command_input.value_and_reset();
                parse_command(app, &v);
                // app.state = UIState::Normal;
            }
            _ => {
                app.command_input.handle_event(event);
            }
        }
    }
    Ok(false)
}

pub fn command_error_invalid_offset_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from(&format!(
        "Invalid offset. Maximum offset for this file: {:X}",
        app.file_info.size - 1
    ));
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}

pub fn command_error_invalid_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from("Invalid command");
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}
