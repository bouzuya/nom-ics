use nom::{IResult, Parser};

/// ESCAPED-CHAR = ("\\" / "\;" / "\," / "\N" / "\n")
///    ; \\ encodes \, \N or \n encodes newline
///    ; \; encodes ;, \, encodes ,
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11>
pub fn escaped_char(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        nom::bytes::complete::tag("\\\\").map(|_| '\\'),
        nom::bytes::complete::tag("\\;").map(|_| ';'),
        nom::bytes::complete::tag("\\,").map(|_| ','),
        nom::bytes::complete::tag("\\N").map(|_| '\n'),
        nom::bytes::complete::tag("\\n").map(|_| '\n'),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escaped_char() {
        assert_eq!(escaped_char(r"\\"), Ok(("", '\\')));
        assert_eq!(escaped_char(r"\;"), Ok(("", ';')));
        assert_eq!(escaped_char(r"\,"), Ok(("", ',')));
        assert_eq!(escaped_char(r"\N"), Ok(("", '\n')));
        assert_eq!(escaped_char(r"\n"), Ok(("", '\n')));

        // Test invalid input
        assert!(escaped_char(r"\x").is_err());
        assert!(escaped_char(r"").is_err());
    }
}
