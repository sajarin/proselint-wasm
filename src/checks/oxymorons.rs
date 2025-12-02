//! Oxymoron checks for proselint-wasm
//!
//! Detects contradictory word combinations.

use crate::check::{Check, Severity};

/// Get all oxymoron checks
pub fn get_checks() -> Vec<Check> {
    vec![
        Check::new(
            "oxymorons.amateur_expert",
            "'amateur expert' is an oxymoron.",
            r"amateur expert",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.increasingly_less",
            "'increasingly less' is an oxymoron.",
            r"increasingly less",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.advancing_backwards",
            "'advancing backwards' is an oxymoron.",
            r"advancing backwards",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.alludes_explicitly",
            "'alludes explicitly' is an oxymoron. Allusions are implicit by definition.",
            r"alludes explicitly",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.explicitly_alludes",
            "'explicitly alludes' is an oxymoron. Allusions are implicit by definition.",
            r"explicitly alludes",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.totally_obsolescent",
            "'totally obsolescent' is an oxymoron.",
            r"totally obsolescent",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.completely_obsolescent",
            "'completely obsolescent' is an oxymoron.",
            r"completely obsolescent",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.generally_always",
            "'generally always' is an oxymoron.",
            r"generally always",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.usually_always",
            "'usually always' is an oxymoron.",
            r"usually always",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.build_down",
            "'build down' is an oxymoron.",
            r"build down",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.conspicuous_absence",
            "'conspicuous absence' is an oxymoron.",
            r"conspicuous absence",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.exact_estimate",
            "'exact estimate' is an oxymoron. Estimates are approximate by definition.",
            r"exact estimate",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.found_missing",
            "'found missing' is an oxymoron.",
            r"found missing",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.intense_apathy",
            "'intense apathy' is an oxymoron.",
            r"intense apathy",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.mandatory_choice",
            "'mandatory choice' is an oxymoron.",
            r"mandatory choice",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.organized_mess",
            "'organized mess' is an oxymoron.",
            r"organized mess",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.act_naturally",
            "'act naturally' is an oxymoron.",
            r"act naturally",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.clearly_confused",
            "'clearly confused' is an oxymoron.",
            r"clearly confused",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.deafening_silence",
            "'deafening silence' is an oxymoron (though commonly used for effect).",
            r"deafening silence",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.only_choice",
            "'only choice' is an oxymoron - if there's only one option, there's no choice.",
            r"only choice",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.open_secret",
            "'open secret' is an oxymoron (though commonly used).",
            r"open secret",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.original_copy",
            "'original copy' is an oxymoron.",
            r"original copy",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "oxymorons.same_difference",
            "'same difference' is an oxymoron.",
            r"same difference",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "oxymorons.virtual_reality",
            "'virtual reality' is technically an oxymoron.",
            r"virtual reality",
        )
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oxymoron() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "oxymorons.exact_estimate")
            .unwrap();
        let results = check.run("Give me an exact estimate.");
        assert_eq!(results.len(), 1);
    }
}
