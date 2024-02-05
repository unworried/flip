//! lexer/mod.rs - Performs lexical anaylsis on the input source code. The lexer is responsible for
//! converting the input source code into a token stream attaching a span to each token to be used
//! when parsing.
//!
//! The lexer is implemented as an iterator that returns a token and a span for each token found.
//! The lexer is also responsible for skipping whitespace and comments.
use alloc::string::String;
use alloc::vec::Vec;

pub use self::token::Token;
use crate::span::Span;

#[cfg(test)]
mod tests;
mod token;

const EOF: u8 = 0;

/// Iterator used to lex compiler input.
///
/// Next characters can be peeked using `peek` and read using `read_char`.
/// The `read_char` method will bump the current character being read.
/// The lexer will return a `Token` and a `Span` for each token found.
pub struct Lexer {
    /// Source Input
    input: Vec<u8>,
    /// Current position in the input
    position: usize,
    /// Current character being read
    ch: u8,
}

impl Lexer {
    /// Creates a new lexer from the input source code.
    /// The input is converted into a byte array.
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            input: input.into_bytes(),
            position: 0,
            ch: EOF,
        };
        lex.read_char();

        lex
    }

    /// Returns the next token and matching span.
    // May be able to change a bit to fix indexing properly
    pub fn next_token(&mut self) -> (Token, Span) {
        //self.skip_whitespace();
        self.skip_comment();
        let start_position = self.position - 1;

        if Self::is_whitespace(self.ch) {
            // - 1 to get the last whitespace char not the next non whitespace char
            let span = Span::new(start_position, self.position - 1);
            self.read_char();
            return (self.whitespace(), span);
        }

        let token = match self.ch {
            b'\"' => Token::String(self.read_string()),

            b'0'..=b'9' => Token::Int(self.read_integer()),

            b'a'..=b'z' | b'A'..=b'Z' => Token::from(self.read_identifier()),

            b'=' | b'!' | b'>' | b'<' => {
                if self.peek() == b'=' {
                    let prev_ch = self.ch;
                    self.read_char();

                    Token::from((prev_ch, self.ch))
                    //Token::from(format!("{}{}", prev_ch as char, self.ch as char))
                } else {
                    Token::from(self.ch)
                }
            }

            _ => Token::from(self.ch),
        };

        let span = Span::new(start_position, self.position - 1);
        self.read_char();
        (token, span)
    }

    /// Returns the next character without bumping the current position.
    fn peek(&self) -> u8 {
        if self.position >= self.input.len() {
            return EOF;
        }

        self.input[self.position]
    }

    /// Bumps the current character being read.
    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = EOF;
        } else {
            self.ch = self.input[self.position];
        }

        self.position += 1;
    }

    /// Reads a string input and returns it.
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.read_char();

        while self.ch != b'\"' {
            if self.ch == EOF {
                // TODO: something more robust
                panic!("Unexpected EOF: missing trailing \"");
            }

            string.push(self.ch as char);
            self.read_char();
        }

        string
    }

    /// Reads an integer input and returns it.
    fn read_integer(&mut self) -> isize {
        let mut integer = String::new();

        loop {
            integer.push(self.ch as char);

            match self.peek() {
                b'0'..=b'9' => {}
                _ => break,
            }

            self.read_char();
        }

        integer.parse().unwrap() // TODO HANDLE ERRR
    }

    /// Reads an ident input and returns it.
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        loop {
            identifier.push(self.ch as char);

            if !self.peek().is_ascii_alphanumeric() {
                break;
            }

            self.read_char();
        }

        identifier
    }

    /// Returns true if the character is a whitespace.
    fn is_whitespace(ch: u8) -> bool {
        // u8.is_ascii_whitespace() but without the newline
        matches!(ch, b'\t' | b'\x0C' | b'\r' | b' ')
    }

    /// Combines multiple whitespace characters into a single Token/Span and returns it.
    fn whitespace(&mut self) -> Token {
        while Self::is_whitespace(self.ch) {
            self.read_char();
        }
        Token::Whitespace
    }

    /// Skips all input after a comment character is found until a newline or EOF is found.
    fn skip_comment(&mut self) {
        if self.ch == b'#' {
            while self.ch != b'\n' && self.ch != EOF {
                // TODO: may need to check for EOF
                self.read_char();
            }
        }
    }
}
