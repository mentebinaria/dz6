use crate::{app::App, util::parse_offset, widgets::MessageType};
use ratatui::{
    Frame,
    widgets::{Clear, Paragraph},
};

use crate::{editor::UIState, widgets::Message};

use crate::app::Dz6Error;
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
    Sel {
        start: String,
        length: String,
    },
}

#[derive(Parser, Debug)]
struct CommandLine {
    #[clap(subcommand)]
    command: Option<Command>,
}
#[derive(Debug, PartialEq)]
enum OffsetType {
    Backward,
    Absolute,
    Forward,
}

fn try_goto(app: &mut App, offset: &str) {
    let offset_direction;
    let mut new_offset = offset;

    if offset.starts_with('-') {
        offset_direction = OffsetType::Backward;
        new_offset = &offset[1..];
    } else if offset.starts_with('+') {
        offset_direction = OffsetType::Forward;
        new_offset = &offset[1..];
    } else {
        offset_direction = OffsetType::Absolute;
    }

    if let Ok(mut ofs) = parse_offset(&new_offset) {
        if offset_direction == OffsetType::Forward {
            ofs = app.hex_view.offset.saturating_add(ofs);
        } else if offset_direction == OffsetType::Backward {
            ofs = app.hex_view.offset.saturating_sub(ofs);
        }
        if ofs < app.file_info.size {
            app.dialog_renderer = None;
            app.state = UIState::Normal;
            app.goto(ofs);
        } else {
            app.last_error = Dz6Error {
                message: format!(
                    "Invalid range: {}; maximum offset for this file is {}",
                    offset,
                    app.file_info.size.saturating_sub(1)
                ),
            };
            app.dialog_renderer = Some(command_error_draw);
        }
    } else {
        app.last_error = Dz6Error {
            message: format!("Not an editor command: {}", offset),
        };
        app.dialog_renderer = Some(command_error_draw);
    }
}

pub fn parse_command(app: &mut App, cmdline: &str) {
    if cmdline.is_empty() {
        app.state = UIState::Normal;
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
                app.state = UIState::Normal;
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
                        app.last_error = Dz6Error {
                            message: format!(
                                "Invalid range: {}; maximum offset for this file is {}",
                                cmdline,
                                app.file_info.size.saturating_sub(1)
                            ),
                        };
                        app.dialog_renderer = Some(command_error_draw);
                    }
                } else {
                    app.last_error = Dz6Error {
                        message: format!("Invalid argument: {}", offset),
                    };
                    app.dialog_renderer = Some(command_error_draw);
                }
                app.state = UIState::Normal;
            }
            // set
            Some(Command::Set { option, value }) => {
                match option.as_str() {
                    // bytes per line
                    "byteline" => {
                        if let Some(val) = value {
                            if let Ok(bpl) = val.parse::<usize>() {
                                // Bound user typed value by `max` - 1
                                if app.screen.width > 0 {
                                    let max = ((app.screen.width - 9) / 4) as usize;
                                    app.config.hex_mode_bytes_per_line = bpl.min(max - 1);
                                } else {
                                    app.config.hex_mode_bytes_per_line = bpl.min(64);
                                }
                                app.config.hex_mode_bytes_per_line_auto = false;
                            } else if let Ok(bpl) = val.parse::<String>()
                                && bpl == "auto"
                            {
                                app.config.hex_mode_bytes_per_line_auto = true;
                                if app.screen.width > 0 {
                                    let max = ((app.screen.width - 9) / 4) as usize;
                                    app.config.hex_mode_bytes_per_line = max - 1;
                                }
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
                                "dark" => {
                                    app.config.theme = crate::themes::DARK;
                                    app.dialog_renderer = None;
                                }
                                "light" => {
                                    app.config.theme = crate::themes::LIGHT;
                                    app.dialog_renderer = None;
                                }
                                _ => {
                                    app.last_error = Dz6Error {
                                        message: format!("Invalid theme: {}", val),
                                    };
                                    app.dialog_renderer = Some(command_error_draw);
                                }
                            }
                        }
                    }
                    // saarch wrap
                    "wrapscan" => {
                        app.config.search_wrap = true;
                        app.dialog_renderer = None;
                    }
                    "nowrapscan" => {
                        app.config.search_wrap = false;
                        app.dialog_renderer = None;
                    }
                    "hlsearch" => {
                        app.config.hlsearch = true;
                        app.dialog_renderer = None;
                    }
                    "nohlsearch" => {
                        app.config.hlsearch = false;
                        app.dialog_renderer = None;
                    }
                    _ => {
                        app.dialog_renderer = None;
                    }
                }
                app.state = UIState::Normal;
            }
            Some(Command::Sel { start, length }) => {
                app.state = UIState::HexSelection;
                app.dialog_renderer = None;

                if let Ok(st) = parse_offset(&start)
                    && let Ok(len) = parse_offset(&length)
                {
                    app.hex_view.selection.start = st;
                    app.hex_view.selection.end = st.saturating_add(len);
                    app.goto(st);
                }
            }
            None => {
                try_goto(app, cmdline);
            }
        },
        Err(_) => {
            // goto as :offset
            try_goto(app, cmdline);
        }
    }
}

// command bar

pub fn command_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(format!(":{}", app.command_input.input.value()));

    frame.render_widget(Clear, app.command_area);
    frame.render_widget(para, app.command_area);
    let x = app.command_input.input.visual_cursor();
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
                let v = app.command_input.input.value_and_reset();
                parse_command(app, &v);
                app.command_input.push(v);
            }
            KeyCode::Up => {
                app.command_input.up();
            }
            KeyCode::Down => {
                app.command_input.down();
            }
            _ => {
                app.command_input.input.handle_event(event);
            }
        }
    }
    Ok(false)
}

pub fn command_error_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from(&app.last_error.message);
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
    app.state = UIState::Error;
}
