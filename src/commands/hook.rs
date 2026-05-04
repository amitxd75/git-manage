use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use crate::cli::HookAction;
use crate::config::Config;
use crate::git;
use crate::ui;

/// The pre-commit script that git-manage installs.
/// Calls `git-manage whoami` and aborts if there's an identity mismatch.
/// The whoami command exits 1 when it detects a mismatch.
const HOOK_SCRIPT: &str = r#"#!/bin/sh
# git-manage: identity guard — installed by `git-manage hook install`
if command -v git-manage >/dev/null 2>&1; then
    output=$(git-manage whoami 2>&1)
    if echo "$output" | grep -q "warn:"; then
        echo "$output"
        echo ""
        echo "  ✗  commit blocked: git identity mismatch"
        echo "     run \`git-manage apply\` to fix it, then commit again"
        echo ""
        exit 1
    fi
fi
exit 0
"#;

const HOOK_MARKER: &str = "git-manage: identity guard";

/// Runs the hook command.
///
/// # Arguments
/// * `cfg` - The project configuration.
/// * `action` - The hook action to perform.
pub fn run(cfg: &Config, action: HookAction) -> Result<()> {
    let _ = cfg; // cfg reserved for future per-account hook config

    match action {
        HookAction::Install => install(),
        HookAction::Uninstall => uninstall(),
        HookAction::Status => status(),
    }
}

/// Returns the path to the pre-commit hook in the current repository.
fn hook_path() -> Result<std::path::PathBuf> {
    let git_dir = git::git_dir().context("not inside a git repo")?;
    Ok(git_dir.join("hooks").join("pre-commit"))
}

/// Installs the pre-commit guard hook.
fn install() -> Result<()> {
    let path = hook_path()?;
    let hooks_dir = path.parent().unwrap();
    fs::create_dir_all(hooks_dir)?;

    if path.exists() {
        let existing = fs::read_to_string(&path)?;
        if existing.contains(HOOK_MARKER) {
            println!();
            ui::ok("hook already installed");
            println!();
            return Ok(());
        }
        // append to existing hook
        let merged = format!("{}\n{}", existing.trim_end(), HOOK_SCRIPT);
        fs::write(&path, merged)?;
    } else {
        fs::write(&path, HOOK_SCRIPT)?;
    }

    // make executable
    let mut perms = fs::metadata(&path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms)?;

    println!();
    ui::ok("pre-commit guard hook installed");
    println!("    path: {}", path.display().to_string().dimmed());
    println!("    commits will be blocked if identity doesn't match active account");
    println!();

    Ok(())
}

/// Uninstalls the pre-commit guard hook.
fn uninstall() -> Result<()> {
    let path = hook_path()?;

    if !path.exists() {
        println!();
        println!("  {} no pre-commit hook found", "○".dimmed());
        println!();
        return Ok(());
    }

    let existing = fs::read_to_string(&path)?;
    if !existing.contains(HOOK_MARKER) {
        bail!("pre-commit hook exists but wasn't installed by git-manage — not touching it");
    }

    // If the hook is only ours, remove the file entirely
    let without_ours: String = existing
        .lines()
        .collect::<Vec<_>>()
        .split(|l: &&str| l.contains(HOOK_MARKER))
        .next()
        .unwrap_or(&[])
        .join("\n");

    let trimmed = without_ours.trim();
    if trimmed.is_empty() || trimmed == "#!/bin/sh" {
        fs::remove_file(&path)?;
        println!();
        ui::ok("hook removed");
    } else {
        // other content was in the file, just strip our block
        fs::write(&path, format!("{}\n", trimmed))?;
        println!();
        ui::ok("git-manage section removed from existing hook");
    }

    println!();
    Ok(())
}

/// Shows the status of the pre-commit guard hook for the current repository.
fn status() -> Result<()> {
    let path = hook_path()?;
    println!();

    if !path.exists() {
        println!("  {} no pre-commit hook installed", "○".dimmed());
        ui::tip("run `git-manage hook install` to add the identity guard");
    } else {
        let content = fs::read_to_string(&path)?;
        if content.contains(HOOK_MARKER) {
            println!("  {} pre-commit guard is {} for this repo", "●".green(), "active".green().bold());
        } else {
            println!("  {} pre-commit hook exists but not managed by git-manage", "○".yellow());
        }
        println!("    path: {}", path.display().to_string().dimmed());
    }

    println!();
    Ok(())
}
