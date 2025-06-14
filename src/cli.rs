use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitshift")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available accounts
    Ls,
    /// Activate a specific account
    Activate { account_name: String },
    /// Clone a repository using active account
    Clone { repo_url: String },
    /// Add a new github account
    Add,
    /// Get information about a specific account
    Info { account_name: String },
}
