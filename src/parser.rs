use crate::{
    ast::Program,
    lexer::{Lexer, Token},
};
use std::mem;

/// Owned Smart Pointer::: may not need this, inspired by rustc
pub struct P<T: ?Sized> {
    pub ptr: Box<T>,
}

#[allow(non_snake_case)]
pub fn P<T: 'static>(value: T) -> P<T> {
    P {
        ptr: Box::new(value),
    }
}

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

    pub fn parse(&mut self) -> Program {
        Program::parse(self)
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

    pub fn current_token(&self, token: Token) -> bool {
        self.current_token == token
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
        assert!(parser.current_token(Token::Int(1.to_string())));
    }

    #[test]
    fn parser_comp_next_token() {
        let input = "1";
        let mut lex = Lexer::new(input.to_string());
        let parser = Parser::new(&mut lex);
        assert!(parser.next_token(Token::Eof));
    }
}
