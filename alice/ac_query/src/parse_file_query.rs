use ac_db::db::AcDbTrait;
use ac_diag::{Diagnostic, DiagnosticKind};
use ac_ir::{
    source::{SourceFile, Span},
    syntax::AliceFile,
};
use salsa::Accumulator;

#[salsa::tracked]
pub fn parse_file(db: &dyn AcDbTrait, file: SourceFile) -> AliceFile<'_> {
    db.add_source_file(file);
    Diagnostic::new(
        DiagnosticKind::Error,
        String::from("what the heck"),
        Span::new(db, 10, 150, file),
    )
    .accumulate(db);
    AliceFile::new(db, Vec::new())
}
