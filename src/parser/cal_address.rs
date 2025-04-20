use nom::IResult;

use crate::model::CalendarUserAddress;

/// cal-address        = uri
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3>
pub fn cal_address(input: &str) -> IResult<&str, CalendarUserAddress> {
    let (i, t) =
        nom_uri::uri(nom_locate::LocatedSpan::new(input)).map_err(|e| e.map_input(|_| input))?;
    Ok((
        i.fragment(),
        CalendarUserAddress(t.span.fragment().to_string()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "";
        assert!(cal_address(input).is_err());

        let input = "mailto:jane_doe@example.com";
        let expected = Ok((
            "",
            CalendarUserAddress("mailto:jane_doe@example.com".to_owned()),
        ));
        assert_eq!(cal_address(input), expected);
    }
}
