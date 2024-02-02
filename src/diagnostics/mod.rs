use crate::span::Span;

pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
}

pub enum DiagnosticKind {
    Error,
    Warning,
}
