use crate::token::Token;

pub(crate) fn scan(source: &str) -> Vec<Token> {
    scan_tokens(source, 0, 0, vec![])
}

fn scan_tokens(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let is_at_end = current >= source.len();

    if !is_at_end {
        let c = current_char(source, current);
        let peek = peek(source, current);
        if is_alpha(c) { return ident(source, start, current, tokens); }
        if is_digit(c) { return num(source, start, current, tokens); }
        match c {
            '(' => return advance(source, current, tokens, Token::LParen),
            ')' => return advance(source, current, tokens, Token::RParen),
            ';' => return advance(source, current, tokens, Token::Semicolon),
            ',' => return advance(source, current, tokens, Token::Comma),
            '+' => return advance(source, current, tokens, Token::Plus),
            '-' => return advance(source, current, tokens, Token::Minus),
            '*' => return advance(source, current, tokens, Token::Star),
            ':' => return advance(source, current, tokens, Token::Colon),
            '/' => {
                if peek == '/' {
                    return comment(source, current, tokens);
                } else {
                    return advance(source, current, tokens, Token::Slash);
                }
            }
            '=' => {
                return if peek == '=' {
                    advance(source, current + 1, tokens, Token::EqEq)
                } else {
                    advance(source, current, tokens, Token::Eq)
                }
            }
            '!' => {
                return if peek == '=' {
                    advance(source, current + 1, tokens, Token::BangEq)
                } else {
                    advance(source, current, tokens, Token::Bang)
                }
            }
            '<' => {
                return if peek == '=' {
                    advance(source, current + 1, tokens, Token::LtEq)
                } else {
                    advance(source, current, tokens, Token::Lt)
                }
            }
            '>' => {

                return if peek == '=' {
                    advance(source, current + 1, tokens, Token::GtEq)
                } else {
                    advance(source, current, tokens, Token::Gt)
                }
            }
            '"' => {
                return string(source, start, current, tokens);
            }
            '\n' => {
                return advance(source, current, tokens, Token::NewLine);
            }
            ' ' | '\t' | '\r' => return scan_tokens(source, current + 1, current + 1, tokens),
            _ => (),
        }
    }
    push(tokens, Token::Eof)
}

fn ident(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let current_char = current_char(source, current);
    if is_alpha(current_char) || is_digit(current_char) {
        return ident(source, start, current + 1, tokens);
    }

    let string = &source[start..current];
    lookup_keyword(source, start, current, tokens)
}

