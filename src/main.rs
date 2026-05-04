//! git-manage: Switch between GitHub accounts without the pain.
//! 
//! This tool manages multiple Git identities (username, email, SSH hosts)
//! and provides utilities to switch between them at a global or repository level.

mod cli;
mod commands;
mod config;
mod git;
mod ui;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Cmd};
use config::Config;

/// Entry point for the git-manage CLI.
///
/// Parses command-line arguments, loads configuration, and dispatches to the
/// appropriate command handler.
fn main() {
    let cli = Cli::parse();
    let mut cfg = Config::load().unwrap_or_else(|e| {
        eprintln!("{} failed to load config: {}", "error:".red().bold(), e);
        Config::default()
    });

    let result: Result<()> = match cli.command {
        Cmd::Init => commands::init::run(&mut cfg),
        Cmd::Switch { alias, force } => commands::switch::run(&mut cfg, alias, force),
        Cmd::Apply { global } => commands::apply::run(&mut cfg, global),
        Cmd::Status => commands::status::run(&cfg),
        Cmd::List => commands::list::run(&cfg),
        Cmd::Add { alias, username, email, primary, ssh_host } => {
            commands::add::run(&mut cfg, alias, username, email, primary, ssh_host)
        }
        Cmd::Remove { alias } => commands::remove::run(&mut cfg, &alias),
        Cmd::Whoami => commands::whoami::run(&cfg),
        Cmd::Hook { action } => commands::hook::run(&cfg, action),
        Cmd::Bind { alias, path } => commands::bind::run(&mut cfg, alias, path),
        Cmd::Log { count } => commands::log::run(&cfg, count),
        Cmd::Prune => commands::prune::run(&mut cfg),
        Cmd::ClearLog => commands::clear_log::run(&mut cfg),
        Cmd::Completions { shell } => commands::completions::run(shell),
    };

    if let Err(e) = result {
        eprintln!("\n  {} {}\n", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
