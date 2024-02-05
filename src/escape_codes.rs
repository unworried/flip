use std::fmt::Display;

pub enum Color {
    Red,
    Orange,
    Reset,
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m\x1b[1m",
            Color::Orange => "\x1b[33m\x1b[1m",
            Color::Reset => "\x1b[0m",
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
