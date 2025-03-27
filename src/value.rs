use crate::value_char;
use nom::{IResult, Parser};

/// value         = *VALUE-CHAR
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn value(input: &str) -> IResult<&str, String> {
    nom::multi::many0(value_char)
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(value("hello"), Ok(("", "hello".to_string())));
        assert_eq!(value("héllo"), Ok(("", "héllo".to_string())));
        assert_eq!(value("hello world"), Ok(("", "hello world".to_string())));
        assert_eq!(value("héllo\tworld"), Ok(("", "héllo\tworld".to_string())));
    }
}
