use crate::ast::Expr;
use crate::parser::Parser;

pub(crate) enum Precedence {
    None = 0,
    Assignment = 1,
    Conditional = 2,
    Sum = 3,
    Factor = 4,
    Prefix = 5,
    Call = 6,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Token {
    Eof,
    LParen, RParen, Colon, Comma, Semicolon,
    Let, If, Else, Fun, For, Return, End, True, False, Nil, Print, In,
    Ident(String), Int(i32), Float(f64), String(String),
    Eq, EqEq, Plus, Minus, Slash, Star, Bang, BangEq, Lt, LtEq, Gt, GtEq,
    NewLine,
}

type PrefixParseFn = fn(parser: &mut Parser) -> Expr;
type InfixParseFn = fn(parser: &mut Parser, left: Expr) -> Expr;

impl Token {
    pub(crate) fn prefix_fn(&self) -> Option<PrefixParseFn> {
        match self {
            Token::LParen => Some(Parser::parse_grouping),
            Token::Minus | Token::Bang => Some(Parser::parse_unary),
            Token::Int(_) => Some(Parser::parse_int),
            Token::Float(_) => Some(Parser::parse_float),
            Token::String(_) => Some(Parser::parse_string),
            Token::Ident(_) => Some(Parser::parse_name),
            _ => None,
        }
    }

    pub(crate) fn infix_fn(&self) -> Option<InfixParseFn> {
        match self {
            Token::Plus | Token::Minus | Token::Slash | Token::Star => Some(Parser::parse_binary),
            _ => None,
        }
    }

    pub(crate) fn precedence(&self) -> u8 {
        match self {
            Token::Plus | Token::Minus => Precedence::Sum as u8,
            Token::Slash | Token::Star => Precedence::Factor as u8,
            _ => Precedence::None as u8,
        }
    }
}