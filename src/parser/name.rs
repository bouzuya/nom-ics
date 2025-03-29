use nom::{IResult, Parser};

use crate::parser::{iana_token, x_name};

/// name          = iana-token / x-name
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn name(input: &str) -> IResult<&str, String> {
    // x-name must be checked before iana-token
    nom::branch::alt((x_name, iana_token)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
