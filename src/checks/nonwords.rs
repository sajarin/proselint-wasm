//! Nonword checks for proselint-wasm
//!
//! Detects invented or improperly formed words.

use crate::check::{Check, Severity};

/// Get all nonword checks
pub fn get_checks() -> Vec<Check> {
    vec![
        Check::new(
            "nonwords.affrontery",
            "'affrontery' is not a word. Use 'effrontery'.",
            r"affrontery",
        )
        .with_severity(Severity::Error)
        .with_replacement("effrontery"),

        Check::new(
            "nonwords.analyzation",
            "'analyzation' is not a word. Use 'analysis'.",
            r"analyzation",
        )
        .with_severity(Severity::Error)
        .with_replacement("analysis"),

        Check::new(
            "nonwords.annoyment",
            "'annoyment' is not a word. Use 'annoyance'.",
            r"annoyment",
        )
        .with_severity(Severity::Error)
        .with_replacement("annoyance"),

        Check::new(
            "nonwords.confirmant",
            "'confirmant' is not a word. Use 'confirmand'.",
            r"confirmants?",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("confirmand"),

        Check::new(
            "nonwords.conversate",
            "'conversate' is not a word. Use 'converse'.",
            r"conversate",
        )
        .with_severity(Severity::Error)
        .with_replacement("converse"),

        Check::new(
            "nonwords.discomforture",
            "'discomforture' is not a word. Use 'discomfort' or 'discomfiture'.",
            r"discomforture",
        )
        .with_severity(Severity::Error)
        .with_replacement("discomfort"),

        Check::new(
            "nonwords.dispersement",
            "'dispersement' is not a word. Use 'disbursement' or 'dispersal'.",
            r"dispersement",
        )
        .with_severity(Severity::Error)
        .with_replacement("dispersal"),

        Check::new(
            "nonwords.doubtlessly",
            "'doubtlessly' is not a word. Use 'doubtless' or 'undoubtedly'.",
            r"doubtlessly",
        )
        .with_severity(Severity::Error)
        .with_replacement("undoubtedly"),

        Check::new(
            "nonwords.forebearance",
            "'forebearance' is not a word. Use 'forbearance'.",
            r"forebearance",
        )
        .with_severity(Severity::Error)
        .with_replacement("forbearance"),

        Check::new(
            "nonwords.improprietous",
            "'improprietous' is not a word. Use 'improper'.",
            r"improprietous",
        )
        .with_severity(Severity::Error)
        .with_replacement("improper"),

        Check::new(
            "nonwords.inclimate",
            "'inclimate' is not a word. Use 'inclement'.",
            r"inclimate",
        )
        .with_severity(Severity::Error)
        .with_replacement("inclement"),

        Check::new(
            "nonwords.inimicable",
            "'inimicable' is not a word. Use 'inimical'.",
            r"inimicable",
        )
        .with_severity(Severity::Error)
        .with_replacement("inimical"),

        Check::new(
            "nonwords.irregardless",
            "'irregardless' is not a word. Use 'regardless'.",
            r"irregardless",
        )
        .with_severity(Severity::Error)
        .with_replacement("regardless"),

        Check::new(
            "nonwords.minimalize",
            "'minimalize' is not a word. Use 'minimize'.",
            r"minimaliz(?:e|es|ed|ing)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("minimize"),

        Check::new(
            "nonwords.optimalize",
            "'optimalize' is not a word. Use 'optimize'.",
            r"optimalize",
        )
        .with_severity(Severity::Error)
        .with_replacement("optimize"),

        Check::new(
            "nonwords.paralyzation",
            "'paralyzation' is not a word. Use 'paralysis'.",
            r"paralyzation",
        )
        .with_severity(Severity::Error)
        .with_replacement("paralysis"),

        Check::new(
            "nonwords.proprietous",
            "'proprietous' is not a word. Use 'proper'.",
            r"proprietous",
        )
        .with_severity(Severity::Error)
        .with_replacement("proper"),

        Check::new(
            "nonwords.seldomly",
            "'seldomly' is not a word. Use 'seldom'.",
            r"seldomly",
        )
        .with_severity(Severity::Error)
        .with_replacement("seldom"),

        Check::new(
            "nonwords.thusly",
            "'thusly' is not a word. Use 'thus'.",
            r"thusly",
        )
        .with_severity(Severity::Error)
        .with_replacement("thus"),

        Check::new(
            "nonwords.uncategorically",
            "'uncategorically' is not a word. Use 'categorically'.",
            r"uncategorically",
        )
        .with_severity(Severity::Error)
        .with_replacement("categorically"),

        Check::new(
            "nonwords.undoubtably",
            "'undoubtably' is not a word. Use 'undoubtedly'.",
            r"undoubtably",
        )
        .with_severity(Severity::Error)
        .with_replacement("undoubtedly"),

        Check::new(
            "nonwords.unequivocable",
            "'unequivocable' is not a word. Use 'unequivocal'.",
            r"unequivocable",
        )
        .with_severity(Severity::Error)
        .with_replacement("unequivocal"),

        Check::new(
            "nonwords.unmercilessly",
            "'unmercilessly' is not a word. Use 'mercilessly'.",
            r"unmercilessly",
        )
        .with_severity(Severity::Error)
        .with_replacement("mercilessly"),

        Check::new(
            "nonwords.unrelentlessly",
            "'unrelentlessly' is not a word. Use 'unrelentingly' or 'relentlessly'.",
            r"unrelentlessly",
        )
        .with_severity(Severity::Error)
        .with_replacement("relentlessly"),

        Check::new(
            "nonwords.orientate",
            "'orientate' is nonstandard. Use 'orient'.",
            r"orientate",
        )
        .with_severity(Severity::Warning)
        .with_replacement("orient"),

        Check::new(
            "nonwords.administrate",
            "'administrate' is nonstandard. Use 'administer'.",
            r"administrate",
        )
        .with_severity(Severity::Warning)
        .with_replacement("administer"),

        Check::new(
            "nonwords.commentate",
            "'commentate' is nonstandard. Use 'comment'.",
            r"commentate",
        )
        .with_severity(Severity::Warning)
        .with_replacement("comment"),

        // Additional nonwords from proselint
        Check::new(
            "nonwords.crained",
            "'crained' is not a word. Use 'craned'.",
            r"crained",
        )
        .with_severity(Severity::Error)
        .with_replacement("craned"),

        Check::new(
            "nonwords.pettifogger",
            "'pettifogger' is not standard. Use 'pettifog'.",
            r"pettifogger",
        )
        .with_severity(Severity::Warning)
        .with_replacement("pettifog"),

        Check::new(
            "nonwords.relative_inexpense",
            "'relative inexpense' is not a phrase. Use 'relatively low price' or 'affordability'.",
            r"relative inexpense",
        )
        .with_severity(Severity::Error)
        .with_replacement("affordability"),

        Check::new(
            "nonwords.squelch",
            "'squelch' may not be the word you want. Consider 'quell' or 'quash'.",
            r"\bsquelch\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irregardless() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "nonwords.irregardless").unwrap();
        let results = check.run("Irregardless of the outcome...");
        assert_eq!(results.len(), 1);
    }
}
