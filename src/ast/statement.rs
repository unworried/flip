use super::{Ast, Expr, Stmt, StmtKind};
use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

impl Stmt {
    /// Grammar: "if" (condition) "{" \n {statement}* "}"
    pub fn parse_if(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::LBrace) {
            panic!("expected: '{{', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        if !parser.current_token(&Token::RBrace) {
            panic!("expected: '}}', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        StmtKind::If(condition, resolution)
    }

    /// Grammar: "while" (condition) "{" \n {statement}* "}"
    pub fn parse_while(parser: &mut Parser) -> StmtKind {
        let condition = Expr::parse(parser);

        if !parser.current_token(&Token::LBrace) {
            panic!("expected: '{{', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        // Newline is optional May not need if allow newlines at start of file in ast root struct
        while parser.current_token(&Token::Newline) {
            parser.step();
        }

        let resolution = Ast::parse(parser, Token::RBrace).items;

        if !parser.current_token(&Token::RBrace) {
            panic!("expected: '}}', actual: '{:?}'", parser.current_token);
        }
        parser.step();

        StmtKind::While(condition, resolution)
    }

    /// Grammar: "let" (ident) "=" (expression)
    pub fn parse_let(parser: &mut Parser) -> StmtKind {
        //let ident = Ident::parse(parser);
        // Temp solution to seperate assignment from refernece. do this properly later...
        let ident = match &parser.current_token {
            Token::Ident(value) => value.to_owned(),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();

        if !parser.current_token(&Token::Assign) {
            panic!("expected: Assignment, actual: {:?}", parser.current_token);
        }
        parser.step();

        let expression = Expr::parse(parser);

        if !parser.symbols.insert(ident.to_owned()) {
            // Should this be to_owned??
            panic!("symbol: {:?} already defined", ident);
        }

        StmtKind::Let(ident, expression)
    }
}

