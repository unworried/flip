use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

use super::Expression;

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub operator: UnOp,
    pub right: Box<Expression>, //Check out rust_ast::ptr smart pointer
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    //Not,
    Neg,
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub operator: BinOp,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl<'a> Parse<'a> for Binary {
    fn parse(parser: &mut Parser<'a>) -> Self {
        println!("{:#?}", parser.current_token);
        let left = Box::new(Expression::parse(parser)); // TODO: need to fix step positioning
        parser.step();

        let operator = match &parser.current_token {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            Token::Asterisk => BinOp::Mul,
            Token::ForwardSlash => BinOp::Div,
            Token::Equal => BinOp::Eq,
            Token::NotEqual => BinOp::NotEq,
            Token::LesserThan => BinOp::LessThan,
            Token::LesserThanEqual => BinOp::LessThanEq,
            Token::GreaterThan => BinOp::GreaterThan,
            Token::GreaterThanEqual => BinOp::GreaterThanEq,
            token => unimplemented!("Unexpected token {:?}", token),
        };

        parser.step();

        let right = Box::new(Expression::parse(parser));

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

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.next_token {
            Token::Ident(value) => Self {
                value: value.to_owned(),
            },
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
        let primitive = match &parser.current_token {
            Token::Int(value) => Self::Int(value.parse().unwrap()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();
        primitive
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    // Add more
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let literal = match &parser.current_token {
            Token::String(value) => Self::String(value.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();
        literal
    }
}

/*pub struct Condition {
    pub left: Expression,
    pub operator: Reop,
    pub right: Expression,
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("KEYWORD test".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(
            Identifier::parse(&mut parser),
            Identifier {
                value: "test".to_owned()
            }
        );
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
}
