use std::fmt::Display;

use super::{ExprKind, Program, StmtKind};

// TODO: Move out of here and create builder pattern in extern crate. Doesn't need to be apart of
// lib

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement.kind)?;
        }

        Ok(())
    }
}

#[allow(unreachable_patterns)] // TODO: Remove eventually
impl Display for StmtKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Statement: ")?;

        write!(f, "{:?}", self)
    }
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
