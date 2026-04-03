use std::path::Path;

use crate::{frontend::Config, source::SourceMap};

#[derive(Debug)]
pub struct Alice {
    cfg: Config,
    source_map: SourceMap,
}
