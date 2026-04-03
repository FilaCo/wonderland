use std::path::{Path, PathBuf};

use ac_db::db::AliceDatabaseTrait;
use ac_ir::source::SourceFile;
use dashmap::{DashMap, Entry};

#[derive(Clone, Debug)]
pub(super) struct SourceMap {
    files: DashMap<PathBuf, SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: DashMap::new(),
        }
    }

    pub fn add(&self, db: &dyn AliceDatabaseTrait, file: SourceFile) {
        self.files.insert(file.path(db).clone(), file);
    }

    pub fn get(&self, path: &Path) -> Option<SourceFile> {
        match self.files.entry(PathBuf::from(path)) {
            Entry::Occupied(entry) => Some(*entry.get()),
            Entry::Vacant(_) => None,
        }
    }
}
