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

//#[cfg(test)]
//mod util;

#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Item>, // HashMap<ItemIdm, Item>
}

/// Grammar: {(statement);}*
impl Ast {
    pub fn parse(parser: &mut Parser, end_delim: Token) -> Self {
        let mut items = Vec::new();
        while !parser.current_token(&end_delim) {
            items.push(Item::parse(parser));

            while parser.current_token(&Token::Newline) {
                parser.step();
            }
        }

        Self { items }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    //pub id: ItemId,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    //Function(Function),
    Statement(Stmt),
}

impl<'a> Parse<'a> for Item {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let kind = ItemKind::Statement(Stmt::parse(parser));

        Self { kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    // "if" (condition) "{" \n {statement}* "}"
    If(Expr, Vec<Item>), // WARN: When funcs are added. need to change this to only allow stmts
    // "while" (condition) "{" \n {statement}* "}"
    While(Expr, Vec<Item>),
    // "let" (identifier) "=" (expression)
    Let(Ident, Expr),
    // "print" (expression)
    Print(Expr),
}

impl<'a> Parse<'a> for Stmt {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let token = parser.eat();

        let kind = match &token {
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            Token::Let => Self::parse_let(parser),
            Token::Print => Self::parse_print(parser),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        if !parser.current_token(&Token::SemiColon) {
            panic!("expected ';', actual: '{:?}'", parser.current_token);
        } // Seems to be working. needs more testing
        parser.step();

        Self { kind }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Ident(String), // Might not belong here
    Literal(expression::Literal),
}

impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        // Check which match order is faster. e.g. token first or op first
        if BinOp::token_match(&parser.next_token) {
            match &parser.current_token {
                Token::Int(_) | Token::Ident(_) => {
                    let kind = Self::parse_binary(parser);
                    return Expr { kind }; // May be cleaner solution
                }
                _ => {}
            }
        }

        if UnOp::token_match(&parser.current_token) {
            let kind = Self::parse_unary(parser);
            return Expr {
                kind, // Need further testing
            };
        }

        let kind = match &parser.current_token {
            Token::Int(_) | Token::String(_) => Self::parse_literal(parser),
            Token::Ident(_) => Self::parse_ident(parser),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        };

        parser.step();

        Expr { kind }
    }
}
