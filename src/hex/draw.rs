use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Cell, Clear, Row, Table, Paragraph},
};

use crate::{app::App, editor::UIState};

// Left column with offsets
pub fn draw_hex_offsets(app: &mut App, frame: &mut Frame, area: Rect) {
    // Offset lines
    let mut rows: Vec<Row> =
        Vec::with_capacity(app.reader.page_current_size / app.config.hex_mode_bytes_per_line);
    let mut ofs = app.reader.page_start;
    let height = frame.area().height as usize;

    for _ in 0..height {
        rows.push(Row::new([format!("{ofs:08X}")]));
        ofs += app.config.hex_mode_bytes_per_line;

        // Prevent further offsets to appear
        // TODO: Fix bug if the window is resized
        if ofs >= app.file_info.size {
            break;
        }
    }

    // Show filesize as last offset
    if app.file_info.size > 0 {
        rows.push(Row::new([format!("{:08X}", app.file_info.size)]));
    }

    app.hex_view
        .offset_state
        .select(Some(app.hex_view.cursor.y));

    let table = Table::new(rows, [Constraint::Length(12); 1]).style(app.config.theme.offsets);

    frame.render_stateful_widget(table, area, &mut app.hex_view.offset_state);
}

// Middle area with the actual hex dump
// TODO: refactor this as I did for draw_hex_ascii()
pub fn draw_hex_contents(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut rows: Vec<Row> =
        Vec::with_capacity(app.reader.page_current_size / app.config.hex_mode_bytes_per_line);
    // A cell for each byte as they need different styles when edited
    let mut byte_row: Vec<Cell> = Vec::with_capacity(app.reader.page_current_size);
    let mut cell_hl_style = app.config.theme.highlight;
    let mut byte_style = app.config.theme.main;
    let mut col_len = 2u16;

    let buffer = app.file_info.get_buffer();
    for (i, byte) in buffer
        .iter()
        .skip(app.reader.page_start)
        .take(app.reader.page_current_size)
        .enumerate()
    {
        // we need the absolute offset of this byte to check
        // whether there's a new value for it in the hashmap
        // if yes, we draw the new one and style it
        let offset = i + app.reader.page_start;

        let mut byte_content = format!("{byte:02X}");
        byte_style =
            if app.state == UIState::HexSelection && app.hex_view.selection.contains(offset) {
                app.config.theme.highlight
            } else if *byte == b'\0' && app.config.dim_zeroes {
                app.config.theme.dimmed
            } else if !byte.is_ascii_graphic() && app.config.dim_control_chars {
                app.config.theme.dimmed
            } else {
                app.config.theme.main
            };

        if app.state == UIState::HexEditing && app.hex_view.editing_hex {
            cell_hl_style = app.config.theme.editing;
        } else if app.state == UIState::HexSelection {
            cell_hl_style = app.config.theme.highlight;
        }

        for b in &app.hex_view.blocks {
            if offset >= b.start && offset <= b.end {
                byte_style = Style::new()
                    .bg(Color::from_u32(b.bg_color))
                    .fg(Color::from_u32(b.fg_color));
                col_len = 3;
            }
        }

        if app.hex_view.changed_bytes.contains_key(&offset) {
            // typed chars in content instead of original ones
            byte_content = app.hex_view.changed_bytes[&offset].clone();

            if !app.hex_view.selection.contains(offset) {
                byte_style = app.config.theme.changed_bytes;
            }

            // prepend a '0' while the user doesn't type the highest nibble
            if byte_content.len() == 1 {
                byte_content.insert(0, '0');
            }
        }

        // byte highlight
        if app.hex_view.highlights.contains(byte) {
            byte_style = app.config.theme.byte_highlight;
        }

        // TODO: column size (2) keep the separator char from being shown :(
        // if i > 0 && i % 4 == 0 {
        //     content.push(app.config.hex_mode_dword_separator);
        // }

        // Push the byte to the line
        byte_row.push(Cell::new(byte_content).style(byte_style));

        // If we reach EOL, push the line
        if (i + 1) % app.config.hex_mode_bytes_per_line == 0 {
            rows.push(Row::new(byte_row.clone()));
            byte_row.clear();
        }
    }

    // Last line when total file size is not multiple of 16
    // In other words, the last line contains less than 16 bytes
    if !byte_row.is_empty() {
        rows.push(Row::new(byte_row));
    }

    // Update table state (selected/highlighted byte) between frames
    app.hex_view.table_state.select(Some(app.hex_view.cursor.y));
    app.hex_view
        .table_state
        .select_column(Some(app.hex_view.cursor.x));

    // small trick to make selection looks better
    if app.state == UIState::HexSelection {
        col_len = 3;
    }

    let constraints = vec![Constraint::Length(col_len); app.config.hex_mode_bytes_per_line];

    let table = Table::new(rows, constraints)
        .column_spacing(3 - col_len)
        .style(byte_style)
        .cell_highlight_style(cell_hl_style);

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut app.hex_view.table_state);
}

