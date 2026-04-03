use std::path::Path;

use ac_ir::source::SourceFile;
use salsa::Database;

#[salsa::db]
pub trait AliceDatabaseTrait: Database {
    fn input(&self) -> Option<&Path>;

    fn get_source_file(&self, path: &Path) -> Option<SourceFile>;
    fn add_source_file(&self, file: SourceFile);
}
