use std::fmt::Display;

use super::{statement::Print, Ast, Expression, Statement};

// TODO: Move out of here and create builder pattern in extern crate. Doesn't need to be apart of
// lib

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

#[allow(unreachable_patterns)] // TODO: Remove eventually
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement: ")?;

        match self {
            Self::Print(stmt) => write!(f, "{}", stmt),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Print")?;
        write!(f, "{}", self.expression)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(value) => write!(f, "{}", value),
            Self::Primitive(value) => write!(f, "{:?}", value),
            Self::Literal(value) => write!(f, "{:?}", value),
        }
    }
}
