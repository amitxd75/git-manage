//! Terminal UI utilities for consistent formatting and coloring.

use colored::Colorize;

use crate::config::Account;

pub const SEP_WIDTH: usize = 50;

/// Prints a horizontal separator line to the terminal.
pub fn separator() {
    println!("{}", "─".repeat(SEP_WIDTH).dimmed());
}

/// Prints a header with a bold title and a separator.
///
/// # Arguments
/// * `title` - The title text to display in the header.
pub fn header(title: &str) {
    println!();
    println!("  {}", title.bold());
    separator();
}

/// Prints a footer with a separator and an empty line.
pub fn footer() {
    separator();
    println!();
}

/// Formats and prints detailed information about a GitHub account.
///
/// # Arguments
/// * `acc` - The account to print.
/// * `is_active` - Whether this account is currently active.
pub fn print_account(acc: &Account, is_active: bool) {
    let kind = if acc.primary {
        "[primary]".yellow().to_string()
    } else {
        "[alt]".cyan().to_string()
    };

    let active = if is_active {
        format!("  {}", "◀ active".green().bold())
    } else {
        String::new()
    };

    println!("  {} {}{}", acc.alias.bold(), kind, active);
    println!("    username : {}", acc.username.white());
    println!("    email    : {}", acc.email.white());

    if let Some(host) = &acc.ssh_host {
        println!("    ssh host : {}", host.white());
    }
}

/// Prints a tip message with a yellow "tip:" label.
///
/// # Arguments
/// * `msg` - The tip message to display.
pub fn tip(msg: &str) {
    println!("  {}  {}", "tip:".yellow().bold(), msg);
}

/// Prints a success message with a green checkmark.
///
/// # Arguments
/// * `msg` - The success message to display.
pub fn ok(msg: &str) {
    println!("  {} {}", "✓".green(), msg);
}

/// Prints a warning message with a yellow "warn:" label.
///
/// # Arguments
/// * `msg` - The warning message to display.
pub fn warn(msg: &str) {
    println!("  {} {}", "warn:".yellow().bold(), msg);
}

/// Prints a labeled piece of information with consistent indentation.
///
/// # Arguments
/// * `label` - The label for the information.
/// * `value` - The value to display next to the label.
pub fn info(label: &str, value: &str) {
    println!("    {:<12}: {}", label, value.white());
}

