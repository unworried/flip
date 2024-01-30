use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

mod display;
mod expression;
mod statement;

#[derive(Debug)]
pub struct Ast {
    statements: Vec<Statement>,
}

impl<'a> Parse<'a> for Ast {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let mut statements = Vec::new();
        while !parser.current_token(Token::Eof) {
            statements.push(Statement::parse(parser));

            //while parser.current_token(Token::Newline) { // TODO: Review This
             //   parser.step();
            //}
        }

        Self { statements }
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    Print(statement::Print),
    Conditional(statement::Conditional),
}

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Print => Self::Print(statement::Print::parse(parser)),
            Token::If => Self::Conditional(statement::Conditional::parse(parser)),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        }
    }
}

#[derive(Debug, PartialEq)]
enum Expression {
    Identifier(String), // TODO: Change to struct
    Primitive(expression::Primitive),
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Ident(value) => Self::Identifier(value.to_owned()),
            Token::Int(_) => Self::Primitive(expression::Primitive::parse(parser)),
            Token::String(_) => Self::Literal(expression::Literal::parse(parser)),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        }
    }
}
