use nom::{IResult, Parser};

use crate::{non_us_ascii, wsp};

/// SAFE-CHAR     = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-7E
///               / NON-US-ASCII
/// ; Any character except CONTROL, DQUOTE, ";", ":", ","
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn safe_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        // except \x00-\x08 / \x0A-\x1F / \x7F (=CONTROL)
        wsp,
        nom::character::complete::char('\x21'),
        // except \x22 (=DQUOTE)
        nom::character::complete::satisfy(|c| matches!(c, '\x23'..='\x2B')),
        // except \x2C (=",")
        nom::character::complete::satisfy(|c| matches!(c, '\x2D'..='\x39')),
        // except \x3A (=":")
        // except \x3B (=";")
        nom::character::complete::satisfy(|c| matches!(c, '\x3C'..='\x7E')),
        non_us_ascii,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_char() {
        // WSP
        assert_eq!(safe_char(" "), Ok(("", ' ')));
        assert_eq!(safe_char("\t"), Ok(("", '\t')));

        // %x21
        assert_eq!(safe_char("!"), Ok(("", '!')));

        // %x23-2B
        assert_eq!(safe_char("#"), Ok(("", '#')));
        assert_eq!(safe_char("+"), Ok(("", '+')));
        assert!(safe_char(",").is_err()); // %x2C is excluded

        // %x2D-39
        assert_eq!(safe_char("-"), Ok(("", '-')));
        assert_eq!(safe_char("9"), Ok(("", '9')));
        assert!(safe_char(":").is_err()); // %x3A is excluded

        // %x3C-7E
        assert_eq!(safe_char("<"), Ok(("", '<')));
        assert_eq!(safe_char("~"), Ok(("", '~')));
        assert!(safe_char(";").is_err()); // %x3B is excluded

        // NON-US-ASCII
        assert_eq!(safe_char("é"), Ok(("", 'é')));
        assert_eq!(safe_char("ñ"), Ok(("", 'ñ')));
    }
}
