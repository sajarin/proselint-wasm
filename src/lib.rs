//! proselint-wasm: A full Rust/WASM port of proselint
//!
//! This library provides prose linting capabilities for English text,
//! compiled to WebAssembly for high-performance client-side usage.
//!
//! # Example (Native Rust)
//!
//! ```rust
//! use proselint_wasm::{Linter, LintResult};
//!
//! let linter = Linter::new();
//! let results = linter.check("This is very very important.");
//!
//! for result in results {
//!     println!("{}: {} (line {}, col {})",
//!         result.check, result.message, result.line, result.column);
//! }
//! ```
//!
//! # Example (WASM/JavaScript)
//!
//! ```javascript
//! import init, { Proselint } from 'proselint-wasm';
//!
//! await init();
//! const linter = new Proselint();
//! const results = JSON.parse(linter.lint("This is very very important."));
//! ```

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

mod engine;
mod position;
mod check;
mod checks;
mod config;

// Re-export core types
pub use engine::*;
pub use position::*;
pub use check::*;
pub use config::*;

// Re-export check registry functions for native Rust users
pub use checks::{get_all_checks, get_all_check_ids, get_checks_by_category};

/// A single lint result representing a detected issue
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LintResult {
    /// The check identifier (e.g., "typography.symbols.ellipsis")
    pub check: String,
    /// Human-readable message describing the issue
    pub message: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Start position in the text (0-indexed byte offset)
    pub start: usize,
    /// End position in the text (0-indexed byte offset)
    pub end: usize,
    /// Severity level (error, warning, suggestion)
    pub severity: String,
    /// Optional replacement text
    pub replacement: Option<String>,
}

impl LintResult {
    /// Returns true if this is an error-level issue
    pub fn is_error(&self) -> bool {
        self.severity == "error"
    }

    /// Returns true if this is a warning-level issue
    pub fn is_warning(&self) -> bool {
        self.severity == "warning"
    }

    /// Returns true if this is a suggestion-level issue
    pub fn is_suggestion(&self) -> bool {
        self.severity == "suggestion"
    }

    /// Get the category of this check (e.g., "typography" from "typography.symbols.ellipsis")
    pub fn category(&self) -> &str {
        self.check.split('.').next().unwrap_or(&self.check)
    }

    /// Get the span of text that triggered this issue
    pub fn span(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

impl fmt::Display for LintResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: [{}] {} ({})",
            self.line, self.column, self.severity, self.message, self.check
        )
    }
}

// ============================================================================
// Native Rust API
// ============================================================================

/// Native Rust linter for prose checking
///
/// This provides a more ergonomic API for Rust users compared to the WASM API.
///
/// # Example
///
/// ```rust
/// use proselint_wasm::Linter;
///
/// let linter = Linter::new();
/// let results = linter.check("This is very very important.");
///
/// for result in &results {
///     if result.is_error() {
///         eprintln!("Error: {}", result);
///     }
/// }
///
/// // Filter by category
/// let typography_issues: Vec<_> = results.iter()
///     .filter(|r| r.category() == "typography")
///     .collect();
/// ```
#[derive(Debug, Clone)]
pub struct Linter {
    config: Config,
}

impl Linter {
    /// Create a new Linter with default configuration
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Create a new Linter with custom configuration
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// Check text and return lint results
    pub fn check(&self, text: &str) -> Vec<LintResult> {
        engine::lint_text(text, &self.config)
    }

    /// Check text and return only errors
    pub fn check_errors(&self, text: &str) -> Vec<LintResult> {
        self.check(text).into_iter().filter(|r| r.is_error()).collect()
    }

    /// Check text and return only warnings and errors
    pub fn check_warnings(&self, text: &str) -> Vec<LintResult> {
        self.check(text).into_iter().filter(|r| !r.is_suggestion()).collect()
    }

    /// Check text and return results for a specific category
    pub fn check_category(&self, text: &str, category: &str) -> Vec<LintResult> {
        self.check(text).into_iter().filter(|r| r.category() == category).collect()
    }

    /// Returns true if the text has any issues
    pub fn has_issues(&self, text: &str) -> bool {
        !self.check(text).is_empty()
    }

    /// Returns the number of issues found in the text
    pub fn issue_count(&self, text: &str) -> usize {
        self.check(text).len()
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a mutable reference to the configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Get a list of all available check IDs
    pub fn available_checks() -> Vec<&'static str> {
        checks::get_all_check_ids()
    }

    /// Get the library version
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to quickly check text with default settings
pub fn check_text(text: &str) -> Vec<LintResult> {
    Linter::new().check(text)
}

/// The main linter struct exposed to JavaScript
#[wasm_bindgen]
pub struct Proselint {
    config: Config,
}

#[wasm_bindgen]
impl Proselint {
    /// Create a new Proselint instance with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Create a new Proselint instance with custom configuration (JSON string)
    #[wasm_bindgen]
    pub fn with_config(config_json: &str) -> Result<Proselint, JsValue> {
        let config: Config = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid config: {}", e)))?;
        Ok(Self { config })
    }

    /// Lint the provided text and return results as JSON
    #[wasm_bindgen]
    pub fn lint(&self, text: &str) -> String {
        let results = engine::lint_text(text, &self.config);
        serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string())
    }

    /// Lint the provided text and return the number of issues found
    #[wasm_bindgen]
    pub fn lint_count(&self, text: &str) -> usize {
        engine::lint_text(text, &self.config).len()
    }

    /// Get the version of proselint-wasm
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Get a list of all available checks
    #[wasm_bindgen]
    pub fn available_checks() -> String {
        serde_json::to_string(&checks::get_all_check_ids()).unwrap_or_else(|_| "[]".to_string())
    }
}

impl Default for Proselint {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick lint function for simple usage
#[wasm_bindgen]
pub fn lint(text: &str) -> String {
    let proselint = Proselint::new();
    proselint.lint(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lint() {
        let proselint = Proselint::new();
        let results = proselint.lint("This is very very bad.");
        assert!(!results.is_empty());
    }
}
