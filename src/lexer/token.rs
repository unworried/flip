use alloc::string::String;
use core::fmt::{Display, Formatter, Result};

/// Token enum representing a lexical token in the input source.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special
    Assign,
    Ident(String),
    Illegal,
    Eof,
    Newline,
    // Whitespace sequence of length n
    Whitespace,

    // Literals
    Int(isize),
    String(String),

    /// Operators
    Equal,
    NotEqual,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    /// Keywords
    Let,
    If,
    Else,
    While,
    Print,

    // Separators
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let display = match self {
            Token::Assign => "=",
            Token::Ident(val) => return write!(f, "Ident({})", val),
            Token::Illegal => "<Illegal>",
            Token::Eof => "EoF",
            Token::Newline => r#"\n"#,
            Token::Int(val) => return write!(f, "Integer({})", val),
            Token::String(val) => return write!(f, "String({})", val),
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::ForwardSlash => "/",
            Token::LessThan => "<",
            Token::LessThanEqual => "<=",
            Token::GreaterThan => ">",
            Token::GreaterThanEqual => ">=",
            Token::Let => "let",
            Token::If => "if",
            Token::Else => "else",
            Token::While => "while",
            Token::Print => "print",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::SemiColon => ";",
            Token::Whitespace => r#" "#,
        };

        write!(f, "{}", display)
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
            b'<' => Self::LessThan,
            b'>' => Self::GreaterThan,
            b'\n' => Self::Newline,
            b'\0' => Self::Eof,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b';' => Self::SemiColon,

            _ => Self::Illegal,
        }
    }
}

impl From<(u8, u8)> for Token {
    fn from((ch, next_ch): (u8, u8)) -> Self {
        match (ch, next_ch) {
            (b'=', b'=') => Self::Equal,
            (b'!', b'=') => Self::NotEqual,
            (b'<', b'=') => Self::LessThanEqual,
            (b'>', b'=') => Self::GreaterThanEqual,

            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            /*
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "<=" => Self::LessThanEqual,
            ">=" => Self::GreaterThanEqual,
            */
            "let" => Self::Let,
            "if" => Self::If,
            "else" => Self::Else,
            "while" => Self::While,
            "print" => Self::Print,

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
        assert_eq!(Token::from((b'=', b'=')), Token::Equal);
    }

    #[test]
    fn not_equal() {
        assert_eq!(Token::from((b'!', b'=')), Token::NotEqual);
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
        assert_eq!(Token::from(b'<'), Token::LessThan);
    }

    #[test]
    fn lesser_than_equal() {
        assert_eq!(Token::from((b'<', b'=')), Token::LessThanEqual);
    }

    #[test]
    fn greater_than() {
        assert_eq!(Token::from(b'>'), Token::GreaterThan);
    }

    #[test]
    fn greater_than_equal() {
        assert_eq!(Token::from((b'>', b'=')), Token::GreaterThanEqual);
    }
}
