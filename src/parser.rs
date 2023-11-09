use crate::token::{Precedence, Token};
use crate::ast::Expr;
use crate::ast::Expr::{Binary, Unary};
use crate::lexer::Lexer;

pub(crate) struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens
        }
    }

    pub(crate) fn parse_expr(&mut self, precedence: u8) -> Expr {
        let mut token = self.advance();
        let prefix = token.prefix_fn();

        if prefix == None {
            todo!("no prefix")
        }

        let mut left = prefix.unwrap()(self);

        while precedence < self.peek().precedence() {
            token = self.advance();
            let infix = token.infix_fn();

            if infix == None {
                return left;
            }

            left = infix.unwrap()(self, left)
        }

        return left
    }

    pub(crate) fn parse_grouping(&mut self) -> Expr {
        let expr = self.parse_expr(0);
        self.consume(&Token::RParen);
        Expr::Grouping { expr: Box::new(expr) }
    }

    pub(crate) fn parse_unary(&mut self) -> Expr {
        let op = self.previous().clone();
        Unary { op, right: Box::new(self.parse_expr(Precedence::Prefix as u8)) }
    }

    pub(crate) fn parse_binary(&mut self, left: Expr) -> Expr {
        let op = self.previous().clone();
        let right = self.parse_expr(op.precedence());

        Binary { op, left: Box::new(left), right: Box::new(right) }
    }

    pub(crate) fn parse_int(&mut self) -> Expr {
        let token = self.previous();
        match token {
            Token::Int(i) => Expr::Int { val: *i },
            _ => todo!(),
        }
    }

    pub(crate) fn parse_float(&mut self) -> Expr {
        let token = self.previous();
        match token {
            Token::Float(f) => Expr::Float { val: *f },
            _ => todo!(),
        }
    }

    pub(crate) fn parse_string(&mut self) -> Expr {
        let token = self.previous();
        match token {
            Token::String(s) => Expr::String { val: s.clone() },
            _ => todo!(),
        }
    }

    pub(crate) fn parse_name(&mut self) -> Expr {
        let token = self.previous();
        match token {
            Token::Ident(i) => Expr::Name { val: i.clone() },
            _ => todo!(),
        }
    }

    //Helpers
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn consume(&mut self, token: &Token) -> &Token {
        if token == self.peek() {
            return self.advance();
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        if self.is_at_end() { return &Token::Eof; }
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current] == Token::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_expression() {
        let sources = vec![
            "(1 + 3) * 4",
            "1 + 3 * 4",
        ];

        let exp = vec![
            Expr::Binary {
                op: Token::Star,
                left: Box::new(Expr::Grouping{expr: Box::new(Expr::Binary {left: Box::new(Expr::Int {val: 1}), right: Box::new(Expr::Int {val: 3}), op: Token::Plus}) }),
                right: Box::new(Expr::Int {val: 4})
            },
            Expr::Binary {
                op: Token::Plus,
                left: Box::new(Expr::Int {val: 1}),
                right: Box::new(Expr::Binary {op: Token::Star, left: Box::new(Expr::Int {val: 3}), right: Box::new(Expr::Int {val: 4})}),
            },
        ];

        for (i, s) in sources.iter().enumerate() {
            let mut l = Lexer::new(s);
            let tokens = l.scan();
            let mut p = Parser::new(tokens);
            let expr = p.parse_expr(0);
            let e = &exp[i];
            assert_eq!(&expr, e);
        }
    }
}