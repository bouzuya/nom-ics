use crate::safe_char;
use nom::{IResult, Parser};

/// paramtext     = *SAFE-CHAR
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn paramtext(input: &str) -> IResult<&str, String> {
    nom::multi::many0(safe_char)
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paramtext() {
        assert_eq!(paramtext("value"), Ok(("", "value".to_string())));
        assert_eq!(paramtext("héllo"), Ok(("", "héllo".to_string())));
        assert_eq!(paramtext("value123"), Ok(("", "value123".to_string())));
        assert_eq!(paramtext(""), Ok(("", "".to_string())));
        assert_eq!(paramtext(";invalid"), Ok((";invalid", "".to_string())));
    }
}
