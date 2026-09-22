// The log window (Alt+l). It just shows what crate::logging collected

use std::borrow::Cow;
use std::io::Result;

use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Clear, Paragraph};
use tracing::Level;

use crate::util::center_widget;
use crate::{app::App, editor::UIState, logging};

// how many columns the left/right keys scroll
const COLUMN_STEP: u16 = 8;

/// Log window state
#[derive(Default)]
pub struct LogView {
    /// scroll position, as (record, column)
    pub scroll: (u16, u16),
    /// how many records fit on the screen; set while drawing
    pub height: u16,
}

impl LogView {
    /// Jump to the newest records. The draw function fixes the exact position
    pub fn scroll_to_end(&mut self) {
        self.scroll = (u16::MAX, 0);
    }
}

// we use the terminal colors here so the levels are readable in any theme
fn level_style(level: Level) -> Style {
    Style::new().fg(match level {
        Level::ERROR => Color::LightRed,
        Level::WARN => Color::LightYellow,
        Level::INFO => Color::LightGreen,
        Level::DEBUG => Color::LightBlue,
        Level::TRACE => Color::Gray,
    })
}

pub fn dialog_log_draw(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    let dialog_area = center_widget(
        area.width.saturating_sub(5),
        area.height.saturating_sub(5),
        area,
    );

    let buffer = logging::buffer().lock();
    let records = buffer.records();
    let dropped = buffer.dropped();

    let title = if dropped == 0 {
        format!(" Log ({}) ", records.len())
    } else {
        format!(" Log ({}, {} dropped) ", records.len(), dropped)
    };

    let block = Block::bordered()
        .title(Line::from(title).centered())
        .title_bottom(Line::from(format!(" RUST_LOG={} ", logging::filter())).centered());

    let inner = block.inner(dialog_area);
    app.log_view.height = inner.height;

    // one record per line, so we know exactly where the last page starts and
    // we only need to build the lines that are visible
    let visible = inner.height as usize;
    let last = u16::try_from(records.len().saturating_sub(visible)).unwrap_or(u16::MAX);
    app.log_view.scroll.0 = app.log_view.scroll.0.min(last);

    let dim = Style::new().add_modifier(Modifier::DIM);

    let text: Text = if records.is_empty() {
        Text::from(Line::styled(
            "Nothing logged yet. Run with RUST_LOG=debug for more detail.",
            dim,
        ))
    } else {
        records
            .iter()
            .skip(app.log_view.scroll.0 as usize)
            .take(visible)
            .map(|record| {
                let scope: Cow<str> = match record.span {
                    Some(span) => Cow::Owned(format!("{}{{{}}}", record.module(), span)),
                    None => Cow::Borrowed(record.module()),
                };

                Line::from(vec![
                    Span::styled(record.time().to_string(), dim),
                    Span::raw(" "),
                    Span::styled(
                        format!("{:>5}", record.level.as_str()),
                        level_style(record.level),
                    ),
                    Span::raw(" "),
                    Span::styled(scope, dim),
                    Span::styled(": ", dim),
                    Span::raw(record.message.as_str()),
                ])
            })
            .collect()
    };

    let paragraph = Paragraph::new(text)
        .style(app.config.theme.dialog)
        .block(block)
        .scroll((0, app.log_view.scroll.1));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(paragraph, dialog_area);
}

pub fn dialog_log_events(app: &mut App, key: KeyEvent) -> Result<bool> {
    let page = app.log_view.height.max(1);

    match key.code {
        // close log window
        KeyCode::Esc | KeyCode::Char('q') => {
            app.dialog_renderer = None;
            app.state = UIState::Normal;
        }
        // scroll; the draw function clamps it to the last record
        KeyCode::Down | KeyCode::Char('j') => {
            app.log_view.scroll.0 = app.log_view.scroll.0.saturating_add(1);
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.log_view.scroll.0 = app.log_view.scroll.0.saturating_sub(1);
        }
        KeyCode::PageDown | KeyCode::Char('f') => {
            app.log_view.scroll.0 = app.log_view.scroll.0.saturating_add(page);
        }
        KeyCode::PageUp | KeyCode::Char('b') => {
            app.log_view.scroll.0 = app.log_view.scroll.0.saturating_sub(page);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.log_view.scroll.1 = app.log_view.scroll.1.saturating_add(COLUMN_STEP);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.log_view.scroll.1 = app.log_view.scroll.1.saturating_sub(COLUMN_STEP);
        }
        KeyCode::Home | KeyCode::Char('g') => app.log_view.scroll = (0, 0),
        KeyCode::End | KeyCode::Char('G') => app.log_view.scroll_to_end(),
        // clear the messages
        KeyCode::Char('c') => logging::buffer().lock().clear(),
        _ => {}
    }

    Ok(false)
}
