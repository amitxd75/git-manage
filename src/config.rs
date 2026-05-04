//! Configuration management for git-manage.
//! Handles account storage, switch history, and repository bindings.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// ─── Types ────────────────────────────────────────────────────────────────────

/// Represents a GitHub account profile.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    /// Unique alias for the account (e.g. "work", "personal")
    pub alias: String,
    pub username: String,
    pub email: String,
    /// Whether this is the primary machine-wide account
    pub primary: bool,
    /// Optional SSH host alias from ~/.ssh/config (e.g. github-alt)
    pub ssh_host: Option<String>,
}

/// An entry in the account switch history.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchEntry {
    /// Alias of the account switched to
    pub alias: String,
    /// Timestamp of the switch
    pub at: DateTime<Utc>,
    /// Canonical path to the repository if applied locally
    pub repo: Option<String>,
}

/// The main application configuration state.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    /// Currently active account alias
    pub active: Option<String>,
    /// List of all configured accounts
    pub accounts: Vec<Account>,
    /// Mapping of repository paths (canonical) to account aliases
    pub bindings: HashMap<String, String>,
    /// Recent switch history, limited to the last 100 entries
    pub switch_log: Vec<SwitchEntry>,
}

// ─── Impl ─────────────────────────────────────────────────────────────────────

impl Config {
    /// Returns the standard path for the configuration file: ~/.config/git-manage/config.json
    pub fn path() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .context("couldn't resolve config directory")?
            .join("git-manage");
        fs::create_dir_all(&dir).context("failed to create config directory")?;
        Ok(dir.join("config.json"))
    }

    /// Loads the configuration from disk, or returns a default config if it doesn't exist.
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(&path).context("failed to read config file")?;
        serde_json::from_str(&raw).context("failed to parse config file")
    }

    /// Persists the configuration to disk as pretty-printed JSON.
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json).context("failed to write config file")?;
        Ok(())
    }

    // ─── Account helpers ──────────────────────────────────────────────────────

    /// Finds an account by its alias.
    ///
    /// # Arguments
    /// * `alias` - The unique alias of the account to find.
    pub fn find(&self, alias: &str) -> Option<&Account> {
        self.accounts.iter().find(|a| a.alias == alias)
    }

    /// Finds a mutable reference to an account by its alias.
    ///
    /// # Arguments
    /// * `alias` - The unique alias of the account to find.
    pub fn find_mut(&mut self, alias: &str) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.alias == alias)
    }

    /// Returns the currently active account based on the stored alias.
    pub fn active_account(&self) -> Option<&Account> {
        self.active.as_ref().and_then(|a| self.find(a))
    }

    /// Returns the primary account if one is configured.
    pub fn primary(&self) -> Option<&Account> {
        self.accounts.iter().find(|a| a.primary)
    }

    // ─── Log helpers ──────────────────────────────────────────────────────────

    /// Records a new switch entry in the history, keeping the log bounded to 100 entries.
    ///
    /// # Arguments
    /// * `alias` - The alias of the account switched to.
    /// * `repo` - The optional path to the repository where the switch occurred.
    pub fn log_switch(&mut self, alias: &str, repo: Option<String>) {
        self.switch_log.insert(
            0,
            SwitchEntry {
                alias: alias.to_string(),
                at: Utc::now(),
                repo,
            },
        );
        self.switch_log.truncate(100);
    }
}
