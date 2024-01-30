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
    fn read_char() {
        let input = "A";
        let mut lex = Lexer::new(input.to_string());

        assert_eq!(lex.ch, b'A');

        lex.read_char();
        assert_eq!(lex.ch, 0);
    }

    #[test]
    fn read_chars_from_input() {
        let input = "LET foo = 1";
        let mut lex = Lexer::new(input.to_string());

        for ch in input.chars() {
            assert_eq!(lex.ch, ch as u8);
            lex.read_char();
        }
    }

    #[test]
    fn read_char_from_empty_input() {
        let input = "";
        let lex = Lexer::new(input.to_string());

        assert_eq!(lex.ch, 0);
    }

    #[test]
    fn peek_char() {
        let input = "AB";
        let lex = Lexer::new(input.to_string());

        assert_eq!(lex.ch, b'A');
        assert_eq!(lex.peek(), b'B');
    }

    #[test]
    fn peek_char_test_pure() {
        let input = "AB";
        let lex = Lexer::new(input.to_string());

        assert_eq!(lex.peek(), b'B');
        assert_eq!(lex.peek(), b'B');
    }

    #[test]
    fn peek_all_chars() {
        let input = "LET bar = 55";
        let mut lex = Lexer::new(input.to_string());

        while lex.peek() != 0 {
            let next_char = lex.peek();
            lex.read_char();
            assert_eq!(lex.ch, next_char);
        }
    }

    #[test]
    fn peek_char_eof() {
        let input = "A";
        let lex = Lexer::new(input.to_string());

        assert_eq!(lex.ch, b'A');
        assert_eq!(lex.peek(), 0);
    }
}
