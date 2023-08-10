use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short = 't', long = "verbose", help = "Verbose output")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get/Set config values
    Config {
        #[arg(help = "The key to get/set")]
        key: Option<String>,
        #[arg(help = "The value to set")]
        value: Option<String>,
        #[arg(short, long, help = "List known config keys")]
        list: bool,
    },
    /// Get info about repository
    Repo,
}
