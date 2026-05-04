//! Command to add or update an account profile.

use anyhow::Result;
use colored::Colorize;

use crate::config::{Account, Config};
use crate::ui;

/// Executes the 'add' command to create or update an account profile.
///
/// # Arguments
/// * `cfg` - The application configuration to update.
/// * `alias` - The unique alias for the account.
/// * `username` - The GitHub username for the account.
/// * `email` - The email address associated with the account.
/// * `primary` - Whether this should be marked as the primary account.
/// * `ssh_host` - Optional SSH host alias from ~/.ssh/config.
pub fn run(
    cfg: &mut Config,
    alias: String,
    username: String,
    email: String,
    primary: bool,
    ssh_host: Option<String>,
) -> Result<()> {
    if primary {
        for acc in &mut cfg.accounts {
            acc.primary = false;
        }
    }

    let existed = cfg.find(&alias).is_some();

    if let Some(existing) = cfg.find_mut(&alias) {
        existing.username = username.clone();
        existing.email = email.clone();
        existing.primary = primary;
        if ssh_host.is_some() {
            existing.ssh_host = ssh_host;
        }
    } else {
        cfg.accounts.push(Account {
            alias: alias.clone(),
            username,
            email,
            primary,
            ssh_host,
        });
    }

    if cfg.accounts.len() == 1 || primary {
        cfg.active = Some(alias.clone());
    }

    cfg.save()?;

    println!();
    let verb = if existed { "updated" } else { "added" };
    ui::ok(&format!("{} '{}'", verb, alias.bold()));
    println!();

    Ok(())
}
