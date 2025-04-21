use nom::{IResult, Parser};

/// date-mday          = 2DIGIT        ;01-28, 01-29, 01-30, 01-31
///                                    ;based on month/year
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4>
pub fn date_mday(input: &str) -> IResult<&str, String> {
    nom::multi::count(nom::character::satisfy(|c| c.is_ascii_digit()), 2)
        .parse(input)
        .map(|(i, digits)| (i, digits.iter().collect::<String>()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_mday() {
        assert_eq!(date_mday("01"), Ok(("", "01".to_owned())));
        assert_eq!(date_mday("31"), Ok(("", "31".to_owned())));
        assert_eq!(date_mday("15"), Ok(("", "15".to_owned())));
        assert_eq!(date_mday("28"), Ok(("", "28".to_owned())));

        assert_eq!(date_mday("02rest"), Ok(("rest", "02".to_owned())));
        assert_eq!(date_mday("15T"), Ok(("T", "15".to_owned())));

        let result = date_mday("");
        assert!(result.is_err());

        let result = date_mday("1");
        assert!(result.is_err());

        let result = date_mday("123");
        assert_eq!(result, Ok(("3", "12".to_owned())));

        let result = date_mday("ab");
        assert!(result.is_err());

        let result = date_mday("A2");
        assert!(result.is_err());
    }
}
