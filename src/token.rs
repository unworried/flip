#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Newline,

    Ident,
    Assign,

    Number,
    String,

    /// Operators
    Equal,
    NotEqual,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    LessThan,
    GreaterThan,

    /// Keywords
    Label,
    Goto,
    Print,
    Input,
    Let,
    If,
    Then,
    EndIf,
    While,
    Repeat,
    EndWhile,
}

impl From<u8> for Token {
    fn from(ch: u8) -> Self {
        match ch {
            //b'=' => Self::Equal,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Asterisk,
            b'/' => Self::ForwardSlash,
            //b'<' => Self::LessThan,
            //b'>' => Self::GreaterThan,
            b'\n' => Self::Newline,
            0 => Self::Eof,
            _ => unimplemented!(),
        }
    }
}
