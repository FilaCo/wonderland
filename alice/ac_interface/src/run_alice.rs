use ac_db::db::AliceDatabaseTrait;

use crate::{Config, db::AliceDatabase};

pub fn run_alice<R>(cfg: Config, f: impl Fn(&dyn AliceDatabaseTrait) -> R) -> R {
    f(&AliceDatabase::new(cfg))
}
