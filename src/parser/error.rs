use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

use nom;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ParseError {
    InvalidIdentifier,
    InvalidString,
    InvalidStringLiteral,
    InvalidEscapeSequence,
    InvalidEscapeSequenceDetail(String),
    InvalidUnicodeValue,
    InvalidUnicodeValueDetail(String),
    InvalidStringPrefix,
    InvalidPunctuation(char),
    InvalidKeyword(&'static str),
    InvalidStringLikeLiteral,
    InvalidIntegerLikeLiteral,
    InvalidCharacterLiteral,
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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ParseError::InvalidKeyword(expected) => {
                write!(f, r#"InvalidKeyword - Expected "{}""#, expected)
            },
            ParseError::InvalidPunctuation(expected) => {
                write!(f, "InvalidPunctuation - Expected {}", expected)
            },
            _ => Debug::fmt(self, f)
        }
    }
}