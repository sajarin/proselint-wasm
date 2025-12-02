//! Check registry module for proselint-wasm
//!
//! Registers and exposes all available checks.

pub mod annotations;
pub mod archaism;
pub mod cliches;
pub mod dates_times;
pub mod hedging;
pub mod industrial_language;
pub mod lexical_illusions;
pub mod malapropisms;
pub mod misc;
pub mod mixed_metaphors;
pub mod mondegreens;
pub mod needless_variants;
pub mod nonwords;
pub mod oxymorons;
pub mod preferred_forms;
pub mod psychology;
pub mod redundancy;
pub mod restricted;
pub mod skunked_terms;
pub mod social_awareness;
pub mod spelling;
pub mod terms;
pub mod typography;
pub mod uncomparables;
pub mod weasel_words;

use crate::check::Check;

// Cache the checks vector so we don't recreate it on every lint call
static ALL_CHECKS: once_cell::sync::Lazy<Vec<Check>> =
    once_cell::sync::Lazy::new(build_all_checks);

/// Get all registered checks (cached)
pub fn get_all_checks() -> &'static [Check] {
    &ALL_CHECKS
}

/// Validate all registered checks
/// Returns a list of validation errors, or empty vector if all checks are valid
pub fn validate_all_checks() -> Vec<String> {
    let mut errors = Vec::new();

    for check in get_all_checks() {
        if let Err(e) = check.validate_regex() {
            errors.push(e);
        }
    }

    errors
}

/// Build all checks (called once during initialization)
fn build_all_checks() -> Vec<Check> {
    let mut checks = Vec::new();

    // Typography checks
    checks.extend(typography::get_checks());

    // Weasel words
    checks.extend(weasel_words::get_checks());

    // Hedging
    checks.extend(hedging::get_checks());

    // Redundancy
    checks.extend(redundancy::get_checks());

    // Cliches
    checks.extend(cliches::get_checks());

    // Miscellaneous
    checks.extend(misc::get_checks());

    // Archaisms
    checks.extend(archaism::get_checks());

    // Annotations
    checks.extend(annotations::get_checks());

    // Dates and times
    checks.extend(dates_times::get_checks());

    // Industrial/corporate language
    checks.extend(industrial_language::get_checks());

    // Lexical illusions (repeated words)
    checks.extend(lexical_illusions::get_checks());

    // Malapropisms
    checks.extend(malapropisms::get_checks());

    // Mixed metaphors
    checks.extend(mixed_metaphors::get_checks());

    // Mondegreens (misheard phrases)
    checks.extend(mondegreens::get_checks());

    // Needless variants
    checks.extend(needless_variants::get_checks());

    // Nonwords
    checks.extend(nonwords::get_checks());

    // Oxymorons
    checks.extend(oxymorons::get_checks());

    // Psychology (clinical term misuse)
    checks.extend(psychology::get_checks());

    // Restricted vocabulary
    checks.extend(restricted::get_checks());

    // Skunked terms
    checks.extend(skunked_terms::get_checks());

    // Social awareness
    checks.extend(social_awareness::get_checks());

    // Spelling/word confusion
    checks.extend(spelling::get_checks());

    // Terms (animal adjectives, denizens, venery)
    checks.extend(terms::get_checks());

    // Uncomparables
    checks.extend(uncomparables::get_checks());

    // Preferred forms and phrasal adjectives
    checks.extend(preferred_forms::get_checks());

    checks
}

/// Get all check IDs
pub fn get_all_check_ids() -> Vec<&'static str> {
    get_all_checks().iter().map(|c| c.id).collect()
}

/// Get checks by category
pub fn get_checks_by_category(category: &str) -> Vec<&'static Check> {
    get_all_checks()
        .iter()
        .filter(|c| c.id.starts_with(category))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_checks() {
        let checks = get_all_checks();
        assert!(!checks.is_empty());
    }

    #[test]
    fn test_get_typography_checks() {
        let checks = get_checks_by_category("typography");
        assert!(!checks.is_empty());
    }
}
