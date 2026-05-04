use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::ui;

/// Executes the `list` command, displaying all configured accounts and repository bindings.
///
/// # Arguments
/// * `cfg` - Reference to the application configuration.
pub fn run(cfg: &Config) -> Result<()> {
    ui::header("accounts");

    if cfg.accounts.is_empty() {
        println!("  {} use `git-manage add` to add accounts", "none yet —".dimmed());
    } else {
        let last = cfg.accounts.len() - 1;
        for (i, acc) in cfg.accounts.iter().enumerate() {
            let is_active = cfg.active.as_deref() == Some(&acc.alias);
            ui::print_account(acc, is_active);
            if i < last {
                println!();
            }
        }
    }

    ui::footer();

    if !cfg.bindings.is_empty() {
        ui::header("repo bindings");
        for (path, alias) in &cfg.bindings {
            println!("  {} → {}", path.dimmed(), alias.bold());
        }
        ui::footer();
    }

    Ok(())
}
