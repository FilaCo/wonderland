use ac_db::db::AliceDatabaseTrait;
use ac_ir::{source::SourceFile, syntax::AliceFile};

#[salsa::tracked]
pub fn parse_file(db: &dyn AliceDatabaseTrait, file: SourceFile) -> AliceFile<'_> {
    db.add_source_file(file);
    AliceFile::new(db, Vec::new())
}
