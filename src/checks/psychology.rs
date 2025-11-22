//! Psychology checks for proselint-wasm
//!
//! Detects misuse of psychological and clinical terms.

use crate::check::{Check, Severity};

/// Get all psychology checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Commonly misused psychological terms
        Check::new(
            "psychology.ocd",
            "'OCD' should not be used casually. It's a serious mental health condition.",
            r"(?:I'm|I am|he's|she's|they're|we're) (?:so |a little |kind of )?OCD",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.bipolar",
            "'bipolar' should not be used to mean 'changeable'. It's a serious mental health condition.",
            r"(?:so |being |acting )?bipolar(?:\s+about)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.schizophrenic",
            "'schizophrenic' should not be used to mean 'contradictory'. It's a serious mental health condition.",
            r"(?:acting |being |sounds? )?schizophrenic",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.psychotic",
            "'psychotic' should not be used casually. It refers to a serious mental state.",
            r"(?:going |being |acting |sounds? )?psychotic(?:\s+about)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.depressed_sad",
            "'depressed' should not be used casually for 'sad'. Depression is a clinical condition.",
            r"(?:I'm|I am|feeling) (?:so |kind of )?depressed(?:\s+(?:about|that|because))",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.ptsd_casual",
            "'PTSD' should not be used casually. It's a serious trauma-related condition.",
            r"(?:gave|gives|giving) (?:me|him|her|them|us) PTSD",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.adhd_casual",
            "'ADHD' should not be used casually to mean 'distracted'.",
            r"(?:I'm|I am|being|acting) (?:so |a little )?ADHD",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.triggered",
            "'triggered' has clinical meaning related to trauma. Consider 'upset' or 'bothered' for casual use.",
            r"(?:I'm|I am|he's|she's|they're|getting) (?:so )?triggered",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.gaslighting",
            "'gaslighting' is a specific form of psychological manipulation. Ensure you're using it correctly.",
            r"gaslight(?:ing|ed|s)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.narcissist",
            "'narcissist' refers to a clinical personality disorder. Consider 'self-centered' or 'egotistical' for casual use.",
            r"(?:such a |total |complete )?narcissist",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.sociopath",
            "'sociopath' is a clinical term. Avoid casual use.",
            r"(?:such a |total |complete )?sociopath",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.psychopath",
            "'psychopath' is a clinical term. Avoid casual use.",
            r"(?:such a |total |complete )?psychopath",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.anorexic",
            "'anorexic' should not be used to mean 'thin'. Anorexia is a serious eating disorder.",
            r"(?:looks?|looking) (?:so )?anorexic",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.autistic",
            "'autistic' should not be used as an insult or casual descriptor.",
            r"(?:so |being |acting )?autistic(?:\s+(?:about|when))",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.mental",
            "'mental' used as an insult is inappropriate. Consider 'wild' or 'unbelievable'.",
            r"(?:that's|it's|you're|he's|she's|they're) (?:so |absolutely )?mental",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "psychology.insane",
            "'insane' used casually may be insensitive. Consider 'incredible', 'extreme', or 'wild'.",
            r"(?:that's|it's|this is) (?:so |absolutely |totally )?insane",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.crazy",
            "'crazy' used casually may be insensitive. Consider 'wild', 'incredible', or 'unbelievable'.",
            r"(?:that's|it's|you're|he's|she's|they're) (?:so |absolutely )?crazy",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.manic",
            "'manic' refers to a clinical state in bipolar disorder. Consider 'energetic' or 'frenzied' for casual use.",
            r"(?:being |acting |seems? )?manic(?:\s+(?:about|energy))",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.paranoid",
            "'paranoid' has clinical meaning. Consider 'suspicious' or 'worried' for casual use.",
            r"(?:I'm|I am|being|acting|getting) (?:so )?paranoid",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "psychology.phobia",
            "'-phobia' suffix has clinical meaning. Ensure you're not trivializing anxiety disorders.",
            r"(?:my|his|her|their) (?:\w+)?phobia of",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocd_casual() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "psychology.ocd").unwrap();
        let results = check.run("I'm so OCD about keeping things clean.");
        assert_eq!(results.len(), 1);
    }
}
