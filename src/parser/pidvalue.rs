use nom::{IResult, Parser};

use crate::{model::PropertyValue, parser::text};

/// pidvalue   = text
/// ;Any text that describes the product and version
/// ;and that is generally assured of being unique.
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.7.3>
pub fn pidvalue(input: &str) -> IResult<&str, PropertyValue> {
    text.parse(input)
        .map(|(next_input, value)| (next_input, PropertyValue::Text(value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "-//ABC Corporation//NONSGML My Product//EN";
        assert_eq!(
            pidvalue(input),
            Ok(("", PropertyValue::Text(input.to_string())))
        );
    }
}
