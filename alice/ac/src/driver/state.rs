use std::{io, path::PathBuf, str::FromStr};

use clap::Parser;
use thiserror::Error;

use crate::frontend::Config;

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct AliceDriver {
    /// Input source file
    #[arg(default_value = "-")]
    pub input: AliceInput,
}

impl Default for AliceDriver {
    fn default() -> Self {
        AliceDriver::parse()
    }
}

impl From<AliceDriver> for Config {
    fn from(value: AliceDriver) -> Self {
        let input = match value.input {
            AliceInput::Stdin => None,
            AliceInput::File(path_buf) => Some(path_buf),
        };

        Self { input }
    }
}

#[derive(Clone, Debug)]
pub enum AliceInput {
    Stdin,
    File(PathBuf),
}

impl FromStr for AliceInput {
    type Err = AliceInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const STDIN_INPUT: &str = "-";
        match s {
            STDIN_INPUT => Ok(AliceInput::Stdin),
            s => {
                let fpath = PathBuf::from_str(s)
                    .unwrap()
                    .canonicalize()
                    .map_err(AliceInputError::InvalidInput)?;

                if fpath.is_file() {
                    Ok(AliceInput::File(fpath))
                } else {
                    Err(AliceInputError::NotSupportedInput(fpath))
                }
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum AliceInputError {
    #[error("{0}")]
    InvalidInput(#[from] io::Error),
    #[error("unsupported input `{0}`")]
    NotSupportedInput(PathBuf),
}
