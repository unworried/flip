use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

mod statements;
mod types;

pub struct Ast {
    statements: Vec<Statement>,
}

impl<'a> Parse<'a> for Ast {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let mut statements = Vec::new();
        while !parser.is_eof() {
            statements.push(Statement::parse(parser));

            parser.step();
        }

        Self { statements }
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Print(statements::Print),
}

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Print => Self::Print(statements::Print::parse(parser)),
            _ => unimplemented!(), // Handle Err
        }
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Identifier(String), // TODO: Change to struct
    Primitive(types::Primitive),
    Literal(types::Literal),
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Ident(value) => Self::Identifier(value.to_owned()),
            Token::Int(_) => Self::Primitive(types::Primitive::parse(parser)),
            Token::String(_) => Self::Literal(types::Literal::parse(parser)),
            _ => unimplemented!(), // Handle Err
        }
    }
}
