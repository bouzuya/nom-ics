use nom::{IResult, Parser};

use crate::parser::{name, param, value};

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
pub fn contentline(input: &str) -> IResult<&str, (String, Vec<(String, Vec<String>)>, String)> {
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
}
