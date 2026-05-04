//! Command-line interface definition using clap.

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(
    name = "git-manage",
    about = "Switch GitHub accounts without the pain",
    version,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Interactive first-time setup wizard
    Init,

    /// Switch to an account by alias, or cycle if no alias given
    Switch {
        /// Account alias (omit to cycle through accounts)
        alias: Option<String>,
        /// Automatically apply the identity to the current repo
        #[arg(short, long, alias = "F")]
        force: bool,
    },

    /// Apply active account identity to the current git repo
    Apply {
        /// Apply to ~/.gitconfig instead of repo-local config
        #[arg(long)]
        global: bool,
    },

    /// Show active account and git identity for this repo
    Status,

    /// List all configured accounts
    List,

    /// Add or update an account
    Add {
        alias: String,
        username: String,
        email: String,
        /// Mark as primary account
        #[arg(long)]
        primary: bool,
        /// SSH host alias from ~/.ssh/config (e.g. github-alt)
        #[arg(long)]
        ssh_host: Option<String>,
    },

    /// Remove an account
    Remove { alias: String },

    /// Show who this repo thinks you are, warn if it mismatches active account
    Whoami,

    /// Manage the pre-commit guard hook for this repo
    Hook {
        #[command(subcommand)]
        action: HookAction,
    },

    /// Bind a repo path to an account so the hook auto-applies it
    Bind {
        alias: String,
        /// Repo path (defaults to current directory)
        path: Option<String>,
    },

    /// Show recent account switches
    Log {
        /// Number of entries to show (default: 10)
        #[arg(short, long, default_value = "10")]
        count: usize,
    },

    /// Remove bindings for paths that no longer exist
    Prune,

    /// Clear the switch history
    ClearLog,

    /// Print shell completions to stdout
    Completions { shell: Shell },
}

#[derive(Subcommand)]
pub enum HookAction {
    /// Install pre-commit guard hook in current repo
    Install,
    /// Remove pre-commit guard hook from current repo
    Uninstall,
    /// Show hook status for current repo
    Status,
}
