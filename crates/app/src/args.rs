use clap::{Parser, Subcommand};
use std::path::PathBuf; // Import PathBuf

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // 1. Global Option: Configuration file path
    #[arg(
        short, 
        long, 
        // Use PathBuf and set the default value to "./config.toml"
        default_value = "./config.toml", 
        long_help = "Path to the configuration file. Defaults to './config.toml'"
    )]
    pub config: PathBuf, // Changed from String to PathBuf

    // 2. Subcommand: The action to perform
    #[command(subcommand)]
    pub command: Command,
}

// Define the available subcommands (actions)
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Starts the application server
    Serve,
}

pub fn parse() -> Args {
    Args::parse()
}