use nom::{IResult, Parser};

use crate::parser::qsafe_char;

/// quoted-string = DQUOTE *QSAFE-CHAR DQUOTE
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn quoted_string(input: &str) -> IResult<&str, String> {
    nom::sequence::delimited(
        nom::character::complete::char('"'),
        nom::multi::many0(qsafe_char),
        nom::character::complete::char('"'),
    )
    .map(|chars| chars.iter().collect::<String>())
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quoted_string() {
        assert_eq!(quoted_string("\"hello\""), Ok(("", "hello".to_string())));
        assert_eq!(quoted_string("\"héllo\""), Ok(("", "héllo".to_string())));
        assert_eq!(quoted_string("\"\""), Ok(("", "".to_string())));
        assert!(quoted_string("\"hello").is_err());
        assert!(quoted_string("hello\"").is_err());
        assert_eq!(
            quoted_string("\"he\"llo\""),
            Ok(("llo\"", "he".to_string()))
        );
    }
}
