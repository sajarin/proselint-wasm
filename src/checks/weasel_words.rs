//! Weasel words checks for proselint-wasm
//!
//! Detects vague or weak language that undermines writing.

use crate::check::{Check, Severity};

/// Get all weasel words checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // "Very" - classic weasel word
        Check::new(
            "weasel_words.very",
            "'Very' is a weak word. Consider using a stronger adjective or removing it entirely.",
            r"very",
        )
        .with_severity(Severity::Warning),

        // "Really"
        Check::new(
            "weasel_words.really",
            "'Really' is often unnecessary. Consider removing it or using more precise language.",
            r"really",
        )
        .with_severity(Severity::Warning),

        // "Quite"
        Check::new(
            "weasel_words.quite",
            "'Quite' is vague. Consider being more specific.",
            r"quite",
        )
        .with_severity(Severity::Suggestion),

        // "Rather"
        Check::new(
            "weasel_words.rather",
            "'Rather' can be vague. Consider being more specific.",
            r"rather",
        )
        .with_severity(Severity::Suggestion),

        // "Somewhat"
        Check::new(
            "weasel_words.somewhat",
            "'Somewhat' is imprecise. Consider being more specific.",
            r"somewhat",
        )
        .with_severity(Severity::Suggestion),

        // "Extremely"
        Check::new(
            "weasel_words.extremely",
            "'Extremely' is often overused. Consider using more precise language.",
            r"extremely",
        )
        .with_severity(Severity::Warning),

        // "Basically"
        Check::new(
            "weasel_words.basically",
            "'Basically' is often filler. Consider removing it.",
            r"basically",
        )
        .with_severity(Severity::Warning),

        // "Actually"
        Check::new(
            "weasel_words.actually",
            "'Actually' is often unnecessary. Consider removing it.",
            r"actually",
        )
        .with_severity(Severity::Suggestion),

        // "Literally" (often misused)
        Check::new(
            "weasel_words.literally",
            "'Literally' is often misused as an intensifier. Use only for factual statements.",
            r"literally",
        )
        .with_severity(Severity::Warning),

        // "Virtually"
        Check::new(
            "weasel_words.virtually",
            "'Virtually' can be vague. Consider being more precise.",
            r"virtually",
        )
        .with_severity(Severity::Suggestion),

        // "Probably"
        Check::new(
            "weasel_words.probably",
            "'Probably' introduces uncertainty. Be more definite if possible.",
            r"probably",
        )
        .with_severity(Severity::Suggestion),

        // "Perhaps"
        Check::new(
            "weasel_words.perhaps",
            "'Perhaps' introduces uncertainty. Be more definite if possible.",
            r"perhaps",
        )
        .with_severity(Severity::Suggestion),

        // "Maybe"
        Check::new(
            "weasel_words.maybe",
            "'Maybe' introduces uncertainty. Be more definite if possible.",
            r"maybe",
        )
        .with_severity(Severity::Suggestion),

        // "Just" (minimizing)
        Check::new(
            "weasel_words.just",
            "'Just' can minimize your statement. Consider removing it.",
            r"just",
        )
        .with_severity(Severity::Suggestion),

        // "Simply"
        Check::new(
            "weasel_words.simply",
            "'Simply' is often unnecessary. Consider removing it.",
            r"simply",
        )
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_very_check() {
        let checks = get_checks();
        let very_check = checks.iter().find(|c| c.id == "weasel_words.very").unwrap();

        // Should match "very"
        let results = very_check.run("This is very good.");
        assert_eq!(results.len(), 1);

        // Should also match "very" in "very well"
        let results = very_check.run("He did very well.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_literally_check() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "weasel_words.literally").unwrap();

        let results = check.run("I literally died laughing.");
        assert_eq!(results.len(), 1);
    }
}
