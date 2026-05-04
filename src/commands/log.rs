use anyhow::Result;
use colored::Colorize;
use chrono::Local;

use crate::config::Config;
use crate::ui;

/// Executes the `log` command, showing the history of account switches.
///
/// # Arguments
/// * `cfg` - Reference to the application configuration.
/// * `count` - The number of recent switch entries to display.
pub fn run(cfg: &Config, count: usize) -> Result<()> {
    ui::header("switch log");

    if cfg.switch_log.is_empty() {
        println!("  {} no switches recorded yet", "○".dimmed());
    } else {
        for entry in cfg.switch_log.iter().take(count) {
            let local_time = entry.at.with_timezone(&Local);
            let time_str = local_time.format("%Y-%m-%d %H:%M:%S").to_string();

            let acc = cfg.find(&entry.alias);
            let kind = acc.map(|a| if a.primary { "[primary]".yellow().to_string() } else { "[alt]".cyan().to_string() }).unwrap_or_default();

            print!("  {} {} {}", time_str.dimmed(), entry.alias.bold(), kind);

            if let Some(repo) = &entry.repo {
                print!("  {}", repo.dimmed());
            }
            println!();
        }
    }

    ui::footer();
    Ok(())
}
