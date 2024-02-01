pub use self::ptr::*;
use std::collections::HashSet;

use crate::{
    ast::{Block, Ident},
    lexer::{Lexer, Token},
};
use std::mem;

mod ptr;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub current_token: Token,
    pub next_token: Token,

    // Look at better solutions in future
    pub symbols: HashSet<Ident>,
}

pub trait Parse<'a>
where
    Self: Sized,
{
    type Item;
    fn parse(parser: &mut Parser<'a>) -> Self::Item;
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            next_token,
            symbols: HashSet::new(),
        }
    }

    pub fn parse(&mut self) -> Block {
        Block::parse(self, Token::Eof)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_step() {
        let input = "1 2 3 4";
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);
        assert_eq!(parser.current_token, Token::Int(1.to_string()));
        assert_eq!(parser.next_token, Token::Int(2.to_string()));
        parser.step();
        assert_eq!(parser.current_token, Token::Int(2.to_string()));
        assert_eq!(parser.next_token, Token::Int(3.to_string()));
        parser.step();
        assert_eq!(parser.current_token, Token::Int(3.to_string()));
        assert_eq!(parser.next_token, Token::Int(4.to_string()));
        parser.step();
        assert_eq!(parser.current_token, Token::Int(4.to_string()));
        assert_eq!(parser.next_token, Token::Eof);
    }

    #[test]
    fn parser_eat() {
        let input = "1 2 3 4";
        let mut lex = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lex);
        assert_eq!(parser.current_token, Token::Int(1.to_string()));
        assert_eq!(parser.eat(), Token::Int(1.to_string()));
        assert_eq!(parser.current_token, Token::Int(2.to_string()));
        assert_eq!(parser.eat(), Token::Int(2.to_string()));
        assert_eq!(parser.current_token, Token::Int(3.to_string()));
        assert_eq!(parser.eat(), Token::Int(3.to_string()));
        assert_eq!(parser.current_token, Token::Int(4.to_string()));
        assert_eq!(parser.eat(), Token::Int(4.to_string()));
        assert_eq!(parser.current_token, Token::Eof);
    }

    #[test]
    fn parser_comp_current_token() {
        let input = "1";
        let mut lex = Lexer::new(input.to_string());
        let parser = Parser::new(&mut lex);
        assert!(parser.current_token(&Token::Int(1.to_string())));
    }

    #[test]
    fn parser_comp_next_token() {
        let input = "1";
        let mut lex = Lexer::new(input.to_string());
        let parser = Parser::new(&mut lex);
        assert!(parser.next_token(Token::Eof));
    }
}
