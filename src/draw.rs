use ratatui::{Frame, prelude::*, widgets::Paragraph};

use crate::{
    app::App,
    editor::AppView,
    global,
    hex::{self, comment},
    ruler, text,
};

/// This is the main drawing/rendering function that
/// draws the layout areas and renders all Ratatui
/// widgets by calling the right functions to do so.
/// It is passed as callback function to terminal.draw()
/// in the main() loop.
pub fn draw(frame: &mut Frame, app: &mut App) {
    if frame.area().width < 68 || frame.area().height < 10 {
        let err = Paragraph::new("dz6 needs at least a 68x10 terminal.");
        frame.render_widget(err, frame.area());
        return;
    }

    // Draw things depending on the view
    match app.editor_view {
        AppView::Hex => {
            let constraints = vec![
                Constraint::Length(1),       // ruler
                Constraint::Percentage(100), // middle area (hex content)
                Constraint::Length(1),       // status bar
                Constraint::Length(1),       // command bar
            ];

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            // Draw ruler at the top
            ruler::ruler_draw(app, frame, vertical_layout[0]);

            // Draw status bar at the bottom
            global::status_bar::status_bar_draw(app, frame, vertical_layout[2]);

            app.command_area = vertical_layout[3];

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(10),
                    Constraint::Length((app.config.hex_mode_bytes_per_line * 3 + 2) as u16),
                    Constraint::Min(app.config.hex_mode_bytes_per_line as u16),
                ])
                .split(vertical_layout[1]);

            hex::draw::draw_hex_offsets(app, frame, horizontal_layout[0]);
            hex::draw::draw_hex_contents(app, frame, horizontal_layout[1]);
            hex::draw::draw_hex_ascii(app, frame, horizontal_layout[2]);
            comment::comment_show_draw(app, frame);
        }
        AppView::Text => {
            let constraints = vec![
                Constraint::Percentage(100), // middle area (text content)
                Constraint::Length(1),       // status bar
                Constraint::Length(1),       // command bar
            ];

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(frame.area());

            // Draw status bar at the bottom
            global::status_bar::status_bar_draw(app, frame, vertical_layout[1]);

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(100)])
                .split(vertical_layout[0]);

            text::draw::text_contents_draw(app, frame, horizontal_layout[0]);
            app.text_view.area_height = horizontal_layout[0].height;
        }
    }

    // The right event handler function is set by the keypress
    // for example, in hex/events.rs, F5 (Goto) will set app.dialog_renderer
    // to Some(global::goto::draw_hex_dialog_goto). The code below just executes
    // the function pointed by this field if there's any.
    if let Some(f) = app.dialog_renderer {
        f(app, frame);
    }

    if let Some(f) = app.dialog_2nd_renderer {
        f(app, frame);
    }
}
