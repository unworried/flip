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
pub struct Block {
    pub statements: Vec<Stmt>,
}

/// Grammar: {statement} \n
impl<'a> Block {
    pub fn parse(parser: &mut Parser<'a>, end_delim: Token) -> Self {
        let mut statements = Vec::new();
        while !parser.current_token(&end_delim) {
            statements.push(Stmt::parse(parser));

            //     parser.step(); // Change to do while
            while parser.current_token(&Token::Newline) {
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
    Label(Ident),
    // "GOTO" (identifier)
    Goto(Ident),
    // "LET" (identifier) "=" (expression)
    Let(Ident, Expr),
    // "INPUT" (identifier)
    Input(Ident),
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
        // Check which match order is faster. e.g. token first or op first
        if BinOp::token_match(&parser.next_token) {
            match &parser.current_token {
                Token::Int(_) | Token::Ident(_) => {
                    return Expr {
                        kind: Binary::parse(parser),
                    }; // May be cleaner solution
                }
                _ => {}
            }
        }

        let kind = match &parser.current_token {
            Token::Int(_) => ExprKind::Literal(Literal::parse(parser)),
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

pub type Ident = String;
impl<'a> Parse<'a> for Ident {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self {
        match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn identifier() {
        let mut lexer = Lexer::new("test".to_string());
        let mut parser = Parser::new(&mut lexer);

        assert_eq!(Ident::parse(&mut parser), "test".to_owned());
    }
}
