use colored::Colorize;
use anyhow::Result;

use crate::config::Config;
use crate::git;
use crate::ui;

/// Executes the `status` command, showing the active account and current repository/global git identity.
///
/// # Arguments
/// * `cfg` - Reference to the application configuration.
pub fn run(cfg: &Config) -> Result<()> {
    ui::header("status");

    // Active account
    match cfg.active_account() {
        Some(acc) => {
            println!("  {} active account", "●".green());
            ui::print_account(acc, false);
        }
        None => println!("  {} no active account set", "○".dimmed()),
    }

    ui::separator();

    // Repo-local identity
    if git::is_repo() {
        let local_name = git::get("user.name", false);
        let local_email = git::get("user.email", false);
        println!("  {}", "repo (local)".cyan().bold());
        if local_name.is_some() || local_email.is_some() {
            if let Some(n) = &local_name {
                ui::info("user.name", n);
            }
            if let Some(e) = &local_email {
                ui::info("user.email", e);
            }
        } else {
            println!("    {} (falling through to global)", "not set locally".dimmed());
        }
        ui::separator();
    }

    // Global identity
    let global_name = git::get("user.name", true);
    let global_email = git::get("user.email", true);
    println!("  {}", "global".yellow().bold());
    match (global_name, global_email) {
        (Some(n), Some(e)) => {
            ui::info("user.name", &n);
            ui::info("user.email", &e);
        }
        _ => println!("    {}", "user.name / user.email not set".dimmed()),
    }

    ui::footer();
    Ok(())
}
