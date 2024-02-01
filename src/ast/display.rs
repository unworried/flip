use std::fmt::Display;

use super::{statement::{If, Print, While}, Expr, Program, Stmt};

// TODO: Move out of here and create builder pattern in extern crate. Doesn't need to be apart of
// lib

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

#[allow(unreachable_patterns)] // TODO: Remove eventually
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement: ")?;

        match self {
            Self::Print(stmt) => write!(f, "{}", stmt),
            Self::If(stmt) => write!(f, "{}", stmt),
            Self::While(stmt) => write!(f, "{}", stmt),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Print")?;
        write!(f, " {}", self.expression)
    }
}

impl Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "If")?;
        writeln!(f, "Condition: {}", self.condition)?;
        writeln!(f, "Resolution:")?;
        for statement in &self.resolution {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

impl Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "While")?;
        writeln!(f, "Condition: {}", self.condition)?;
        writeln!(f, "Repeat:")?;
        for statement in &self.resolution {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(value) => write!(f, "{:#?}", value),
            Self::Ident(value) => write!(f, "{:?}", value),
            Self::Primitive(value) => write!(f, "{:?}", value),
            Self::Literal(value) => write!(f, "{:?}", value),
        }
    }
}
