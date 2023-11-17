use crate::token::Token;
struct Lexer {
    source: String,
    current: usize,
    start: usize,
    line: u32,
}

pub(crate) fn scan(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan();
    tokens
}

impl Lexer {
    fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            current: 0,
            start: 0,
            line: 1,
        }
    }

    fn scan(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.skip_whitespace();
            self.start = self.current;
            let c = self.advance();

            if is_alpha(c) { tokens.push(self.ident()) }
            if is_digit(c) { tokens.push(self.num()) }

            match c {
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ';' => tokens.push(Token::Semicolon),
                ',' => tokens.push(Token::Comma),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                ':' => tokens.push(Token::Colon),
                '=' => {
                    if self.peek() == '=' {
                        tokens.push(Token::EqEq);
                        self.advance();
                    } else {
                        tokens.push(Token::Eq);
                    }
                }
                '!' => {
                    if self.peek() == '=' {
                        tokens.push(Token::BangEq);
                        self.advance();
                    } else {
                        tokens.push(Token::Bang);
                    }
                }
                '<' => {
                    if self.peek() == '=' {
                        tokens.push(Token::LtEq);
                        self.advance();
                    } else {
                        tokens.push(Token::Lt);
                    }
                }
                '>' => {
                    if self.peek() == '=' {
                        tokens.push(Token::GtEq);
                        self.advance();
                    } else {
                        tokens.push(Token::Gt);
                    }
                }
                '"' => { tokens.push(self.string()); }
                '\n' => {
                    self.line += 1;
                    tokens.push(Token::NewLine);
                }
                _ => (),
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    //Helpers
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let current = self.char_at(self.current);
        self.current += 1;
        current
    }

    fn char_at(&self, i: usize) -> char {
        if i >= self.source.len() { return '\0' }
        self.source.chars().nth(i).expect("could not get char")
    }

    fn peek(&self) -> char {
        self.char_at(self.current)
    }

    fn peek_next(&self) -> char {
        self.char_at(self.current + 1)
    }

    fn skip_whitespace(&mut self) {
        let c = self.peek();

        match c {
            ' ' | '\t' | '\r' => { self.advance(); }
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                }
            }
            _ => return,
        }
    }

    fn ident(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        self.lookup_keyword()
    }

    fn get_string(&self, start: usize, end: usize) -> String {
        self.source[start..end].to_string()
    }

    fn current_string(&self) -> String {
        self.get_string(self.start, self.current)
    }

    fn lookup_keyword(&self) -> Token {
        let c = self.char_at(self.start);
        match c {
            'l' => { return self.check_keyword("et", 1, 2, Token::Let); }
            'r' => { return self.check_keyword("eturn", 1, 5, Token::Return); }
            'i' => {
                if self.current - self.start > 1 && self.current_string().len() == 2 {
                    match self.char_at(self.start + 1) {
                        'f' => { return Token::If; }
                        'n' => { return Token::In; }
                        _ => (),
                    }
                }
            }
            'n' => { return self.check_keyword("il", 1 , 2, Token::Nil); }
            't' => { return self.check_keyword("rue", 1, 3, Token::True); }
            'p' => { return self.check_keyword("rint", 1, 4, Token::Print); }
            'e' => {
                if self.current - self.start > 1 {
                    match self.char_at(self.start + 1) {
                        'n' => { return self.check_keyword("d", 2, 1, Token::End); }
                        'l' => { return self.check_keyword("se", 2, 2, Token::Else); }
                        _ => (),
                    }
                }
            }
            'f' => {
                if self.current - self.start > 1 {
                    match self.char_at(self.start + 1) {
                        'u' => { return self.check_keyword("n", 2, 1, Token::Fun); }
                        'a' => { return self.check_keyword("lse", 2, 3, Token::False); }
                        'o' => { return self.check_keyword("r", 2, 1, Token::For); }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        Token::Ident(self.current_string())
    }

    fn check_keyword(&self, rest: &str, start: usize, len: usize, token: Token) -> Token {
        if self.current - self.start == start + len {
            let slice = &self.source[self.start+start..self.start+start+len];
            if rest == slice {
                return token;
            }
        }
        Token::Ident(self.current_string())
    }

    fn num(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' {
            self.advance();
            while is_digit(self.peek()) { self.advance(); }
            if is_alpha(self.peek()) {
                todo!("handle error");
            }
            let string = self.current_string();
            return Token::Float(string.parse().expect("cannot parse float"))
        } else {
            let string = self.current_string();
            return Token::Int(string.parse().expect("cannot parse int"))
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();

            if self.peek() == '\n' { self.line += 1; }
        }

        self.advance();

        if self.is_at_end() {
            todo!("error")
        }

        return Token::String(self.get_string(self.start+1, self.current-1))
    }
}

pub(crate) fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub(crate) fn is_alpha(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let s = r#"let num1 = 5
        let num2 = 10.5

        let string = "name"; let anotherString = "another name"

        fun add(x, y)
            return x + y
        end

        !-/*<> <= >= == !=

        if true
            return nil
        else
            print "free
            numba 9"
        end

        if x == 1: return "1"

        // test a comment

        for i in array
            print iffy
            print inny
        end
        10.
        "#;

        let exp = vec![
            Token::Let,
            Token::Ident("num1".to_string()),
            Token::Eq,
            Token::Int(5),
            Token::NewLine,
            Token::Let,
            Token::Ident("num2".to_string()),
            Token::Eq,
            Token::Float(10.5),
            Token::NewLine,
            Token::NewLine,
            Token::Let,
            Token::Ident("string".to_string()),
            Token::Eq,
            Token::String("name".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("anotherString".to_string()),
            Token::Eq,
            Token::String("another name".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::Fun,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::NewLine,
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::NewLine,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Star,
            Token::Lt,
            Token::Gt,
            Token::LtEq,
            Token::GtEq,
            Token::EqEq,
            Token::BangEq,
            Token::NewLine,
            Token::NewLine,
            Token::If,
            Token::True,
            Token::NewLine,
            Token::Return,
            Token::Nil,
            Token::NewLine,
            Token::Else,
            Token::NewLine,
            Token::Print,
            Token::String("free\n            numba 9".to_string()),
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::NewLine,
            Token::If,
            Token::Ident("x".to_string()),
            Token::EqEq,
            Token::Int(1),
            Token::Colon,
            Token::Return,
            Token::String("1".to_string()),
            Token::NewLine,
            Token::NewLine,
            Token::NewLine,
            Token::NewLine,
            Token::For,
            Token::Ident("i".to_string()),
            Token::In,
            Token::Ident("array".to_string()),
            Token::NewLine,
            Token::Print,
            Token::Ident("iffy".to_string()),
            Token::NewLine,
            Token::Print,
            Token::Ident("inny".to_string()),
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::Float(10.0),
            Token::NewLine,
            Token::Eof
        ];

        let tokens = scan(s);

        for (i, t) in tokens.iter().enumerate() {
            let e = &exp[i];

            assert_eq!(e, t, "pos = {}", i);
        }
        assert_eq!(tokens.len(), exp.len());
    }
}