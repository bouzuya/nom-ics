use nom::{IResult, Parser};

/// date-fullyear      = 4DIGIT
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4>
pub fn date_fullyear(input: &str) -> IResult<&str, String> {
    nom::multi::count(nom::character::satisfy(|c| c.is_ascii_digit()), 4)
        .parse(input)
        .map(|(i, digits)| (i, digits.iter().collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_fullyear() {
        assert_eq!(date_fullyear("2023"), Ok(("", "2023".to_owned())));
        assert_eq!(date_fullyear("1970"), Ok(("", "1970".to_owned())));
        assert_eq!(date_fullyear("0000"), Ok(("", "0000".to_owned())));
        assert_eq!(date_fullyear("9999"), Ok(("", "9999".to_owned())));

        assert_eq!(date_fullyear("2023rest"), Ok(("rest", "2023".to_owned())));
        assert_eq!(
            date_fullyear("2023-10-15"),
            Ok(("-10-15", "2023".to_owned()))
        );

        let result = date_fullyear("");
        assert!(result.is_err());

        let result = date_fullyear("123");
        assert!(result.is_err());

        let result = date_fullyear("12345");
        assert_eq!(result, Ok(("5", "1234".to_owned())));

        let result = date_fullyear("abcd");
        assert!(result.is_err());

        let result = date_fullyear("20AB");
        assert!(result.is_err());
    }
}
