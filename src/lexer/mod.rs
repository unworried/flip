pub use self::token::Token;

mod token;

const EOF: u8 = 0;

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
            ch: EOF,
        };
        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let token = match self.ch {
            b'\"' => Token::String(self.read_string()),

            b'0'..=b'9' => Token::Int(self.read_integer()),

            b'a'..=b'z' | b'A'..=b'Z' => Token::from(self.read_identifier()),

            b'=' | b'!' | b'>' | b'<' => {
                if self.peek() == b'=' {
                    let prev_ch = self.ch;
                    self.read_char();
                    // FIXME: Change to Vec<u8> From impl or seperate pub fn
                    Token::from(format!("{}{}", prev_ch as char, self.ch as char))
                } else {
                    Token::from(self.ch)
                }
            }

            _ => Token::from(self.ch),
        };

        self.read_char();
        token
    }

    fn peek(&self) -> u8 {
        if self.position >= self.input.len() {
            return EOF;
        }

        self.input[self.position]
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = EOF;
        } else {
            self.ch = self.input[self.position];
        }

        self.position += 1;
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.read_char();

        while self.ch != b'\"' {
            if self.ch == EOF {
                // TODO: something more robust
                panic!("Unexpected EOF: missing trailing \" ");
            }

            string.push(self.ch as char);
            self.read_char();
        }

        string
    }

    fn read_integer(&mut self) -> String {
        let mut integer = String::new();

        loop {
            integer.push(self.ch as char);

            match self.peek() {
                b'0'..=b'9' => {}
                _ => break,
            }

            self.read_char();
        }

        integer
    }

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

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.ch == b'#' {
            while self.ch != b'\n' && self.ch != EOF {
                // TODO: may need to check for EOF
                self.read_char();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_tokens(input: &str, expected: Vec<Token>) {
        let mut lex = Lexer::new(input.to_string());

        for token in expected {
            let next_token = lex.next_token();
            println!("expected: {:?}, got: {:?}", token, next_token);
            assert_eq!(next_token, token);
        }
    }

    #[test]
    fn read_char() {
        let input = "A";
        let mut lex = Lexer::new(input.to_string());

        assert_eq!(lex.ch, b'A');

        lex.read_char();
        assert_eq!(lex.ch, 0);
    }

    #[test]
    fn read_multiple_chars() {
        let input = "LET foo = 1";
        let mut lex = Lexer::new(input.to_string());

        for ch in input.chars() {
            assert_eq!(lex.ch, ch as u8);
            lex.read_char();
        }
    }

    #[test]
    fn read_char_empty_input() {
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
    fn peek_multiple_chars() {
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
        assert_eq!(lex.peek(), EOF);
    }

    #[test]
    fn tokenize_whitespace() {
        let input = " \t\n\r";
        let mut lex = Lexer::new(input.to_string());

        let next_token = lex.next_token();
        assert_eq!(next_token, Token::Eof);
    }

    #[test]
    fn tokenize_arithmetic_operations() {
        let input = "+-*/";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_with_whitespace() {
        let input = "+- */";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_operations() {
        let input = "+- */ >>= = !=";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::ForwardSlash,
            Token::GreaterThan,
            Token::GreaterThanEqual,
            Token::Assign,
            Token::NotEqual,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_comment() {
        let input = "# this is a comment";
        let mut lex = Lexer::new(input.to_string());

        let next_token = lex.next_token();
        assert_eq!(next_token, Token::Eof);
    }

    #[test]
    fn tokenize_comment_with_newline() {
        let input = "# this is a comment\n";
        let mut lex = Lexer::new(input.to_string());

        let next_token = lex.next_token();
        assert_eq!(next_token, Token::Newline);
        let next_token = lex.next_token();
        assert_eq!(next_token, Token::Eof);
    }

    #[test]
    fn tokenize_with_comment() {
        let input = "+- # comment here == <= >= != = -- , ;\n */";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Newline,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_string() {
        let input = "+- \"string12345\" # comment \n */";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::String(String::from("string12345")),
            Token::Newline,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_int() {
        let input = "+-123 98654#comment\n*/";

        let expected = vec![
            Token::Plus,
            Token::Minus,
            Token::Int(String::from("123")),
            Token::Int(String::from("98654")),
            Token::Newline,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected);
    }

    #[test]
    fn tokenize_complete() {
        let input = "IF+-123 foo*THEN/98654#comment\n*/";

        let expected = vec![
            Token::If,
            Token::Plus,
            Token::Minus,
            Token::Int(String::from("123")),
            Token::Ident(String::from("foo")),
            Token::Asterisk,
            Token::Then,
            Token::ForwardSlash,
            Token::Int(String::from("98654")),
            Token::Newline,
            Token::Asterisk,
            Token::ForwardSlash,
        ];

        check_tokens(input, expected)
    }
}
