use salsa::Database;

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

impl<'db> Span<'db> {
    pub fn to(&self, other: &Self, db: &'db dyn Database) -> Self {
        assert!(self.file(db) == other.file(db));
        Self::new(db, self.start(db), other.end(db), *self.file(db))
    }
}
