use crate::source::SourceFile;

#[salsa::tracked(debug)]
pub struct Span<'db> {
    #[tracked]
    pub start: usize,
    #[tracked]
    pub end: usize,
    #[tracked]
    #[returns(ref)]
    pub file: SourceFile,
}
