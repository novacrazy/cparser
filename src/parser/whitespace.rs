pub mod parsing {
    use nom::*;
    use ::parser::error::ParseError;

    named!(pub sp_parse_error<&[u8], &[u8], ParseError>, fix_error!(ParseError, sp));
}