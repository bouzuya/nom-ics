use nom::{IResult, Parser};

/// vendorid      = 3*(ALPHA / DIGIT)
/// ; Vendor identification
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn vendorid(input: &str) -> IResult<&str, String> {
    nom::multi::count(
        nom::character::complete::satisfy(|c| c.is_ascii_alphanumeric()),
        3,
    )
    .map(|chars| chars.iter().collect::<String>())
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendorid() {
        assert_eq!(vendorid("ABC"), Ok(("", "ABC".to_string())));
        assert_eq!(vendorid("123"), Ok(("", "123".to_string())));
        assert_eq!(vendorid("A1B"), Ok(("", "A1B".to_string())));
        assert!(vendorid("AB").is_err());
        assert!(vendorid("AB!").is_err());
    }
}
