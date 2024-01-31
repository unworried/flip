use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl<'a> Parse<'a> for Identifier {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let identifier = match &parser.current_token {
            Token::Ident(value) => Self {
                value: value.to_owned(),
            },
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();
        identifier
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
        let mut lexer = Lexer::new("test".to_string());
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

        assert_eq!(
            Primitive::parse(&mut parser),
            Primitive::Int(123)
        );
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
