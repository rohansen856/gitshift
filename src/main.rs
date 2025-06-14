use clap::Parser;
use service::GitShift;
use std::error::Error;

mod sshkey;

mod cli;
use cli::{Cli, Commands};
mod service;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let gitshift = GitShift::new()?;

    match cli.command {
        Commands::Ls => gitshift.list_accounts()?,
        Commands::Activate { account_name } => gitshift.activate_account(&account_name)?,
        Commands::Clone { repo_url } => gitshift.clone_repo(&repo_url)?,
        Commands::Info { account_name } => gitshift.get_account_info(&account_name)?,
        Commands::Add => gitshift.add_account()?,
        Commands::Rm => gitshift.remove_account()?,
    }

    Ok(())
}
