use alloc::vec::Vec;

pub use self::expression::*;
use self::statement::Local;
use crate::lexer::Token;
use crate::parser::{Parse, Parser, P};
use crate::span::Span;

// For testing/debugging
mod display;
pub mod expression;
pub mod statement;

#[derive(Debug)]
pub struct Ast {
    pub items: Vec<Item>, // HashMap<ItemIdm, Item>
}

/// Grammar: {(statement);}*
impl Ast {
    pub fn parse(parser: &mut Parser, end_delim: Token) -> Self {
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
pub struct Item {
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
    pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
    // "let" (identifier) "=" (expression)
    Let(P<Local>), // Fix this
    // (variable) "=" (expression)
    Assignment(P<Local>),
    // "if" (condition) "{" \n {statement}* "}"
    If(Expr, Vec<Item>), // WARN: When funcs are added. need to change this to only allow stmts
    // "while" (condition) "{" \n {statement}* "}"
    While(Expr, Vec<Item>),
    Error,
}

impl<'a> Parse<'a> for Stmt {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let (token, start_span) = parser.consume();

        let kind = match &token {
            Token::Let => Self::parse_let(parser),
            Token::Ident(ident) => {
                Self::parse_assignment(parser, (ident.to_owned(), start_span.clone()))
            }
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
            span: Span::combine(vec![&start_span, &parser.current_span()]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
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
            span: Span::combine(vec![&start_span, &parser.current_span()]),
        }
    }
}
