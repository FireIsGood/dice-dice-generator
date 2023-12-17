//! Configuration struct
use crate::{
    args,
    format::{answers_padded, num_to_emoji},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

/// Configuration struct.
///
/// This holds the data to be deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub rules: Option<String>,
    pub min_length: Option<i32>,
    pub answers: Vec<DiceResult>,
}

/// Answer struct.
///
/// A single answer comprised of an operation and optional number
#[derive(Serialize, Deserialize, Debug)]
pub struct DiceResult {
    pub operation: String,
    pub number: Option<i32>,
}

/// Get the config given a file path.
///
/// Bubbles errors through anyhow.
pub fn get_config() -> anyhow::Result<Config> {
    // Get cli args
    let cli = args::Cli::parse();

    // Read the given config file
    let test_file = fs::read_to_string(cli.config)?;
    let config: Config = toml::from_str(&test_file)?;

    Ok(config)
}

impl Config {
    /// Prints the configuration file
    ///
    /// Takes a string slice of rules to be printed under the name section.
    pub fn print(&self) {
        println!("Config name: {}", self.name);
        println!("{}", self.rules.clone().unwrap_or("".into()));
        for (index, answer) in answers_padded(self, self.min_length.unwrap_or_default())
            .iter()
            .enumerate()
        {
            println!(
                "{} ||`{}`||",
                num_to_emoji((index + 1).try_into().unwrap()),
                answer
            );
        }
    }
}
