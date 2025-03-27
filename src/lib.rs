use nom::{IResult, Parser};

/// contentline   = name *(";" param ) ":" value CRLF
/// ; This ABNF is just a general definition for an initial parsing
/// ; of the content line into its property name, parameter list,
/// ; and value string
/// ; When parsing a content line, folded lines MUST first
/// ; be unfolded according to the unfolding procedure
/// ; described above.  When generating a content line, lines
/// ; longer than 75 octets SHOULD be folded according to
/// ; the folding procedure described above.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn contentline(input: &str) -> IResult<&str, (String, Vec<(String, Vec<String>)>, String)> {
    (
        name,
        nom::multi::many0((nom::character::complete::char(';'), param)),
        nom::character::complete::char(':'),
        value,
        nom::character::complete::line_ending,
    )
        .map(|(name, params, _, value, _)| {
            (
                name,
                params
                    .into_iter()
                    .map(|(_, param)| param)
                    .collect::<Vec<(String, Vec<String>)>>(),
                value,
            )
        })
        .parse(input)
}

/// iana-token    = 1*(ALPHA / DIGIT / "-")
/// ; iCalendar identifier registered with IANA
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn iana_token(input: &str) -> IResult<&str, String> {
    nom::multi::many1(nom::character::complete::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '-'
    }))
    .map(|chars| chars.iter().collect::<String>())
    .parse(input)
}

/// name          = iana-token / x-name
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn name(input: &str) -> IResult<&str, String> {
    // x-name must be checked before iana-token
    nom::branch::alt((x_name, iana_token)).parse(input)
}

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

/// param         = param-name "=" param-value *("," param-value)
/// ; Each property defines the specific ABNF for the parameters
/// ; allowed on the property.  Refer to specific properties for
/// ; precise parameter ABNF.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn param(input: &str) -> IResult<&str, (String, Vec<String>)> {
    (
        param_name,
        nom::character::complete::char('='),
        param_value,
        nom::multi::many0((nom::character::complete::char(','), param_value)),
    )
        .map(|(name, _, value, param_values)| {
            (
                name,
                std::iter::once(value)
                    .chain(param_values.into_iter().map(|(_, param_value)| param_value))
                    .collect::<Vec<String>>(),
            )
        })
        .parse(input)
}

/// param-name    = iana-token / x-name
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn param_name(input: &str) -> IResult<&str, String> {
    // x-name must be checked before iana-token
    nom::branch::alt((x_name, iana_token)).parse(input)
}

/// param-value   = paramtext / quoted-string
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn param_value(input: &str) -> IResult<&str, String> {
    // quoted-string must be checked before paramtext
    nom::branch::alt((quoted_string, paramtext)).parse(input)
}

