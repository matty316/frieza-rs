use crate::token::Token;

pub(crate) enum Expr {
    Binary { op: Token, left: Box<Expr>, right: Box<Expr> },
    Unary { op: Token, right: Box<Expr> },
    Int { val: i32 }, Float { val: f64 }, String { val: String },
    Grouping { expr: Box<Expr> }
}

pub(crate) enum Stmt {
    Expression { expr: Box<Expr> }
}