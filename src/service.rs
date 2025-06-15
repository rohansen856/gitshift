use anyhow::{Context, Result};
use colored::Colorize;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use ssh_key::Algorithm;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use crate::sshkey::SSHKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub ssh_key_path: PathBuf,
    pub email: String,
    pub public_key: String,
}

// Main application structure
pub struct GitShift {
    config_path: PathBuf,
    state_path: PathBuf,
    ssh_key_dir: PathBuf,
}

impl GitShift {
    pub fn new() -> Result<Self> {
        let proj_dirs =
            ProjectDirs::from("", "", "gitshift").context("Could not determine home directory")?;
        let config_dir = proj_dirs.config_dir();

        // Create all required directories
        fs::create_dir_all(config_dir)?;
        let ssh_key_dir = config_dir.join("ssh_keys");
        fs::create_dir_all(&ssh_key_dir)?;

        Ok(GitShift {
            config_path: config_dir.join("config.json"),
            state_path: config_dir.join("state.json"),
            ssh_key_dir,
        })
    }

    fn save_acc_info(&self, name: &str, email: &str, algorithm: Algorithm) -> Result<()> {
        let mut accounts = self.load_config()?;

        // Check if account exists
        if accounts.iter().any(|a| a.name == name) {
            anyhow::bail!("Account '{}' already exists", name);
        }

        // Generate and save keys
        let key = SSHKey::generate(algorithm, email, &self.ssh_key_dir, name)?;
        let (private_path, _public_path) = key.save_keypair()?;

        // Add to config
        accounts.push(Account {
            name: name.to_string(),
            ssh_key_path: private_path,
            email: email.to_string(),
            public_key: key.public_key,
        });

        self.save_config(&accounts)?;
        Ok(())
    }

    fn save_config(&self, accounts: &[Account]) -> Result<()> {
        let content = serde_json::to_string_pretty(accounts)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    fn load_config(&self) -> Result<Vec<Account>> {
        if !self.config_path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&self.config_path)?;
        serde_json::from_str(&content).context("Failed to parse config file")
    }

    fn load_state(&self) -> Result<Option<String>> {
        if !self.state_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&self.state_path)?;
        serde_json::from_str(&content).context("Failed to parse state file")
    }

    fn save_state(&self, account_name: Option<&str>) -> Result<()> {
        let content = serde_json::to_string_pretty(&account_name)?;
        fs::write(&self.state_path, content)?;
        Ok(())
    }

    pub fn list_accounts(&self) -> Result<()> {
        let accounts = self.load_config()?;
        if accounts.is_empty() {
            println!("No accounts configured.");
            return Ok(());
        }

        println!("Available accounts:");
        for account in accounts {
            println!("- {}", account.name);
        }
        Ok(())
    }

    pub fn activate_account(&self, account_name: &str) -> Result<()> {
        let accounts = self.load_config()?;
        if !accounts.iter().any(|a| a.name == account_name) {
            anyhow::bail!("Account '{}' not found", account_name);
        }

        self.save_state(Some(account_name))?;
        println!("Activated account: {}", account_name);
        Ok(())
    }

    pub fn clone_repo(&self, repo_url: &str) -> Result<()> {
        let active_account = self
            .load_state()?
            .context("No active account. Use 'activate' first")?;

        let accounts = self.load_config()?;
        let account = accounts
            .iter()
            .find(|a| a.name == active_account)
            .context("Active account not found in config")?;

        let ssh_command = format!("ssh -i {}", account.ssh_key_path.display());

        Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .env("GIT_SSH_COMMAND", ssh_command)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn get_account_info(&self, account_name: &str) -> Result<()> {
        let accounts = self.load_config()?;
        let account = accounts
            .iter()
            .find(|a| a.name == account_name)
            .context(format!("Account '{}' not found", account_name))?;

        println!("{}", "Account Information:".bold());
        println!("{}  {}", "Name:".cyan(), account.name.bold());
        println!("{}  {}", "Email:".cyan(), account.email);
        println!(
            "{}  {}",
            "SSH Key Path:".cyan(),
            account.ssh_key_path.display()
        );

        if let Ok(Some(active)) = self.load_state() {
            if active == account.name {
                println!("{}  {}", "Status:".cyan(), "Active".green().bold());
            } else {
                println!("{}  {}", "Status:".cyan(), "Inactive".yellow());
            }
        }

        println!("\n{}  ", "Public Key:".cyan().bold());
        println!("{}", account.public_key.green());

        Ok(())
    }

    pub fn add_account(&self) -> Result<()> {
        let algorithm = Algorithm::Ed25519;

        print!("{}", "Enter a name for this account: ".cyan());
        std::io::stdout().flush()?;
        let mut name = String::new();
        std::io::stdin().read_line(&mut name)?;
        let name = name.trim().to_string();

        print!("{}", "Enter email for this account: ".cyan());
        std::io::stdout().flush()?;
        let mut email = String::new();
        std::io::stdin().read_line(&mut email)?;
        let email = email.trim().to_string();

        // Add account with validated parameters
        self.save_acc_info(&name, &email, algorithm.clone())?;

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

        Ok(())
    }

    pub fn remove_account(&self) -> Result<()> {
        let accounts = self.load_config()?;
        if accounts.is_empty() {
            println!("No accounts to remove.");
            return Ok(());
        }

        println!("Available accounts:");
        for (i, account) in accounts.iter().enumerate() {
            println!("{}: {}", i + 1, account.name);
        }

        print!("{}", "Enter the number to remove the account: ".cyan());
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let index: usize = input.trim().parse().context("Invalid input")?;

        if index == 0 || index > accounts.len() {
            anyhow::bail!("Invalid account number");
        }

        let account_to_remove = &accounts[index - 1];
        fs::remove_file(&account_to_remove.ssh_key_path)
            .context("Failed to remove SSH key file")?;

        let mut updated_accounts = accounts.clone();
        updated_accounts.remove(index - 1);
        self.save_config(&updated_accounts)?;

        // Clear state if the removed account was active
        if self.load_state()? == Some(account_to_remove.name.clone()) {
            self.save_state(None)?;
            println!("Cleared active account state.");
        }

        println!(
            "{} Removed account '{}' successfully",
            "✓".green(),
            account_to_remove.name.bold()
        );

        Ok(())
    }
}
