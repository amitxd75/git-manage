//! Command to generate shell completion scripts.

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;

use crate::cli::Cli;

/// Runs the completions command.
///
/// # Arguments
/// * `shell` - The shell to generate completions for.
pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "git-manage", &mut io::stdout());
    Ok(())
}
