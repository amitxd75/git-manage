use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};

use crate::config::{Account, Config};
use crate::ui;

/// Prompts the user for input with a bold label.
///
/// # Arguments
/// * `label` - The text to display as the prompt label.
fn prompt(label: &str) -> Result<String> {
    print!("  {} ", label.bold());
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

/// Prompts the user for optional input; returns None if the input is empty.
///
/// # Arguments
/// * `label` - The text to display as the prompt label.
fn prompt_optional(label: &str) -> Result<Option<String>> {
    let val = prompt(label)?;
    Ok(if val.is_empty() { None } else { Some(val) })
}

/// Interactive wizard to add a new account to the configuration.
///
/// # Arguments
/// * `cfg` - Mutable reference to the application configuration.
/// * `primary` - Whether this account should be marked as the primary account.
fn add_account(cfg: &mut Config, primary: bool) -> Result<()> {
    let kind = if primary { "primary" } else { "alt" };
    println!("\n  {} {} account", "→".cyan(), kind.bold());

    let alias = prompt("alias (e.g. amitxd75):")?;
    let username = prompt("github username:")?;
    let email = prompt("commit email:")?;
    let ssh_host = prompt_optional("ssh host alias (optional, press enter to skip):")?;

    // demote existing primary if needed
    if primary {
        for acc in &mut cfg.accounts {
            acc.primary = false;
        }
    }

    if let Some(existing) = cfg.find_mut(&alias) {
        existing.username = username;
        existing.email = email;
        existing.primary = primary;
        existing.ssh_host = ssh_host;
    } else {
        cfg.accounts.push(Account { alias: alias.clone(), username, email, primary, ssh_host });
    }

    if primary || cfg.accounts.len() == 1 {
        cfg.active = Some(alias);
    }

    Ok(())
}

/// Executes the `init` command, running the first-time setup wizard.
///
/// # Arguments
/// * `cfg` - Mutable reference to the application configuration.
pub fn run(cfg: &mut Config) -> Result<()> {
    println!();
    println!("{}", "  git-manage setup".bold());
    ui::separator();
    println!("  Let's configure your GitHub accounts.");
    println!("  You can always run {} later to change things.\n", "`git-manage add`".bold());

    add_account(cfg, true)?;

    let want_alt = prompt("\n  add an alt account? [y/N]:")?;
    if want_alt.to_lowercase() == "y" {
        add_account(cfg, false)?;
    }

    cfg.save()?;

    println!();
    ui::separator();
    ui::ok("setup complete!");
    println!();
    println!("  Next steps:");
    println!("    {}  switch accounts", "`git-manage switch`".bold());
    println!("    {}   apply identity to current repo", "`git-manage apply`".bold());
    println!("    {}    install pre-commit guard", "`git-manage hook install`".bold());
    println!();

    Ok(())
}
