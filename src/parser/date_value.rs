use nom::{IResult, Parser};

use crate::parser::{date_fullyear, date_mday, date_month};

/// date-value         = date-fullyear date-month date-mday
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4>
pub fn date_value(input: &str) -> IResult<&str, String> {
    (date_fullyear, date_month, date_mday)
        .map(|(fullyear, month, mday)| format!("{}{}{}", fullyear, month, mday))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_value() {
        assert_eq!(date_value("19970714"), Ok(("", "19970714".to_owned())));

        assert_eq!(date_value("20230101"), Ok(("", "20230101".to_owned())));
        assert_eq!(date_value("19991231"), Ok(("", "19991231".to_owned())));
        assert_eq!(date_value("20000229"), Ok(("", "20000229".to_owned())));

        assert_eq!(
            date_value("20230101rest"),
            Ok(("rest", "20230101".to_owned()))
        );
        assert_eq!(
            date_value("20230101T120000Z"),
            Ok(("T120000Z", "20230101".to_owned()))
        );

        let result = date_value("");
        assert!(result.is_err());

        let result = date_value("2023");
        assert!(result.is_err());

        let result = date_value("202301");
        assert!(result.is_err());

        let result = date_value("2023AB01");
        assert!(result.is_err());

        let result = date_value("202301AB");
        assert!(result.is_err());

        let result = date_value("ABCD0101");
        assert!(result.is_err());

        // TODO
        let result = date_value("20231301");
        assert_eq!(result, Ok(("", "20231301".to_owned())));

        // TODO
        let result = date_value("20230132");
        assert_eq!(result, Ok(("", "20230132".to_owned())));

        assert_eq!(date_value("20230101"), Ok(("", "20230101".to_owned())));
        assert_eq!(date_value("20231201"), Ok(("", "20231201".to_owned())));
        assert_eq!(date_value("20230401"), Ok(("", "20230401".to_owned())));
        assert_eq!(date_value("20230430"), Ok(("", "20230430".to_owned())));
        assert_eq!(date_value("20230131"), Ok(("", "20230131".to_owned())));
    }
}
