//! Command to clear the account switch history.

use anyhow::Result;
use crate::config::Config;
use crate::ui;

/// Runs the clear-log command.
///
/// # Arguments
/// * `cfg` - The project configuration.
pub fn run(cfg: &mut Config) -> Result<()> {
    let count = cfg.switch_log.len();
    cfg.switch_log.clear();
    cfg.save()?;

    println!();
    ui::ok(&format!("cleared {} entries from switch log", count));
    println!();

    Ok(())
}
