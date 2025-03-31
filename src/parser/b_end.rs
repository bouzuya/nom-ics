use nom::{IResult, Parser};

use crate::parser::b_char::b_char;

/// b-end      = (2b-char "==") / (3b-char "=")
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.1>
pub fn b_end(input: &str) -> IResult<&str, String> {
    nom::branch::alt((
        (
            nom::multi::count(b_char, 2),
            nom::bytes::complete::tag("=="),
        ),
        (nom::multi::count(b_char, 3), nom::bytes::complete::tag("=")),
    ))
    .map(|(v, b_end)| {
        let mut s = String::new();
        for c in v {
            s.push(c);
        }
        s.push_str(b_end);
        s
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "ab==";
        let expected = Ok(("", "ab==".to_owned()));
        assert_eq!(b_end(input), expected);

        let input = "abc=";
        let expected = Ok(("", "abc=".to_owned()));
        assert_eq!(b_end(input), expected);

        let input = "a==";
        assert!(b_end(input).is_err());
    }
}
