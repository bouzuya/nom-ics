use nom::{IResult, Parser};

use crate::{model::Property, parser::prodid};

/// calprops   = *(
///               ;
///               ; The following are REQUIRED,
///               ; but MUST NOT occur more than once.
///               ;
///               prodid / version /
///               ;
///               ; The following are OPTIONAL,
///               ; but MUST NOT occur more than once.
///               ;
///               calscale / method /
///               ;
///               ; The following are OPTIONAL,
///               ; and MAY occur more than once.
///               ;
///               x-prop / iana-prop
///               ;
///               )
///
/// <https://datatracker.ietf.org/doc/html/rfc5545#section-3.6>
pub fn calprops(input: &str) -> IResult<&str, Vec<Property>> {
    // FIXME
    nom::multi::many0(prodid).parse(input)
}

// TODO: tests