/// paramtext     = *SAFE-CHAR
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn paramtext(input: &str) -> IResult<&str, String> {
    nom::multi::many0(safe_char)
        .map(|chars| chars.iter().collect::<String>())
        .parse(input)
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

/// vendorid      = 3*(ALPHA / DIGIT)
/// ; Vendor identification
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn vendorid(input: &str) -> IResult<&str, String> {
    nom::multi::count(
        nom::character::complete::satisfy(|c| c.is_ascii_alphanumeric()),
        3,
    )
    .map(|chars| chars.iter().collect::<String>())
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

/// x-name        = "X-" [vendorid "-"] 1*(ALPHA / DIGIT / "-")
/// ; Reserved for experimental use.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
fn x_name(input: &str) -> IResult<&str, String> {
    (
        nom::bytes::tag("X-"),
        nom::combinator::opt((vendorid, nom::character::complete::char('-'))),
        nom::multi::many1(nom::character::complete::satisfy(|c| {
            c.is_ascii_alphanumeric() || c == '-'
        })),
    )
        .map(|(x_, vendorid, chars)| {
            vec![
                x_.to_owned(),
                vendorid
                    .map(|(v, h)| format!("{}{}", v, h))
                    .unwrap_or_default(),
                chars.iter().collect::<String>(),
            ]
            .join("")
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contentline() {
        assert_eq!(
            contentline("NAME;PARAM=value:VALUE\r\n"),
            Ok((
                "",
                (
                    "NAME".to_string(),
                    vec![("PARAM".to_string(), vec!["value".to_string()])],
                    "VALUE".to_string()
                )
            ))
        );
        assert_eq!(
            contentline("NAME:VALUE\r\n"),
            Ok(("", ("NAME".to_string(), vec![], "VALUE".to_string())))
        );
        assert!(contentline("NAME;PARAM=value:VALUE").is_err()); // Missing CRLF
        assert!(contentline("NAME;PARAM=value").is_err()); // Missing ':' and CRLF
    }

    #[test]
    fn test_iana_token() {
        assert_eq!(iana_token("CALENDAR"), Ok(("", "CALENDAR".to_string())));
        assert_eq!(iana_token("123-456"), Ok(("", "123-456".to_string())));
        assert_eq!(iana_token("CAL-123"), Ok(("", "CAL-123".to_string())));
        assert!(iana_token("").is_err());
        assert!(iana_token("!CALENDAR").is_err()); // Invalid character
    }

    #[test]
    fn test_name() {
        // iana-token
        assert_eq!(name("CALENDAR"), Ok(("", "CALENDAR".to_string())));
        assert_eq!(name("123-456"), Ok(("", "123-456".to_string())));

        // x-name
        assert_eq!(name("X-TEST"), Ok(("", "X-TEST".to_string())));
        assert_eq!(name("X-VND-123"), Ok(("", "X-VND-123".to_string())));

        assert!(name("").is_err());
        assert!(name("!CALENDAR").is_err());

        // X- is an iana-token
        assert_eq!(name("X-"), Ok(("", "X-".to_string())));
    }

    #[test]
    fn test_non_us_ascii() {
        assert_eq!(non_us_ascii("é"), Ok(("", 'é')));
        assert_eq!(non_us_ascii("ñ"), Ok(("", 'ñ')));
        assert!(non_us_ascii("a").is_err());
        assert!(non_us_ascii("\x7F").is_err());
    }

    #[test]
    fn test_param() {
        assert_eq!(
            param("NAME=value1,value2,value3"),
            Ok((
                "",
                (
                    "NAME".to_string(),
                    vec![
                        "value1".to_string(),
                        "value2".to_string(),
                        "value3".to_string()
                    ]
                )
            ))
        );
        assert_eq!(
            param("NAME=value1"),
            Ok(("", ("NAME".to_string(), vec!["value1".to_string()])))
        );
        assert_eq!(
            param("NAME="),
            Ok(("", ("NAME".to_string(), vec!["".to_string()])))
        );
        assert!(param("NAME").is_err());
        assert!(param("=value1").is_err());
    }

    #[test]
    fn test_param_name() {
        // iana-token
        assert_eq!(param_name("CALENDAR"), Ok(("", "CALENDAR".to_string())));
        assert_eq!(param_name("123-456"), Ok(("", "123-456".to_string())));

        // x-name
        assert_eq!(param_name("X-TEST"), Ok(("", "X-TEST".to_string())));
        assert_eq!(param_name("X-VND-123"), Ok(("", "X-VND-123".to_string())));

        assert!(param_name("").is_err());
        assert!(param_name("!CALENDAR").is_err());

        // X- is an iana-token
        assert_eq!(param_name("X-"), Ok(("", "X-".to_string())));
    }

    #[test]
    fn test_param_value() {
        // paramtext
        assert_eq!(param_value("value"), Ok(("", "value".to_string())));
        assert_eq!(param_value("héllo"), Ok(("", "héllo".to_string())));

        // quoted-string
        assert_eq!(param_value("\"quoted\""), Ok(("", "quoted".to_string())));
        assert_eq!(param_value("\"héllo\""), Ok(("", "héllo".to_string())));

        assert_eq!(param_value(""), Ok(("", "".to_string())));
        assert_eq!(
            param_value("\"unterminated"),
            Ok(("\"unterminated", "".to_string()))
        );
    }

    #[test]
    fn test_paramtext() {
        assert_eq!(paramtext("value"), Ok(("", "value".to_string())));
        assert_eq!(paramtext("héllo"), Ok(("", "héllo".to_string())));
        assert_eq!(paramtext("value123"), Ok(("", "value123".to_string())));
        assert_eq!(paramtext(""), Ok(("", "".to_string())));
        assert_eq!(paramtext(";invalid"), Ok((";invalid", "".to_string())));
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
    fn test_vendorid() {
        assert_eq!(vendorid("ABC"), Ok(("", "ABC".to_string())));
        assert_eq!(vendorid("123"), Ok(("", "123".to_string())));
        assert_eq!(vendorid("A1B"), Ok(("", "A1B".to_string())));
        assert!(vendorid("AB").is_err());
        assert!(vendorid("AB!").is_err());
    }

    #[test]
    fn test_wsp() {
        assert_eq!(wsp(" "), Ok(("", ' ')));
        assert_eq!(wsp("\t"), Ok(("", '\t')));
        assert!(wsp("a").is_err());
    }

    #[test]
    fn test_x_name() {
        assert_eq!(x_name("X-TEST"), Ok(("", "X-TEST".to_string())));
        assert_eq!(x_name("X-VND-123"), Ok(("", "X-VND-123".to_string())));
        assert_eq!(x_name("X-123-ABC"), Ok(("", "X-123-ABC".to_string())));
        assert!(x_name("X-").is_err());
        assert!(x_name("!X-TEST").is_err());
        assert!(x_name("TEST").is_err());
    }
}
