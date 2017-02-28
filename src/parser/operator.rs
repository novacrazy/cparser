#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Operator {
    MUL,
    DIV,
    MOD,
    ADD,
    SUB,
    LEFT,
    RIGHT,
    AND,
    XOR,
    OR,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct AssignmentOperator(pub Option<Operator>);

pub mod parsing {
    use super::*;
    use nom::*;

    named!(pub operator<Operator>, ws!(alt_complete!(
        char!('*') => {|_| Operator::MUL   } |
        char!('/') => {|_| Operator::DIV   } |
        char!('%') => {|_| Operator::MOD   } |
        char!('+') => {|_| Operator::ADD   } |
        char!('-') => {|_| Operator::SUB   } |
        tag!("<<") => {|_| Operator::LEFT  } |
        tag!(">>") => {|_| Operator::RIGHT } |
        char!('&') => {|_| Operator::AND   } |
        char!('^') => {|_| Operator::XOR   } |
        char!('|') => {|_| Operator::OR    }
    )));

    named!(pub assignment<AssignmentOperator>, ws!(
        do_parse!(
            op: opt!(operator)  >>
            char!('=')          >>
            (AssignmentOperator(op))
        )
    ));
}