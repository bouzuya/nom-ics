use nom::{IResult, Parser};

/// WSP            =  SP / HTAB
/// SP             =  %x20
/// HTAB           =  %x09
///
/// <https://datatracker.ietf.org/doc/html/rfc5234>
pub fn wsp(input: &str) -> IResult<&str, char> {
    nom::branch::alt((
        nom::character::complete::char(' '),
        nom::character::complete::char('\t'),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wsp() {
        assert_eq!(wsp(" "), Ok(("", ' ')));
        assert_eq!(wsp("\t"), Ok(("", '\t')));
        assert!(wsp("a").is_err());
    }
}
