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
mod tests;

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

#[derive(Debug)]
pub struct Item {
    //pub id: ItemId,
    pub kind: ItemKind,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
}

#[derive(Debug)]
pub enum StmtKind {
    // "let" (identifier) "=" (expression)
    Let(Ident, Expr),
    // "if" (condition) "{" \n {statement}* "}"
    If(Expr, Vec<Item>), // WARN: When funcs are added. need to change this to only allow stmts
    // "while" (condition) "{" \n {statement}* "}"
    While(Expr, Vec<Item>),
}

impl<'a> Parse<'a> for Stmt {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let token = parser.eat();

        let kind = match &token {
            Token::Let => Self::parse_let(parser),
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        if !parser.current_token(&Token::SemiColon) {
            panic!("expected ';', actual: '{:?}'", parser.current_token);
        } // Seems to be working. needs more testing
        parser.step();

        Self { kind }
    }
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Ident(String), // Might not belong here
    Literal(expression::Literal),
}
// PAREN ONLY WORKS WHEN Atomic val is on left. e.g. 1 + (2 + 3)
impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        if BinOp::token_match(&parser.next_token) | parser.current_token(&Token::LParen) {
            match &parser.current_token {
                Token::Int(_) | Token::Ident(_) => {
                    let kind = Self::parse_binary(parser, 0);
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

        // Check which match order is faster. e.g. token first or op first


        let kind = match &parser.current_token {
            Token::Int(_) | Token::String(_) => Self::parse_literal(parser),
            Token::Ident(_) => Self::parse_ident(parser),
            _ => unimplemented!("{}", &parser.current_token), // Handle Err
        };

        parser.step();

        Expr { kind }
    }
}
