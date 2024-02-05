use alloc::{borrow::ToOwned, string::String};
use core::cmp;

use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
    span::Span,
};

use super::{Expr, ExprKind};

impl Expr {
    /// Grammar: (expression) (operator) (expression)
    pub fn parse_binary(parser: &mut Parser, mut left: ExprKind, precedence: u8) -> ExprKind {
        let start_span = parser.current_span().clone();

        while let Some(operator) = Self::parse_binary_operator(parser) {
            if operator.precedence() < precedence {
                break;
            }
            parser.step();

            let mut right = Self::parse_unary_or_primary(parser);

            while let Some(inner_operator) = Self::parse_binary_operator(parser) {
                let greater_precedence = inner_operator.precedence() > operator.precedence();
                let equal_precedence = inner_operator.precedence() == operator.precedence();
                if !greater_precedence && !equal_precedence {
                    break;
                }

                right = Self::parse_binary(
                    parser,
                    right,
                    cmp::max(operator.precedence(), inner_operator.precedence()),
                );
            }
            left = ExprKind::Binary(
                operator,
                P(Expr {
                    kind: left,
                    span: start_span.clone(),
                }),
                P(Expr {
                    kind: right,
                    span: parser.current_span().clone(),
                }),
            );
        }
        left
    }

    fn parse_binary_operator(parser: &mut Parser) -> Option<BinOp> {
        match &parser.current_token() {
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

    pub fn parse_unary_or_primary(parser: &mut Parser) -> ExprKind {
        if UnOp::token_match(parser.current_token()) {
            Self::parse_unary(parser)
        } else {
            Self::parse_primary(parser)
        }
    }

    /// Grammar: (operator) (expression)
    pub fn parse_unary(parser: &mut Parser) -> ExprKind {
        let start_span = parser.current_span().clone();

        let operator = match &parser.current_token() {
            Token::Minus => UnOp::Neg,
            token => {
                parser
                    .diagnostics
                    .borrow_mut()
                    .invalid_operator(token, parser.current_span());
                return ExprKind::Error;
            }
        };

        /*
         * Should this really be caught here?
         * Catches cases where whitespace between operator and expression
         * e.g. - 1, let foo = - bar;
         * instead of:
         * -1, let foo = -bar;
         */
        if parser.next_token_is(&Token::Whitespace) {
            parser.diagnostics.borrow_mut().unexpected_token(
                parser.current_token(),
                parser.next_token(),
                parser.next_span(),
            );
        }

        parser.step();

        let expr = P(Expr {
            kind: Self::parse_unary_or_primary(parser),
            span: Span::combine(vec![&start_span, &parser.current_span()]),
        });

        ExprKind::Unary(operator, expr)
    }

    pub fn parse_primary(parser: &mut Parser) -> ExprKind {
        let (mut token, mut span) = parser.consume();

        // Recursivly catches invalid tokens until a valid token is found to continue parsing
        // correctly after an error
        while !match_primary(&token) {
            let new_span;
            (token, new_span) = parser.consume();

            if match_primary(parser.current_token()) {
                let span_combine = Span::combine(vec![&span, &new_span]);
                parser
                    .diagnostics
                    .borrow_mut()
                    .unknown_expression(&token, &span_combine);
            }
            span = new_span;
        }

        match &token {
            // Temp before i split into parse_int and parse string
            Token::Int(value) => ExprKind::Literal(Literal::Integer(value.to_owned())),
            Token::String(value) => ExprKind::Literal(Literal::String(value.to_owned())),
            Token::LParen => Self::parse_group(parser),
            // Grammar: (identifier) => Token::Ident
            Token::Ident(symbol) => ExprKind::Variable((symbol.to_owned(), span)),
            _ => panic!("Really shouldn't reach here, implement fatal error instead"),
        }
    }

    /// Grammar: "("(expression)")"
    pub fn parse_group(parser: &mut Parser) -> ExprKind {
        let expr = Expr::parse(parser);
        parser.consume_and_check(Token::RParen);

        expr.kind
    }

    /// Grammar: (literal) => Token::Int | Token::String
    pub fn parse_literal(parser: &mut Parser) -> ExprKind {
        let litkind = match &parser.current_token() {
            Token::String(value) => Literal::String(value.to_owned()),
            Token::Int(value) => Literal::Integer(value.to_owned()),
            token => {
                parser
                    .diagnostics
                    .borrow_mut()
                    .unknown_expression(token, parser.current_span());
                return ExprKind::Error;
            }
        };

        ExprKind::Literal(litkind)
    }
}

fn match_primary(token: &Token) -> bool {
    matches!(
        token,
        Token::Int(_) | Token::String(_) | Token::LParen | Token::Ident(_)
    )
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
            BinOp::Add | BinOp::Sub => 18,
            BinOp::Mul | BinOp::Div => 19,
            BinOp::Eq | BinOp::NotEq => 30,
            BinOp::LessThan | BinOp::LessThanEq | BinOp::GreaterThan | BinOp::GreaterThanEq => 29,
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

pub type Ident = (String, Span);

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(isize),
    // Add more
}
