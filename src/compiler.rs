use crate::ast::Expr;
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

        return self.code.clone();
    }

    pub(crate) fn interpret(&self) -> i32 {
        let mut vm = VM::new();
        vm.interpret(&self.code);
        return 0;
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