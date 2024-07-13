use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::fmt::{self, Display};

use self::display::DiagnosticsDisplay;
use crate::error::{CompilerError, Result};
use crate::lexer::Token;
use crate::passes::symbol_table::Type;
use crate::source::Source;
use crate::span::Span;

mod display;

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
}

#[repr(u8)]
#[derive(Debug)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiagnosticKind::Error => write!(f, "error"),
            DiagnosticKind::Warning => write!(f, "warning"),
        }
    }
}

#[derive(Default, Debug)]
pub struct DiagnosticBag {
    pub warnings: Vec<Diagnostic>,
    pub errors: Vec<Diagnostic>,
}

pub type DiagnosticsCell = Rc<RefCell<DiagnosticBag>>;

impl DiagnosticBag {
    pub fn new() -> DiagnosticsCell {
        let bag = Self {
            warnings: Vec::new(),
            errors: Vec::new(),
        };

        Rc::new(RefCell::new(bag))
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty() && self.errors.is_empty()
    }

    pub fn check(&self, src: &Source) -> Result<()> {
        let mut error: Option<CompilerError> = None;

        if !self.warnings.is_empty() {
            let diagnostics_display = DiagnosticsDisplay::new(src, &self.warnings);
            diagnostics_display.print(DiagnosticKind::Warning)?;

            error = Some(CompilerError::DiagnosticWarning);
        }

        if !self.errors.is_empty() {
            let diagnostics_display = DiagnosticsDisplay::new(src, &self.errors);
            diagnostics_display.print(DiagnosticKind::Error)?;

            error = Some(CompilerError::DiagnosticError);
        }

        error.map_or(Ok(()), Err)
    }

    fn error(&mut self, message: String, span: Span) {
        self.errors.push(Diagnostic {
            message,
            span: Some(span),
        });
    }

    fn program_error(&mut self, message: String) {
        self.errors.push(Diagnostic {
            message,
            span: None,
        });
    }

    fn warning(&mut self, message: String, span: Span) {
        self.warnings.push(Diagnostic {
            message,
            span: Some(span),
        });
    }

    pub fn expected_token(&mut self, expected: &Token, actual: &Token, span: Span) {
        self.error(
            format!("expected: '{}', found: `{}`", expected, actual),
            span,
        );
    }

    pub fn unexpected_token(&mut self, token: &Token, span: Span) {
        self.error(format!("unexpected token: `{}`", token), span);
    }

    pub fn expected_expression(&mut self, expected: &Token, span: Span) {
        self.error(format!("expected expression, found `{}`", expected), span);
    }

    pub fn illegal_token(&mut self, span: Span) {
        self.error("illegal token".to_owned(), span);
    }

    pub fn unknown_statement(&mut self, token: &Token, span: Span) {
        self.error(format!("unknown statement `{}`", token), span);
    }

    pub fn invalid_operator(&mut self, token: &Token, span: Span) {
        self.error(format!("invalid operator `{}`", token), span);
    }

    pub fn unknown_expression(&mut self, token: &Token, span: Span) {
        self.error(format!("unknown expression `{}`", token), span);
    }

    pub fn variable_already_declared(&mut self, pattern: &String, span: Span) {
        self.error(
            format!("variable: `{}` already exists in scope", pattern),
            span,
        );
    }

    pub fn function_already_declared(&mut self, pattern: &String, span: Span) {
        // TODO: better message maybe?
        self.error(format!("function: `{}` already exists", pattern), span);
    }

    pub fn undeclared_assignment(&mut self, ident: &String, span: Span) {
        self.error(format!("undeclared symbol: `{}`", ident), span);
    }

    pub fn undefined_reference(&mut self, ident: &String, span: Span) {
        self.error(format!("symbol: `{}` is undefined", ident), span);
    }

    pub fn reference_before_assignment(&mut self, ident: &String, span: Span) {
        self.error(
            format!("symbol: `{}` referenced before assignment", ident),
            span,
        );
    }

    pub fn unused_variable(&mut self, ident: &String, span: Span) {
        self.warning(format!("unused variable: `{}`", ident), span);
    }

    pub fn unused_function(&mut self, ident: &String, span: Span) {
        self.warning(format!("unused function: `{}`", ident), span);
    }

    pub fn empty_block(&mut self, span: Span) {
        self.warning("empty block found".to_owned(), span);
    }

    pub fn main_not_found(&mut self) {
        self.program_error("`main` function not found".to_owned());
    }

    pub fn mismatched_type(&mut self, expected: &Type, found: &Type, span: Span) {
        self.error(
            format!(
                "mismatched types: expected `{}`, found `{}`",
                expected, found
            ),
            span,
        );
    }
}
