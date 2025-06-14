# GitShift 🔀

[![Rust](https://img.shields.io/badge/Rust-1.65%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A secure CLI tool for managing multiple GitHub accounts with SSH keys. Switch between work and personal accounts seamlessly! 🔐

![GitShift Demo](https://github.com/user-attachments/assets/fb29cf9f-0e87-4ef9-b12a-691cdc15fc48)

## Features ✨

- 🧩 **Multi-Account Management** - Store unlimited GitHub accounts
- 🔒 **Secure Storage** - Automatically handles SSH key permissions
- 🚀 **Context-Aware Operations** - Clone/push with active account credentials
- 📋 **Interactive Setup** - Guided account creation process
- 🌐 **Cross-Platform** - Works on Linux, macOS, and Windows

## Installation 📦

### From Source
```bash
cargo install --git https://github.com/rohansen856/gitshift.git
```

### From crates.io (Recommended)
```bash
# Nightly required or edition2024 support
cargo +nightly install gitshift 
```

## Quick Start 🚀
1. Add New Account
```bash
gitshift add --name work --algorithm ed25519
```

2. List Accounts
```bash
gitshift ls
```

3. Activate Account

```bash
gitshift activate work
```

4. Clone Repository
```bash
gitshift clone git@github.com:company/project.git
```
## Configuration ⚙️
### File Structure
```bash
~/.config/gitshift/
├── config.json    # Account configurations
├── state.json     # Active account state
└── ssh_keys/      # SSH key storage (700 permissions)
```
### Example config
```json
[
    {
        "name": "work",
        "email": "dev@company.com",
        "ssh_key_path": "~/.config/gitshift/ssh_keys/work_id",
        "public_key": "ssh-ed25519 AAAAC3NzaC1lZDI1..."
    }
]
```

## Command Reference 📚

Command	 |    Description	        |    Example
add	     |    Create new account	|    gitshift add --name dev --algorithm rsa
ls	     |    List accounts	        |    gitshift ls
activate |	  Switch account	    |    gitshift activate personal
clone	 |    Clone repository	    |    gitshift clone git@github.com:user/repo.git
info     |    Show account details  |    gitshift info work

## Development 🛠️
### Build Instructions
```bash
git clone https://github.com/rohansen856/gitshift.git
cd gitshift
cargo build --release
```

## Troubleshooting 🐞
Q: Getting "Permission denied" when cloning
A: 
1. Verify active account: gitshift ls

2. Check GitHub SSH setup: ssh -T git@github.com

3. Confirm key permissions: ls -la ~/.config/gitshift/ssh_keys

Q: Error saving configuration
A: Ensure proper directory permissions:
```bash
chmod 700 ~/.config/gitshift
```