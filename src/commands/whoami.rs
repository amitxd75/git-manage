use anyhow::Result;
use colored::Colorize;

use crate::config::Config;
use crate::git;
use crate::ui;

/// Executes the `whoami` command, identifying the current git identity and warning if it mismatches the active account.
///
/// # Arguments
/// * `cfg` - Reference to the application configuration.
pub fn run(cfg: &Config) -> Result<()> {
    let name = git::get_resolved("user.name");
    let email = git::get_resolved("user.email");

    println!();

    match (&name, &email) {
        (Some(n), Some(e)) => {
            let matched = cfg
                .accounts
                .iter()
                .find(|a| a.username == *n && a.email == *e);

            if let Some(acc) = matched {
                let kind = if acc.primary {
                    "[primary]".yellow()
                } else {
                    "[alt]".cyan()
                };
                println!("  {} you are {} {}", "●".green(), acc.alias.bold(), kind);
            } else {
                println!("  {} identity not matched to any known account", "?".yellow());
            }

            ui::info("name", n);
            ui::info("email", e);

            // Mismatch warning — the main reason this command exists
            if let Some(active) = cfg.active_account() {
                let matches = active.username == *n && active.email == *e;
                if !matches {
                    println!();
                    ui::warn(&format!(
                        "active account is '{}' but this repo commits as a different identity!",
                        active.alias.bold()
                    ));
                    ui::warn(&format!(
                        "run {} to fix it",
                        "`git-manage apply`".bold()
                    ));
                }
            }

            // SSH host advisory
            if let Some(host) = matched.and_then(|a| a.ssh_host.as_ref()) {
                println!();
                if git::ssh_host_exists(host) {
                    println!(
                        "  {} ssh host {} found in ~/.ssh/config",
                        "✓".green(),
                        host.bold()
                    );
                } else {
                    ui::warn(&format!(
                        "ssh host '{}' not found in ~/.ssh/config",
                        host
                    ));
                }
            }
        }
        _ => {
            println!(
                "  {} git user.name / user.email not set for this repo",
                "○".dimmed()
            );
            ui::tip("run `git-manage apply` to set them");
        }
    }

    println!();
    Ok(())
}
