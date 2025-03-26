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
fn non_us_ascii(input: &str) -> IResult<&str, char> {
    // `char` primitive type is a single UTF-8 character
    nom::character::complete::satisfy(|c| !matches!(c, '\x00'..='\x7F')).parse(input)
}

/// QSAFE-CHAR    = WSP / %x21 / %x23-7E / NON-US-ASCII
/// ; Any character except CONTROL and DQUOTE
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn qsafe_char(input: &str) -> IResult<&str, char> {
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

/// quoted-string = DQUOTE *QSAFE-CHAR DQUOTE
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn quoted_string(input: &str) -> IResult<&str, String> {
    nom::sequence::delimited(
        nom::character::complete::char('"'),
        nom::multi::many0(qsafe_char),
        nom::character::complete::char('"'),
    )
    .map(|chars| chars.iter().collect::<String>())
    .parse(input)
}

/// SAFE-CHAR     = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-7E
///               / NON-US-ASCII
/// ; Any character except CONTROL, DQUOTE, ";", ":", ","
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn safe_char(input: &str) -> IResult<&str, char> {
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

/// value         = *VALUE-CHAR
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn value(input: &str) -> IResult<&str, String> {
    nom::multi::many0(value_char)
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
}

/// VALUE-CHAR    = WSP / %x21-7E / NON-US-ASCII
/// ; Any textual character
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn value_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        wsp,
        nom::character::complete::satisfy(|c| matches!(c, '\x21'..='\x7E')),
        non_us_ascii,
    ))
    .parse(input)
}

/// WSP            =  SP / HTAB
/// SP             =  %x20
/// HTAB           =  %x09
///
/// <https://datatracker.ietf.org/doc/html/rfc5234>
fn wsp(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        nom::character::complete::char(' '),
        nom::character::complete::char('\t'),
    ))
    .parse(input)
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

    #[test]
    fn test_value() {
        assert_eq!(value("hello"), Ok(("", "hello".to_string())));
        assert_eq!(value("héllo"), Ok(("", "héllo".to_string())));
        assert_eq!(value("hello world"), Ok(("", "hello world".to_string())));
        assert_eq!(value("héllo\tworld"), Ok(("", "héllo\tworld".to_string())));
    }

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

    #[test]
    fn test_wsp() {
        assert_eq!(wsp(" "), Ok(("", ' ')));
        assert_eq!(wsp("\t"), Ok(("", '\t')));
        assert!(wsp("a").is_err());
    }
}
