use anyhow::{Result, bail};
use colored::Colorize;

use crate::config::Config;
use crate::ui;

/// Executes the `switch` command, changing the active account.
///
/// # Arguments
/// * `cfg` - Mutable reference to the application configuration.
/// * `alias` - Optional alias of the account to switch to. If None, cycles to the next account.
/// * `force` - If true, automatically apply the identity to the current repository.
pub fn run(cfg: &mut Config, alias: Option<String>, force: bool) -> Result<()> {
    if cfg.accounts.is_empty() {
        bail!("no accounts configured — run `git-manage init` first");
    }

    let target = match alias {
        Some(ref a) => {
            if cfg.find(a).is_none() {
                bail!("unknown alias '{}' — run `git-manage list` to see options", a);
            }
            a.clone()
        }
        None => {
            // cycle: current index + 1
            let cur = cfg
                .active
                .as_ref()
                .and_then(|a| cfg.accounts.iter().position(|acc| &acc.alias == a))
                .unwrap_or(usize::MAX);
            let next = (cur.wrapping_add(1)) % cfg.accounts.len();
            cfg.accounts[next].alias.clone()
        }
    };

    let acc = cfg.find(&target).unwrap().clone();
    cfg.active = Some(target.clone());
    cfg.log_switch(&target, None);
    cfg.save()?;

    ui::header("switched account");
    ui::print_account(&acc, true);
    ui::footer();

    if force {
        super::apply::run(cfg, false)?;
    } else {
        ui::tip(&format!(
            "run {} to apply to current repo",
            "`git-manage apply`".bold()
        ));
        println!();
    }

    Ok(())
}
