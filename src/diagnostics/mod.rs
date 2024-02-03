use std::{cell::RefCell, rc::Rc};

use crate::{lexer::Token, span::Span};

pub mod display;

pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}

pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Default)]
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
}