/// Essa função desenha o ASCII dump em modo hexa. Ela tabmém permite a edição,
/// de modo que aceita texto normal do teclado. A função precisa:
///
/// 1. Criar uma Cell com cada char (porque precisa estilizá-la individualmente)
/// 2. Se estiver editando, estilizar o highlight (pode ser fora do loop)
/// 3. Se estiver editando E os bytes forem alterados, aplicar os estilos individualmente
/// 4. Se chegar em 16 bytes, pushar no vetor de Rows
///
/// OBS.: Table é criada a partir de Row, que são conjuntos de Cell
pub fn draw_hex_ascii(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();
    let char_style = app.config.theme.main;

    let cell_hl_style = if app.state == UIState::HexEditing && !app.hex_view.editing_hex {
        app.config.theme.editing
    } else {
        app.config.theme.highlight
    };

    let bytes_per_line = app.config.hex_mode_bytes_per_line;
    let page_start = app.reader.page_start;
    let page_current_size = app.reader.page_current_size;
    let file_size = app.file_info.size;
    let buffer = app.file_info.get_buffer();
    let page_bytes_len = file_size.saturating_sub(page_start).min(page_current_size);

    if page_bytes_len == 0 {
        let paragraph = Paragraph::new(lines);
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
        return;
    }

    // Step 1: Build the entire page bytes (applying changed_bytes)
    let mut page_bytes = vec![0u8; page_bytes_len];
    for (i, b) in page_bytes.iter_mut().enumerate() {
        let offset = page_start + i;
        *b = if let Some(s) = app.hex_view.changed_bytes.get(&offset) {
            u8::from_str_radix(s, 16).unwrap_or(buffer[offset])
        } else {
            buffer[offset]
        };
    }

    // Step 2: Decode the entire page to find character boundaries
    let mut char_cells = vec![(app.config.hex_mode_non_graphic_char.to_string(), 1usize); page_bytes_len];

    let mut idx = 0;
    while idx < page_bytes_len {
        if char_cells[idx].1 == 0 {
            idx += 1;
            continue;
        }

        let mut found = false;
        for len in 1..=4 {
            if idx + len > page_bytes_len {
                continue;
            }
            let slice = &page_bytes[idx..idx + len];
            let (decoded_str, _, had_errors) = app.text_view.table.decode(slice);

            if !had_errors && decoded_str.chars().count() == 1 {
                let c = decoded_str.chars().next().unwrap();
                if c != '\u{FFFD}' && !c.is_control() {
                    let cell_char = if c.is_ascii() {
                        if c.is_ascii_graphic() {
                            c
                        } else {
                            app.config.hex_mode_non_graphic_char
                        }
                    } else if !c.is_whitespace() {
                        c
                    } else {
                        app.config.hex_mode_non_graphic_char
                    };

                    char_cells[idx] = (cell_char.to_string(), len);
                    for j in 1..len {
                        if idx + j < page_bytes_len {
                            char_cells[idx + j] = (String::new(), 0);
                        }
                    }
                    found = true;
                    break;
                }
            }
        }

        if !found {
            char_cells[idx] = (app.config.hex_mode_non_graphic_char.to_string(), 1);
        }

        idx += 1;
    }

    // Step 3: Split into rows and build styled lines
    let mut row_start = 0;
    while row_start < page_bytes_len {
        let row_end = (row_start + bytes_per_line).min(page_bytes_len);

        let mut spans: Vec<Span> = Vec::new();
        let mut col_idx = row_start;
        while col_idx < row_end {
            let (cell_str, byte_len) = &char_cells[col_idx];

            if *byte_len == 0 {
                col_idx += 1;
                continue;
            }

            let offset = page_start + col_idx;
            let local_col = col_idx - row_start;

            let is_cursor_on_char = app.hex_view.cursor.y == (row_start / bytes_per_line)
                && app.hex_view.cursor.x >= local_col
                && app.hex_view.cursor.x < local_col + byte_len;

            let mut span_style = char_style;

            if is_cursor_on_char {
                span_style = cell_hl_style;
            } else if app.state == UIState::HexSelection && app.hex_view.selection.contains(offset) {
                span_style = app.config.theme.highlight;
            } else if app.hex_view.changed_bytes.contains_key(&offset) {
                if !app.hex_view.selection.contains(offset) {
                    span_style = app.config.theme.changed_bytes;
                }
            }

            // If this multi-byte char spans across the row boundary, show non-graphic for
            // the bytes within this row and let the next row handle the rest
            if col_idx + byte_len > row_end {
                for _ in col_idx..row_end {
                    spans.push(Span::styled(
                        app.config.hex_mode_non_graphic_char.to_string(),
                        span_style,
                    ));
                }
                break;
            }

            spans.push(Span::styled(cell_str.clone(), span_style));
            col_idx += byte_len;
        }
        lines.push(Line::from(spans));
        row_start += bytes_per_line;
    }

    let paragraph = Paragraph::new(lines);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

