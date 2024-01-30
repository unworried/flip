use anyhow::Result;

#[derive(Debug)]
pub enum Token {}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            input: input.into_bytes(),
            position: 0,
            ch: 0,
        };
        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Result<Token> {
        todo!()
    }

    fn peek(&self) -> u8 {
        if self.position >= self.input.len() {
            return 0;
        }

        self.input[self.position]
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.position];
        }

        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_char() {
        let input = "LET foo = 1";
        let mut lex = Lexer::new(input.to_string());

        for ch in input.chars() {
            assert_eq!(lex.ch, ch as u8);
            lex.read_char();
        }
    }

    #[test]
    fn test_peek_next_char() {
        let input = "LET bar = 55";
        let mut lex = Lexer::new(input.to_string());

        while lex.peek() != 0 {
            let next_char = lex.peek();
            lex.read_char();
            assert_eq!(lex.ch, next_char);
        }
    }
}
