use nom::{IResult, Parser};

/// NON-US-ASCII  = UTF8-2 / UTF8-3 / UTF8-4
/// ; UTF8-2, UTF8-3, and UTF8-4 are defined in [RFC3629]
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
///
/// UTF8-char   = UTF8-1 / UTF8-2 / UTF8-3 / UTF8-4
/// UTF8-1      = %x00-7F
/// UTF8-2      = %xC2-DF UTF8-tail
/// UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
///               %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
/// UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
///               %xF4 %x80-8F 2( UTF8-tail )
/// UTF8-tail   = %x80-BF
///
/// <https://datatracker.ietf.org/doc/html/rfc3629>
pub fn non_us_ascii(input: &str) -> IResult<&str, char> {
    nom::character::complete::satisfy(|c| !matches!(c, '\x00'..='\x7F')).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_us_ascii() {
        assert_eq!(non_us_ascii("é"), Ok(("", 'é')));
        assert_eq!(non_us_ascii("ñ"), Ok(("", 'ñ')));
        assert!(non_us_ascii("a").is_err());
        assert!(non_us_ascii("\x7F").is_err());
    }
}
