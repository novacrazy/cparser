#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum FunctionSpecifier {
    Inline,
    NoReturn,
}

pub mod parsing {
    use nom::*;
    use ::parser::error::ParseError;
    use super::*;

    named!(pub function_specifier<FunctionSpecifier>, ws!(alt_complete!(
        tag!("inline")      => {|_| FunctionSpecifier::Inline   } |
        tag!("_Noreturn")   => {|_| FunctionSpecifier::NoReturn }
    )));
}