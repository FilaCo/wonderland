use clap::Parser;

use crate::compiler::Config;

#[derive(Parser, Clone, Debug)]
#[command(version)]
pub struct AliceDriver {
    /// Input source file
    #[arg(default_value = "-")]
    pub input: String,
}

impl Default for AliceDriver {
    fn default() -> Self {
        AliceDriver::parse()
    }
}

impl From<AliceDriver> for Config {
    fn from(value: AliceDriver) -> Self {
        Self { input: value.input }
    }
}
