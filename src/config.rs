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

    /// Cache for check_enabled lookups (not serialized)
    /// Maps check IDs to their enabled status for O(1) lookups
    #[serde(skip)]
    cache: HashMap<String, bool>,

    /// Sorted list of prefixes for efficient prefix matching (not serialized)
    #[serde(skip)]
    prefixes: Vec<String>,
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
            cache: HashMap::new(),
            prefixes: Vec::new(),
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
        let mut config = Self {
            checks,
            ..Default::default()
        };
        config.rebuild_cache();
        config
    }

    /// Rebuild the internal cache and prefix list
    /// Should be called after modifying the checks HashMap
    fn rebuild_cache(&mut self) {
        self.cache.clear();
        self.prefixes = self.checks.keys().cloned().collect();
        // Sort by length (descending) for better prefix matching
        // Longer prefixes should be checked first
        self.prefixes.sort_by_key(|b| std::cmp::Reverse(b.len()));
    }

    /// Check if a specific check is enabled
    /// Uses O(1) cache lookup for repeated queries
    pub fn is_check_enabled(&self, check_id: &str) -> bool {
        // If no specific checks are configured, all are enabled
        if self.checks.is_empty() {
            return true;
        }

        // Check cache first for O(1) lookup
        if let Some(&enabled) = self.cache.get(check_id) {
            return enabled;
        }

        // Check for exact match
        if let Some(&enabled) = self.checks.get(check_id) {
            // Don't cache here as it would require mutable access
            return enabled;
        }

        // Check for prefix match using sorted prefixes
        // Longer prefixes are checked first for more specific matches
        for prefix in &self.prefixes {
            if check_id.starts_with(prefix.as_str()) {
                let enabled = self.checks.get(prefix).copied().unwrap_or(true);
                // Don't cache here as it would require mutable access
                return enabled;
            }
        }

        // Default to enabled if no matching config found
        true
    }

    /// Disable a specific check or category
    /// Clears cache to ensure consistency
    pub fn disable(&mut self, check_id: &str) {
        self.checks.insert(check_id.to_string(), false);
        self.rebuild_cache();
    }

    /// Enable a specific check or category
    /// Clears cache to ensure consistency
    pub fn enable(&mut self, check_id: &str) {
        self.checks.insert(check_id.to_string(), true);
        self.rebuild_cache();
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
