use nom::{IResult, Parser};

/// iana-token    = 1*(ALPHA / DIGIT / "-")
/// ; iCalendar identifier registered with IANA
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn iana_token(input: &str) -> IResult<&str, String> {
    nom::multi::many1(nom::character::complete::satisfy(|c| {
        c.is_ascii_alphanumeric() || c == '-'
    }))
    .map(|chars| chars.iter().collect::<String>())
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iana_token() {
        assert_eq!(iana_token("CALENDAR"), Ok(("", "CALENDAR".to_string())));
        assert_eq!(iana_token("123-456"), Ok(("", "123-456".to_string())));
        assert_eq!(iana_token("CAL-123"), Ok(("", "CAL-123".to_string())));
        assert!(iana_token("").is_err());
        assert!(iana_token("!CALENDAR").is_err()); // Invalid character
    }
}
