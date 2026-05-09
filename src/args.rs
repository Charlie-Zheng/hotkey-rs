use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// path to macro
    #[arg(value_name = "FILE")]
    pub config: PathBuf,
}
