use super::ident::Ident;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Enumerator;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Enum {
    ident: Ident,
    enumerators: Vec<Enumerator>,
}