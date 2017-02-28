#![allow(unused_imports, dead_code)]

#[macro_use]
extern crate nom;

use nom::{IResult, ErrorKind};

pub mod parser;

fn test_idents() {
    let res = parser::ident::parsing::identifier_list(b"test4, test, testing,sdfsdf, sdf, sdf_sdf3");

    match res {
        IResult::Done(i, o) => {
            println!("Input:  {}", String::from_utf8_lossy(i));
            println!("Result: {:?}", o);
        },
        IResult::Error(err) => {
            for err in nom::error_to_list(&err) {
                if let Some(p_err) = self::parser::error::ParseError::from_nom(&err) {
                    println!("Error: {:?}", p_err);
                } else {
                    println!("Error: {:?}", err);
                }
            }
        },
        _ => println!("{:?}", res)
    }
}


fn test_strings() {
    let k = br#####" "fxdsdfs \u2154 d \"#   sdf" "#####;

    let res = parser::lit::parsing::string_literal(k);

    match res {
        IResult::Done(i, o) => {
            println!("Input:  {}", String::from_utf8_lossy(i));
            println!("Result: {:?}", o);
        },
        IResult::Error(err) => {
            for err in nom::error_to_list(&err) {
                if let Some(p_err) = self::parser::error::ParseError::from_nom(&err) {
                    println!("Error: {:?}", p_err);
                } else {
                    println!("Error: {:?}", err);
                }
            }
        },
        _ => println!("{:?}", res)
    }
}

fn main() {
    test_idents();
    test_strings();
}