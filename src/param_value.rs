use crate::{paramtext, quoted_string};
use nom::{IResult, Parser};

/// param-value   = paramtext / quoted-string
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn param_value(input: &str) -> IResult<&str, String> {
    // quoted-string must be checked before paramtext
    nom::branch::alt((quoted_string, paramtext)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
