pub use self::ptr::*;

use crate::{
    ast::Ast,
    lexer::{Lexer, Token},
};
use std::mem;

mod ptr;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub current_token: Token,
    pub next_token: Token,
}

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>) -> Self;
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            next_token,
        }
    }

    pub fn parse(&mut self) -> Ast {
        Ast::parse(self, Token::Eof)
    }

    pub fn step(&mut self) {
        // Cheaper than cloning
        self.current_token = self.lexer.next_token();
        mem::swap(&mut self.current_token, &mut self.next_token);
    }

    pub fn eat(&mut self) -> Token {
        // Not sure how mem safe this solution is, seems hacky. Clone Copy expensive but may be
        // safer option
        let prev_token = mem::replace(&mut self.current_token, Token::Illegal);
        self.step();
        prev_token
    }

    pub fn current_token(&self, token: &Token) -> bool {
        &self.current_token == token
    }

    pub fn next_token(&self, token: Token) -> bool {
        self.next_token == token
    }

    pub fn current_position(&self) -> usize {
        self.lexer.position()
    }
}
