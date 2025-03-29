use nom::{IResult, Parser};

use crate::parser::{non_us_ascii, wsp};

/// VALUE-CHAR    = WSP / %x21-7E / NON-US-ASCII
/// ; Any textual character
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn value_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        wsp,
        nom::character::complete::satisfy(|c| matches!(c, '\x21'..='\x7E')),
        non_us_ascii,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_char() {
        // WSP
        assert_eq!(value_char(" "), Ok(("", ' ')));
        assert_eq!(value_char("\t"), Ok(("", '\t')));

        // %x21-7E
        assert_eq!(value_char("!"), Ok(("", '!')));
        assert_eq!(value_char("~"), Ok(("", '~')));
        assert!(value_char("\x7F").is_err());

        // NON-US-ASCII
        assert_eq!(value_char("é"), Ok(("", 'é')));
        assert_eq!(value_char("ñ"), Ok(("", 'ñ')));
    }
}
