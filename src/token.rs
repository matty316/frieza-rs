// use crate::ast::Expr;
// use crate::parser::Parser;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Token {
    Eof,
    LParen, RParen, Colon, Comma, Semicolon,
    Let, If, Else, Fun, For, Return, End, True, False, Nil, Print, In,
    Ident(String), Int(i32), Float(f64), String(String),
    Eq, EqEq, Plus, Minus, Slash, Star, Bang, BangEq, Lt, LtEq, Gt, GtEq,
    And, Or,
    NewLine,
}
