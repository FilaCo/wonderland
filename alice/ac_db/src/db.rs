use std::path::Path;

use salsa::Database;

#[salsa::db]
pub trait AcDbTrait: Database {
    fn input(&self) -> Option<&Path>;
}
