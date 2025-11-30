use ratatui::{
    Frame,
    widgets::{Clear, Paragraph},
};

use ratatui::crossterm::event::{Event, KeyCode};
use std::io::Result;

use tui_input::backend::crossterm::EventHandler;

use crate::{
    app::{App, Comment},
    commands::Commands,
    editor::UIState,
};

pub fn dialog_comment_draw(app: &mut App, frame: &mut Frame) {
    let para = Paragraph::new(format!(";{}", app.hex_view.comment_input.value()));

    frame.render_widget(Clear, app.command_area);
    frame.render_widget(para, app.command_area);
    let x = app.hex_view.comment_input.visual_cursor();
    frame.set_cursor_position((app.command_area.x + 1 + x as u16, app.command_area.y));
}

fn sync_comments(app: &mut App) {
    let ofs = app.hex_view.offset;
    if let Some(idx) = app
        .hex_view
        .comment_name_list
        .iter()
        .position(|x| x.offset == ofs)
    {
        app.hex_view.comment_name_list.remove(idx);
    }
}

impl Commands {
    pub fn comment(app: &mut App, offset: usize, comment: String) {
        if comment.is_empty() {
            // remove the comment; no effect if it doesn't exist
            app.hex_view.comments.remove(&offset);
            sync_comments(app);
        } else {
            app.hex_view.comments.insert(offset, comment.clone());
            sync_comments(app);
            app.hex_view
                .comment_name_list
                .push(Comment { offset, comment });
        }
        app.dialog_renderer = None;
        app.state = UIState::Normal;
    }
}

pub fn dialog_comment_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            KeyCode::Enter => {
                let ofs = app.hex_view.offset;
                let cmt = app.hex_view.comment_input.value_and_reset();
                Commands::comment(app, ofs, cmt);
            }
            _ => {
                app.hex_view.comment_input.handle_event(event);
            }
        }
    }
    Ok(false)
}

pub fn comment_show_draw(app: &mut App, frame: &mut Frame) {
    // check if the current offset has a comment to be shown
    if let Some(cmt) = app.hex_view.comments.get(&app.hex_view.offset) {
        // format comment
        let para = Paragraph::new(format!(";{}", cmt)).style(app.config.theme.main);

        frame.render_widget(Clear, app.command_area);
        frame.render_widget(para, app.command_area);
    }
}
