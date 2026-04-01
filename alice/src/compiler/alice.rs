use crate::{compiler::Config, source::SourceMap};

#[derive(Debug)]
pub struct Alice {
    cfg: Config,
    source_map: SourceMap,
}

impl Alice {
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            source_map: SourceMap::new(),
        }
    }

    pub fn input(&self) -> &str {
        &self.cfg.input
    }
}
