//! Lexical illusion checks for proselint-wasm
//!
//! Detects unintentional word repetitions.

use crate::check::{Check, Severity};

/// Get all lexical illusion checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Common repeated words (not "had had" or "that that" which are valid)
        Check::new(
            "lexical_illusions.the_the",
            "Repeated word 'the the' - likely a typo.",
            r"the the",
        )
        .with_severity(Severity::Error)
        .with_replacement("the"),
        Check::new(
            "lexical_illusions.a_a",
            "Repeated word 'a a' - likely a typo.",
            r"a a",
        )
        .with_severity(Severity::Error)
        .with_replacement("a"),
        Check::new(
            "lexical_illusions.an_an",
            "Repeated word 'an an' - likely a typo.",
            r"an an",
        )
        .with_severity(Severity::Error)
        .with_replacement("an"),
        Check::new(
            "lexical_illusions.is_is",
            "Repeated word 'is is' - likely a typo.",
            r"is is",
        )
        .with_severity(Severity::Error)
        .with_replacement("is"),
        Check::new(
            "lexical_illusions.are_are",
            "Repeated word 'are are' - likely a typo.",
            r"are are",
        )
        .with_severity(Severity::Error)
        .with_replacement("are"),
        Check::new(
            "lexical_illusions.was_was",
            "Repeated word 'was was' - likely a typo.",
            r"was was",
        )
        .with_severity(Severity::Error)
        .with_replacement("was"),
        Check::new(
            "lexical_illusions.were_were",
            "Repeated word 'were were' - likely a typo.",
            r"were were",
        )
        .with_severity(Severity::Error)
        .with_replacement("were"),
        Check::new(
            "lexical_illusions.to_to",
            "Repeated word 'to to' - likely a typo.",
            r"to to",
        )
        .with_severity(Severity::Error)
        .with_replacement("to"),
        Check::new(
            "lexical_illusions.of_of",
            "Repeated word 'of of' - likely a typo.",
            r"of of",
        )
        .with_severity(Severity::Error)
        .with_replacement("of"),
        Check::new(
            "lexical_illusions.and_and",
            "Repeated word 'and and' - likely a typo.",
            r"and and",
        )
        .with_severity(Severity::Error)
        .with_replacement("and"),
        Check::new(
            "lexical_illusions.or_or",
            "Repeated word 'or or' - likely a typo.",
            r"or or",
        )
        .with_severity(Severity::Error)
        .with_replacement("or"),
        Check::new(
            "lexical_illusions.in_in",
            "Repeated word 'in in' - likely a typo.",
            r"in in",
        )
        .with_severity(Severity::Error)
        .with_replacement("in"),
        Check::new(
            "lexical_illusions.on_on",
            "Repeated word 'on on' - likely a typo.",
            r"on on",
        )
        .with_severity(Severity::Error)
        .with_replacement("on"),
        Check::new(
            "lexical_illusions.it_it",
            "Repeated word 'it it' - likely a typo.",
            r"it it",
        )
        .with_severity(Severity::Error)
        .with_replacement("it"),
        Check::new(
            "lexical_illusions.for_for",
            "Repeated word 'for for' - likely a typo.",
            r"for for",
        )
        .with_severity(Severity::Error)
        .with_replacement("for"),
        Check::new(
            "lexical_illusions.be_be",
            "Repeated word 'be be' - likely a typo.",
            r"be be",
        )
        .with_severity(Severity::Error)
        .with_replacement("be"),
        Check::new(
            "lexical_illusions.been_been",
            "Repeated word 'been been' - likely a typo.",
            r"been been",
        )
        .with_severity(Severity::Error)
        .with_replacement("been"),
        Check::new(
            "lexical_illusions.have_have",
            "Repeated word 'have have' - likely a typo.",
            r"have have",
        )
        .with_severity(Severity::Error)
        .with_replacement("have"),
        Check::new(
            "lexical_illusions.has_has",
            "Repeated word 'has has' - likely a typo.",
            r"has has",
        )
        .with_severity(Severity::Error)
        .with_replacement("has"),
        Check::new(
            "lexical_illusions.will_will",
            "Repeated word 'will will' - likely a typo.",
            r"will will",
        )
        .with_severity(Severity::Error)
        .with_replacement("will"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_illusion() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "lexical_illusions.the_the")
            .unwrap();
        let results = check.run("I went to the the store.");
        assert_eq!(results.len(), 1);
    }
}
