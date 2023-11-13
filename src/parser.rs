use std::thread::current;
use crate::token::Token;
use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Stmt::FunDeclaration;

type Program = Vec<Stmt>;

pub(crate) fn parse(tokens: Vec<Token>) -> Program {
    parse_statements(&tokens, vec![], 0)
}

fn parse_statements(tokens: &Vec<Token>, program: Program, current: usize) -> Program {
    let is_at_end = current >= tokens.len();

    if !is_at_end {
        return declaration(program, &tokens, current);
    }

    return program
}

fn declaration(program: Program, tokens: &Vec<Token>, current: usize) -> Program {
    let token = token_at(tokens, current);
    if token == &Token::Fun { return fun(program, tokens, current + 1); }
    // if token == &Token::Let { return let_declaration(); }
    // statement()
    program
}

fn fun(program: Program, tokens: &Vec<Token>, current: usize) -> Program {
    let token = token_at(tokens, current);
    match token {
        Token::Ident(_) => {
            let name = token;
            return parse_params(tokens, current + 1, vec![]);
        }
        _ => (),
    }
    program
}

fn parse_params(tokens: &Vec<Token>, current: usize, params: Vec<Token>) -> Program {
    let token = token_at(tokens, current);
    match token {
        Token::LParen => return parse_params(tokens, current + 1, params),
        Token::Ident(s) => {
            let new_params = add_param(params, token);
            return parse_params(tokens, current + 1, new_params);
        }
        Token::Comma => return parse_params(tokens, current + 1, params),
        Token::RParen => return params,
        _ => todo!("Error"),
    }
}

fn add_param(params: Vec<Token>, param: Token) -> Vec<Token> {
    let mut mut_params = params;
    mut_params.push(param);
    mut_params
}

//Helpers
fn check(ops: &[Token], current: &Token) -> bool {
    for o in ops {
        if o == current {
            return true
        }
    }
    false
}

fn advance(tokens: &Vec<Token>, current: usize, program: Program, stmt: Stmt) -> Program {
    let new_program = push(program, stmt);
    let new_current = current + 1;
    parse_statements(tokens, new_program, new_current)
}

fn push(program: Program, stmt: Stmt) -> Program {
    let mut mut_program = program;
    mut_program.push(stmt);
    mut_program
}

fn token_at(tokens: &Vec<Token>, current: usize) -> &Token {
    if current >= tokens.len() { return &Token::Eof; }
    &tokens[current]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::scanner::scan;

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
                    expr: Expr::Binary {
                        left: Box::new(Expr::Name { val: "x".to_string() }),
                        right: Box::new(Expr::Name { val: "y".to_string() }),
                        op: Token::Plus,
                    }
                }
            ]
        };

        assert_eq!(p.first().unwrap(), &function);
    }

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
            let tokens = scan(s);
            let p = parse(tokens);
        }
    }
}