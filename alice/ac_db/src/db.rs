use salsa::Database;

#[salsa::db]
pub trait AcDbTrait: Database {}
