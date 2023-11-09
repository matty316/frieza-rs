use crate::token::Token;
use crate::ast::Expr;

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

    fn parse_expr(&mut self) -> Expr {
        let token = self.advance();
        let prefix_fn = token.prefix_fn();

        if prefix_fn == None {
            todo!("error")
        }

        return prefix_fn.unwrap()(self);
    }

    pub(crate) fn parse_grouping(&mut self) -> Expr {
        Expr::Grouping { expr: Box::new(self.parse_expr()) }
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
    #[test]
    fn test_parse_expression() {

    }
}