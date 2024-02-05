use alloc::borrow::ToOwned;

use super::{Ast, Expr, Ident, Stmt, StmtKind};
use crate::lexer::Token;
use crate::parser::{Parse, Parser, P};

#[derive(Debug)]
pub struct Local {
    pub pattern: Ident,
    pub init: P<Expr>,
}

impl Stmt {
    pub fn parse_assignment(parser: &mut Parser, ident: Ident) -> StmtKind {
        // Temp solution to seperate assignment from refernece. do this properly later...
        parser.consume_and_check(Token::Assign);

        let expression = P(Expr::parse(parser));

        let local = Local {
            pattern: ident,
            init: expression,
        };

        StmtKind::Assignment(P(local))
    }
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
        let start_span = parser.current_span();
        let ident = match &parser.current_token() {
            Token::Ident(value) => (value.to_owned(), start_span.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        };
        parser.step();

        parser.consume_and_check(Token::Assign);

        let expression = P(Expr::parse(parser));

        let local = Local {
            pattern: ident,
            init: expression,
        };
        StmtKind::Let(P(local))
    }
}
