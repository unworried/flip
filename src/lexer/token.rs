use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)] // TODO: try to remove clone
pub enum Token {
    // Special
    Assign,
    Ident(String),
    Illegal,
    Eof,
    Newline,
    Whitespace, // Whitespace sequence of length n

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Token::Assign => "=",
            Token::Ident(_) => "Identifer",
            Token::Illegal => "Illegal Token",
            Token::Eof => "EoF",
            Token::Newline => r#"\n"#,
            Token::Int(_) => "Integer",
            Token::String(_) => "String",
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
            Token::Let => "Let",
            Token::If => "If",
            Token::Else => "Else",
            Token::While => "While",
            Token::Print => "Print",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::SemiColon => ";",
            Token::Whitespace => r#"' '"#,
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

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "==" => Self::Equal,
            "!=" => Self::NotEqual,
            "<=" => Self::LessThanEqual,
            ">=" => Self::GreaterThanEqual,

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
        assert_eq!(Token::from(b'<'), Token::LessThan);
    }

    #[test]
    fn lesser_than_equal() {
        assert_eq!(Token::from(String::from("<=")), Token::LessThanEqual);
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
