use anyhow::Result;
use colored::Colorize;
use ssh_key::{Algorithm, LineEnding, PrivateKey, PublicKey};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
fn set_file_permissions(path: &Path, mode: u32) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perm = fs::Permissions::from_mode(mode);
    fs::set_permissions(path, perm)?;
    Ok(())
}

pub struct SSHKey {
    private_key: String,
    pub public_key: String,
    key_dir: PathBuf,
    account_name: String,
}

impl SSHKey {
    pub fn generate(
        algorithm: Algorithm,
        comment: &str,
        key_dir: &Path,
        account_name: &str,
    ) -> Result<Self> {
        let mut rng = rand::thread_rng();

        let private_key = PrivateKey::random(&mut rng, algorithm)?;
        let public_key = private_key.public_key();
        let key_data = public_key.key_data().clone();
        let public_key = PublicKey::new(key_data, comment).to_string();

        Self::display_colored(&Self {
            private_key: private_key.to_openssh(LineEnding::LF)?.to_string(),
            public_key: public_key.clone(),
            key_dir: key_dir.to_path_buf(),
            account_name: account_name.to_string(),
        });

        Ok(Self {
            private_key: private_key.to_openssh(LineEnding::LF)?.to_string(),
            public_key,
            key_dir: key_dir.to_path_buf(),
            account_name: account_name.to_string(),
        })
    }

    pub fn save_keypair(&self) -> Result<(PathBuf, PathBuf)> {
        let private_path = self.key_dir.join(format!("{}_id", self.account_name));
        let public_path = private_path.with_extension("pub");

        // Save private key with secure permissions
        fs::write(&private_path, &self.private_key)?;
        set_file_permissions(&private_path, 0o600)?;

        // Save public key
        fs::write(&public_path, &self.public_key)?;

        Ok((private_path, public_path))
    }

    pub fn display_colored(&self) {
        // Show storage location matching GitShift's config path
        println!("\n{}", "🔐 Keys stored in:".bold().cyan());
        println!("{}", self.key_dir.display().to_string().green());

        // Rest of the display logic remains the same
        println!(
            "\n{}",
            "╔══════════════════════════════════════════╗".blue()
        );
        println!("{}", "║               PRIVATE KEY                ║".blue());
        println!("{}", "╚══════════════════════════════════════════╝".blue());
        println!(
            "{}{}",
            "⚠️  ".yellow(),
            "Keep this private!".bold().yellow()
        );
        println!("{}", self.private_key.red());

        println!(
            "\n{}",
            "╔══════════════════════════════════════════╗".green()
        );
        println!("{}", "║                PUBLIC KEY                ║".green());
        println!("{}", "╚══════════════════════════════════════════╝".green());
        println!("{}{}", "🔑 ".cyan(), "Can be shared safely".bold().cyan());
        println!("{}", self.public_key.green());
    }
}
