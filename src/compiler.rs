use crate::ast::{Expr, Stmt};
use crate::token::Token;
use crate::vm::VM;
use crate::parser::Program;
use crate::opcodes::OpCode;
use crate::visitor::{ExprVisitor, StmtVisitor};

struct Compiler {
    code: Vec<u8>,
}

fn compile(program: Program) -> Vec<u8> {
    let mut compiler = Compiler::new();
    compiler.compile(program);
    compiler.code
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            code: vec![],
        }
    }

    fn compile(&mut self, program: Program) {
        for stmt in program {
            match stmt {
                Stmt::Expression { .. } => self.visit_expr_stmt(&stmt),
                _ => todo!("remove when all stmts compiled"),
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { .. } => self.visit_binary(expr),
            Expr::Int { .. } => self.visit_int(expr),
            Expr::Float { .. } => self.visit_float(expr),
            _ => (),
        }
    }

    fn add_op(&mut self, op: &Token) {
        match op {
            Token::Plus => self.code.push(OpCode::Add as u8),
            Token::Minus => self.code.push(OpCode::Subtract as u8),
            Token::Star => self.code.push(OpCode::Multiply as u8),
            Token::Slash => self.code.push(OpCode::Divide as u8),
            _ => (),
        }
    }
}

impl StmtVisitor for Compiler {
    fn visit_expr_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression { expr } => self.compile_expr(expr),
            _ => todo!("Error"),
        }
    }
}

impl ExprVisitor for Compiler {
    fn visit_binary(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { op, left, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                self.add_op(op)
            }
            _ => todo!("error")
        }
    }

    fn visit_int(&mut self, expr: &Expr) {
        match expr {
            Expr::Int { val } => {
                self.code.push(OpCode::Int as u8);
                let bytes = val.to_be_bytes();
                self.code.push(bytes[0]);
                self.code.push(bytes[1]);
                self.code.push(bytes[2]);
                self.code.push(bytes[3]);
            }
            _ => todo!("error"),
        }
    }

    fn visit_float(&mut self, expr: &Expr) {
        match expr {
            Expr::Float { val } => {
                self.code.push(OpCode::Float as u8);
                let bytes = val.to_be_bytes();
                self.code.push(bytes[0]);
                self.code.push(bytes[1]);
                self.code.push(bytes[2]);
                self.code.push(bytes[3]);
                self.code.push(bytes[4]);
                self.code.push(bytes[5]);
                self.code.push(bytes[6]);
                self.code.push(bytes[7]);
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::scan;
    use crate::parser::parse;
    use super::*;

    #[test]
    fn test_compile_expression_stmt() {
        let s = "1 + 2";
        let t = scan(s);
        let p = parse(t);
        let code = compile(p);
        let exp = vec![
            1, // Constant
            0,
            0,
            0,
            1,
            1, // Constant
            0,
            0,
            0,
            2,
            3, // Add
        ];

        assert_eq!(exp, code);
    }

    #[test]
    fn test_compile_big_num() {
        let s = "2147483647 + 2147483647";
        let t = scan(s);
        let p = parse(t);
        let code = compile(p);
        let exp = vec![
            0x01, // Int OpCode
            0x7F,
            0xFF,
            0xFF,
            0xFF,
            0x01, // Int OpCode
            0x7F,
            0xFF,
            0xFF,
            0xFF,
            0x03, // Add OpCode
        ];

        assert_eq!(exp, code);
    }

    #[test]
    fn test_float() {
        let s = "10.44492";
        let t = scan(s);
        println!("{t:?}");
        let p = parse(t);
        println!("{p:?}");
        assert_eq!(p.len(), 1);
        let c = compile(p);
        println!("{c:?}");
        assert_eq!(c[0], 7); // Float opcode
        let byte1 = c[1];
        let byte2 = c[2];
        let byte3 = c[3];
        let byte4 = c[4];
        let byte5 = c[5];
        let byte6 = c[6];
        let byte7 = c[7];
        let byte8 = c[8];

        let num = f64::from_be_bytes([byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8]);
        assert_eq!(num, 10.44492)
    }
}
