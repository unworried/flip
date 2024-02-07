use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use self::display::DiagnosticsDisplay;
use crate::error::{CompilerError, Result};
use crate::lexer::Token;
use crate::source::Source;
use crate::span::Span;

mod display;

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Default, Debug)]
pub struct DiagnosticBag {
    pub diagnostics: Vec<Diagnostic>,
}

pub type DiagnosticsCell = Rc<RefCell<DiagnosticBag>>;

impl DiagnosticBag {
    pub fn new() -> DiagnosticsCell {
        let bag = Self {
            diagnostics: Vec::new(),
        };

        Rc::new(RefCell::new(bag))
    }

    pub fn check(&self, src: &Source) -> Result<()> {
        if !self.diagnostics.is_empty() {
            let diagnostics_display = DiagnosticsDisplay::new(src, &self.diagnostics);
            diagnostics_display.print()?;

            return Err(CompilerError::Diagnostics);
        }

        Ok(())
    }

    fn report(&mut self, kind: DiagnosticKind, message: String, span: Span) {
        self.diagnostics.push(Diagnostic {
            kind,
            message,
            span,
        });
    }

    fn error(&mut self, message: String, span: Span) {
        self.report(DiagnosticKind::Error, message, span);
    }

    fn warning(&mut self, message: String, span: Span) {
        self.report(DiagnosticKind::Warning, message, span);
    }

    pub fn unexpected_token(&mut self, expected: &Token, actual: &Token, span: &Span) {
        self.error(
            format!("expected: '{}', found: '{}'", expected, actual),
            span.clone(),
        );
    }

    pub fn expected_expression(&mut self, expected: &Token, span: &Span) {
        self.error(
            format!("expected expression, found '{}'", expected),
            span.clone(),
        );
    }

    pub fn illegal_token(&mut self, span: &Span) {
        self.error("illegal token".to_owned(), span.clone());
    }

    pub fn unknown_statement(&mut self, token: &Token, span: &Span) {
        self.error(format!("unknown statement '{}'", token), span.clone());
    }

    pub fn invalid_operator(&mut self, token: &Token, span: &Span) {
        self.error(format!("invalid operator '{}'", token), span.clone());
    }

    pub fn unknown_expression(&mut self, token: &Token, span: &Span) {
        self.error(format!("unknown expression '{}'", token), span.clone());
    }

    pub fn symbol_already_declared(&mut self, pattern: &String, span: &Span) {
        self.error(
            format!("symbol: '{}' already exists in scope", pattern),
            span.clone(),
        );
    }

    pub fn undeclared_assignment(&mut self, ident: &String, span: &Span) {
        self.error(format!("undeclared symbol: '{}'", ident), span.clone());
    }

    pub fn undeclared_reference(&mut self, ident: &String, span: &Span) {
        self.error(
            format!("symbol: '{}' is never declared", ident),
            span.clone(),
        );
    }

    pub fn reference_before_assignment(&mut self, ident: &String, span: &Span) {
        self.error(
            format!("symbol: '{}' referenced before assignment", ident),
            span.clone(),
        );
    }
}
