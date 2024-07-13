use core::fmt;

use crate::Ast;

use super::{Literal, LiteralKind};

#[derive(Debug, PartialEq, Default, Clone)]
pub enum Type {
    #[default]
    Unresolved,
    Error,
    Int,
    Char,
    String,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unresolved => write!(f, "unresolved"),
            Type::Error => write!(f, "err"),
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
        }
    }
}

impl From<String> for Type {
    fn from(value: String) -> Self {
        match value.as_str() {
            "int" => Type::Int,
            "char" => Type::Char,
            "string" => Type::String,
            "void" => Type::Void,
            _ => Type::Error,
        }
    }
}

impl From<&Ast> for Type {
    fn from(value: &Ast) -> Self {
        match value {
            Ast::Literal(Literal { kind, .. }) => match kind {
                LiteralKind::Int(_) => Type::Int,
                LiteralKind::Char(_) => Type::Char,
                LiteralKind::String(_) => Type::String,
            },
            _ => Type::Unresolved,
        }
    }
}
