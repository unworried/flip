use core::mem;

use self::combinators::parse_program;
use crate::ast::Program;
use crate::diagnostics::DiagnosticsCell;
use crate::lexer::{Lexer, Token};
use crate::span::Span;

pub mod combinators;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: (Token, Span),
    next_token: (Token, Span),

    pub diagnostics: DiagnosticsCell,
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

    pub fn parse(&mut self) -> Program {
        parse_program(self)
    }

    pub fn step(&mut self) {
        if self.current_token_is(&Token::Eof) {
            return;
        }

        // Cheaper than cloning
        self.current_token = self.lexer.next_token();
        mem::swap(&mut self.current_token, &mut self.next_token);
        if self.current_token.0 == Token::Whitespace {
            self.step();
        }

        if self.current_token.0 == Token::Illegal {
            self.diagnostics
                .borrow_mut()
                .illegal_token(&self.current_token.1);

            self.step();
        }
    }

    pub fn consume(&mut self) -> (Token, Span) {
        // Not sure how mem safe this solution is, seems hacky. Clone Copy expensive but may be
        // safer option
        //let prev_token = mem::replace(&mut self.current_token.0, Token::Illegal);
        let prev_token = self.current_token.clone();
        self.step();
        prev_token
    }

    pub fn expect(&mut self, expected: Token) {
        self.expect_with_outcome(expected);
    }

    pub fn expect_with_outcome(&mut self, expected: Token) -> bool {
        let (token, span) = self.consume();
        if token != expected {
            self.diagnostics
                .borrow_mut()
                .expected_token(&expected, &token, &span);

            return false;
        }

        true
    }

    pub fn optional(&mut self, optional: Token) {
        if self.current_token_is(&optional) {
            self.step();
        }
    }

    fn step_until(&mut self, token: &Token) {
        while !self.current_token_is(token) && !self.current_token_is(&Token::Eof) {
            self.step();
        }
    }

    fn expect_flush(&mut self) {
        if Span::difference(&self.current_token.1, &self.next_token.1) > 1 {
            self.diagnostics.borrow_mut().expected_token(
                self.current_token(),
                &Token::Whitespace,
                &Span::new(self.current_token.1.end + 1, self.next_token.1.start - 1),
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

    pub fn current_span(&self) -> Span {
        self.current_token.1
    }

    pub fn next_token(&self) -> &Token {
        &self.next_token.0
    }
}
