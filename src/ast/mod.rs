use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
};

use self::expression::*;

// Tmp for primitive symbol checking
pub use expression::Ident;

// For testing/debugging
mod display;
mod expression;
mod statement;
mod visitor;

#[cfg(test)]
mod util;

#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Item>, // HashMap<ItemIdm, Item>
}

impl Ast {
    pub fn parse(parser: &mut Parser, end_delim: Token) -> Self {
        let mut items = Vec::new();
        while !parser.current_token(&end_delim) {
            items.push(Item::parse(parser));

            //     parser.step(); // Change to do while
            while parser.current_token(&Token::Newline) {
                parser.step();
            }
        }

        Self { items }
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    //pub id: ItemId,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq)]
pub enum ItemKind {
    //Function(Function),
    Statement(Stmt),
}

/// Grammar: {statement} \n
impl<'a> Parse<'a> for Item {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let kind = ItemKind::Statement(Stmt::parse(parser));
        Self { kind }
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
    If(Expr, Vec<Item>), // WARN: When funcs are added. need to change this to only allow stmts
    // "WHILE" (condition) "REPEAT" \n {statement} "ENDWHILE"
    While(Expr, Vec<Item>),
    // "LET" (identifier) "=" (expression)
    Let(Ident, Expr),
}

impl<'a> Parse<'a> for Stmt {
    type Item = Self;

    fn parse(parser: &mut Parser<'a>) -> Self::Item {
        let token = parser.eat();

        let kind = match &token {
            Token::Print => Self::parse_print(parser),
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            Token::Let => Self::parse_let(parser),
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
                        kind: Self::parse_binary(parser),
                    }; // May be cleaner solution
                }
                _ => {}
            }
        }

        let kind = match &parser.current_token {
            Token::Int(_) | Token::String(_) => Self::parse_literal(parser),
            Token::Ident(_) => Self::parse_ident(parser),
            _ => {
                if UnOp::token_match(&parser.current_token) {
                    Self::parse_unary(parser)
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
mod tests {}
