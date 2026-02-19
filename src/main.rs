use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod commands;
mod config;
mod error;
mod keychain;
mod rules;

use commands::{check, load, setup, validate};

#[derive(Parser)]
#[command(name = "keychain-cli")]
#[command(about = "Secure Keychain Management CLI for Claude Code", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, long, help = "Service name in Keychain")]
    #[arg(default_value = "claude-dev")]
    service_name: String,

    #[arg(global = true, long, help = "Enable verbose logging")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup: Store .env secrets into Keychain
    Setup {
        /// Path to .env file
        #[arg(short, long)]
        env_file: PathBuf,

        /// Only setup specific keys (comma-separated)
        #[arg(short, long)]
        keys: Option<String>,

        /// Skip interactive confirmation
        #[arg(long)]
        force: bool,
    },

    /// Load: Retrieve secrets from Keychain
    Load {
        /// Output format: bash, json, or export
        #[arg(short, long, default_value = "bash")]
        format: String,

        /// Only load specific keys (comma-separated)
        #[arg(short, long)]
        keys: Option<String>,
    },

    /// Validate: Check if command violates security rules (for Hook)
    Validate {
        /// Command to validate (from stdin or argument)
        #[arg(value_name = "COMMAND")]
        command: Option<String>,
    },

    /// Check: Verify Keychain configuration and security status
    Check {
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> error::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    } else {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("keychain-cli started with service: {}", cli.service_name);

    match cli.command {
        Commands::Setup {
            env_file,
            keys,
            force,
        } => {
            setup::execute(&env_file, keys.as_deref(), force, &cli.service_name)?;
        }
        Commands::Load { format, keys } => {
            load::execute(&format, keys.as_deref(), &cli.service_name)?;
        }
        Commands::Validate { command } => {
            validate::execute(command, &cli.service_name)?;
        }
        Commands::Check { verbose } => {
            check::execute(verbose, &cli.service_name)?;
        }
    }

    Ok(())
}
