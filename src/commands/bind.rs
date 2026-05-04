//! Command to bind a repository path to a specific account.

use anyhow::{Context, Result, bail};
use colored::Colorize;

use crate::config::Config;
use crate::git;
use crate::ui;

/// Runs the bind command.
///
/// # Arguments
/// * `cfg` - The project configuration.
/// * `alias` - The account alias to bind to the path.
/// * `path` - Optional path to bind (defaults to current repo root if omitted).
pub fn run(cfg: &mut Config, alias: String, path: Option<String>) -> Result<()> {
    if cfg.find(&alias).is_none() {
        bail!("unknown alias '{}' — run `git-manage list` to see options", alias);
    }

    let repo_path = match path {
        Some(p) => {
            let canonical = std::fs::canonicalize(&p)
                .with_context(|| format!("path '{}' not found", p))?;
            canonical.to_string_lossy().to_string()
        }
        None => {
            let root = git::repo_root()
                .context("not inside a git repo and no path given")?;
            root.to_string_lossy().to_string()
        }
    };

    cfg.bindings.insert(repo_path.clone(), alias.clone());
    cfg.save()?;

    println!();
    ui::ok(&format!("bound {} → {}", repo_path.dimmed(), alias.bold()));
    println!(
        "    the pre-commit hook will auto-apply '{}' in this repo",
        alias.bold()
    );
    println!();

    Ok(())
}
