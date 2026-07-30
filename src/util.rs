use std::num::ParseIntError;

use ratatui::layout::Rect;

/// This function is used returns the right offset
/// for goto(). Hexa is the default. Add 't' suffix for decimal
pub fn parse_offset(expr: &str) -> Result<usize, ParseIntError> {
    if expr.ends_with("t") {
        expr[0..expr.len() - 1].parse()
    } else {
        usize::from_str_radix(expr, 16)
    }
}

pub fn center_widget(width: u16, height: u16, area: Rect) -> Rect {
    Rect {
        x: area.width / 2 - width / 2,
        y: area.height / 2 - height / 2 - 1,
        width,
        height,
    }
}

pub fn encode_char(c: char, enc: &'static encoding_rs::Encoding) -> Vec<u8> {
    match enc.name() {
        "UTF-16LE" => {
            let mut buf = [0u16; 2];
            let u16_slice = c.encode_utf16(&mut buf);
            let mut bytes = Vec::new();
            for &val in u16_slice.iter() {
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            bytes
        }
        "UTF-16BE" => {
            let mut buf = [0u16; 2];
            let u16_slice = c.encode_utf16(&mut buf);
            let mut bytes = Vec::new();
            for &val in u16_slice.iter() {
                bytes.extend_from_slice(&val.to_be_bytes());
            }
            bytes
        }
        _ => {
            let c_str = c.to_string();
            let (encoded_bytes, _, has_unmappable) = enc.encode(&c_str);
            if has_unmappable {
                vec![b'?']
            } else {
                encoded_bytes.into_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_expr_test() {
        assert_eq!(Ok(255), parse_offset("ff"));
        assert_eq!(Ok(16), parse_offset("10"));
        assert_eq!(Ok(255), parse_offset("ff"));
        assert_eq!(Ok(255), parse_offset("255t"));
        // Errors
        assert!(parse_offset("255th").is_err());
        assert!(parse_offset("255ht").is_err());
        assert!(parse_offset("ht").is_err());
        assert!(parse_offset("h3").is_err());
        assert!(parse_offset("-5").is_err());
        assert!(parse_offset("4h4h").is_err());
    }
}
