use nom::{IResult, Parser};

use crate::wsp;

/// TSAFE-CHAR = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-5B /
///              %x5D-7E / NON-US-ASCII
///    ; Any character except CONTROLs not needed by the current
///    ; character set, DQUOTE, ";", ":", "\", ","
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
pub fn tsafe_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        // except '\x00'..='\x08' | '\x0A'..='\x1F' (part of CONTROL)
        wsp,
        nom::character::complete::char('\x21'),
        // except '\x22' (DQUOTE)
        nom::character::complete::satisfy(|c| matches!(c, '\x23'..='\x2B')),
        // except '\x2C' (',')
        nom::character::complete::satisfy(|c| matches!(c, '\x2D'..='\x39')),
        // except '\x3A' (':')
        // except '\x3B' (';')
        nom::character::complete::satisfy(|c| matches!(c, '\x3C'..='\x5B')),
        // except '\x5C' ('\')
        nom::character::complete::satisfy(|c| matches!(c, '\x5D'..='\x7E')),
        // except '\x7F' (part of CONTROL)
        crate::non_us_ascii,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsafe_char() {
        assert_eq!(tsafe_char(" "), Ok(("", ' ')));
        assert_eq!(tsafe_char("\t"), Ok(("", '\t')));
        assert_eq!(tsafe_char("!"), Ok(("", '!')));
        assert_eq!(tsafe_char("#"), Ok(("", '#')));
        assert_eq!(tsafe_char("+"), Ok(("", '+')));
        assert_eq!(tsafe_char("-"), Ok(("", '-')));
        assert_eq!(tsafe_char("9"), Ok(("", '9')));
        assert_eq!(tsafe_char("<"), Ok(("", '<')));
        assert_eq!(tsafe_char("["), Ok(("", '[')));
        assert_eq!(tsafe_char("]"), Ok(("", ']')));
        assert_eq!(tsafe_char("~"), Ok(("", '~')));

        assert!(tsafe_char("\"").is_err());
        assert!(tsafe_char(",").is_err());
        assert!(tsafe_char(":").is_err());
        assert!(tsafe_char(";").is_err());
        assert!(tsafe_char("\\").is_err());
        assert!(tsafe_char("\x7F").is_err());
        assert!(tsafe_char("\x08").is_err());
    }
}
