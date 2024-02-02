pub use self::token::Token;

#[cfg(test)]
mod tests;
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

    pub fn position(&self) -> usize {
        self.position
    }

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
        // u8.is_ascii_whitespace() but without the newline
        while matches!(self.ch, b'\t' | b'\x0C' | b'\r' | b' ') {
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
