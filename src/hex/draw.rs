use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Cell, Clear, Row, Table},
};

use crate::{app::App, config::APP_PAGE_SIZE, editor::UIState};

// Left column with offsets
pub fn draw_hex_offsets(app: &mut App, frame: &mut Frame, area: Rect) {
    // Offset lines
    let mut rows: Vec<Row> = Vec::with_capacity(APP_PAGE_SIZE / 16);
    let mut ofs = app.reader.page_start;
    let num_offsets = app
        .reader
        .page_current_size
        .div_ceil(app.config.hex_mode_bytes_per_line);

    for _ in 0..num_offsets {
        rows.push(Row::new([format!("{ofs:08X}")]));
        ofs += app.config.hex_mode_bytes_per_line;
    }

    app.hex_view
        .offset_state
        .select(Some(app.hex_view.cursor.y));

    let table = Table::new(rows, [Constraint::Length(12); 1])
        .column_spacing(1)
        .style(app.config.theme.offsets);

    frame.render_stateful_widget(table, area, &mut app.hex_view.offset_state);
}

// Middle area with the actual hex dump
// TODO: refactor this as I did for draw_hex_ascii()
pub fn draw_hex_contents(app: &mut App, frame: &mut Frame, area: Rect) {
    let mut rows: Vec<Row> = Vec::with_capacity(APP_PAGE_SIZE / 16);
    // A cell for each byte as they need different styles when edited
    let mut byte_row: Vec<Cell> = Vec::with_capacity(APP_PAGE_SIZE);
    let mut cell_hl_style = app.config.theme.highlight;
    let mut byte_style = app.config.theme.main;

    for (i, byte) in app
        .buffer
        .iter()
        .skip(app.reader.offset_location_in_cache)
        .take(app.reader.page_current_size)
        .enumerate()
    {
        // we need the absolute offset of this byte to check
        // whether there's a new value for it in the hashmap
        // if yes, we draw the new one and style it
        let offset = i + app.reader.page_start;

        let mut byte_content = format!("{byte:02X}");
        byte_style = if app.hex_view.highlihts.contains(byte) {
            app.config.theme.byte_highlight
        } else if *byte == b'\0' && app.config.dim_zeroes {
            app.config.theme.dimmed
        } else if !byte.is_ascii_graphic() && app.config.dim_control_chars {
            app.config.theme.dimmed
        } else {
            app.config.theme.main
        };

        if app.state == UIState::HexEditing && app.hex_view.editing_hex {
            cell_hl_style = app.config.theme.editing;
        }

        if app.hex_view.changed_bytes.contains_key(&offset) {
            // typed chars in content instead of original ones
            byte_content = app.hex_view.changed_bytes[&offset].clone();
            byte_style = app.config.theme.changed_bytes;
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

    let constraints = vec![Constraint::Length(2); app.config.hex_mode_bytes_per_line];

    let table = Table::new(rows, constraints)
        .column_spacing(1)
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
    // Uma linha é um conjunto de células, cada uma contendo um caractere
    let mut row: Vec<Cell> = Vec::with_capacity(app.config.hex_mode_bytes_per_line);

    // A tabela precisa receber um conjunto de linhas
    let mut rows: Vec<Row> = Vec::new();

    // O estilo que será usado no caractere individual
    let mut char_style = app.config.theme.main;

    // Se estiver editando e apertou tab para editar via ASCII, usa o highlight
    // correto a partir do tema
    let cell_hl_style = if app.state == UIState::HexEditing && !app.hex_view.editing_hex {
        app.config.theme.editing
    } else {
        app.config.theme.highlight
    };

    for (i, byte) in app
        .buffer
        .iter()
        .skip(app.reader.offset_location_in_cache)
        .take(app.reader.page_current_size)
        .enumerate()
    {
        // Antes de criar a Cell a partir do byte, preciso tratar
        // os bytes inválidos em ASCII
        let c = if (*byte).is_ascii_graphic() {
            *byte as char
        } else {
            app.config.hex_mode_non_graphic_char
        };

        // O conteúdo e o estilo da célula agora vai depender se
        // o offset atual está no hashmap de bytes alterados
        let offset = i + app.reader.page_start;
        let cell = if app.hex_view.changed_bytes.contains_key(&offset) {
            // Define o estilo
            char_style = app.config.theme.changed_bytes;
            // Recupera o byte alterado (como hex string)
            let s = &app.hex_view.changed_bytes[&offset];

            // Converte para um u8 numérico. Se não rolar, é porque deu uma
            // merda muito grande pois só deveria ter hex strings no hashmap.
            let num = u8::from_str_radix(s, 16).unwrap();
            let c = num as char;
            // Agora cria uma string a partir do char `c`
            // Parece doido, mas isso faz "41" -> 0x41 -> "A"
            let s = String::from(c);
            // Por fim, retorna a célula
            Cell::new(s.clone()).style(char_style)
        } else {
            // Se não for um byte alterado, usa o estilo padrão do tema
            char_style = app.config.theme.main;
            // Cria a string a partir do char `c` e retorna a célula
            let s = String::from(c);
            Cell::new(s).style(char_style)
        };

        // Agora a célula tá pronta para ser colocado na linha,
        // mas antes aplico o estilo nela
        row.push(cell.style(char_style));

        // Se chegamos no fim da linha
        if (i + 1) % app.config.hex_mode_bytes_per_line == 0 {
            // Cria uma linha a partir do vetor de células
            // e a adiciona no vetor de linhas
            rows.push(Row::new(row.clone()));
            // Limpa a linha para ser reutilizada
            row.clear();
        }
    } // for

    // Se a última linha não estiver vazia, significa que ela não foi
    // incluída no vetor de linhas ainda. É o caso onde a última linha
    // tem menos de `app.config.hex_mode_bytes_per_line` bytes.
    if !row.is_empty() {
        rows.push(Row::new(row));
    }

    // Atualiza o estado da tabela com a seleção do cursor
    app.hex_view.ascii_state.select(Some(app.hex_view.cursor.y));
    app.hex_view
        .ascii_state
        .select_column(Some(app.hex_view.cursor.x));

    // O Constraint contém as dimensões da tabela
    let constraint = vec![Constraint::Length(1); app.config.hex_mode_bytes_per_line];

    // Cria a tabela
    let table = Table::new(rows, constraint)
        .column_spacing(0)
        .style(char_style)
        .cell_highlight_style(cell_hl_style);

    // Desenha a tabela
    frame.render_widget(Clear, area);
    frame.render_stateful_widget(table, area, &mut app.hex_view.ascii_state);
}
