use crate::token::Token;
use crate::visitor::{ExprVisitor, StmtVisitor};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expr {
    Binary { op: Token, left: Box<Expr>, right: Box<Expr> },
    Unary { op: Token, right: Box<Expr> },
    Int { val: i32 }, Float { val: f64 }, String { val: String },
    Grouping { expr: Box<Expr> },
    Name { val: String },
}

impl Expr {
    pub(crate) fn string(&self) -> String {
        match self {
            Expr::Binary { left, right, op } => format!("({} {:?} {})", left.string(), op, right.string()),
            Expr::Unary { right, op } => format!("{:?} ({})", op, right.string()),
            Expr::Int { val } => val.to_string(),
            Expr::Float { val } => val.to_string(),
            Expr::String { val } => val.to_string(),
            Expr::Grouping { expr } => format!("({})", expr.string()),
            Expr::Name { val } => val.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt {
    FunDeclaration { name: Token, params: Vec<Token>, body: Vec<Stmt> },
    Block { stmts: Vec<Stmt> },
    Return { expr: Option<Expr> },
    Expression { expr: Expr },
    Let { name: Token, expr: Expr },
    Print { expr: Expr },
    If { condition: Expr, consequence: Vec<Stmt>, alternative: Option<Vec<Stmt>> }
}
