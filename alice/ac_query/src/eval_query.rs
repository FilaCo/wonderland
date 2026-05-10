use ac_db::db::AliceDatabaseTrait;

#[salsa::tracked]
pub fn eval(db: &dyn AliceDatabaseTrait) {
    todo!()
}
