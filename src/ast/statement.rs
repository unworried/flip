use alloc::borrow::ToOwned;

use super::{Ast, Expr, Stmt, StmtKind};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

impl Stmt {
    /// Grammar: "if" (condition) "{" \n {statement}* "}"
    pub fn parse_if(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        parser.consume_and_check(Token::LBrace);

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        parser.consume_and_check(Token::RBrace);

        StmtKind::If(condition, resolution)
    }

    /// Grammar: "while" (condition) "{" \n {statement}* "}"
    pub fn parse_while(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        parser.consume_and_check(Token::LBrace);

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token_is(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        parser.consume_and_check(Token::RBrace);

        StmtKind::While(condition, resolution)
    }

    /// Grammar: "let" (ident) "=" (expression)
    pub fn parse_let(parser: &mut Parser) -> StmtKind {
        //let ident = Ident::parse(parser);
        // Temp solution to seperate assignment from refernece. do this properly later...
        let ident = match &parser.current_token() {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };
        parser.step();

        parser.consume_and_check(Token::Assign);

        let expression = Expr::parse(parser);

        StmtKind::Let(ident, expression)
    }
}
