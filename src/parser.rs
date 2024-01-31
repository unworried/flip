use crate::{
    ast::Ast,
    lexer::{Lexer, Token},
};
use std::mem;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub current_token: Token,
    pub next_token: Token,
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
        Ast::parse(self)
    }

    pub fn step(&mut self) {
        // Cheaper than cloning
        self.current_token = self.lexer.next_token();
        mem::swap(&mut self.current_token, &mut self.next_token);
        println!("cur: {}, next: {}", self.current_token, self.next_token)
    }

    pub fn current_token(&self, token: Token) -> bool {
        self.current_token == token
    }

    pub fn next_token(&self, token: Token) -> bool {
        self.next_token == token
    }
}

pub trait Parse<'a>
where
    Self: Sized,
{
    fn parse(parser: &mut Parser<'a>) -> Self;
}
