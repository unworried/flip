use alloc::vec::Vec;

use crate::{
    lexer::Token,
    parser::{Parse, Parser, P},
    span::Span,
};

pub use self::expression::*;
use self::statement::Local;

// For testing/debugging
mod display;
pub mod expression;
pub mod statement;

#[derive(Debug)]
pub struct Ast<'a> {
    pub items: Vec<Item<'a>>, // HashMap<ItemIdm, Item>
}

/// Grammar: {(statement);}*
impl<'a> Ast<'a> {
    pub fn parse(parser: &mut Parser<'a>, end_delim: Token) -> Self {
        let mut items = Vec::new();
        while !parser.current_token_is(&end_delim) {
            items.push(Item::parse(parser));

            while parser.current_token_is(&Token::Newline) {
                parser.step();
            }
        }

        Self { items }
    }
}

#[derive(Debug)]
pub struct Item<'a> {
    pub kind: ItemKind<'a>,
}

#[derive(Debug)]
pub enum ItemKind<'a> {
    //Function(Function),
    Statement(Stmt<'a>),
}

impl<'a> Parse<'a> for Item<'a> {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let kind = ItemKind::Statement(Stmt::parse(parser));

        Self { kind }
    }
}

#[derive(Debug)]
pub struct Stmt<'a> {
    pub kind: StmtKind<'a>,
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind<'a> {
    // "let" (identifier) "=" (expression)
    Let(P<Local<'a>>), // Fix this
    // (variable) "=" (expression)
    Assignment(Ident, P<Expr>),
    // "if" (condition) "{" \n {statement}* "}"
    If(Expr, Vec<Item<'a>>), // WARN: When funcs are added. need to change this to only allow stmts
    // "while" (condition) "{" \n {statement}* "}"
    While(Expr, Vec<Item<'a>>),
    Error,
}

impl<'a> Parse<'a> for Stmt<'a> {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let (token, start_span) = parser.consume();

        let kind = match &token {
            Token::Let => Self::parse_let(parser),
            Token::Ident(ident) => Self::parse_assignment(parser, (ident.to_owned(), start_span.clone())),
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            token => {
                parser
                    .diagnostics
                    .borrow_mut()
                    .unexpected_statement(token, &start_span);
                StmtKind::Error
            } // Handle Err
        };

        parser.consume_and_check(Token::SemiColon);

        Self {
            kind,
            span: Span::combine(vec![start_span, parser.current_span().clone()]),
        }
    }
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
    Binary(BinOp, P<Expr>, P<Expr>),
    Unary(UnOp, P<Expr>),
    Literal(expression::Literal),
    Variable(Ident),
    Error,
}

impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let start_span = parser.current_span().clone();

        let mut kind = Self::parse_unary_or_primary(parser);

        if BinOp::token_match(parser.current_token()) {
            kind = Self::parse_binary(parser, kind, 0);
        }

        Expr {
            kind,
            span: Span::combine(vec![start_span, parser.current_span().clone()]),
        }
    }
}
