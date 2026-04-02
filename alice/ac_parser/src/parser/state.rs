use ac_db::db::AcDbTrait;

pub struct Parser<'db> {
    db: &'db dyn AcDbTrait,
}

impl<'db> Parser<'db> {
    pub fn new(db: &'db dyn AcDbTrait) -> Self {
        Self { db }
    }
}
