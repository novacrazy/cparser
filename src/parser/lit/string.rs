pub mod parsing {
    use nom::*;

    use ::parser::error::ParseError;
    use ::parser::ident::Ident;
    use ::parser::lit::Lit;

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
    named!(character         <&[u8], &[u8]>, recognize!(none_of!("\\\n\"")));

    named!(escaped_char      <&[u8], &[u8]>, recognize!(one_of!("\"abfnrtv'?\\")));
    named!(escaped_oct_digit <&[u8], &[u8]>, recognize!(many_m_n!(1, 3, oct_digit)));
    named!(escaped_hex_digit <&[u8], &[u8]>, recognize!(preceded!(char!('x'), many1!(hex_digit))));
    named!(escaped_unicode   <&[u8], &[u8]>, recognize!(preceded!(one_of!("uU"), many1!(hex_digit))));

    // Any valid escaped character
    named!(escaped_character <&[u8], &[u8]>, alt_complete!(
        escaped_char      |
        escaped_oct_digit |
        escaped_hex_digit |
        escaped_unicode
    ));

    // Normal and escaped string characters
    named!(character_sequence,
        escaped!(character, '\\', escaped_character));

    // Normal and escaped string characters, with correct error value
    named!(string_character<&[u8], &[u8], ParseError>, add_return_error!(
        ParseError::InvalidEscapeSequence.into_nom(),
        fix_error!(ParseError, character_sequence)
    ));

    // One of more internal string characters
    named!(string_internals<&[u8], Vec<&[u8]>, ParseError>,
        many1!(string_character));

    // Zero or more string prefixes
    named!(raw_string_prefix<&[u8], Vec<StringPrefix>, ParseError>,
        many1!(string_literal_prefix));

    // String literal internals delimited by " characters
    named!(raw_delimited_string_literal<&[u8], Vec<&[u8]>, ParseError>, complete!(delimited!(
        // If this doesn't match, we missed an invalid prefix
        // E.g., h"sdf" will skip the h and pass h" to this, so it fails
        add_return_error!(
            ParseError::InvalidStringPrefix.into_nom(),
            punct!('"')
        ),
        string_internals,
        punct!('"'))
    ));

    named!(raw_string_literal<&[u8], Lit, ParseError>, add_return_error!(
        ParseError::InvalidStringLiteral.into_nom(),
        do_parse!(
            prefix: opt!(raw_string_prefix)      >>
            value:  raw_delimited_string_literal >> ({
                let value = value.into_iter()
                                 .map(String::from_utf8_lossy)
                                 .fold(String::new(), |acc, s| acc + s.as_ref());

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

    named!(
        #[doc = "Matches a string literal and consumes whitespace"],
        pub string_literal<&[u8], Lit, ParseError>,
        wse!(raw_string_literal)
    );

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