use anyhow::{Result, bail};
use colored::Colorize;

use crate::config::Config;
use crate::ui;

/// Executes the `remove` command, deleting an account and its associated repository bindings.
///
/// # Arguments
/// * `cfg` - Mutable reference to the application configuration.
/// * `alias` - The alias of the account to be removed.
pub fn run(cfg: &mut Config, alias: &str) -> Result<()> {
    let before = cfg.accounts.len();
    cfg.accounts.retain(|a| a.alias != alias);

    if cfg.accounts.len() == before {
        bail!("no account with alias '{}'", alias);
    }

    // remove any bindings pointing to this alias
    cfg.bindings.retain(|_, v| v != alias);

    // if removed account was active, fall back to primary or first
    if cfg.active.as_deref() == Some(alias) {
        cfg.active = cfg
            .primary()
            .map(|a| a.alias.clone())
            .or_else(|| cfg.accounts.first().map(|a| a.alias.clone()));
    }

    cfg.save()?;

    println!();
    ui::ok(&format!("removed '{}'", alias.bold()));
    println!();

    Ok(())
}
