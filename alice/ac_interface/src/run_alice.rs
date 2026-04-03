use ac_db::db::AcDbTrait;

use crate::{Config, db::AcDb};

pub fn run_alice<R>(cfg: Config, f: impl Fn(&dyn AcDbTrait) -> R) -> R {
    f(&AcDb::new(cfg))
}
