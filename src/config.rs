//! Configuration module for proselint-wasm
//!
//! Handles check enabling/disabling and linting options.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the linter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Maximum number of errors to return (0 = unlimited)
    #[serde(default = "default_max_errors")]
    pub max_errors: usize,

    /// Whether to check inside quoted text
    #[serde(default = "default_true")]
    pub check_quotes: bool,

    /// Enabled/disabled status for each check category
    #[serde(default)]
    pub checks: HashMap<String, bool>,
}

fn default_max_errors() -> usize {
    0 // unlimited
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_errors: 0,
            check_quotes: true,
            checks: HashMap::new(),
        }
    }
}

impl Config {
    /// Create a new config with all checks enabled
    pub fn all_enabled() -> Self {
        Self::default()
    }

    /// Create a config with specific checks enabled
    pub fn with_checks(check_ids: Vec<&str>) -> Self {
        let mut checks = HashMap::new();
        for id in check_ids {
            checks.insert(id.to_string(), true);
        }
        Self {
            checks,
            ..Default::default()
        }
    }

    /// Check if a specific check is enabled
    pub fn is_check_enabled(&self, check_id: &str) -> bool {
        // If no specific checks are configured, all are enabled
        if self.checks.is_empty() {
            return true;
        }

        // Check for exact match
        if let Some(&enabled) = self.checks.get(check_id) {
            return enabled;
        }

        // Check for category match (e.g., "typography" matches "typography.symbols.ellipsis")
        for (key, &enabled) in &self.checks {
            if check_id.starts_with(key) {
                return enabled;
            }
        }

        // Default to enabled if no matching config found
        true
    }

    /// Disable a specific check or category
    pub fn disable(&mut self, check_id: &str) {
        self.checks.insert(check_id.to_string(), false);
    }

    /// Enable a specific check or category
    pub fn enable(&mut self, check_id: &str) {
        self.checks.insert(check_id.to_string(), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.is_check_enabled("typography.symbols.ellipsis"));
        assert!(config.is_check_enabled("weasel_words.very"));
    }

    #[test]
    fn test_disabled_check() {
        let mut config = Config::default();
        config.disable("typography");
        assert!(!config.is_check_enabled("typography.symbols.ellipsis"));
        assert!(config.is_check_enabled("weasel_words.very"));
    }
}
