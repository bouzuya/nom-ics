use nom::{IResult, Parser};

use crate::{
    model::Text,
    parser::{escaped_char, tsafe_char},
};

/// text       = *(TSAFE-CHAR / ":" / DQUOTE / ESCAPED-CHAR)
/// ; Folded according to description above
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
pub fn text(input: &str) -> IResult<&str, Text> {
    nom::multi::many0(nom::branch::alt((
        tsafe_char.map(|c| c.to_string()),
        nom::character::complete::char(':').map(|c| c.to_string()),
        nom::character::complete::char('"').map(|c| c.to_string()),
        escaped_char.map(|s| s.to_owned()),
    )))
    .map(|v| Text(v.join("")))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_with_tsafe_char() {
        let input = "abc";
        let expected = Ok(("", Text("abc".to_owned())));
        assert_eq!(text(input), expected);
    }

    #[test]
    fn test_text_with_colon() {
        let input = ":";
        let expected = Ok(("", Text(":".to_owned())));
        assert_eq!(text(input), expected);
    }

    #[test]
    fn test_text_with_double_quote() {
        let input = "\"";
        let expected = Ok(("", Text("\"".to_owned())));
        assert_eq!(text(input), expected);
    }

    #[test]
    fn test_text_with_escaped_char() {
        let input = "\\n";
        let expected = Ok(("", Text("\\n".to_owned())));
        assert_eq!(text(input), expected);
    }

    #[test]
    fn test_text_with_mixed_input() {
        let input = "a:b\"\\n";
        let expected = Ok(("", Text("a:b\"\\n".to_owned())));
        assert_eq!(text(input), expected);
    }
}
