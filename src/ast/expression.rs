use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use super::{Expr, ExprKind};

impl Expr {
    /// Grammar: (expression) (operator) (expression)
    pub fn parse_binary(parser: &mut Parser, precedence: u8) -> ExprKind {
        // Move this to a parse_primary section to avoid code duplication
        let leftkind = match &parser.current_token {
            Token::Int(_) => Self::parse_literal(parser), // TODO: change
            Token::Ident(_) => Self::parse_ident(parser),
            Token::LParen => Self::parse_group(parser),
            _ => {
                if UnOp::token_match(&parser.current_token) {
                    return Self::parse_unary(parser);
                }

                unimplemented!("Unexpected token {:?}", parser.current_token);
            }
        };


        let mut left = P(Expr { kind: leftkind });

        parser.step();
        println!("{:?}", parser.current_token);
        // May need parse step here
        while let Some(operator) = Self::parse_binary_operator(parser) {
            if operator.precedence() <= precedence {
                break;
            }

            parser.step();
            let right = P(Expr {
                kind: Self::parse_binary(parser, operator.precedence()),
            });

            left = P(Expr {
                kind: ExprKind::Binary(operator, left, right),
            });
        }

        left.ptr.kind // Change to return box maybe
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

        println!("{:?}", parser.current_token);

        let expr = P(Expr::parse(parser));

        ExprKind::Unary(operator, expr)
    }

    /// Grammar: "("(expression)")"
    pub fn parse_group(parser: &mut Parser) -> ExprKind {
        parser.step();
        let expr = Expr::parse(parser);
        if !parser.current_token(&Token::RParen) {
            panic!("expected: ')', actual: '{:?}'", parser.current_token);
        }

        expr.kind
    }

    /// Grammar: (identifier) => Token::Ident
    pub fn parse_ident(parser: &mut Parser) -> ExprKind {
        let symbol = match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        if !parser.symbols.contains(&symbol) {
            panic!("symbol: {:?} referenced before assignment", symbol);
        }

        ExprKind::Ident(symbol)
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
