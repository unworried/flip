use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use self::{expression::*, statement::*};

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
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
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
pub struct Stmt {
    pub kind: StmtKind,
}

#[derive(Debug, PartialEq)]
pub enum StmtKind {
    // "PRINT" (expression)
    Print(Expr),
    // "IF" (condition) "THEN" \n {statement} "ENDIF"
    If(Expr, Vec<Stmt>),
    // "WHILE" (condition) "REPEAT" \n {statement} "ENDWHILE"
    While(Expr, Vec<Stmt>),
    // "LABEL" (identifier)
    Label(expression::Ident), // Move out of Expression file
    // "GOTO" (identifier)
    Goto(expression::Ident),
    // "LET" (identifier) "=" (expression)
    Let(expression::Ident, Expr),
    // "INPUT" (identifier)
    Input(expression::Ident),
}

impl<'a> Parse<'a> for Stmt {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let token = parser.eat();

        let kind = match &token {
            Token::Print => Print::parse(parser),
            Token::If => If::parse(parser),
            Token::While => While::parse(parser),
            Token::Label => Label::parse(parser),
            Token::Goto => Goto::parse(parser),
            Token::Let => Let::parse(parser),
            Token::Input => Input::parse(parser),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        while parser.current_token(Token::Newline) {
            parser.step();
        } // Dont think this should be here

        Self { kind }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Ident(String), // Might not belong here
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expr {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self {
        let kind = match &parser.current_token {
            Token::Int(_) => {
                if BinOp::token_match(&parser.next_token) {
                    Binary::parse(parser)
                } else {
                    ExprKind::Literal(Literal::parse(parser))
                }
            }
            Token::Ident(_) => ExprKind::Ident(Ident::parse(parser)),
            Token::String(_) => ExprKind::Literal(Literal::parse(parser)),
            _ => {
                if UnOp::token_match(&parser.current_token) {
                    expression::Unary::parse(parser)
                } else {
                    unimplemented!("{}", &parser.current_token) // Handle Err
                }
            }
        };

        parser.step();

        Expr { kind }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn tmp_panic_out() {
        /*let input = r#"WHILE 1 REPEAT
                IF 1 == 2 THEN
                    PRINT 1
                ENDIF
            ENDWHILE"#;
        */
        //let input = r#"PRINT 1 + 2 * -4"#;
        let input = r#"LET foo =3 + 2"#;
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);

        let result = parser.parse();
        //println!("{}", result);

        println!("{:#?}", result.statements[0]);

        panic!("Debug Panic");
    }
}
