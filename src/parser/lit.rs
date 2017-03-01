use super::ident::Ident;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum SignedIntegerValue {
    Short(i32),
    Int(i32),
    Long(i64),
    LongLong(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum UnsignedIntegerValue {
    Short(u32),
    Int(u32),
    Long(u64),
    LongLong(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum IntegerLiteral {
    Signed(SignedIntegerValue),
    Unsigned(UnsignedIntegerValue),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FloatLiteral {
    F32(f32),
    F64(f64),
    Arbitrary(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Str {
        value: String,
        unicode: bool,
        wide: bool,
    },
    StringLike(Ident),
    Integer(IntegerLiteral),
    IntegerLike(Ident),
    Float(FloatLiteral),
    Char(char)
}

pub mod parsing {
    use nom::*;

    use ::parser::error::ParseError;
    use ::parser::ident::Ident;
    use ::parser::lit::*;
    use ::parser::utils::{map_character, map_characters};

    named!(
        #[doc = "Matches string-like literals and consumes whitespace"],
        pub string_like_literal<&[u8], Lit, ParseError>,
        add_return_error!(
            ParseError::InvalidStringLikeLiteral.into_nom(),
            wse!(alt_complete!(
                keyword!("__func__")        => { |_| Lit::StringLike(Ident::from("__func__"))       } |
                keyword!("__FUNCTION__")    => { |_| Lit::StringLike(Ident::from("__FUNCTION__"))   } |
                keyword!("__DATE__")        => { |_| Lit::StringLike(Ident::from("__DATE__"))       } |
                keyword!("__TIME__")        => { |_| Lit::StringLike(Ident::from("__TIME__"))       } |
                keyword!("__FILE__")        => { |_| Lit::StringLike(Ident::from("__FILE__"))       }
            ))
        )
    );

    named!(
        #[doc = "Matches integer-like literals and consumes whitespace"],
        pub integer_like_literal<&[u8], Lit, ParseError>,
        add_return_error!(
            ParseError::InvalidIntegerLikeLiteral.into_nom(),
            wse!(alt_complete!(
                keyword!("__LINE__")            => { |_| Lit::IntegerLike(Ident::from("__LINE__"))           } |
                keyword!("__STDC__")            => { |_| Lit::IntegerLike(Ident::from("__STDC__"))           } |
                keyword!("__STDC_VERSION__")    => { |_| Lit::IntegerLike(Ident::from("__STDC_VERSION__"))   } |
                keyword!("__STDC_HOSTED__")     => { |_| Lit::IntegerLike(Ident::from("__STDC_HOSTED__"))    } |
                keyword!("__cplusplus")         => { |_| Lit::IntegerLike(Ident::from("__cplusplus"))        } |
                keyword!("__OBJC__")            => { |_| Lit::IntegerLike(Ident::from("__OBJC__"))           } |
                keyword!("__ASSEMBLER__")       => { |_| Lit::IntegerLike(Ident::from("__ASSEMBLER__"))      }
            ))
        )
    );

    #[derive(Debug, Clone, Copy, PartialEq, Hash)]
    enum StringPrefix {
        Unicode,
        Wide
    }

    // String literal prefixes
    named!(string_literal_prefix<&[u8], StringPrefix, ParseError>, add_return_error!(
        ParseError::InvalidStringPrefix.into_nom(),
        wse!(alt!(
            tag!("u8") => {|_| StringPrefix::Unicode } |
            char!('u') => {|_| StringPrefix::Unicode } |
            char!('l') => {|_| StringPrefix::Wide    } |
            char!('L') => {|_| StringPrefix::Wide    }
        ))
    ));

    // Any valid non-escaped string literal character
    named!(raw_string_character <&[u8], &[u8]>, recognize!(none_of!("\\\n\"")));
    named!(raw_char_character   <&[u8], &[u8]>, recognize!(none_of!("\\\n'")));

    named!(escaped_char         <&[u8], &[u8]>, recognize!(one_of!("\"abfner0tv'?\\")));
    named!(escaped_oct_digit    <&[u8], &[u8]>, recognize!(many_m_n!(1, 3, oct_digit)));
    named!(escaped_hex_digit    <&[u8], &[u8]>, recognize!(preceded!(char!('x'), many1!(hex_digit))));
    named!(escaped_unicode      <&[u8], &[u8]>, recognize!(preceded!(one_of!("uU"), many1!(hex_digit))));

    // Any valid escaped character
    named!(escaped_character <&[u8], &[u8]>, alt_complete!(
        escaped_char      |
        escaped_oct_digit |
        escaped_hex_digit |
        escaped_unicode
    ));

    // Normal and escaped string characters
    named!(escaped_string_bytes,
        escaped!(raw_string_character, '\\', escaped_character));

    named!(escaped_char_bytes,
        escaped!(raw_char_character, '\\', escaped_character));

    // Normal and escaped string characters, with correct error value
    named!(string_character<&[u8], String, ParseError>, add_return_error!(
        ParseError::InvalidEscapeSequence.into_nom(),
        map_res!(
            fix_error!(ParseError, escaped_string_bytes),
            map_characters
        )
    ));

    named!(char_character<&[u8], char, ParseError>, add_return_error!(
        ParseError::InvalidEscapeSequence.into_nom(),
        map_res!(
            fix_error!(ParseError, escaped_char_bytes),
            map_character
        )
    ));

    // One of more internal string characters
    named!(string_characters<&[u8], Vec<String>, ParseError>,
        many1!(string_character));

    // Zero or more string prefixes
    named!(raw_string_prefix<&[u8], Vec<StringPrefix>, ParseError>,
        many1!(string_literal_prefix));

    // String literal internals delimited by " characters
    named!(raw_delimited_string_literal<&[u8], Vec<String>, ParseError>, complete!(delimited!(
        // If this doesn't match, we missed an invalid prefix
        // E.g., h"sdf" will skip the h and pass h" to this, so it fails
        add_return_error!(
            ParseError::InvalidStringPrefix.into_nom(),
            punct!('"')
        ),
        string_characters,
        punct!('"'))
    ));

    named!(raw_delimited_char_literal<&[u8], char, ParseError>, complete!(delimited!(
        punct!('\''),
        char_character,
        punct!('\'')
    )));

    named!(raw_string_literal<&[u8], Lit, ParseError>, add_return_error!(
        ParseError::InvalidStringLiteral.into_nom(),
        do_parse!(
            prefix: opt!(raw_string_prefix)      >>
            value:  raw_delimited_string_literal >> ({
                let value = value.into_iter().fold(String::new(), |mut acc, s| {
                    acc += s.as_str();
                    acc
                });

                if let Some(prefix) = prefix {
                    Lit::Str {
                        value: value,
                        unicode: prefix.contains(&StringPrefix::Unicode),
                        wide: prefix.contains(&StringPrefix::Wide),
                    }
                } else {
                    Lit::Str {value: value, unicode: false, wide: false}
                }
            })
        )
    ));

    named!(raw_char_literal<&[u8], Lit, ParseError>, add_return_error!(
        ParseError::InvalidCharacterLiteral.into_nom(),
        map!(raw_delimited_char_literal, |character| {
            Lit::Char(character)
        })
    ));

    named!(
        #[doc = "Matches a string literal and consumes whitespace"],
        pub string_literal<&[u8], Lit, ParseError>,
        wse!(raw_string_literal)
    );

    named!(
        #[doc = "Matches a character literal and consumes whitespace"],
        pub char_literal<&[u8], Lit, ParseError>,
        wse!(raw_char_literal)
    );

    named!(
        #[doc = "Matches a string literals and string-like literals"],
        pub string<&[u8], Lit, ParseError>,
        wse!(add_return_error!(
            ParseError::InvalidString.into_nom(),
            alt!(
                string_like_literal |
                raw_string_literal
            )
        ))
    );
}