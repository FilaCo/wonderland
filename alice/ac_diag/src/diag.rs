use ac_ir::source::Span;

use crate::span_id::SpanId;

#[salsa::accumulator]
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    span_id: SpanId,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, msg: String, span: Span<'_>) -> Self {
        Self {
            kind,
            msg,
            span_id: SpanId::new(span),
        }
    }

    pub fn span(&self) -> Span<'_> {
        self.span_id.resolve()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Notice,
}
