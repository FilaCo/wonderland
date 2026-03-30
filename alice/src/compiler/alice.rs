use crate::compiler::Config;

#[derive(Debug)]
pub struct Alice {
    cfg: Config,
}

impl Alice {
    pub const fn new(cfg: Config) -> Self {
        Self { cfg }
    }

    pub fn input(&self) -> &str {
        &self.cfg.input
    }
}
