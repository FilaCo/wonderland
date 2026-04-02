use std::path::Path;

use crate::{frontend::Config, source::SourceMap};

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

    pub fn input(&self) -> Option<&Path> {
        self.cfg.input.as_deref()
    }
}
