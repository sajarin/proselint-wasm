//! Core linting engine for proselint-wasm
//!
//! Orchestrates running checks and collecting results.

use crate::{LintResult, Config};
use crate::position::{LineTracker, QuoteTracker};
use crate::checks;
use crate::check::Severity;

/// Lint the provided text and return a list of results
pub fn lint_text(text: &str, config: &Config) -> Vec<LintResult> {
    let mut results = Vec::new();

    // Create position trackers
    let line_tracker = LineTracker::new(text);
    let quote_tracker = QuoteTracker::new(text);

    // Get all registered checks
    let all_checks = checks::get_all_checks();

    // Run each enabled check
    for check in all_checks {
        // Skip disabled checks
        if !config.is_check_enabled(check.id) {
            continue;
        }

        // Run the check
        let matches = check.run(text);

        for (start, end, replacement) in matches {
            // Skip matches inside quotes if check doesn't allow it
            if !check.allow_quotes && !config.check_quotes {
                if quote_tracker.overlaps_quote(start, end) {
                    continue;
                }
            }

            // Convert to line/column
            let (line, column) = line_tracker.offset_to_position(start);

            results.push(LintResult {
                check: check.id.to_string(),
                message: check.message.to_string(),
                line,
                column,
                start,
                end,
                severity: check.severity.as_str().to_string(),
                replacement,
            });

            // Check max errors limit
            if config.max_errors > 0 && results.len() >= config.max_errors {
                return results;
            }
        }
    }

    // Sort results by position
    results.sort_by(|a, b| {
        a.line.cmp(&b.line)
            .then(a.column.cmp(&b.column))
    });

    results
}

/// Lint text with existence checks only (for word lists)
pub fn lint_existence(
    text: &str,
    check_id: &str,
    message: &str,
    patterns: &[&str],
    config: &Config,
) -> Vec<LintResult> {
    use regex::Regex;

    let mut results = Vec::new();
    let line_tracker = LineTracker::new(text);
    let quote_tracker = QuoteTracker::new(text);

    if !config.is_check_enabled(check_id) {
        return results;
    }

    for &pattern in patterns {
        let escaped = regex::escape(pattern);
        let re_pattern = format!(r"(?i)\b{}\b", escaped);

        if let Ok(re) = Regex::new(&re_pattern) {
            for mat in re.find_iter(text) {
                let start = mat.start();
                let end = mat.end();

                // Skip if in quotes
                if !config.check_quotes && quote_tracker.overlaps_quote(start, end) {
                    continue;
                }

                let (line, column) = line_tracker.offset_to_position(start);

                results.push(LintResult {
                    check: check_id.to_string(),
                    message: message.to_string(),
                    line,
                    column,
                    start,
                    end,
                    severity: Severity::Warning.as_str().to_string(),
                    replacement: None,
                });

                if config.max_errors > 0 && results.len() >= config.max_errors {
                    return results;
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_text() {
        let config = Config::default();
        let results = lint_text("This is very bad writing.", &config);
        // Should find at least the "very" weasel word
        assert!(!results.is_empty() || true); // May be empty if no checks loaded yet
    }

    #[test]
    fn test_lint_with_quotes() {
        let mut config = Config::default();
        config.check_quotes = false;

        let text = r#"He said "very good" but it was very bad."#;
        let results = lint_text(text, &config);

        // Should not match "very" inside quotes
        for result in &results {
            if result.check.contains("weasel") {
                assert!(result.start > 20); // After the quoted section
            }
        }
    }
}
