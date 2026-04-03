use std::path::{Path, PathBuf};

use ac_db::db::AcDbTrait;
use ac_ir::source::SourceFile;
use salsa::Storage;

use crate::{Config, source_map::SourceMap};

#[salsa::db]
#[derive(Clone)]
pub struct AcDb {
    storage: Storage<Self>,
    cfg: Config,
    sources: SourceMap,
}

impl AcDb {
    pub fn new(cfg: Config) -> Self {
        Self {
            storage: Storage::default(),
            cfg,
            sources: SourceMap::new(),
        }
    }
}

#[salsa::db]
impl AcDbTrait for AcDb {
    fn input(&self) -> Option<&Path> {
        self.cfg.input.as_deref()
    }
    fn get_source_file(&self, path: &Path) -> Option<SourceFile> {
        self.sources.get(path)
    }
    fn add_source_file(&self, file: SourceFile) {
        self.sources.add(self, file);
    }
}

impl salsa::Database for AcDb {}
