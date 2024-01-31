use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

use self::expression::BinOp;

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
    // "PRINT" (expression)
    Print(statement::Print),
    // "IF" (condition) "THEN" \n {statement} "ENDIF"
    If(statement::If),
    // "WHILE" (condition) "REPEAT" \n {statement} "ENDWHILE"
    While(statement::While),
    // "LABEL" (identifier)
    //Label(statement::Label),
    // "GOTO" (identifier)
    //Goto(statement::Goto),
    // "LET" (identifier) "=" (expression)
    //Let(statement::Let),
    // "INPUT" (identifier)
    //Input(statement::Input),
}

impl<'a> Parse<'a> for Statement {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let statement = match &parser.current_token {
            Token::Print => Self::Print(statement::Print::parse(parser)),
            Token::If => Self::If(statement::If::parse(parser)),
            Token::While => Self::While(statement::While::parse(parser)),
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
    Binary(expression::Binary),
    // Unary(expression::Unary
    Identifier(expression::Identifier), // Might not belong here
    Primitive(expression::Primitive),
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut Parser<'a>) -> Self {
        parser.step();

        match &parser.current_token {
            Token::Int(_) => {
                if BinOp::token_match(&parser.next_token) {
                    Self::Binary(expression::Binary::parse(parser))
                } else {
                    Self::Primitive(expression::Primitive::parse(parser))
                }
            }
            Token::Ident(_) => Self::Identifier(expression::Identifier::parse(parser)),
            Token::String(_) => Self::Literal(expression::Literal::parse(parser)),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn tmp_panic_out() {
        let input = "PRINT 1 + 2";
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);

        let result = parser.parse();
        println!("{:#?}", result);

        panic!("tmp panic out");
    }
}
