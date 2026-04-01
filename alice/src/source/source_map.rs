use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct SourceMap {
    files: RwLock<Vec<Arc<SourceFile>>>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: RwLock::new(Vec::new()),
        }
    }

    pub fn add(&self, file: SourceFile) -> SourceFileId {
        let mut files = self.files.write().expect("unable to acquire write lock");
        files.push(Arc::new(file));
        SourceFileId(files.len())
    }

    pub fn get(&self, id: SourceFileId) -> Option<Arc<SourceFile>> {
        let files = self.files.read().expect("unable to acquire read lock");

        files.get(usize::from(id)).cloned()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceFileId(usize);

impl From<usize> for SourceFileId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<SourceFileId> for usize {
    fn from(value: SourceFileId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    path: PathBuf,
    contents: String,
}

impl SourceFile {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}
