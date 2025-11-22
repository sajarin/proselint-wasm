//! Check definition module for proselint-wasm
//!
//! Defines the Check trait and common check types.

use regex::Regex;
use std::fmt;
use std::str::FromStr;

/// Severity levels for lint results
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Suggestions are optional style improvements
    Suggestion,
    /// Warnings indicate likely issues that should be addressed
    Warning,
    /// Errors are definite problems that must be fixed
    Error,
}

impl Severity {
    /// Convert to lowercase string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Suggestion => "suggestion",
        }
    }

    /// Returns true if this severity is at least as severe as the given level
    pub fn at_least(&self, other: Severity) -> bool {
        *self >= other
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Severity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Severity::Error),
            "warning" => Ok(Severity::Warning),
            "suggestion" => Ok(Severity::Suggestion),
            _ => Err("Invalid severity: expected 'error', 'warning', or 'suggestion'"),
        }
    }
}

impl Default for Severity {
    fn default() -> Self {
        Severity::Warning
    }
}

/// A single check definition
pub struct Check {
    /// Unique identifier (e.g., "typography.symbols.ellipsis")
    pub id: &'static str,
    /// Human-readable message
    pub message: &'static str,
    /// The regex pattern to match
    pub pattern: &'static str,
    /// Severity level
    pub severity: Severity,
    /// Whether this check can match inside quotes
    pub allow_quotes: bool,
    /// Optional replacement text (can contain capture groups like $1)
    pub replacement: Option<&'static str>,
    /// Whether to use raw pattern (no word boundaries)
    pub raw_pattern: bool,
    /// Compiled regex (lazily initialized)
    #[doc(hidden)]
    pub compiled: Option<Regex>,
}

impl Check {
    /// Create a new check with default settings
    pub const fn new(
        id: &'static str,
        message: &'static str,
        pattern: &'static str,
    ) -> Self {
        Self {
            id,
            message,
            pattern,
            severity: Severity::Warning,
            allow_quotes: false,
            replacement: None,
            raw_pattern: false,
            compiled: None,
        }
    }

    /// Create a check with custom severity
    pub const fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Allow matching inside quotes
    pub const fn allow_in_quotes(mut self) -> Self {
        self.allow_quotes = true;
        self
    }

    /// Add a replacement suggestion
    pub const fn with_replacement(mut self, replacement: &'static str) -> Self {
        self.replacement = Some(replacement);
        self
    }

    /// Use raw pattern without word boundaries (for symbols, punctuation)
    pub const fn raw(mut self) -> Self {
        self.raw_pattern = true;
        self
    }

    /// Get or compile the regex pattern
    pub fn regex(&self) -> Option<Regex> {
        let pattern = if self.raw_pattern {
            // Use pattern as-is (for symbols, punctuation)
            format!(r"(?i){}", self.pattern)
        } else {
            // Wrap pattern with word boundaries (for words/phrases)
            format!(r"(?i)\b{}\b", self.pattern)
        };
        Regex::new(&pattern).ok()
    }

    /// Run this check on text and return matches
    pub fn run(&self, text: &str) -> Vec<(usize, usize, Option<String>)> {
        let mut results = Vec::new();

        if let Some(re) = self.regex() {
            for mat in re.find_iter(text) {
                let replacement = self.replacement.map(|r| {
                    // Simple replacement (no capture group handling for now)
                    r.to_string()
                });
                results.push((mat.start(), mat.end(), replacement));
            }
        }

        results
    }
}

/// A check that matches simple word/phrase existence
pub struct ExistenceCheck {
    pub id: &'static str,
    pub message: &'static str,
    pub patterns: &'static [&'static str],
    pub severity: Severity,
    pub allow_quotes: bool,
    pub exceptions: &'static [&'static str],
}

impl ExistenceCheck {
    /// Run this check on text
    pub fn run(&self, text: &str) -> Vec<(usize, usize, Option<String>)> {
        let mut results = Vec::new();

        for &pattern in self.patterns {
            // Skip if pattern matches an exception
            let mut is_exception = false;
            for &exception in self.exceptions {
                if pattern.contains(exception) || exception.contains(pattern) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            // Build word-boundary regex
            let escaped = regex::escape(pattern);
            let re_pattern = format!(r"(?i)\b{}\b", escaped);

            if let Ok(re) = Regex::new(&re_pattern) {
                for mat in re.find_iter(text) {
                    results.push((mat.start(), mat.end(), None));
                }
            }
        }

        results
    }
}

/// A check that looks for pairs of words/phrases
pub struct PairCheck {
    pub id: &'static str,
    pub message: &'static str,
    pub first: &'static str,
    pub second: &'static str,
    pub max_distance: usize,
    pub severity: Severity,
}

impl PairCheck {
    /// Run this check on text
    pub fn run(&self, text: &str) -> Vec<(usize, usize, Option<String>)> {
        let mut results = Vec::new();

        let first_re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(self.first))).ok();
        let second_re = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(self.second))).ok();

        if let (Some(first_re), Some(second_re)) = (first_re, second_re) {
            let first_matches: Vec<_> = first_re.find_iter(text).collect();
            let second_matches: Vec<_> = second_re.find_iter(text).collect();

            for first_match in &first_matches {
                for second_match in &second_matches {
                    if second_match.start() > first_match.end() {
                        let distance = second_match.start() - first_match.end();
                        if distance <= self.max_distance {
                            results.push((first_match.start(), second_match.end(), None));
                        }
                    }
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_check() {
        let check = Check::new(
            "test.very",
            "Consider removing 'very'",
            r"very",
        );

        let results = check.run("This is very good.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_existence_check() {
        let check = ExistenceCheck {
            id: "test.cliche",
            message: "Avoid cliches",
            patterns: &["at the end of the day", "think outside the box"],
            severity: Severity::Warning,
            allow_quotes: false,
            exceptions: &[],
        };

        let results = check.run("At the end of the day, we need to think outside the box.");
        assert_eq!(results.len(), 2);
    }
}
