mod args;
mod commands;

use std::path::PathBuf;
use args::{Cli, Commands};

use clap::Parser;

use script_herder_core::config::AppConfig;

fn main() {
    let cli = Cli::parse();
    let config_path = match cli.config {
        Some(path) => path,
        None => get_config_path()
    };

    let mut config = AppConfig::from_json(config_path).unwrap();
    config.use_env();

    match cli.command {
        Some(Commands::Config { key, value, list }) =>
            commands::config::run_config(config, key.unwrap_or("".to_string()), value, list),
        None => print!("Not a valid command")
    }
}

fn get_config_path() -> PathBuf {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".config-sh.json");
            path
        },
        None => PathBuf::from(".config-sh.json")
    }
}
