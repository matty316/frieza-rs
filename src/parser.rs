use crate::token::Token;
use crate::ast::Expr;
use crate::ast::Expr::*;
use crate::ast::Stmt;
use crate::ast::Stmt::*;

pub(crate) type Program = Vec<Stmt>;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    program: Program,
    line: usize,
}

pub(crate) fn parse(tokens: Vec<Token>) -> Program {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            program: vec![],
            current: 0,
            line: 1,
        }
    }

    fn parse(&mut self) -> Program {
        let mut program = vec![];

        while !self.is_at_end() {
            if let Some(declaration) = self.declaration() {
                program.push(declaration);
            }
        }

        program
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.check(vec![Token::NewLine, Token::Eof]) { return None }
        if self.check(vec![Token::Fun]) { return Some(self.fun()); }
        if self.check(vec![Token::Let]) { return Some(self.let_declaration()); }
        Some(self.statement())
    }

    fn statement(&mut self) -> Stmt {
        if self.check(vec![Token::Return]) { return self.return_stmt(); }
        if self.check(vec![Token::Print]) { return self.print_stmt(); }
        if self.check(vec![Token::If]) { return self.if_stmt(); }
        self.expr_statement()
    }

    fn fun(&mut self) -> Stmt {
        let token = self.advance();
        match token {
            Token::Ident(_) => {
                let params = self.params();
                let body = self.block();

                self.consume(Token::End);
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
        while !self.is_at_end() && self.peek() != Token::End && self.peek() != Token::Else {
            if let Some(declaration) = self.declaration() {
                stmts.push(declaration);
            }
        }
        stmts
    }

    fn return_stmt(&mut self) -> Stmt {
        let expr: Option<Expr>;
        if self.peek() != Token::NewLine && !self.is_at_end() {
            expr = Some(self.expr());
        } else {
            expr = None;
        }
        self.advance();
        if self.peek() == Token::NewLine {
            self.advance();
        }

        return Return { expr }
    }

    fn if_stmt(&mut self) -> Stmt {
        let condition = self.expr();
        self.consume(Token::NewLine);
        let consequence = self.block();

        let alternative: Option<Vec<Stmt>>;
        
        if self.check(vec![Token::Else]) {
            alternative = Some(self.block());
        } else {
            alternative = None;
        }
        self.consume(Token::End);

        If { condition, consequence, alternative }
            
    }

    fn print_stmt(&mut self) -> Stmt {
        let expr = self.expr();
        self.consume(Token::NewLine);
        return Stmt::Print { expr }
    }

    fn let_declaration(&mut self) -> Stmt {
        match self.peek() {
            Token::Ident(_) => {
                let name = self.advance();
                self.consume(Token::Eq);
                let expr = self.expr();
                return Let { name, expr };
            }
            _ => todo!("error"),
        }
    }

    fn expr_statement(&mut self) -> Stmt {
        let expr = self.expr();
        Expression { expr }
    }

    fn expr(&mut self) -> Expr {
        self.comparison()
    }

    fn comparison(&mut self) -> Expr {
        let mut left = self.term();

        while self.check(vec![Token::Gt, Token::GtEq, Token::Lt, Token::LtEq]) {
            let op = self.previous();
            let right = self.term();
            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) }
        }

        left
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();

        while self.check(vec![Token::Plus, Token::Minus]) {
            let op = self.previous();
            let right = self.factor();

            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) }
        }
        return left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.primary();

        while self.check(vec![Token::Star, Token::Slash]) {
            let op = self.previous();
            let right = self.primary();

            left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) }
        }
        return left
    }

    fn primary(&mut self) -> Expr {
        match self.peek() {
            Token::Int(i) => {
                self.advance();
                Expr::Int { val: i }
            },
            Token::Ident(s) => {
                self.advance();
                Name { val: s }
            },
            Token::String(s) => {
                self.advance();
                String { val: s }
            },
            Token::Float(f) => {
                self.advance();
                Float { val: f }
            }
            _ => {
                println!("{:?}", self.peek());
                todo!("error");
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn consume(&mut self, token: Token) -> Token {
        if token == self.peek() {
            return self.advance();
        }
                println!("{:?}", token);
                println!("{:?}", self.peek());
        todo!("error")
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn check(&mut self, tokens: Vec<Token>) -> bool {
        for t in tokens {
            if t == self.peek() {
                self.advance();
                return true
            }
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr::{Binary, Int};
    use super::*;
    use crate::parser::parse;
    use crate::lexer::scan;

    #[test]
    fn test_fun() {
        let s = r#"
        fun add(x, y)
            return x + y
        end
        "#;

        let t = scan(s);
        let p = parse(t);
        assert_eq!(p.len(), 1);

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

        assert_eq!(p[0], function);
    }

    #[test]
    fn test_let() {
        let s = r#"
        let juice = "juice"
        let wrld = "wrld"
        let helloWrld = juice + " " + wrld
        "#;

        let t = scan(s);
        let p = parse(t);

        assert_eq!(p.len(), 3);

        let exp1 = Stmt::Let { name: Token::Ident("juice".to_string()), expr: String {val: "juice".to_string()}};
        let exp2 = Stmt::Let { name: Token::Ident("wrld".to_string()), expr: String {val: "wrld".to_string()}};
        let exp3 = Stmt::Let {
            name: Token::Ident("helloWrld".to_string()),
            expr: Binary {
                left: Box::new(Binary {
                    left: Box::new(Expr::Name { val: "juice".to_string() }),
                    op: Token::Plus,
                    right: Box::new(Expr::String { val: " ".to_string() }),
                }),
                op: Token::Plus,
                right: Box::new(Expr::Name { val: "wrld".to_string() })
            }
        };

        assert_eq!(exp1, p[0]);
        assert_eq!(exp2, p[1]);
        assert_eq!(exp3, p[2]);
    }

    #[test]
    fn test_expr_stmt() {
        let s = r#"
        1 + 1
        x + y
        x + 1
        juice + " " + wrld
        "#;

        let exp = vec![
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Plus,
                    right: Box::new(Expr::Int { val: 1 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Name { val: "x".to_string() }),
                    op: Token::Plus,
                    right: Box::new(Expr::Name { val: "y".to_string() }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Name { val: "x".to_string() }),
                    op: Token::Plus,
                    right: Box::new(Expr::Int { val: 1 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Binary {
                        left: Box::new(Expr::Name { val: "juice".to_string() }),
                        op: Token::Plus,
                        right: Box::new(Expr::String { val: " ".to_string() }),
                    }),
                    op: Token::Plus,
                    right: Box::new(Expr::Name { val: "wrld".to_string() })
                }
            },
        ];

        check_stmt(s, exp);
    }

    #[test]
    fn test_binary() {
        let s = r#"
        1 + 2
        1 - 2
        1 * 2
        1 / 2
        1 + 2 + 3
        1 + 2 * 3
        1 < 2
        1 > 2
        1 <= 2
        1 >= 2
        "#;

        let exp = vec![
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Plus,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Minus,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Star,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Slash,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Binary {
                        left: Box::new(Expr::Int { val: 1 }),
                        op: Token::Plus,
                        right: Box::new(Expr::Int { val: 2 }),
                    }),
                    op: Token::Plus,
                    right: Box::new(Expr::Int { val: 3 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Plus,
                    right: Box::new(Binary {
                        left: Box::new(Expr::Int { val: 2 }),
                        op: Token::Star,
                        right: Box::new(Expr::Int { val: 3 }),
                    }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Lt,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::Gt,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::LtEq,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
            Stmt::Expression {
                expr: Expr::Binary {
                    left: Box::new(Expr::Int { val: 1 }),
                    op: Token::GtEq,
                    right: Box::new(Expr::Int { val: 2 }),
                }
            },
      ];

        check_stmt(s, exp);
    }

    #[test]
    fn test_print() {
        let s = r#"
        print "free" + "lil" + "steve"
        "#;

        let exp = vec![Stmt::Print {
            expr: Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::String { val: "free".to_string() }),
                    op: Token::Plus,
                    right: Box::new(Expr::String { val: "lil".to_string() }),
                }),
                op: Token::Plus,
                right: Box::new(Expr::String { val: "steve".to_string() }),
            }
        }];

        check_stmt(s, exp);
    }

    #[test]
    fn test_literal() {
        let s = r#"
        1
        2
        10.5
        35.5353
        "#;

        let exp = vec![
            Expression {expr: Int {val: 1}},
            Expression {expr: Int {val: 2}},
            Expression {expr: Float {val: 10.5}},
            Expression {expr: Float {val: 35.5353}}
        ];

        check_stmt(s, exp);
    }

    fn check_stmt(s: &str, exp: Vec<Stmt>) {
        let t = scan(s);
        let p = parse(t);

        assert_eq!(p.len(), exp.len());

        for (i, stmt) in p.iter().enumerate() {
            assert_eq!(stmt, &exp[i]);
        }
    }

    #[test]
    fn test_if() {
        let s = r#"
        if 1 < 2 
            print "yah"
        else 
            print "nah"
        end
        "#;

        let exp = vec![
           Stmt::If { 
                condition: Expr::Binary {
                    left: Box::new(Expr::Int {val: 1}),
                    right: Box::new(Expr::Int {val: 2}),
                    op: Token::Lt,
                },
                consequence: vec![Stmt::Print {expr: Expr::String {val: "yah".to_string()}}],
                alternative: Some(vec![Stmt::Print {expr: Expr::String {val: "nah".to_string()}}]),
           },
        ];
        check_stmt(s, exp);
    }
}
