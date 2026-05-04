use crate::config::Config;
use crate::ui;
use anyhow::Result;
use std::path::Path;
use colored::Colorize;

/// Executes the `prune` command, removing bindings for repository paths that no longer exist on disk.
///
/// # Arguments
/// * `cfg` - Mutable reference to the application configuration.
pub fn run(cfg: &mut Config) -> Result<()> {
    let before = cfg.bindings.len();

    // Remove bindings for paths that don't exist
    cfg.bindings.retain(|path, _| Path::new(path).exists());

    let removed = before - cfg.bindings.len();

    println!();
    if removed > 0 {
        ui::ok(&format!("pruned {} stale bindings", removed));
    } else {
        println!("  {} no stale bindings found", "○".dimmed());
    }
    println!();

    Ok(())
}
