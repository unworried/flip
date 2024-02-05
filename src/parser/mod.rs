use self::ast::Ast;
pub use self::ptr::*;

use crate::{
    cache::Cache,
    diagnostics::DiagnosticsCell,
    lexer::{Lexer, Token},
    span::Span,
};
use core::mem;

pub mod ast;
mod evaluator;
mod ptr;
pub mod visitor;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: (Token, Span),
    next_token: (Token, Span),

    pub diagnostics: DiagnosticsCell,
}

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>) -> Self;
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, diagnostics: DiagnosticsCell) -> Self {
        // Need to handle case where these tokens may be illegal
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            next_token,
            diagnostics,
        }
    }

    pub fn parse(&mut self) -> Ast {
        Ast::parse(self, Token::Eof)
    }

    pub fn step(&mut self) {
        // Cheaper than cloning
        self.current_token = self.lexer.next_token();
        mem::swap(&mut self.current_token, &mut self.next_token);
        if self.current_token.0 == Token::Whitespace {
            self.step();
        }

        /*if self.current_token.0 == Token::Illegal {
            self.diagnostics
                .borrow_mut()
                .illegal_token(&self.current_token.1);
        }*/
    }

    pub fn consume(&mut self) -> (Token, Span) {
        // Not sure how mem safe this solution is, seems hacky. Clone Copy expensive but may be
        // safer option
        //let prev_token = mem::replace(&mut self.current_token.0, Token::Illegal);
        let prev_token = self.current_token.clone();
        self.step();
        prev_token
    }

    pub fn consume_and_check(&mut self, token: Token) {
        if !self.current_token_is(&token) {
            self.diagnostics.borrow_mut().unexpected_token(
                &token,
                &self.current_token.0,
                &self.current_token.1,
            );
        }
        self.step();
    }

    pub fn current_token(&self) -> &Token {
        &self.current_token.0
    }

    pub fn current_token_is(&self, token: &Token) -> bool {
        &self.current_token.0 == token
    }

    pub fn current_span(&self) -> &Span {
        &self.current_token.1
    }

    pub fn next_token(&self) -> &Token {
        &self.next_token.0
    }

    pub fn next_token_is(&self, token: &Token) -> bool {
        &self.next_token.0 == token
    }

    pub fn next_span(&self) -> &Span {
        &self.next_token.1
    }

    pub fn current_position(&self) -> usize {
        self.lexer.position()
    }
}
