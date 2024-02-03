use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use super::{Expr, ExprKind};

impl Expr {
    /// Grammar: (expression) (operator) (expression)
    pub fn parse_binary(parser: &mut Parser, mut left: ExprKind, precedence: u8) -> ExprKind {
        while let Some(operator) = Self::parse_binary_operator(parser) {
            if operator.precedence() <= precedence {
                break;
            }
            parser.step();

            let mut right = Expr::parse(parser);

            while let Some(inner_operator) = Self::parse_binary_operator(parser) {
                let greater_precedence = inner_operator.precedence() > operator.precedence();
                let equal_precedence = inner_operator.precedence() == operator.precedence();
                if !greater_precedence && !equal_precedence {
                    break;
                }

                right = Expr {
                    kind: Self::parse_binary(
                        parser,
                        right.kind,
                        std::cmp::max(operator.precedence(), inner_operator.precedence()),
                    ),
                };
            }
            left = ExprKind::Binary(operator, P(Expr { kind: left }), P(right));
        }
        left
    }

    fn parse_binary_operator(parser: &mut Parser) -> Option<BinOp> {
        match &parser.current_token {
            Token::Plus => Some(BinOp::Add),
            Token::Minus => Some(BinOp::Sub),
            Token::ForwardSlash => Some(BinOp::Div),
            Token::Asterisk => Some(BinOp::Mul),
            Token::GreaterThan => Some(BinOp::GreaterThan),
            Token::GreaterThanEqual => Some(BinOp::GreaterThanEq),
            Token::LessThan => Some(BinOp::LessThan),
            Token::LessThanEqual => Some(BinOp::LessThanEq),
            Token::Equal => Some(BinOp::Eq),
            Token::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }
    }

    /// Grammar: (operator) (expression)
    pub fn parse_unary(parser: &mut Parser) -> ExprKind {
        let operator = match &parser.current_token {
            Token::Minus => UnOp::Neg,
            token => unimplemented!("Unexpected token {:?}", token),
        };
        parser.step();

        let expr = P(Expr::parse(parser));

        ExprKind::Unary(operator, expr)
    }

    pub fn parse_primary(parser: &mut Parser) -> ExprKind {
        let token = parser.eat();
        match &token {
            // Temp before i split into parse_int and parse string
            Token::Int(value) => ExprKind::Literal(Literal::Integer(value.to_owned())),
            Token::String(value) => ExprKind::Literal(Literal::String(value.to_owned())),
            Token::LParen => Self::parse_group(parser),
            // Grammar: (identifier) => Token::Ident
            Token::Ident(symbol) => ExprKind::Ident(symbol.to_owned()),
            _ => {
                unimplemented!("Unexpected token {:?}", token);
            }
        }
    }

    /// Grammar: "("(expression)")"
    pub fn parse_group(parser: &mut Parser) -> ExprKind {
        let expr = Expr::parse(parser);
        if !parser.current_token(&Token::RParen) {
            panic!("expected: ')', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        expr.kind
    }

    /// Grammar: (literal) => Token::Int | Token::String
    pub fn parse_literal(parser: &mut Parser) -> ExprKind {
        let litkind = match &parser.current_token {
            Token::String(value) => Literal::String(value.to_owned()),
            Token::Int(value) => Literal::Integer(value.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        ExprKind::Literal(litkind)
    }
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
}

impl BinOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(
            token,
            Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::ForwardSlash
                | Token::Equal
                | Token::NotEqual
                | Token::LessThan
                | Token::LessThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual
        )
    }

    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Add | BinOp::Sub => 1,
            BinOp::Mul | BinOp::Div => 2,
            BinOp::Eq | BinOp::NotEq => 3,
            BinOp::LessThan | BinOp::LessThanEq | BinOp::GreaterThan | BinOp::GreaterThanEq => 4,
        }
    }
}

#[derive(Debug)]
pub enum UnOp {
    //Not,
    Neg,
}

impl UnOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(token, Token::Minus)
    }
}

pub type Ident = String;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(isize),
    // Add more
}
