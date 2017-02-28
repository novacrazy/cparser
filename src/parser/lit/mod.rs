pub mod string;
pub mod constant;
pub mod integer;

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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum CharLiteral {
    Char(i8),
    UnsignedChar(u8),
    Unicode(char)
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
    Char(CharLiteral)
}
