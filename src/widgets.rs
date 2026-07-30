use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, Paragraph},
};

use crate::app::App;
use crate::util::center_widget;

#[derive(PartialEq)]
pub enum MessageType {
    Error,
    Info,
}

pub struct Message {
    pub buffer: String,
    pub kind: MessageType,
}

impl Message {
    pub fn from(message: &str) -> Self {
        Self {
            buffer: message.to_string(),
            kind: MessageType::Info,
        }
    }

    pub fn render(&mut self, app: &mut App, frame: &mut Frame) {
        let text = self.buffer.clone();
        let style = if self.kind == MessageType::Error {
            app.config.theme.error
        } else {
            app.config.theme.main
        };
        let paragraph = Paragraph::new(text).style(style);

        frame.render_widget(Clear, app.command_area);
        frame.render_widget(paragraph, app.command_area);
    }
}

pub struct ListChoice {
    pub choices: Vec<String>,
    title: String,
}

impl ListChoice {
    pub fn new() -> Self {
        Self {
            choices: vec![],
            title: String::with_capacity(50),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn render(&mut self, app: &mut App, frame: &mut Frame) {
        let area = frame.area();
        
        // Calculate height dynamically: choices count + 2 (borders)
        let dialog_height = (self.choices.len() as u16 + 2).min(area.height);
        
        // Calculate width dynamically: longest choice + padding
        let max_choice_width = self.choices.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
        let dialog_width = (max_choice_width + 12).max(area.width / 3).min(area.width);
        
        // Calculate centered position, then shift it upwards dynamically based on height (roughly 2 lines for standard heights)
        let mut dialog_area = center_widget(dialog_width, dialog_height, area);
        dialog_area.y = dialog_area.y.saturating_sub((area.height / 12).max(1));
        
        let block = Block::new()
            .title(Line::raw(self.title.clone()).centered())
            .borders(Borders::ALL)
            .style(app.config.theme.dialog);

        let lines: Vec<Line> = self
            .choices
            .iter()
            .map(|s| Line::raw(s).style(app.config.theme.dialog).centered())
            .collect();

        let list = List::new(lines)
            .block(block)
            .style(app.config.theme.dialog)
            .highlight_style(app.config.theme.highlight)
            .repeat_highlight_symbol(true);

        frame.render_widget(Clear, dialog_area);
        frame.render_stateful_widget(list, dialog_area, &mut app.list_state);
    }
}
