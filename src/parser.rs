use crate::token::Token;
use crate::ast::Expr;
use crate::ast::Expr::Name;
use crate::ast::Stmt;
use crate::ast::Stmt::{FunDeclaration, Return};

type Program = Vec<Stmt>;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    program: Program,
    line: usize,
}

pub(crate) fn parse(tokens: &[Token]) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl Parser {
    fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            program: vec![],
            current: 0,
            line: 1,
        }
    }

    fn parse(&mut self) -> Program {
        let mut program = vec![];

        while !self.is_at_end() {
            let declaration = self.declaration();
            program.push(declaration)
        }

        program
    }

    fn declaration(&mut self) -> Stmt {

        while self.peek() == Token::NewLine {
            self.line += 1;
            self.advance();
        }

        match self.peek() {
            Token::Fun => { return self.fun(); }
            _ => (),
        }
        return self.statement();
    }

    fn statement(&mut self) -> Stmt {
    println!("statement {:?}", self.peek());
        match self.peek() {
            Token::Return => self.return_stmt(),
            _ => {
                todo!("error")
            },
        }
    }

    fn fun(&mut self) -> Stmt {
        self.consume(Token::Fun);
        let token = self.advance();
        match token {
            Token::Ident(_) => {
                let params = self.params();
                let body = self.block();

                return FunDeclaration { name: token, params, body }
            }
            _ => {
                todo!("error")
            },
        }
    }

    fn params(&mut self) -> Vec<Token> {
        self.consume(Token::LParen);
        let mut params = vec![];
        while self.peek() != Token::RParen {
            let token = self.advance();
            match token {
                Token::Ident(_) => params.push(token.clone()),
                Token::Comma => (),
                _ => todo!("error"),
            }
        }
        self.consume(Token::RParen);

        params
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while self.peek() != Token::End && !self.is_at_end() {
            let declaration = self.declaration();
            stmts.push(declaration);
        }
        if self.peek() == Token::NewLine || self.peek() == Token::End {
            self.advance();
        }
        stmts
    }

    fn return_stmt(&mut self) -> Stmt {
        self.consume(Token::Return);
        let expr: Option<Expr>;
        if self.peek() != Token::NewLine && !self.is_at_end() {
            expr = Some(self.expr());
        } else {
            expr = None;
        }
        self.advance();
        if self.peek() == Token::NewLine || self.peek() == Token::Eof {
            self.advance();
        }

        return Return { expr }
    }

    fn expr(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let left = self.primary();

        self.advance();
        while self.peek() == Token::Plus || self.peek() == Token::Minus {
            let op = self.advance();
            let right = self.primary();

            return Expr::Binary { left: Box::from(left), op, right: Box::from(right) }
        }
        return left
    }

    fn primary(&mut self) -> Expr {
        match self.peek() {
            Token::Int(i) => {
                Expr::Int { val: i }
            },
            Token::Ident(s) => {
                Name { val: s.clone() }
            },
            _ => todo!("error")
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current] == Token::Eof
    }

    fn consume(&mut self, token: Token) -> Token {
        if token == self.peek() {
            return self.advance();
        }
        println!("{:?} != {:?} line {}", token, self.peek(), self.line);
        todo!("error")
    }

    fn advance(&mut self) -> Token {
        let prev = &self.tokens[self.current];
        self.current += 1;
        prev.clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::lexer::scan;
    #[test]
    fn test_fun() {
        let s = r#"
        fun add(x, y)
            return x + y
        end

        fun add(x, y) return x + y
        "#;

        let t = scan(s);
        let p = parse(&t);
        assert_eq!(p.len(), 2);

        let function = Stmt::FunDeclaration {
            name: Token::Ident("add".to_string()),
            params: vec![Token::Ident("x".to_string()), Token::Ident("y".to_string())],
            body: vec![
                Stmt::Return {
                    expr: Some(Expr::Binary {
                        left: Box::new(Expr::Name { val: "x".to_string() }),
                        right: Box::new(Expr::Name { val: "y".to_string() }),
                        op: Token::Plus,
                    })
                }
            ]
        };

        assert_eq!(p.first().unwrap(), &function);
    }
}