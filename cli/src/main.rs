mod args;
mod commands;
mod logger;

use std::path::PathBuf;
use args::{Cli, Commands};

use clap::Parser;

use logger::{configure_logger, configure_logger_from_config};
use script_herder_core::config::AppConfig;

fn main() {
    let cli = Cli::parse();
    let config_path = match cli.config {
        Some(path) => path,
        None => get_config_path()
    };

    let verbose = cli.verbose;

    if verbose {
        configure_logger(log::LevelFilter::Trace).unwrap();
    }

    let mut config = AppConfig::from_json(config_path).unwrap();
    config.use_env();

    if !verbose {
        configure_logger_from_config(&config).unwrap();
    }

    match cli.command {
        Some(Commands::Config { key, value, list }) =>
            commands::config::run_config(config, key.unwrap_or("".to_string()), value, list),
        Some(Commands::Repo) =>
            commands::repo::run_repo_info(config),
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
