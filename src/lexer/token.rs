use std::fmt::Display;

#[derive(Debug, PartialEq)] // TODO: try to remove clone
pub enum Token {
    Illegal,
    Eof,
    Newline,

    Assign,
    Ident(String),
    Int(String), // Seperate into litterals. keep as int(string)??
    String(String),

    /// Operators
    Equal,
    NotEqual,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    LesserThan,
    LesserThanEqual,
    GreaterThan,
    GreaterThanEqual,

    /// Keywords
    Print,
    Let,
    If,
    Then,
    EndIf,
    While,
    Repeat,
    EndWhile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self)
    }
}

impl From<u8> for Token {
    fn from(ch: u8) -> Self {
        match ch {
            b'=' => Self::Assign,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Asterisk,
            b'/' => Self::ForwardSlash,
            b'<' => Self::LesserThan,
            b'>' => Self::GreaterThan,
            b'\n' => Self::Newline,
            b'\0' => Self::Eof,

            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "<=" => Self::LesserThanEqual,
            ">=" => Self::GreaterThanEqual,

            "PRINT" => Self::Print,
            "LET" => Self::Let,
            "IF" => Self::If,
            "THEN" => Self::Then,
            "ENDIF" => Self::EndIf,
            "WHILE" => Self::While,
            "REPEAT" => Self::Repeat,
            "ENDWHILE" => Self::EndWhile,

            _ => Self::Ident(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn illegal() {
        assert_eq!(Token::from(b' '), Token::Illegal);
    }

    #[test]
    fn eof() {
        assert_eq!(Token::from(0), Token::Eof);
    }

    #[test]
    fn newline() {
        assert_eq!(Token::from(b'\n'), Token::Newline);
    }

    #[test]
    fn equal() {
        assert_eq!(Token::from(String::from("==")), Token::Equal);
    }

    #[test]
    fn not_equal() {
        assert_eq!(Token::from(String::from("!=")), Token::NotEqual);
    }

    #[test]
    fn plus() {
        assert_eq!(Token::from(b'+'), Token::Plus);
    }

    #[test]
    fn minus() {
        assert_eq!(Token::from(b'-'), Token::Minus);
    }

    #[test]
    fn asterisk() {
        assert_eq!(Token::from(b'*'), Token::Asterisk);
    }

    #[test]
    fn forward_slash() {
        assert_eq!(Token::from(b'/'), Token::ForwardSlash);
    }

    #[test]
    fn lesser_than() {
        assert_eq!(Token::from(b'<'), Token::LesserThan);
    }

    #[test]
    fn lesser_than_equal() {
        assert_eq!(Token::from(String::from("<=")), Token::LesserThanEqual);
    }

    #[test]
    fn greater_than() {
        assert_eq!(Token::from(b'>'), Token::GreaterThan);
    }

    #[test]
    fn greater_than_equal() {
        assert_eq!(Token::from(String::from(">=")), Token::GreaterThanEqual);
    }
}
