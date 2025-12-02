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

use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

/// Maximum allowed text size for linting (10 MB)
/// Prevents excessive memory usage and potential DoS
pub const MAX_TEXT_SIZE: usize = 10 * 1024 * 1024;

/// Maximum allowed batch size for lint_batch
pub const MAX_BATCH_SIZE: usize = 100;

mod check;
mod checks;
mod config;
mod engine;
mod position;

// Re-export core types
pub use check::*;
pub use config::*;
pub use engine::*;
pub use position::*;

// Re-export check registry functions for native Rust users
pub use checks::{get_all_check_ids, get_all_checks, get_checks_by_category, validate_all_checks};

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
        self.check(text)
            .into_iter()
            .filter(|r| r.is_error())
            .collect()
    }

    /// Check text and return only warnings and errors
    pub fn check_warnings(&self, text: &str) -> Vec<LintResult> {
        self.check(text)
            .into_iter()
            .filter(|r| !r.is_suggestion())
            .collect()
    }

    /// Check text and return results for a specific category
    pub fn check_category(&self, text: &str, category: &str) -> Vec<LintResult> {
        self.check(text)
            .into_iter()
            .filter(|r| r.category() == category)
            .collect()
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

    /// Check multiple texts in parallel (requires `parallel` feature)
    ///
    /// This method uses rayon for parallel processing, which can significantly
    /// improve performance when linting many texts.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "parallel")]
    /// # {
    /// use proselint_wasm::Linter;
    ///
    /// let linter = Linter::new();
    /// let texts = vec!["Text 1", "Text 2", "Text 3"];
    /// let results = linter.check_parallel(&texts);
    /// # }
    /// ```
    #[cfg(feature = "parallel")]
    pub fn check_parallel(&self, texts: &[&str]) -> Vec<Vec<LintResult>> {
        use rayon::prelude::*;

        texts.par_iter().map(|text| self.check(text)).collect()
    }

    /// Check multiple texts in parallel with a custom thread pool
    ///
    /// This allows fine-grained control over the number of threads used.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "parallel")]
    /// # {
    /// use proselint_wasm::Linter;
    /// use rayon::ThreadPoolBuilder;
    ///
    /// let linter = Linter::new();
    /// let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    /// let texts = vec!["Text 1", "Text 2", "Text 3"];
    ///
    /// let results = linter.check_parallel_with_pool(&pool, &texts);
    /// # }
    /// ```
    #[cfg(feature = "parallel")]
    pub fn check_parallel_with_pool(
        &self,
        pool: &rayon::ThreadPool,
        texts: &[&str],
    ) -> Vec<Vec<LintResult>> {
        use rayon::prelude::*;

        pool.install(|| texts.par_iter().map(|text| self.check(text)).collect())
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
    /// Returns a JSON array of lint results, or a JSON object with an "error" field if something goes wrong
    #[wasm_bindgen]
    pub fn lint(&self, text: &str) -> String {
        // Validate text size
        if text.len() > MAX_TEXT_SIZE {
            return format!(
                r#"{{"error": "Text too large: {} bytes (max {} bytes)"}}"#,
                text.len(),
                MAX_TEXT_SIZE
            );
        }

        let results = engine::lint_text(text, &self.config);
        serde_json::to_string(&results).unwrap_or_else(|e| {
            // Return error as JSON object instead of empty array
            format!(r#"{{"error": "Failed to serialize results: {}"}}"#, e)
        })
    }

    /// Lint the provided text and return the number of issues found
    #[wasm_bindgen]
    pub fn lint_count(&self, text: &str) -> usize {
        engine::lint_text(text, &self.config).len()
    }

    /// Lint multiple texts in a single call and return results as JSON array of arrays
    /// Input: JSON array of strings (texts to lint)
    /// Output: JSON array of arrays (results for each text), or JSON object with "error" field
    #[wasm_bindgen]
    pub fn lint_batch(&self, texts_json: &str) -> String {
        let texts: Vec<String> = match serde_json::from_str(texts_json) {
            Ok(t) => t,
            Err(e) => {
                return format!(r#"{{"error": "Failed to parse input JSON: {}"}}"#, e);
            }
        };

        // Validate batch size
        if texts.len() > MAX_BATCH_SIZE {
            return format!(
                r#"{{"error": "Batch too large: {} texts (max {} texts)"}}"#,
                texts.len(),
                MAX_BATCH_SIZE
            );
        }

        // Validate individual text sizes
        for (idx, text) in texts.iter().enumerate() {
            if text.len() > MAX_TEXT_SIZE {
                return format!(
                    r#"{{"error": "Text {} too large: {} bytes (max {} bytes)"}}"#,
                    idx,
                    text.len(),
                    MAX_TEXT_SIZE
                );
            }
        }

        let results: Vec<Vec<LintResult>> = texts
            .iter()
            .map(|text| engine::lint_text(text, &self.config))
            .collect();

        serde_json::to_string(&results)
            .unwrap_or_else(|e| format!(r#"{{"error": "Failed to serialize results: {}"}}"#, e))
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

    /// Pre-compile ALL regexes for maximum performance
    /// Call this once during initialization to pay the compilation cost upfront
    #[wasm_bindgen]
    pub fn warm_all(&self) -> usize {
        engine::warm_all_checks()
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

    #[cfg(test)]
    mod property_tests {
        use super::*;
        use quickcheck_macros::quickcheck;

        #[quickcheck]
        fn prop_lint_never_panics(text: String) -> bool {
            let linter = Linter::new();
            // Should never panic regardless of input
            let _results = linter.check(&text);
            true
        }

        #[quickcheck]
        fn prop_lint_respects_max_size(text: String) -> bool {
            let proselint = Proselint::new();

            // Create oversized text if needed
            let large_text = if text.len() > MAX_TEXT_SIZE {
                text
            } else {
                "x".repeat(MAX_TEXT_SIZE + 1)
            };

            let result = proselint.lint(&large_text);

            // Should return error, not results
            result.contains("error") && result.contains("too large")
        }

        #[quickcheck]
        fn prop_lint_results_valid_positions(text: String) -> bool {
            let linter = Linter::new();
            let results = linter.check(&text);

            for result in results {
                // Positions must be within text bounds
                if result.start > text.len() || result.end > text.len() {
                    return false;
                }

                // Start must be before end
                if result.start > result.end {
                    return false;
                }

                // Line and column must be at least 1
                if result.line < 1 || result.column < 1 {
                    return false;
                }
            }

            true
        }

        #[quickcheck]
        fn prop_config_disable_reduces_results(text: String) -> bool {
            if text.len() < 10 {
                return true; // Discard short texts
            }

            let linter_all = Linter::new();
            let results_all = linter_all.check(&text);

            let mut config = Config::default();
            config.disable("weasel_words");

            let linter_filtered = Linter::with_config(config);
            let results_filtered = linter_filtered.check(&text);

            // Filtered should have <= results than all enabled
            results_filtered.len() <= results_all.len()
        }
    }
}
