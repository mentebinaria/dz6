use crate::widgets::{Message, MessageType};
use crate::{app::App, editor::UIState};

use ratatui::Frame;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::widgets::Paragraph;
use std::io::Result;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use yara_x;

#[derive(Default, Debug)]
pub struct Search {
    pub input_text: Input,
    pub mode: SearchMode,
    pub direction: SearchDirection,
    pub input_hex: Input,
}

#[derive(Default, Debug, PartialEq)]
pub enum SearchMode {
    #[default]
    Utf8,
    // UTF_16,
    // UTF_16_LE,
    Hex,
}

impl SearchMode {
    pub fn next(&mut self) {
        if *self == SearchMode::Utf8 {
            *self = SearchMode::Hex;
        } else {
            *self = SearchMode::Utf8
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub enum SearchDirection {
    #[default]
    Forward,
    Backward,
}

fn hex_string_to_u8(hex_string: &str) -> Option<Vec<u8>> {
    if hex_string.is_empty() || !hex_string.len().is_multiple_of(2) {
        return None;
    }
    let bytes = hex::decode(hex_string).unwrap();
    Some(bytes)
}

/// returns the fist occurrence of needle, ignoring case (ASCII only for now)
/// so, it won't work with utf-8 characters for example
fn find_nocase(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // get first needle char and return early if we can't / needle.len() == 0
    let first = needle.first()?;
    let needle_len = needle.len();

    // we can't search for a needle bigger than the haystack
    if needle_len > haystack.len() {
        return None;
    }

    if first.is_ascii_alphabetic() {
        // if needle's first letter is alphabetic, we use memchr2_iter to find both variants
        for pos in memchr::memchr2_iter(
            first.to_ascii_lowercase(),
            first.to_ascii_uppercase(),
            haystack,
        ) {
            let mut matches = 1; // number of matching characters

            for c in needle.iter().skip(1) {
                if !c.eq_ignore_ascii_case(haystack.get(pos + matches)?) {
                    break;
                }

                matches += 1;
            }

            if matches == needle_len {
                return Some(pos);
            }
        }
    } else {
        // otherwise we call memchr_iter, which is faster
        for pos in memchr::memchr_iter(*first, haystack) {
            // same logic
            let mut matches = 1;

            for c in needle.iter().skip(1) {
                if !c.eq_ignore_ascii_case(haystack.get(pos + matches)?) {
                    break;
                }

                matches += 1;
            }

            if matches == needle_len {
                return Some(pos);
            }
        }
    }

    None
}

// main search entrypoint
// it will decide whether to call search_literal or search_pattern
pub fn search(app: &mut App, needle: &str, next: bool) -> Option<usize> {
    if app.hex_view.search.mode == SearchMode::Hex {
        // SearchMode::Hex will use search_pattern() if it contains patterns
        let pattern_chars = ['?', '[', '('];

        if needle.contains(|c| pattern_chars.contains(&c)) {
            return search_pattern(app, needle, false);
        // otherwise we use literal search, which is faster
        } else if let Some(n) = hex_string_to_u8(needle) {
            return search_literal(app, n, next);
        }
    }
    // SearchMode::Utf8
    search_literal(app, needle, next)
}

// search literal strings with memchr
pub fn search_literal<T: AsRef<[u8]>>(app: &mut App, needle: T, next: bool) -> Option<usize> {
    let text = needle.as_ref();
    let filesize = app.file_info.size;
    let buffer = app.file_info.get_buffer();

    if filesize == 0 || text.is_empty() {
        return None;
    }

    let ofs = if app.hex_view.search.direction == SearchDirection::Forward {
        let start = if next {
            app.hex_view.offset.checked_add(1)?
        } else {
            app.hex_view.offset
        };

        let contais_capital_letter = text.iter().any(|b| b.is_ascii_uppercase());
        let smart_search = contais_capital_letter && app.config.search_smartcase;

        if start < filesize {
            if app.config.search_ignorecase && !smart_search {
                find_nocase(buffer.get(start..)?, text).map(|pos| start + pos)
            } else {
                memchr::memmem::find(buffer.get(start..)?, text).map(|pos| start + pos)
            }
        } else {
            None
        }
    } else {
        let end = app.hex_view.offset;

        if end > 0 {
            memchr::memmem::rfind(buffer.get(..end)?, text)
        } else {
            None
        }
    };

    if ofs.is_some() {
        return ofs;
    }

    // ofs is None, check wrap setting
    if app.config.search_wrapscan {
        let ofs = if app.hex_view.search.direction == SearchDirection::Forward {
            memchr::memmem::find(buffer, text)
        } else {
            memchr::memmem::rfind(buffer, text)
        };

        if ofs.is_some() {
            return ofs;
        }
    }

    crate::beep!();
    None
}

// search patterns with yara-x
pub fn search_pattern(app: &mut App, pattern: &str, nocase: bool) -> Option<usize> {
    if pattern.is_empty() {
        return None;
    }

    let string_decl = match app.hex_view.search.mode {
        SearchMode::Utf8 => format!(
            "$a = \"{}\" {}",
            pattern,
            if nocase { "nocase" } else { "" }
        ),
        SearchMode::Hex => format!("$a = {{{pattern}}}"),
    };

    let rule = format!(
        r#"
    rule dz6 {{
        strings:
            {string_decl}
        condition:
            $a
    }}"#
    );

    let rules = yara_x::compile(&*rule).ok()?;
    let mut scanner = yara_x::Scanner::new(&rules);
    let buffer = app.file_info.get_buffer();

    if app.hex_view.search.direction == SearchDirection::Forward {
        let start = app.hex_view.offset.checked_add(1)?;
        let slice = buffer.get(start..)?;

        // enable fast scan to return the first result only
        scanner.fast_scan(true);

        let result = scanner.scan(slice).unwrap();

        // if we're at the end, the rule won't match...
        if let Some(rule) = result.matching_rules().next() {
            // na segunda vez n tem matching rule

            let pattern = rule.patterns().next()?;

            if let Some(r#match) = pattern.matches().next() {
                // the search result is an offset from start, so we add start to it
                return r#match.range().start.checked_add(start);
            }
        // ...so we check if wrapscan is set to try again from the beginning
        } else if app.config.search_wrapscan {
            let start = 0;
            let slice = buffer.get(start..)?;
            let result = scanner.scan(slice).unwrap();
            let rule = result.matching_rules().next()?;
            let pattern = rule.patterns().next()?;
            let r#match = pattern.matches().next()?;

            return Some(r#match.range().start);
        }
    } else {
        // backward search
        // this works, but fast_scan is not set, thus the scanner will
        // find all matches so we can get the last one
        let slice = buffer.get(..app.hex_view.offset)?;
        let result = scanner.scan(slice).unwrap();

        if let Some(rule) = result.matching_rules().next() {
            let pattern = rule.patterns().next()?;

            // get the last match
            if let Some(r#match) = pattern.matches().last() {
                return Some(r#match.range().start);
            }
        } else if app.config.search_wrapscan {
            let slice = buffer.get(0..)?;
            let result = scanner.scan(slice).unwrap();
            let rule = result.matching_rules().next()?;
            let pattern = rule.patterns().next()?;
            let r#match = pattern.matches().last()?;

            return Some(r#match.range().start);
        }
    }

    None
}

// string
// hex
pub fn dialog_search_draw(app: &mut App, frame: &mut Frame) {
    let prompt_char = if app.hex_view.search.direction == SearchDirection::Forward {
        '/'
    } else {
        '?'
    };

    let (para, x) = match app.hex_view.search.mode {
        SearchMode::Utf8 => (
            Paragraph::new(format!(
                "{}{}",
                prompt_char,
                app.hex_view.search.input_text.value()
            )),
            app.hex_view.search.input_text.visual_cursor(),
        ),
        SearchMode::Hex => (
            Paragraph::new(format!(
                "{}{}",
                prompt_char,
                app.hex_view.search.input_hex.value()
            )),
            app.hex_view.search.input_hex.visual_cursor(),
        ),
    };

    frame.render_widget(para, app.command_area);
    frame.set_cursor_position((app.command_area.x + 1 + x as u16, app.command_area.y));
}

pub fn dialog_search_events(app: &mut App, event: &Event) -> Result<bool> {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => {
                app.dialog_renderer = None;
                app.state = UIState::Normal;
            }
            // if input is empty, backspace works like Esc; otherwise it's handled by tui-input
            KeyCode::Backspace => match app.hex_view.search.mode {
                SearchMode::Utf8 => {
                    if app.hex_view.search.input_text.value().is_empty() {
                        app.dialog_renderer = None;
                        app.state = UIState::Normal;
                    } else {
                        app.hex_view.search.input_text.handle_event(event);
                    }
                }
                SearchMode::Hex => {
                    if app.hex_view.search.input_hex.value().is_empty() {
                        app.dialog_renderer = None;
                        app.state = UIState::Normal;
                    } else {
                        app.hex_view.search.input_hex.handle_event(event);
                    }
                }
            },
            KeyCode::Enter => {
                let needle = match app.hex_view.search.mode {
                    SearchMode::Utf8 => app.hex_view.search.input_text.value().to_string(),
                    SearchMode::Hex => app.hex_view.search.input_hex.value().to_string(),
                };

                if let Some(ofs) = search(app, &needle, false) {
                    app.goto(ofs);
                    app.dialog_renderer = None;
                } else {
                    app.dialog_renderer = Some(dialog_search_error_draw);
                    crate::beep!();
                }
                // TODO: draw "Searching..."
                app.state = UIState::Normal;
            }
            KeyCode::Tab => {
                app.hex_view.search.mode.next();
            }

            KeyCode::Char(c) => {
                match app.hex_view.search.mode {
                    SearchMode::Utf8 => app.hex_view.search.input_text.handle_event(event),
                    SearchMode::Hex => {
                        // as per https://virustotal.github.io/yara-x/docs/writing_rules/hex-patterns/
                        let allowed = [' ', '?', '[', '-', ']', 'x', 'X', '~', '(', ')', '|'];

                        if c.is_ascii_hexdigit() || allowed.contains(&c) {
                            app.hex_view.search.input_hex.handle_event(event)
                        } else {
                            None
                        }
                    }
                };
            }
            _ => {
                match app.hex_view.search.mode {
                    SearchMode::Utf8 => app.hex_view.search.input_text.handle_event(event),
                    SearchMode::Hex => app.hex_view.search.input_hex.handle_event(event),
                };
            }
        }
    }
    Ok(false)
}

pub fn dialog_search_error_draw(app: &mut App, frame: &mut Frame) {
    let mut dialog = Message::from("Pattern not found");
    dialog.kind = MessageType::Error;
    dialog.render(app, frame);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_find_nocase() {
        // found
        for needle in [
            b"opengl", b"OpenGL", b"OpengL", b"opengL", b"Opengl", b"OPENGL",
        ] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, Some(8));
        }
        for needle in [b"i think", b"I think", b"i Think", b"I thinK"] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, Some(0));
        }
        for needle in [b"ce", b"cE", b"Ce", b"CE"] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, Some(20));
        }
        for needle in [b"x", b"X"] {
            let r = find_nocase(b"I think OpenGL is nicX", needle);
            assert_eq!(r, Some(21));
        }

        // not found
        for needle in [
            b"Aopengl", b"OpeanGL", b"OpengaL", b"ope-ngL", b"Open2gl", b"OPExNGL",
        ] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, None);
        }
        for needle in [b"i t?hink", b"I thi]nk", b"i Thin/k", b"I |thinK"] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, None);
        }
        for needle in [b"c]e", b"c,E", b"Ce0", b"CE-"] {
            let r = find_nocase(b"I think OpenGL is nice", needle);
            assert_eq!(r, None);
        }
        for needle in [b"*", b"?"] {
            let r = find_nocase(b"I think OpenGL is nicX", needle);
            assert_eq!(r, None);
        }

        assert_eq!(
            find_nocase(b"\x00\x80\xCD\x6F\x00\x00\xCE\x6F\x00\x80", b"opengl"),
            None
        );
    }
}
