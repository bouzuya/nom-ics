use nom::{IResult, Parser};

/// b-char = ALPHA / DIGIT / "+" / "/"
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.1>
pub fn b_char(input: &str) -> IResult<&str, char> {
    nom::character::complete::satisfy(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/')
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b_char() {
        assert_eq!(b_char("A"), Ok(("", 'A')));
        assert_eq!(b_char("a"), Ok(("", 'a')));
        assert_eq!(b_char("1"), Ok(("", '1')));
        assert_eq!(b_char("+"), Ok(("", '+')));
        assert_eq!(b_char("/"), Ok(("", '/')));

        assert!(b_char("=").is_err());
        assert!(b_char("!").is_err());
        assert!(b_char(" ").is_err());
        assert!(b_char("").is_err());
    }
}
