use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

mod display;
mod expression;
mod statement;
#[cfg(test)]
mod util;

#[derive(Debug)]
pub struct Ast {
    statements: Vec<Statement>,
}

impl<'a> Parse<'a> for Ast {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let mut statements = Vec::new();
        while !parser.current_token(Token::Eof) {
            statements.push(Statement::parse(parser));

            parser.step();


        }

        Self { statements }
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Print(statement::Print),
    If(statement::If),
    Loop(statement::Loop),
}

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let statement = match &parser.current_token {
            Token::Print => Self::Print(statement::Print::parse(parser)),
            Token::If => Self::If(statement::If::parse(parser)),
            Token::While => Self::Loop(statement::Loop::parse(parser)),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        while parser.current_token(Token::Newline) {
            parser.step();
        } // Should this not be in statement parser. return result??

        statement
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(expression::Identifier),
    Primitive(expression::Primitive),
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Ident(_) => Self::Identifier(expression::Identifier::parse(parser)),
            Token::Int(_) => Self::Primitive(expression::Primitive::parse(parser)),
            Token::String(_) => Self::Literal(expression::Literal::parse(parser)),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        }
    }
}
