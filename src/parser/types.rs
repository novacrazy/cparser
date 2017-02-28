#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Bool,
    Complex,
    Atomic(Box<TypeSpecifier>),
    Struct,
    Enum,
    Other(Box<TypeSpecifier>)
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile,
    Atomic,
}

pub mod parsing {
    use nom::*;
    use super::*;

    named!(type_qualifier<TypeQualifier>, ws!(alt!(
        tag!("const")       => {|_| TypeQualifier::Const   } |
        tag!("restrict")    => {|_| TypeQualifier::Restrict} |
        tag!("volatile")    => {|_| TypeQualifier::Volatile} |
        tag!("_Atomic")     => {|_| TypeQualifier::Atomic  }
    )));
}