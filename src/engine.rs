//! Core linting engine for proselint-wasm
//!
//! Uses Aho-Corasick for fast keyword pre-filtering before running expensive regexes.

use crate::{LintResult, Config};
use crate::position::{LineTracker, QuoteTracker};
use crate::checks;
use crate::check::{Check, Severity, get_cached_regex};
use aho_corasick::AhoCorasick;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Pre-built Aho-Corasick automaton for fast keyword scanning
struct AcIndex {
    /// The automaton
    ac: AhoCorasick,
    /// Map from AC pattern index to check indices (a keyword may appear in multiple checks)
    pattern_to_checks: Vec<Vec<usize>>,
    /// Checks that need pure regex (no simple keyword to extract)
    pure_regex_checks: Vec<usize>,
}

/// Global AC index - built once on first use
static AC_INDEX: OnceLock<AcIndex> = OnceLock::new();

/// Extract a simple keyword from a check pattern (if possible)
/// Returns None if the pattern is too complex (has regex metacharacters)
fn extract_keyword(check: &Check) -> Option<String> {
    let pattern = check.pattern;

    // Skip patterns with regex metacharacters that make keyword extraction unreliable
    // We want simple word/phrase patterns only
    let metacharacters = ['[', ']', '(', ')', '{', '}', '|', '*', '+', '?', '^', '$', '.', '\\'];

    if pattern.chars().any(|c| metacharacters.contains(&c)) {
        return None;
    }

    // For simple patterns, return the pattern as lowercase keyword
    let keyword = pattern.to_lowercase();
    if keyword.len() >= 2 {
        Some(keyword)
    } else {
        None
    }
}

/// Build the Aho-Corasick index from all checks
fn build_ac_index() -> AcIndex {
    let all_checks = checks::get_all_checks();

    let mut keywords: Vec<String> = Vec::new();
    let mut keyword_to_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut pattern_to_checks: Vec<Vec<usize>> = Vec::new();
    let mut pure_regex_checks: Vec<usize> = Vec::new();

    for (check_idx, check) in all_checks.iter().enumerate() {
        if let Some(keyword) = extract_keyword(check) {
            // Check if we already have this keyword
            if let Some(&kw_idx) = keyword_to_index.get(&keyword) {
                // Add this check to existing keyword
                pattern_to_checks[kw_idx].push(check_idx);
            } else {
                // New keyword
                let kw_idx = keywords.len();
                keyword_to_index.insert(keyword.clone(), kw_idx);
                keywords.push(keyword);
                pattern_to_checks.push(vec![check_idx]);
            }
        } else {
            // No simple keyword - must run regex unconditionally
            pure_regex_checks.push(check_idx);
        }
    }

    // Build the automaton (case-insensitive matching)
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(&keywords)
        .expect("Failed to build Aho-Corasick automaton");

    AcIndex {
        ac,
        pattern_to_checks,
        pure_regex_checks,
    }
}

/// Get or build the AC index
fn get_ac_index() -> &'static AcIndex {
    AC_INDEX.get_or_init(build_ac_index)
}

/// Pre-compile ALL regexes upfront
/// This eliminates regex compilation from lint calls entirely
/// Returns the number of regexes compiled
pub fn warm_all_checks() -> usize {
    let all_checks = checks::get_all_checks();

    // Force AC index construction
    let _ = get_ac_index();

    // Compile every check's regex
    let mut count = 0;
    for check in all_checks.iter() {
        if check.get_regex().is_some() {
            count += 1;
        }
    }
    count
}

/// Lint the provided text using Aho-Corasick pre-filtering
pub fn lint_text(text: &str, config: &Config) -> Vec<LintResult> {
    let mut results = Vec::new();

    // Create position trackers
    let line_tracker = LineTracker::new(text);
    let quote_tracker = QuoteTracker::new(text);

    // Get all checks and AC index
    let all_checks = checks::get_all_checks();
    let ac_index = get_ac_index();

    // Step 1: Fast AC scan to find which checks might match
    let mut checks_to_run: HashSet<usize> = HashSet::new();

    // Add pure regex checks (must always run)
    for &check_idx in &ac_index.pure_regex_checks {
        if config.is_check_enabled(all_checks[check_idx].id) {
            checks_to_run.insert(check_idx);
        }
    }

    // Scan text with Aho-Corasick to find keyword matches
    let text_lower = text.to_lowercase();
    for mat in ac_index.ac.find_iter(&text_lower) {
        let pattern_idx = mat.pattern().as_usize();
        for &check_idx in &ac_index.pattern_to_checks[pattern_idx] {
            if config.is_check_enabled(all_checks[check_idx].id) {
                checks_to_run.insert(check_idx);
            }
        }
    }

    // Step 2: Run only the checks that might have matches
    for check_idx in checks_to_run {
        let check = &all_checks[check_idx];

        // Run the check's regex
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
/// Uses cached regexes for performance
pub fn lint_existence(
    text: &str,
    check_id: &str,
    message: &str,
    patterns: &[&str],
    config: &Config,
) -> Vec<LintResult> {
    let mut results = Vec::new();
    let line_tracker = LineTracker::new(text);
    let quote_tracker = QuoteTracker::new(text);

    if !config.is_check_enabled(check_id) {
        return results;
    }

    for &pattern in patterns {
        let escaped = regex::escape(pattern);
        let re_pattern = format!(r"(?i)\b{}\b", escaped);

        if let Some(re) = get_cached_regex(&re_pattern) {
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
        println!("Found {} issues", results.len());
    }

    #[test]
    fn test_ac_index_build() {
        let index = get_ac_index();
        println!("AC index: {} keywords, {} pure regex checks",
            index.pattern_to_checks.len(),
            index.pure_regex_checks.len());
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
