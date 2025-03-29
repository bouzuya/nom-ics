use nom::{IResult, Parser};

use crate::parser::{non_us_ascii, wsp};

/// QSAFE-CHAR    = WSP / %x21 / %x23-7E / NON-US-ASCII
/// ; Any character except CONTROL and DQUOTE
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn qsafe_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        // except CONTROL
        // except \x00-\x08 / \x0A-\x1F / \x7F (=CONTROL)
        wsp,
        nom::character::complete::char('\x21'),
        // except \x22 (=DQUOTE)
        nom::character::complete::satisfy(|c| matches!(c, '\x23'..='\x7E')),
        non_us_ascii,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qsafe_char() {
        // WSP
        assert_eq!(qsafe_char(" "), Ok(("", ' ')));
        assert_eq!(qsafe_char("\t"), Ok(("", '\t')));

        // %x21
        assert_eq!(qsafe_char("!"), Ok(("", '!')));

        // %x23-7E
        assert_eq!(qsafe_char("#"), Ok(("", '#')));
        assert_eq!(qsafe_char("~"), Ok(("", '~')));
        assert!(qsafe_char("\"").is_err()); // %x22 (DQUOTE) is excluded

        // NON-US-ASCII
        assert_eq!(qsafe_char("é"), Ok(("", 'é')));
        assert_eq!(qsafe_char("ñ"), Ok(("", 'ñ')));
    }
}
