use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Eq, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new<T>(t: T) -> Self where T: Into<Ident> {
        t.into()
    }
}

impl<T> From<T> for Ident where T: Into<String> {
    fn from(ident: T) -> Ident {
        Ident(ident.into())
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Ident {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        self.0.fmt(formatter)
    }
}

impl<T: ? Sized> PartialEq<T> for Ident where T: AsRef<str>
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

pub mod parsing {
    use nom::*;
    use ::parser::error::ParseError;
    use super::Ident;

    named!(identifier_start, alt_complete!(
        tag!("_") | alpha
    ));

    named!(identifier_character, alt_complete!(
        identifier_start | digit
    ));

    named!(identifier_raw<Ident>, wse!(complete!(
        do_parse!(
            start: many1!(identifier_start)     >>
            rest:  many0!(identifier_character) >> ({
                start.into_iter().chain(rest.into_iter())
                    .map(|slice| String::from_utf8_lossy(slice))
                    .fold(String::new(), |acc, s| acc + s.as_ref())
                    .into()
            })
        )
    )));

    named!(pub identifier<&[u8], Ident, ParseError>, add_return_error!(
        ParseError::InvalidIdentifier.into_nom(),
        fix_error!(ParseError, identifier_raw)
    ));

    named!(pub identifier_list<&[u8], Vec<Ident>, ParseError>,
        separated_nonempty_list!(punct!(','), identifier));

    named!(pub typedef_name<&[u8], Ident, ParseError>, call!(identifier));
}