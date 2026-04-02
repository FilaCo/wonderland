use crate::frontend::{Alice, Config};

pub fn run_alice<R>(cfg: Config, f: impl Fn(&Alice) -> R) -> R {
    f(&Alice::new(cfg))
}
