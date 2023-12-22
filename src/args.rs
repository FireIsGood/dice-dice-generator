//! Argument parsing logic.
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Configuration file
    pub config: String,
}
