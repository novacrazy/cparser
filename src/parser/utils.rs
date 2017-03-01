use std::convert::TryInto;

use ::parser::error::ParseError;

pub fn escaped_to_char<P>(c: char, iter: &mut ::std::str::Chars) -> Result<char, ::nom::Err<P, ParseError>> {
    Ok(match c {
        '\\' => '\x5C', // Backslash
        '\'' => '\x27', // Single quotation mark
        'a' => '\x07', // Alert (Beep, Bell) (added in C89)[1]
        'b' => '\x08', // Backspace
        'f' => '\x0C', // Formfeed
        'n' => '\x0A', // Newline (Line Feed); see notes below
        'r' => '\x0D', // Carriage Return
        't' => '\x09', // Horizontal Tab
        'v' => '\x0B', // Vertical Tab
        '"' => '\x22', // Double quotation mark
        '?' => '\x3F', // Question mark (used to avoid trigraphs)
        'e' => '\x1B', // escape character (some character sets)
        // Hex: \xABCD
        'x' => {
            let digits: String = iter.take_while(|c| c.is_digit(16)).collect();

            if digits.chars().count() == 0 {
                return Err(error_code!(ParseError::InvalidEscapeSequence.into_nom()));
            }

            let unicode_value: u32 = u32::from_str_radix(digits.as_str(), 16).unwrap();

            match unicode_value.try_into() {
                Ok(c) => c,
                Err(_) => {
                    return Err(error_code!(ParseError::InvalidUnicodeValueDetail(format!("\\{}{}", c, digits)).into_nom()));
                }
            }
        },
        // Unicode:
        // \u1234
        // \U12345678
        'u' | 'U' => {
            let expected_len = if c.is_lowercase() { 4 } else { 8 };

            let digits: String = iter.take_while(|c| c.is_digit(16)).take(expected_len).collect();

            if digits.chars().count() != expected_len {
                return Err(error_code!(ParseError::InvalidEscapeSequenceDetail(format!("\\{}{}", c, digits)).into_nom()));
            }

            let unicode_value: u32 = u32::from_str_radix(digits.as_str(), 16).unwrap();

            match unicode_value.try_into() {
                Ok(c) => c,
                Err(_) => {
                    return Err(error_code!(ParseError::InvalidUnicodeValueDetail(format!("\\{}{}", c, digits)).into_nom()));
                }
            }
        },
        // Octal: \123
        oct @ _ if oct.is_digit(8) => {
            let digits: String = ::std::iter::once(oct).chain(iter).take_while(|c| c.is_digit(8)).take(3).collect();

            let unicode_value: u32 = u32::from_str_radix(digits.as_str(), 8).unwrap();

            match unicode_value.try_into() {
                Ok(c) => c,
                Err(_) => {
                    return Err(error_code!(ParseError::InvalidUnicodeValueDetail(format!("\\{}{}", c, digits)).into_nom()));
                }
            }
        },
        _ => {
            return Err(error_code!(ParseError::InvalidEscapeSequence.into_nom()));
        }
    })
}

pub fn map_character<P>(bytes: &[u8]) -> Result<char, ::nom::Err<P, ParseError>> {
    let s = String::from_utf8_lossy(bytes);

    let mut chars = s.chars();

    if let Some(c) = chars.next() {
        Ok(match c {
            '\\' => {
                if let Some(c) = chars.next() {
                    match escaped_to_char(c, &mut chars) {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(error_node!(ParseError::InvalidEscapeSequence.into_nom(), e));
                        }
                    }
                } else {
                    return Err(error_code!(ParseError::InvalidEscapeSequence.into_nom()));
                }
            },
            _ => c,
        })
    } else {
        Err(error_code!(ParseError::InvalidEscapeSequence.into_nom()))
    }
}

pub fn map_characters<P>(bytes: &[u8]) -> Result<String, ::nom::Err<P, ParseError>> {
    let s = String::from_utf8_lossy(bytes);

    let mut st = String::new();

    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        let nc = match c {
            '\\' => {
                if let Some(c) = chars.next() {
                    match escaped_to_char(c, &mut chars) {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(error_node!(ParseError::InvalidEscapeSequence.into_nom(), e));
                        }
                    }
                } else {
                    return Err(error_code!(ParseError::InvalidEscapeSequence.into_nom()));
                }
            },
            _ => c,
        };

        st.push(nc);
    }

    Ok(st)
}