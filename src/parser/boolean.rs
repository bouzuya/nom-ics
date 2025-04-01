use nom::{IResult, Parser};

use crate::model::Boolean;

/// boolean    = "TRUE" / "FALSE"
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.2>
pub fn boolean(input: &str) -> IResult<&str, Boolean> {
    nom::branch::alt((
        nom::bytes::complete::tag("TRUE").map(|_| Boolean::True),
        nom::bytes::complete::tag("FALSE").map(|_| Boolean::False),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Boolean;

    #[test]
    fn test_boolean() {
        let input = "TRUE";
        let expected = Ok(("", Boolean::True));
        assert_eq!(boolean(input), expected);

        let input = "FALSE";
        let expected = Ok(("", Boolean::False));
        assert_eq!(boolean(input), expected);

        let input = "INVALID";
        assert!(boolean(input).is_err());
    }
}
