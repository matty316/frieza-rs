use crate::ast::{Expr, Stmt};

pub(crate) trait ExprVisitor {
    fn visit_binary(&mut self, expr: &Expr);
    fn visit_int(&mut self, expr: &Expr);
    fn visit_float(&mut self, expr: &Expr);
}

pub(crate) trait StmtVisitor {
    fn visit_expr_stmt(&mut self, stmt: &Stmt);
}