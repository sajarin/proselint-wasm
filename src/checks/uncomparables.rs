//! Uncomparable word checks for proselint-wasm
//!
//! Detects improper comparison of absolute adjectives.

use crate::check::{Check, Severity};

/// Get all uncomparable checks
pub fn get_checks() -> Vec<Check> {
    // Words that cannot be compared (absolute adjectives)
    let uncomparables = [
        "absolute", "adequate", "chief", "complete", "correct", "devoid",
        "entire", "false", "fatal", "favorite", "final", "ideal", "impossible",
        "inevitable", "infinite", "irrevocable", "main", "manifest", "only",
        "paramount", "perfect", "perpetual", "possible", "preferable", "principal",
        "singular", "stationary", "sufficient", "true", "unanimous", "unavoidable",
        "unbroken", "uniform", "unique", "universal", "void", "whole"
    ];

    // Comparators that shouldn't be used with uncomparables
    let comparators = [
        ("very", "very"),
        ("more", "more"),
        ("less", "less"),
        ("most", "most"),
        ("least", "least"),
        ("extremely", "extremely"),
        ("quite", "quite"),
        ("rather", "rather"),
        ("somewhat", "somewhat"),
    ];

    let mut checks = Vec::new();

    // Generate checks for common problematic combinations
    for uncomp in &uncomparables {
        for (comp_name, comp_pattern) in &comparators {
            // Skip "more perfect" which is acceptable (Constitution)
            if *uncomp == "perfect" && *comp_name == "more" {
                continue;
            }

            let id = format!("uncomparables.{}_{}", comp_name, uncomp);
            let message = format!(
                "'{}' is an absolute and cannot be compared. Remove '{}'.",
                uncomp, comp_name
            );
            let pattern = format!(r"{}\s+{}", comp_pattern, uncomp);

            checks.push(
                Check::new(
                    Box::leak(id.into_boxed_str()),
                    Box::leak(message.into_boxed_str()),
                    Box::leak(pattern.into_boxed_str()),
                )
                .raw()
                .with_severity(Severity::Warning)
            );
        }
    }

    // Add some specific common violations manually for better messages
    checks.push(
        Check::new(
            "uncomparables.very_unique",
            "'unique' is absolute - something either is or isn't unique. Remove 'very'.",
            r"very unique",
        )
        .with_severity(Severity::Warning)
        .with_replacement("unique")
    );

    checks.push(
        Check::new(
            "uncomparables.most_unique",
            "'unique' is absolute - something either is or isn't unique. Remove 'most'.",
            r"most unique",
        )
        .with_severity(Severity::Warning)
        .with_replacement("unique")
    );

    checks.push(
        Check::new(
            "uncomparables.completely_unique",
            "'unique' is absolute - something either is or isn't unique. Remove 'completely'.",
            r"completely unique",
        )
        .with_severity(Severity::Warning)
        .with_replacement("unique")
    );

    checks.push(
        Check::new(
            "uncomparables.totally_unique",
            "'unique' is absolute - something either is or isn't unique. Remove 'totally'.",
            r"totally unique",
        )
        .with_severity(Severity::Warning)
        .with_replacement("unique")
    );

    checks.push(
        Check::new(
            "uncomparables.very_perfect",
            "'perfect' is absolute. Consider removing 'very'.",
            r"very perfect",
        )
        .with_severity(Severity::Warning)
        .with_replacement("perfect")
    );

    checks.push(
        Check::new(
            "uncomparables.more_infinite",
            "'infinite' is absolute and cannot be compared.",
            r"more infinite",
        )
        .with_severity(Severity::Warning)
        .with_replacement("infinite")
    );

    checks.push(
        Check::new(
            "uncomparables.very_impossible",
            "'impossible' is absolute - something either is or isn't impossible.",
            r"very impossible",
        )
        .with_severity(Severity::Warning)
        .with_replacement("impossible")
    );

    checks.push(
        Check::new(
            "uncomparables.more_complete",
            "'complete' is absolute. Consider 'more nearly complete' if needed.",
            r"more complete",
        )
        .with_severity(Severity::Suggestion)
    );

    checks.push(
        Check::new(
            "uncomparables.very_true",
            "'true' is absolute - something is either true or false.",
            r"very true",
        )
        .with_severity(Severity::Suggestion)
    );

    checks.push(
        Check::new(
            "uncomparables.more_equal",
            "'equal' is absolute. Consider 'more nearly equal' if needed.",
            r"more equal",
        )
        .with_severity(Severity::Suggestion)
    );

    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_very_unique() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "uncomparables.very_unique").unwrap();
        let results = check.run("This is very unique.");
        assert_eq!(results.len(), 1);
    }
}
