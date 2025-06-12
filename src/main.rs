use clap::Parser;
use colored::Colorize;
use service::GitShift;
use ssh_key::Algorithm;
use std::{error::Error, io::Write};

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
        Commands::Add { name } => {
            let algorithm = Algorithm::Ed25519;
            print!("{}", "Enter email for this account: ".cyan());
            std::io::stdout().flush()?;
            let mut email = String::new();
            std::io::stdin().read_line(&mut email)?;
            let email = email.trim().to_string();

            // Add account with validated parameters
            gitshift.add_account(&name, &email, algorithm.clone())?;

            // Display success message
            println!(
                "{} Added new account '{}' with {} keys",
                "✓".green(),
                name.bold(),
                match algorithm {
                    Algorithm::Ed25519 => "Ed25519",
                    Algorithm::Rsa { .. } => "RSA",
                    Algorithm::Dsa => "DSA",
                    Algorithm::Ecdsa { .. } => "ECDSA",
                    Algorithm::SkEcdsaSha2NistP256 => "SkEcdsaSha2NistP256",
                    Algorithm::SkEd25519 => "SkEd25519",
                    Algorithm::Other(_) => "Other",
                    _ => "Unknown",
                }
            );
        }
    }

    Ok(())
}
