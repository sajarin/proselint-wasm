//! Typography checks for proselint-wasm
//!
//! Based on Butterick's Practical Typography recommendations.
//! Includes diacritical mark patterns from proselint.

use crate::check::{Check, Severity};

/// Get all typography checks
pub fn get_checks() -> Vec<Check> {
    let mut checks = vec![
        // Ellipsis check - use proper ellipsis character
        Check::new(
            "typography.symbols.ellipsis",
            "Use the ellipsis character (...) instead of three periods.",
            r"\.\.\.",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{2026}"),
        // Copyright symbol
        Check::new(
            "typography.symbols.copyright",
            "Use the copyright symbol (c) instead of '(c)'.",
            r"\(c\)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{00A9}"),
        // Trademark symbol
        Check::new(
            "typography.symbols.trademark",
            "Use the trademark symbol instead of '(tm)'.",
            r"\(tm\)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{2122}"),
        // Registered trademark
        Check::new(
            "typography.symbols.registered",
            "Use the registered trademark symbol instead of '(r)'.",
            r"\(r\)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{00AE}"),
        // Multiple spaces after period (sentence spacing)
        Check::new(
            "typography.symbols.sentence_spacing",
            "Use a single space after a period, not multiple spaces.",
            r"\. {3,}",
        )
        .raw()
        .with_severity(Severity::Warning),
        // Multiplication symbol
        Check::new(
            "typography.symbols.multiplication",
            "Use the multiplication sign (\u{00D7}) instead of 'x' for dimensions.",
            r"\d+\s*x\s*\d+",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Curly quotes recommendation (straight double quotes)
        Check::new(
            "typography.symbols.curly_quotes",
            "Consider using curly quotes instead of straight quotes for prose.",
            r#"(?<!\w)"(?!\s)"#,
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .allow_in_quotes(),
        // En dash for ranges
        Check::new(
            "typography.dashes.en_dash_range",
            "Use an en dash (\u{2013}) for ranges instead of a hyphen.",
            r"\d+-\d+",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Em dash usage (double hyphens)
        Check::new(
            "typography.dashes.em_dash",
            "Use an em dash (\u{2014}) instead of double hyphens.",
            r"--",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{2014}"),
        // Apostrophe in contractions
        Check::new(
            "typography.symbols.apostrophe",
            "Consider using a curly apostrophe (\u{2019}) instead of a straight apostrophe.",
            r"\w'\w",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ];

    // Diacritical marks - words that should have accents/diacritics
    let diacritical_marks: &[(&str, &str)] = &[
        // French loanwords
        (r"\bcafe\b", "cafe\u{0301}"),                         // café
        (r"\bcliche\b", "cliche\u{0301}"),                     // cliché
        (r"\bresume\b", "re\u{0301}sume\u{0301}"),             // résumé (the document)
        (r"\bnaive\b", "nai\u{0308}ve"),                       // naïve
        (r"\bnaif\b", "nai\u{0308}f"),                         // naïf
        (r"\bsaute\b", "saute\u{0301}"),                       // sauté
        (r"\bsauteed\b", "saute\u{0301}ed"),                   // sautéed
        (r"\bflambe\b", "flambe\u{0301}"),                     // flambé
        (r"\boutré\b", "outre\u{0301}"),                       // outré
        (r"\bpuree\b", "pure\u{0301}e"),                       // purée
        (r"\bsouffle\b", "souffle\u{0301}"),                   // soufflé
        (r"\btouche\b", "touche\u{0301}"),                     // touché
        (r"\bfiancee?\b", "fiance\u{0301}(e)"),                // fiancé/fiancée
        (r"\bprotege\b", "prote\u{0301}ge\u{0301}"),           // protégé
        (r"\bblase\b", "blase\u{0301}"),                       // blasé
        (r"\bpasse\b", "passe\u{0301}"),                       // passé
        (r"\bentree\b", "entre\u{0301}e"),                     // entrée
        (r"\bmelee\b", "me\u{0302}le\u{0301}e"),               // mêlée
        (r"\bprecis\b", "pre\u{0301}cis"),                     // précis
        (r"\bdebris\b", "de\u{0301}bris"),                     // débris
        (r"\bcanape\b", "canape\u{0301}"),                     // canapé
        (r"\bcommunique\b", "communique\u{0301}"),             // communiqué
        (r"\bdecor\b", "de\u{0301}cor"),                       // décor
        (r"\bdecollete\b", "de\u{0301}collete\u{0301}"),       // décolleté
        (r"\bdeja vu\b", "de\u{0301}ja\u{0300} vu"),           // déjà vu
        (r"\beclair\b", "e\u{0301}clair"),                     // éclair
        (r"\belan\b", "e\u{0301}lan"),                         // élan
        (r"\belite\b", "e\u{0301}lite"),                       // élite
        (r"\bfete\b", "fe\u{0302}te"),                         // fête
        (r"\bingenue\b", "inge\u{0301}nue"),                   // ingénue
        (r"\bmatinee\b", "matine\u{0301}e"),                   // matinée
        (r"\bnee\b", "ne\u{0301}e"),                           // née
        (r"\bpapier-mache\b", "papier-ma\u{0302}che\u{0301}"), // papier-mâché
        (r"\bpique\b", "pique\u{0301}"),                       // piqué
        (r"\braison d'etre\b", "raison d'e\u{0302}tre"),       // raison d'être
        (r"\brole\b", "ro\u{0302}le"),                         // rôle
        (r"\bseance\b", "se\u{0301}ance"),                     // séance
        (r"\btete-a-tete\b", "te\u{0302}te-a\u{0300}-te\u{0302}te"), // tête-à-tête
        (r"\bvis-a-vis\b", "vis-a\u{0300}-vis"),               // vis-à-vis
        // German loanwords
        (r"\bdoppelganger\b", "doppelga\u{0308}nger"), // doppelgänger
        (r"\buber\b", "u\u{0308}ber"),                 // über
        (r"\bangstrom\b", "a\u{030A}ngstro\u{0308}m"), // ångström
        // Spanish loanwords
        (r"\bpinata\b", "pin\u{0303}ata"),     // piñata
        (r"\bsenor\b", "sen\u{0303}or"),       // señor
        (r"\bsenora\b", "sen\u{0303}ora"),     // señora
        (r"\bsenorita\b", "sen\u{0303}orita"), // señorita
        (r"\bmanana\b", "man\u{0303}ana"),     // mañana
        (r"\bhabanero\b", "habanero"),         // Note: correctly no accent
        (r"\bjalapeno\b", "jalapen\u{0303}o"), // jalapeño
        (r"\bEl Nino\b", "El Nin\u{0303}o"),   // El Niño
        (r"\bLa Nina\b", "La Nin\u{0303}a"),   // La Niña
        (r"\bcanon\b", "can\u{0303}on"),       // cañon (canyon in Spanish context)
        // Portuguese loanwords
        (r"\bfacade\b", "fac\u{0327}ade"), // façade
        // Other languages
        (r"\bsmorgasbord\b", "smo\u{0308}rga\u{030A}sbord"), // smörgåsbord
        (r"\bnaivete\b", "nai\u{0308}vete\u{0301}"),         // naïveté
        (r"\bcooperate\b", "coo\u{0308}perate"),             // coöperate (archaic)
        (r"\bcoup d'etat\b", "coup d'e\u{0301}tat"),         // coup d'état
        (r"\bcreme brulee\b", "cre\u{0300}me bru\u{0302}le\u{0301}e"), // crème brûlée
        (r"\bcreme fraiche\b", "cre\u{0300}me frai\u{0302}che"), // crème fraîche
        (
            r"\bpiece de resistance\b",
            "pie\u{0300}ce de re\u{0301}sistance",
        ), // pièce de résistance
        // Names with diacritics (commonly missed)
        (r"\bBrontë\b", "Bronte\u{0308}"), // Brontë
        (r"\bZoe\b", "Zoe\u{0308}"),       // Zoë
        (r"\bChloe\b", "Chloe\u{0308}"),   // Chloë
        (r"\bNoel\b", "Noe\u{0308}l"),     // Noël (Christmas)
        // Musical terms
        (r"\bporteno\b", "porten\u{0303}o"), // porteño
        // Food and drink
        (r"\bcreme\b", "cre\u{0300}me"),       // crème
        (r"\bpate\b", "pa\u{0302}te\u{0301}"), // pâté
        (r"\braclette\b", "raclette"),         // correct (no diacritics)
        (r"\bbeurre blanc\b", "beurre blanc"), // correct
    ];

    for (i, (pattern, correct)) in diacritical_marks.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("typography.diacritics.{}", i).into_boxed_str()),
                Box::leak(
                    format!("Consider using the proper diacritical mark: '{}'.", correct)
                        .into_boxed_str(),
                ),
                pattern,
            )
            .raw()
            .with_severity(Severity::Suggestion),
        );
    }

    // Exclamation point overuse
    checks.push(
        Check::new(
            "typography.exclamation.overuse",
            "Avoid using multiple exclamation points. One is sufficient.",
            r"!{2,}",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("!"),
    );

    // Mixed quotes
    checks.push(
        Check::new(
            "typography.quotes.mixed",
            "Inconsistent quote style detected. Stick to one style.",
            r#"["\u{201C}].*['\u{2019}]|['\u{2018}].*["\u{201D}]"#,
        )
        .raw()
        .with_severity(Severity::Suggestion),
    );

    // Spaces around em dash
    checks.push(
        Check::new(
            "typography.dashes.em_dash_spaces",
            "Em dashes typically don't have spaces around them.",
            r" \u{2014} ",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    );

    // Prime vs apostrophe for feet/inches
    checks.push(
        Check::new(
            "typography.symbols.prime_feet",
            "Use prime symbols (\u{2032}\u{2033}) for feet and inches, not quotes.",
            r#"\d+['\"]\s*\d+['\"]"#,
        )
        .raw()
        .with_severity(Severity::Suggestion),
    );

    // Degree symbol
    checks.push(
        Check::new(
            "typography.symbols.degree",
            "Use the degree symbol (\u{00B0}) instead of superscript 'o'.",
            r"\d+\s*degrees?",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    );

    // Number x number (should use multiplication sign)
    checks.push(
        Check::new(
            "typography.symbols.times",
            "Use the multiplication sign (\u{00D7}) for dimensions, not 'x'.",
            r"\d+\s*[xX]\s*\d+",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    );

    // Fractions
    checks.push(
        Check::new(
            "typography.symbols.fraction_half",
            "Consider using the fraction character \u{00BD} instead of 1/2.",
            r"\b1/2\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{00BD}"),
    );

    checks.push(
        Check::new(
            "typography.symbols.fraction_quarter",
            "Consider using the fraction character \u{00BC} instead of 1/4.",
            r"\b1/4\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{00BC}"),
    );

    checks.push(
        Check::new(
            "typography.symbols.fraction_three_quarters",
            "Consider using the fraction character \u{00BE} instead of 3/4.",
            r"\b3/4\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("\u{00BE}"),
    );

    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsis_check() {
        let checks = get_checks();
        let ellipsis_check = checks
            .iter()
            .find(|c| c.id == "typography.symbols.ellipsis")
            .unwrap();

        let results = ellipsis_check.run("Wait... what?");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_em_dash_check() {
        let checks = get_checks();
        let em_dash_check = checks
            .iter()
            .find(|c| c.id == "typography.dashes.em_dash")
            .unwrap();

        let results = em_dash_check.run("This is important--really important.");
        assert_eq!(results.len(), 1);
    }
}
