use std::thread::current;
use crate::token::Token;
use crate::ast::Expr;
use crate::ast::Stmt;

type Program = Vec<Stmt>;

pub(crate) fn parse(tokens: Vec<Token>) -> Program {
    parse_statements(tokens, vec![], 0)
}

fn parse_statements(tokens: Vec<Token>, program: Program, current: usize) -> Program {
    let is_at_end = current >= tokens.len();

    if !is_at_end {
        let declaration = declaration();
        return advance(tokens, current, program, declaration);
    }

    return program
}

fn declaration() -> Stmt {
    Stmt::Expression {expr: Box::new(Expr::Int {val: 1})}
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

fn advance(tokens: Vec<Token>, current: usize, program: Program, stmt: Stmt) -> Program {
    let new_program = push(program, stmt);
    let new_current = current + 1;
    parse_statements(tokens, new_program, new_current)
}

fn push(program: Program, stmt: Stmt) -> Program {
    [program, vec![stmt]].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::scanner::scan;
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