//! Hedging language checks for proselint-wasm
//!
//! Detects language that weakens assertions.

use crate::check::{Check, Severity};

/// Get all hedging checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // "I think"
        Check::new(
            "hedging.i_think",
            "'I think' can weaken your statement. Consider stating it directly.",
            r"I think",
        )
        .with_severity(Severity::Suggestion),

        // "I believe"
        Check::new(
            "hedging.i_believe",
            "'I believe' can weaken your argument. Consider stating it as fact.",
            r"I believe",
        )
        .with_severity(Severity::Suggestion),

        // "I feel"
        Check::new(
            "hedging.i_feel",
            "'I feel' is subjective. Consider using more objective language.",
            r"I feel",
        )
        .with_severity(Severity::Suggestion),

        // "It seems"
        Check::new(
            "hedging.seems",
            "'It seems' hedges your statement. Be more direct.",
            r"it seems",
        )
        .with_severity(Severity::Suggestion),

        // "Appears to be"
        Check::new(
            "hedging.appears",
            "'Appears to be' is hedging. Consider being more direct.",
            r"appears to be",
        )
        .with_severity(Severity::Suggestion),

        // "In my opinion"
        Check::new(
            "hedging.in_my_opinion",
            "'In my opinion' is often unnecessary. State your point directly.",
            r"in my opinion",
        )
        .with_severity(Severity::Suggestion),

        // "To be honest"
        Check::new(
            "hedging.to_be_honest",
            "'To be honest' implies you might not be honest otherwise. Consider removing it.",
            r"to be honest",
        )
        .with_severity(Severity::Warning),

        // "Sort of"
        Check::new(
            "hedging.sort_of",
            "'Sort of' is vague. Be more specific.",
            r"sort of",
        )
        .with_severity(Severity::Warning),

        // "Kind of"
        Check::new(
            "hedging.kind_of",
            "'Kind of' is vague. Be more specific.",
            r"kind of",
        )
        .with_severity(Severity::Warning),

        // "More or less"
        Check::new(
            "hedging.more_or_less",
            "'More or less' is imprecise. Be more specific.",
            r"more or less",
        )
        .with_severity(Severity::Warning),

        // "Could possibly"
        Check::new(
            "hedging.could_possibly",
            "'Could possibly' is redundant hedging. Use 'could' or 'possibly'.",
            r"could possibly",
        )
        .with_severity(Severity::Warning),

        // "Might be able to"
        Check::new(
            "hedging.might_be_able",
            "'Might be able to' is weak. Consider 'can' or 'will'.",
            r"might be able to",
        )
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_think_check() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "hedging.i_think").unwrap();

        let results = check.run("I think this is the right approach.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_to_be_honest_check() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "hedging.to_be_honest").unwrap();

        let results = check.run("To be honest, I didn't like the movie.");
        assert_eq!(results.len(), 1);
    }
}
