use ac_ir::source::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct SpanId(salsa::Id);

impl SpanId {
    pub fn new(span: Span<'_>) -> Self {
        Self(salsa::plumbing::AsId::as_id(&span))
    }
    pub fn resolve(&self) -> Span<'_> {
        salsa::plumbing::FromId::from_id(self.0)
    }
}