fn lookup_keyword(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let c = char_at(source, start);
    match c {
        'l' => { return check_keyword(source, start,current, "et", 1, 2, Token::Let, tokens); }
        'r' => { return check_keyword(source, start,current, "eturn", 1, 5, Token::Return, tokens); }
        'i' => {
            if current - start > 1 && source[start..current].len() == 2 {
                match char_at(source, start + 1) {
                    'f' => { return advance(source, current, tokens, Token::If); }
                    'n' => { return advance(source, current, tokens, Token::In); }
                    _ => (),
                }
            }
        }
        'n' => { return check_keyword(source, start,current, "il", 1 , 2, Token::Nil, tokens); }
        't' => { return check_keyword(source, start,current, "rue", 1, 3, Token::True, tokens); }
        'p' => { return check_keyword(source, start,current, "rint", 1, 4, Token::Print, tokens); }
        'e' => {
            if current - start > 1 {
                match char_at(source, start + 1) {
                    'n' => { return check_keyword(source, start,current, "d", 2, 1, Token::End, tokens); }
                    'l' => { return check_keyword(source, start,current, "se", 2, 2, Token::Else, tokens); }
                    _ => (),
                }
            }
        }
        'f' => {
            if current - start > 1 {
                match char_at(source, start + 1) {
                    'u' => { return check_keyword(source, start,current, "n", 2, 1, Token::Fun, tokens); }
                    'a' => { return check_keyword(source, start,current, "lse", 2, 3, Token::False, tokens); }
                    'o' => { return check_keyword(source, start,current, "r", 2, 1, Token::For, tokens); }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    let token = Token::Ident(source[start..current].to_string());
    let new_tokens = push(tokens, token);
    scan_tokens(source, current, current, new_tokens)
}

fn check_keyword(source: &str,
                 start: usize,
                 current: usize,
                 rest: &str,
                 begin: usize,
                 len: usize,
                 token: Token,
                 tokens: Vec<Token>) -> Vec<Token> {
    if current - start == begin + len {
        let slice = &source[start+begin..start+begin+len];
        if rest == slice {
            let new_tokens = push(tokens, token);
            return scan_tokens(source, current, current, new_tokens);
        }
    }
    let token = Token::Ident(source[start..current].to_string());
    let new_tokens = push(tokens, token);
    scan_tokens(source, current, current, new_tokens)
}

fn num(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let current_char = current_char(source, current);
    if is_digit(current_char) {
        return num(source, start, current + 1, tokens);
    }

    if is_alpha(current_char) {
        todo!("error")
    } else if current_char == '.' {
        return dot(source, start, current, tokens);
    } else {
        return parse_num(source, start, current, tokens);
    }
}

fn dot(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let current_char = current_char(source, current);
    if is_alpha(current_char) {
        todo!("error")
    } else if current_char == '.' {
        return dot(source, start, current + 1, tokens);
    } else {
        return parse_num(source, start, current + 1, tokens);
    }
}

fn parse_num(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let string = &source[start..current];
    let int = string.parse::<i32>();

    match int {
        Ok(i) => {
            let new_tokens = push(tokens, Token::Int(i));
            return scan_tokens(source, current, current, new_tokens)
        }
        _ => {
            let float = string.parse::<f64>();
            match float {
                Ok(f) => {
                    let new_tokens = push(tokens, Token::Float(f));
                    return scan_tokens(source, current, current, new_tokens)
                }
                Err(e) => todo!("error"),
            }
        }
    }
}

fn comment(source: &str, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let current_char = current_char(source, current);
    let is_at_end = current >= source.len();
    if current_char != '\n' &&  !is_at_end {
        return comment(source, current + 1, tokens);
    }

    scan_tokens(source, current, current, tokens)
}

fn string(source: &str, start: usize, current: usize, tokens: Vec<Token>) -> Vec<Token> {
    let peek = peek(source, current);
    let is_at_end = current >= source.len();
    if peek != '"' && !is_at_end {
        return string(source, start, current + 1, tokens);
    }

    if is_at_end {
        todo!("error")
    }

    // println!("{} {} {:?}", start, current, &source[start+1..current-1]);
    let string = source[start+1..current+1].to_string();
    let token = Token::String(string);
    advance(source, current + 1, tokens, token)
}

//Helpers
fn current_char(source: &str, current: usize) -> char {
    if current >= source.len() { return '\0' }
    source.chars().nth(current).unwrap()
}

fn peek(source: &str, current: usize) -> char {
    if current + 1 >= source.len() { return '\0' }
    source.chars().nth(current + 1).unwrap()
}

fn peek_next(source: &str, current: usize) -> char {
    if current + 2 >= source.len() { return '\0' }
    source.chars().nth(current + 2).unwrap()
}

fn char_at(source: &str, i: usize) -> char {
    source.chars().nth(i).unwrap()
}

fn advance(source: &str, current: usize, tokens: Vec<Token>, token: Token) -> Vec<Token> {
    let new_idx = current + 1;
    let new_tokens = push(tokens, token);
    scan_tokens(source, new_idx, new_idx, new_tokens)
}

fn push(tokens: Vec<Token>, token: Token) -> Vec<Token> {
    [tokens, vec![token]].concat().to_vec()
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
    fn test_scanner() {
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
        10.0
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

            assert_eq!(e, t, "pos = {} exp {:?} got {:?}", i, e, t);
        }
        assert_eq!(tokens.len(), exp.len());
    }
}