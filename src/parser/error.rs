use nom;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ParseError {
    InvalidIdentifier,
    InvalidStringLiteral,
    InvalidEscapeSequence,
    InvalidStringPrefix,
    InvalidPunctuation,
    InvalidKeyword,
    InvalidStringLikeLiteral,
    InvalidString,
}

impl ParseError {
    pub fn into_nom(self) -> nom::ErrorKind<ParseError> {
        nom::ErrorKind::Custom(self)
    }

    pub fn from_nom(kind: &nom::ErrorKind<ParseError>) -> Option<&ParseError> {
        if let nom::ErrorKind::Custom(ref value) = *kind {
            Some(value)
        } else { None }
    }
}