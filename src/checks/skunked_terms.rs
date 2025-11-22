//! Skunked terms checks for proselint-wasm
//!
//! Detects words that are impossible to use without issue.

use crate::check::{Check, Severity};

/// Get all skunked terms checks
pub fn get_checks() -> Vec<Check> {
    vec![
        Check::new(
            "skunked_terms.bona_fides",
            "'bona fides' is a skunked term - commonly misused. Consider rephrasing.",
            r"bona fides",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.deceptively",
            "'deceptively' is a skunked term - its meaning is ambiguous. Rephrase for clarity.",
            r"deceptively",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.decimate",
            "'decimate' is a skunked term - disputed usage. Consider 'devastate' or 'destroy'.",
            r"decimat(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.effete",
            "'effete' is a skunked term - commonly misunderstood. Consider rephrasing.",
            r"effete",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.fulsome",
            "'fulsome' is a skunked term - means excessive, not full. Avoid or rephrase.",
            r"fulsome",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.hopefully",
            "'Hopefully' at sentence start is disputed. Consider 'I hope' or 'It is hoped'.",
            r"^Hopefully,",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.impassionate",
            "'impassionate' is a skunked term - ambiguous. Use 'impassioned' or 'dispassionate'.",
            r"impassionate",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.thankfully",
            "'Thankfully' at sentence start is disputed. Consider 'I am thankful that'.",
            r"^Thankfully,",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.literally",
            "'literally' is often misused as an intensifier. Use only for factual statements.",
            r"literally (?:died|exploded|killed|destroyed)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "skunked_terms.bemused",
            "'bemused' means confused, not amused. Verify your intended meaning.",
            r"bemused",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.enormity",
            "'enormity' means wickedness, not enormousness. Verify your intended meaning.",
            r"enormity",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.nonplussed",
            "'nonplussed' means bewildered, not unperturbed. Verify your intended meaning.",
            r"nonplussed",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "skunked_terms.peruse",
            "'peruse' means to read carefully, not skim. Verify your intended meaning.",
            r"perus(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skunked_term() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "skunked_terms.fulsome").unwrap();
        let results = check.run("The fulsome praise was unexpected.");
        assert_eq!(results.len(), 1);
    }
}
