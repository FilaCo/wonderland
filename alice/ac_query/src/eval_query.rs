use ac_db::db::AliceDatabaseTrait;
use ac_ir::{source::SourceFile, syntax::AliceFile};

#[salsa::tracked]
pub fn eval(db: &dyn AliceDatabaseTrait) -> AliceFile<'_> {
    todo!()
}
