use nom::{IResult, Parser};

use crate::parser::{param_name, param_value};

/// param         = param-name "=" param-value *("," param-value)
/// ; Each property defines the specific ABNF for the parameters
/// ; allowed on the property.  Refer to specific properties for
/// ; precise parameter ABNF.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545>
pub fn param(input: &str) -> IResult<&str, (String, Vec<String>)> {
    (
        param_name,
        nom::character::complete::char('='),
        param_value,
        nom::multi::many0((nom::character::complete::char(','), param_value)),
    )
        .map(|(name, _, value, param_values)| {
            (
                name,
                std::iter::once(value)
                    .chain(param_values.into_iter().map(|(_, param_value)| param_value))
                    .collect::<Vec<String>>(),
            )
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param() {
        assert_eq!(
            param("NAME=value1,value2,value3"),
            Ok((
                "",
                (
                    "NAME".to_string(),
                    vec![
                        "value1".to_string(),
                        "value2".to_string(),
                        "value3".to_string()
                    ]
                )
            ))
        );
        assert_eq!(
            param("NAME=value1"),
            Ok(("", ("NAME".to_string(), vec!["value1".to_string()])))
        );
        assert_eq!(
            param("NAME="),
            Ok(("", ("NAME".to_string(), vec!["".to_string()])))
        );
        assert!(param("NAME").is_err());
        assert!(param("=value1").is_err());
    }
}
