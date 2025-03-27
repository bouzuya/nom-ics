use nom::{IResult, Parser};

use crate::vendorid;

/// x-name        = "X-" [vendorid "-"] 1*(ALPHA / DIGIT / "-")
/// ; Reserved for experimental use.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn x_name(input: &str) -> IResult<&str, String> {
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
    fn test_x_name() {
        assert_eq!(x_name("X-TEST"), Ok(("", "X-TEST".to_string())));
        assert_eq!(x_name("X-VND-123"), Ok(("", "X-VND-123".to_string())));
        assert_eq!(x_name("X-123-ABC"), Ok(("", "X-123-ABC".to_string())));
        assert!(x_name("X-").is_err());
        assert!(x_name("!X-TEST").is_err());
        assert!(x_name("TEST").is_err());
    }
}
