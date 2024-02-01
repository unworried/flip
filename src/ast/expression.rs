use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use super::{Expr, ExprKind, Ident};

#[derive(Debug)]
pub struct Unary;
impl<'a> Parse<'a> for Unary {
    type Item = ExprKind;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let operator = match &parser.current_token {
            Token::Minus => UnOp::Neg,
            token => unimplemented!("Unexpected token {:?}", token),
        };
        parser.step();

        let expr = P(Expr::parse(parser));

        ExprKind::Unary(operator, expr)
    }
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    //Not,
    Neg,
}

impl UnOp {
    pub fn token_match(token: &Token) -> bool {
        matches!(token, Token::Minus)
    }
}

// TODO: Add support for P<Expr> for lhs
#[derive(Debug)]
pub struct Binary;
impl<'a> Parse<'a> for Binary {
    type Item = ExprKind;

    fn parse(parser: &mut Parser<'a>) -> ExprKind {
        let leftkind = match &parser.current_token {
            Token::Int(_) => ExprKind::Literal(Literal::parse(parser)), // TODO: change
            Token::Ident(_) => ExprKind::Ident(Ident::parse(parser)),
            _ => unimplemented!("Unexpected token {:?}", parser.current_token),
        };
        let left = P(Expr { kind: leftkind });

        parser.step();

        // CHECK: Will this retain the correct precedence?
        let operator = match &parser.current_token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            Token::ForwardSlash => BinOp::Div,
            Token::Asterisk => BinOp::Mul,
            Token::GreaterThan => BinOp::GreaterThan,
            Token::GreaterThanEqual => BinOp::GreaterThanEq,
            Token::LesserThan => BinOp::LessThan,
            Token::LesserThanEqual => BinOp::LessThanEq,
            Token::Equal => BinOp::Eq,
            Token::NotEqual => BinOp::NotEq,
            token => unimplemented!("Unexpected token {:?}", token),
        };

        parser.step();

        let right = P(Expr::parse(parser));

        ExprKind::Binary(operator, left, right)
    }
}

#[derive(Debug, PartialEq)]
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
                | Token::LesserThan
                | Token::LesserThanEqual
                | Token::GreaterThan
                | Token::GreaterThanEqual
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Integer(isize),
    // Add more
}

impl<'a> Parse<'a> for Literal {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::String(value) => Self::String(value.to_owned()),
            Token::Int(value) => Self::Integer(value.parse().unwrap()),
            value => unimplemented!("Unexpected token {:?}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{int_literal, lexer::Lexer, parser::P};

    #[test]
    fn literal_int() {
        let mut lexer = Lexer::new("123".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(Literal::parse(&mut parser), Literal::Integer(123));
    }

    #[test]
    fn literal_string() {
        let mut lexer = Lexer::new("\"test\"".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Literal::parse(&mut parser),
            Literal::String("test".to_owned())
        );
    }

    #[test]
    fn unary() {
        let mut lexer = Lexer::new("-123".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Unary::parse(&mut parser),
            ExprKind::Unary(UnOp::Neg, P(int_literal!(123)))
        );
    }

    #[test]
    fn binary() {
        let mut lexer = Lexer::new("123 * 456".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Binary::parse(&mut parser),
            ExprKind::Binary(BinOp::Mul, P(int_literal!(123)), P(int_literal!(456))),
        );
    }
}
