use std::fmt::{Display, Formatter, Result as FmtResult};

use super::ident::Ident;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Field {
    ident: Ident,
    ty: ()
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct VariantData {
    ident: Ident,
    fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Variant {
    Struct(VariantData),
    Union(VariantData),
}

pub mod parsing {
    use nom::*;
    use super::*;

    use ::parser::ident::parsing::*;
}