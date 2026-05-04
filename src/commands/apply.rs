//! Command to apply the active account identity to a repository.

use anyhow::{Context, Result, bail};
use colored::Colorize;

use crate::config::Config;
use crate::git;
use crate::ui;

/// Runs the apply command.
///
/// # Arguments
/// * `cfg` - The project configuration.
/// * `global` - Whether to apply the identity globally (~/.gitconfig) instead of repo-local.
pub fn run(cfg: &mut Config, global: bool) -> Result<()> {
    let acc = cfg
        .active_account()
        .context("no active account — run `git-manage switch` first")?
        .clone();

    if !global && !git::is_repo() {
        bail!("not inside a git repo — use --global to apply globally");
    }

    git::set("user.name", &acc.username, global)?;
    git::set("user.email", &acc.email, global)?;

    let scope = if global {
        "globally".to_string()
    } else {
        let root = git::repo_root()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "this repo".to_string());
        
        cfg.log_switch(&acc.alias, Some(root));
        cfg.save()?;
        
        "to this repo".to_string()
    };

    // SSH host advisory
    if let Some(host) = acc.ssh_host.as_ref().filter(|h| !git::ssh_host_exists(h)) {
        ui::warn(&format!(
            "ssh host '{}' not found in ~/.ssh/config",
            host
        ));
    }

    println!();
    ui::ok(&format!(
        "applied {} {}",
        acc.alias.bold(),
        scope.dimmed()
    ));
    ui::info("user.name", &acc.username);
    ui::info("user.email", &acc.email);
    println!();

    Ok(())
}
