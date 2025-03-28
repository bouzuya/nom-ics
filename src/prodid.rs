use nom::{IResult, Parser};

use crate::pidvalue;

/// prodid     = "PRODID" pidparam ":" pidvalue CRLF
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3>
pub fn prodid(input: &str) -> IResult<&str, String> {
    (
        nom::bytes::complete::tag("PRODID"),
        // FIXME:
        nom::character::complete::char(':'),
        pidvalue,
        nom::character::complete::line_ending,
    )
        .map(|(_, _, pidvalue, _)| pidvalue)
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let input = "PRODID:-//Example Corp//NONSGML Example//EN\r\n";
        assert_eq!(
            prodid(input),
            Ok(("", "-//Example Corp//NONSGML Example//EN".to_string()))
        );
    }

    #[test]
    fn test_invalid() {
        let input = "INVALID:-//Example Corp//NONSGML Example//EN\r\n";
        let result = prodid(input);
        assert!(result.is_err());
    }
}
