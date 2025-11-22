//! Restricted vocabulary checks for proselint-wasm
//!
//! Detects overly simple or potentially inappropriate word choices.

use crate::check::{Check, Severity};

/// Get all restricted vocabulary checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Overly casual/informal words
        Check::new(
            "restricted.gonna",
            "'gonna' is informal. Use 'going to'.",
            r"\bgonna\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("going to"),

        Check::new(
            "restricted.wanna",
            "'wanna' is informal. Use 'want to'.",
            r"\bwanna\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("want to"),

        Check::new(
            "restricted.gotta",
            "'gotta' is informal. Use 'got to' or 'have to'.",
            r"\bgotta\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("have to"),

        Check::new(
            "restricted.kinda",
            "'kinda' is informal. Use 'kind of'.",
            r"\bkinda\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("kind of"),

        Check::new(
            "restricted.sorta",
            "'sorta' is informal. Use 'sort of'.",
            r"\bsorta\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("sort of"),

        Check::new(
            "restricted.dunno",
            "'dunno' is informal. Use 'don't know'.",
            r"\bdunno\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("don't know"),

        Check::new(
            "restricted.lemme",
            "'lemme' is informal. Use 'let me'.",
            r"\blemme\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("let me"),

        Check::new(
            "restricted.gimme",
            "'gimme' is informal. Use 'give me'.",
            r"\bgimme\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("give me"),

        Check::new(
            "restricted.aint",
            "'ain't' is nonstandard. Use 'isn't', 'aren't', or 'haven't'.",
            r"\bain't\b",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "restricted.coulda",
            "'coulda' is informal. Use 'could have'.",
            r"\bcoulda\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("could have"),

        Check::new(
            "restricted.shoulda",
            "'shoulda' is informal. Use 'should have'.",
            r"\bshoulda\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("should have"),

        Check::new(
            "restricted.woulda",
            "'woulda' is informal. Use 'would have'.",
            r"\bwoulda\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("would have"),

        Check::new(
            "restricted.ya",
            "'ya' is informal. Use 'you'.",
            r"\bya\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("you"),

        Check::new(
            "restricted.yall",
            "'y'all' or 'yall' is regional/informal. Consider 'you all' or 'everyone'.",
            r"\by'?all\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "restricted.cuz",
            "'cuz' or 'cos' is informal. Use 'because'.",
            r"\b(?:cuz|cos)\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("because"),

        Check::new(
            "restricted.tho",
            "'tho' is informal. Use 'though'.",
            r"\btho\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("though"),

        Check::new(
            "restricted.thru",
            "'thru' is informal. Use 'through'.",
            r"\bthru\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("through"),

        Check::new(
            "restricted.nite",
            "'nite' is informal. Use 'night'.",
            r"\bnite\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("night"),

        Check::new(
            "restricted.lite",
            "'lite' is informal/commercial. Use 'light'.",
            r"\blite\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("light"),

        // Profanity and crude language markers
        Check::new(
            "restricted.expletive_damn",
            "Expletive 'damn' may be inappropriate in formal writing.",
            r"\bdamn(?:ed|it)?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "restricted.expletive_hell",
            "Expletive 'hell' may be inappropriate in formal writing.",
            r"(?:what the |the |go to |bloody )hell\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "restricted.crap",
            "'crap' may be inappropriate in formal writing.",
            r"\bcrap(?:py)?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "restricted.suck",
            "'sucks' or 'sucked' may be inappropriate in formal writing.",
            r"\bsuck(?:s|ed|ing)?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // Text speak
        Check::new(
            "restricted.lol",
            "'LOL' is text speak. Avoid in formal writing.",
            r"\bLOL\b",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "restricted.omg",
            "'OMG' is text speak. Avoid in formal writing.",
            r"\bOMG\b",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "restricted.btw",
            "'BTW' is text speak. Use 'by the way'.",
            r"\bBTW\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("by the way"),

        Check::new(
            "restricted.fyi",
            "'FYI' is informal. Consider 'for your information'.",
            r"\bFYI\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("for your information"),

        Check::new(
            "restricted.imho",
            "'IMHO' is text speak. Use 'in my opinion'.",
            r"\bIMHO\b",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("in my opinion"),

        Check::new(
            "restricted.asap",
            "'ASAP' is informal. Consider 'as soon as possible'.",
            r"\bASAP\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("as soon as possible"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gonna() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "restricted.gonna").unwrap();
        let results = check.run("I'm gonna be there soon.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_lol() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "restricted.lol").unwrap();
        let results = check.run("That was funny LOL");
        assert_eq!(results.len(), 1);
    }
}
