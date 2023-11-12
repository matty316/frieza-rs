use crate::ast::Expr;
use crate::token::Token;
use crate::vm::VM;

struct Compiler {
    expr: Expr,
    code: Vec<u8>,
}

impl Compiler {
    pub(crate) fn new(expr: Expr) -> Self {
        Compiler {
            expr,
            code: vec![],
        }
    }
    pub(crate) fn compile(&mut self) -> Vec<u8> {
        self.compile_expression(&self.expr);
        return self.code.clone();
    }

    fn compile_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { left, right, op } => self.compile_binary_expr(&*left, &*right, op),
            Expr::Unary { .. } => {}
            Expr::Int { val } => self.compile_int(*val),
            Expr::Float { .. } => {}
            Expr::String { .. } => {}
            Expr::Grouping { .. } => {}
            Expr::Name { .. } => {}
        }
    }

    fn compile_binary_expr(&mut self, left: &Expr, right: &Expr, op: &Token) {
        self.compile_expression(left);
        self.compile_expression(right);

    }

    fn compile_int(&mut self, val: i32) {

    }

    pub(crate) fn interpret(&self) -> i32 {
        let mut vm = VM::new();
        vm.interpret(&self.code);
        return 0;
    }

    fn write_constant() {

    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use super::*;

    #[test]
    fn test_interpret() {
        let tests = vec![
            ("1 + 2", 3),
            ("1 + 2 * 3", 7),
            ("(1 + 2) * 3", 9),
        ];

        for t in tests {
            let mut l = Lexer::new(t.0);
            let tokens = l.scan();
            let mut p = Parser::new(tokens);
            let expr = p.parse_expr(0);
            let mut c = Compiler::new(expr);
            c.compile();
            let eval = c.interpret();

            assert_eq!(eval, t.1);
        }
    }
}