use ac_db::db::AcDbTrait;
use ac_ir::{source::SourceFile, syntax::AliceFile};

#[salsa::tracked]
pub fn eval(db: &dyn AcDbTrait) -> AliceFile<'_> {
    todo!()
}
