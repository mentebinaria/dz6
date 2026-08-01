use std::{
    fmt::{Display, UpperHex},
    num::ParseIntError,
};

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

/// Receives a number `n` and returns a string formatted according to `base`,
/// which can be 16 to format it as hex (uppercase). Anything different than 16
/// causes the number to be converted to string (decimal).
pub fn number_to_str_radix<T: UpperHex + Display>(n: T, base: u32) -> String {
    if base == 16 {
        return format!("{:8X}", n);
    }
    format!("{:8}", n)
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
    #[test]
    fn number_to_str_radix_test() {
        assert!(number_to_str_radix(42, 16) == "2A".to_string());
        assert!(number_to_str_radix(42, 10) == "42".to_string());
        assert!(number_to_str_radix(42, 0) == "42".to_string());
        assert!(number_to_str_radix(-42, 10) == "-42".to_string());
        assert!(number_to_str_radix(-42i16, 16) == "FFD6".to_string());
    }
}
