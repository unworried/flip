use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

use super::Expr;

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub operator: UnOp,
    pub expr: Box<Expr>, //Check out rust_ast::ptr smart pointer
}

impl<'a> Parse<'a> for Unary {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let operator = match &parser.current_token {
            Token::Minus => UnOp::Neg,
            token => unimplemented!("Unexpected token {:?}", token),
        };
        parser.step();

        let expr = Box::new(Expr::parse(parser));

        Self { operator, expr }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    //Not,
    Neg,
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub operator: BinOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

// TODO: Add support for Box<Expr> instead of Box<Primitive>
impl<'a> Parse<'a> for Binary {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let left = match &parser.current_token {
            Token::Int(_) => Box::new(Expr::Primitive(Primitive::parse(parser))), // TODO: change
            _ => unimplemented!("Unexpected token {:?}", parser.current_token),
        };

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

        println!("{:?}", parser.current_token);
        println!("{:?}", parser.next_token);
        let right = Box::new(Expr::parse(parser));

        Self {
            operator,
            left,
            right,
        }
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

pub type Ident = String;

impl<'a> Parse<'a> for Ident {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Int(i64),
    // Add more
}

// TODO: Add error handling

impl<'a> Parse<'a> for Primitive {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Int(value) => Self::Int(value.parse().unwrap()),
            value => unimplemented!("Unexpected token {:?}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    // Add more
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::String(value) => Self::String(value.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{int_primitive, lexer::Lexer};

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("test".to_string());
        let mut parser = Parser::new(&mut lexer);

        println!("{:?}", parser.current_token);
        assert_eq!(Ident::parse(&mut parser), "test".to_owned());
    }

    #[test]
    fn primitive_int() {
        let mut lexer = Lexer::new("123".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(Primitive::parse(&mut parser), Primitive::Int(123));
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
            Unary {
                operator: UnOp::Neg,
                expr: Box::new(int_primitive!(123))
            }
        );
    }

    #[test]
    fn binary() {
        let mut lexer = Lexer::new("123 * 456".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Binary::parse(&mut parser),
            Binary {
                operator: BinOp::Mul,
                left: Box::new(int_primitive!(456)),
                right: Box::new(int_primitive!(456))
            }
        );
    }
}
