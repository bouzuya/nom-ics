use nom::{IResult, Parser};

use crate::model::Binary;
use crate::parser::{b_char, b_end};

/// binary     = *(4b-char) [b-end]
/// ; A "BASE64" encoded character string, as defined by [RFC4648].
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.1>
pub fn binary(input: &str) -> IResult<&str, Binary> {
    (
        nom::multi::many0(nom::multi::count(b_char, 4)),
        nom::combinator::opt(b_end),
    )
        .map(|(v, b_end)| {
            let mut s = String::new();
            for chunk in v {
                for c in chunk {
                    s.push(c);
                }
            }
            if let Some(b_end) = b_end {
                s.push_str(&b_end);
            }
            Binary(s)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Binary;

    #[test]
    fn test() {
        let input = "";
        let expected = Ok(("", Binary("".to_owned())));
        assert_eq!(binary(input), expected);

        let input = "abcd";
        let expected = Ok(("", Binary("abcd".to_owned())));
        assert_eq!(binary(input), expected);

        let input = "abcdabcd";
        let expected = Ok(("", Binary("abcdabcd".to_owned())));
        assert_eq!(binary(input), expected);

        let input = "abcdabcdabc=";
        let expected = Ok(("", Binary("abcdabcdabc=".to_owned())));
        assert_eq!(binary(input), expected);

        let input = "abcdabcdab==";
        let expected = Ok(("", Binary("abcdabcdab==".to_owned())));
        assert_eq!(binary(input), expected);
    }
}
