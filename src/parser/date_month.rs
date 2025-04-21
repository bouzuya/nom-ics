use nom::{IResult, Parser};

/// date-month         = 2DIGIT        ;01-12
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4>
pub fn date_month(input: &str) -> IResult<&str, String> {
    nom::multi::count(nom::character::satisfy(|c| c.is_ascii_digit()), 2)
        .parse(input)
        .map(|(i, digits)| (i, digits.iter().collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_month() {
        assert_eq!(date_month("01"), Ok(("", "01".to_owned())));
        assert_eq!(date_month("12"), Ok(("", "12".to_owned())));
        assert_eq!(date_month("06"), Ok(("", "06".to_owned())));
        assert_eq!(date_month("09"), Ok(("", "09".to_owned())));

        assert_eq!(date_month("02rest"), Ok(("rest", "02".to_owned())));
        assert_eq!(date_month("10-15"), Ok(("-15", "10".to_owned())));

        let result = date_month("");
        assert!(result.is_err());

        let result = date_month("1");
        assert!(result.is_err());

        let result = date_month("123");
        assert_eq!(result, Ok(("3", "12".to_owned())));

        let result = date_month("ab");
        assert!(result.is_err());

        let result = date_month("A2");
        assert!(result.is_err());
    }
}
