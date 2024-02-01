use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

use self::expression::BinOp;

// For testing/debugging
mod display;
mod expression;
mod statement;

#[cfg(test)]
mod util;

#[derive(Debug)]
pub struct Program {
    statements: Vec<Stmt>,
}

/// Grammar: {statement} \n
impl<'a> Parse<'a> for Program {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let mut statements = Vec::new();
        while !parser.current_token(Token::Eof) {
            statements.push(Stmt::parse(parser));

            parser.step(); // Change to do while
            while parser.current_token(Token::Newline) {
                parser.step();
            }
        }

        Self { statements }
    }
}

#[derive(Debug, PartialEq)]
enum Stmt {
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

impl<'a> Parse<'a> for Stmt {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let token = parser.eat();

        let statement = match &token {
            Token::Print => Self::Print(statement::Print::parse(parser)),
            Token::If => Self::If(statement::If::parse(parser)),
            Token::While => Self::While(statement::While::parse(parser)),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        while parser.current_token(Token::Newline) {
            parser.step();
        } // Dont think this should be here
        statement
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(expression::Binary),
    // Unary(expression::Unary
    Ident(expression::Ident), // Might not belong here
    Primitive(expression::Primitive),
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let expr = match &parser.current_token {
            Token::Int(_) => {
                if BinOp::token_match(&parser.next_token) {
                    Self::Binary(expression::Binary::parse(parser))
                } else {
                    Self::Primitive(expression::Primitive::parse(parser))
                }
            }
            Token::Ident(_) => Self::Ident(expression::Ident::parse(parser)),
            Token::String(_) => Self::Literal(expression::Literal::parse(parser)),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        };

        parser.step();

        expr
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn tmp_panic_out() {
        let input = r#"WHILE 1 REPEAT
                IF 1 == 2 THEN
                    PRINT 1
                ENDIF
            ENDWHILE"#;
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);

        let result = parser.parse();
        println!("{}", result);

        panic!("Debug Panic");
    }
}
