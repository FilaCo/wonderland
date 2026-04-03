use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub input: Option<PathBuf>,
}
