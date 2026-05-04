//! Utility functions for interacting with the local Git installation and configuration.

use anyhow::{Result, bail};
use std::path::PathBuf;
use std::process::Command;

/// Set a git config key, locally or globally.
///
/// # Arguments
/// * `key` - The git configuration key to set.
/// * `value` - The value to assign to the key.
/// * `global` - Whether to set the key in the global or local scope.
pub fn set(key: &str, value: &str, global: bool) -> Result<()> {
    let scope = if global { "--global" } else { "--local" };
    let status = Command::new("git")
        .args(["config", scope, key, value])
        .status()?;
    if !status.success() {
        bail!("git config {scope} {key} failed — are you inside a git repo?");
    }
    Ok(())
}

/// Get a git config key from a specific scope (local or global).
///
/// # Arguments
/// * `key` - The git configuration key to retrieve.
/// * `global` - Whether to get the key from the global or local scope.
pub fn get(key: &str, global: bool) -> Option<String> {
    let scope = if global { "--global" } else { "--local" };
    Command::new("git")
        .args(["config", scope, "--get", key])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// Get a git config key, searching local → global (git's normal resolution).
///
/// # Arguments
/// * `key` - The git configuration key to retrieve.
pub fn get_resolved(key: &str) -> Option<String> {
    Command::new("git")
        .args(["config", "--get", key])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

/// Returns true if the current working directory is inside a git repository.
pub fn is_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Returns the canonical root path of the current repository.
pub fn repo_root() -> Option<PathBuf> {
    Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
}

/// Returns the .git directory path of the current repository.
pub fn git_dir() -> Option<PathBuf> {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
}

/// Checks if ~/.ssh/config contains a `Host <alias>` block for the given host.
///
/// # Arguments
/// * `host` - The host alias to check for in the SSH configuration.
pub fn ssh_host_exists(host: &str) -> bool {
    let Some(home) = dirs::home_dir() else { return false };
    let ssh_config = home.join(".ssh").join("config");
    let Ok(content) = std::fs::read_to_string(ssh_config) else { return false };
    content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed.starts_with("Host ") && trimmed[5..].trim() == host
    })
}
