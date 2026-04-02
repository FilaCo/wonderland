use crate::source::Span;

#[derive(Debug)]
pub(crate) struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
}

#[derive(Debug)]
pub(crate) enum DiagnosticKind {
    Error,
    Warning,
    Note,
}
