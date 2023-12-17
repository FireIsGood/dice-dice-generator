use crate::{
    args,
    format::{answers_padded, num_to_emoji},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub min_length: Option<i32>,
    pub answers: Vec<DiceResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiceResult {
    pub operation: String,
    pub number: Option<i32>,
}

pub fn get_config() -> anyhow::Result<Config> {
    // Get cli args
    let cli = args::Cli::parse();

    // Read the given config file
    let test_file = fs::read_to_string(cli.config)?;
    let config: Config = toml::from_str(&test_file)?;

    Ok(config)
}

impl Config {
    pub fn print(&self, rules: &str) {
        println!("Config name: {}", self.name);
        println!("{}", rules);
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
