use crate::{
    lexer::Token, parser::{Parse, Parser, P}, span::Span
};

pub use self::expression::*;

// For testing/debugging
mod display;
mod evaluator;
mod expression;
mod statement;
pub mod visitor;

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
        let token = parser.consume();

        let kind = match &token {
            Token::Let => Self::parse_let(parser),
            Token::If => Self::parse_if(parser),
            Token::While => Self::parse_while(parser),
            token => unimplemented!("{:#?}", token), // Handle Err
        };

        //parser.consume_and_check(Token::SemiColon);
        if !parser.current_token_is(&Token::SemiColon) {
            let previous_span = Span {
                start: parser.token_span().start - 1,
                end: parser.token_span().start - 1,
            };
            parser
                .diagnostics
                .borrow_mut()
                .unexpected_token(&Token::SemiColon, parser.current_token(), &previous_span);
        }
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

impl<'a> Parse<'a> for Expr {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let mut kind = Self::parse_unary_or_primary(parser);

        if BinOp::token_match(parser.current_token()) {
            kind = Self::parse_binary(parser, kind, 0);
        }

        Expr { kind }
    }
}
