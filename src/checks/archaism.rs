//! Archaism checks for proselint-wasm
//!
//! Detects archaic and outdated words/phrases.

use crate::check::Check;

/// Get all archaism checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Single archaic words (all 46 from proselint)
        Check::new("archaism.alack", "'alack' is archaic.", r"\balack\b").raw(),
        Check::new("archaism.anent", "'anent' is archaic.", r"\banent\b").raw(),
        Check::new("archaism.begat", "'begat' is archaic.", r"\bbegat\b").raw(),
        Check::new("archaism.belike", "'belike' is archaic.", r"\bbelike\b").raw(),
        Check::new("archaism.betimes", "'betimes' is archaic.", r"\bbetimes\b").raw(),
        Check::new(
            "archaism.boughten",
            "'boughten' is archaic.",
            r"\bboughten\b",
        )
        .raw(),
        Check::new("archaism.brocage", "'brocage' is archaic.", r"\bbrocage\b").raw(),
        Check::new("archaism.brokage", "'brokage' is archaic.", r"\bbrokage\b").raw(),
        Check::new(
            "archaism.camarade",
            "'camarade' is archaic.",
            r"\bcamarade\b",
        )
        .raw(),
        Check::new("archaism.chiefer", "'chiefer' is archaic.", r"\bchiefer\b").raw(),
        Check::new(
            "archaism.chiefest",
            "'chiefest' is archaic.",
            r"\bchiefest\b",
        )
        .raw(),
        Check::new(
            "archaism.christiana",
            "'Christiana' is archaic.",
            r"\bChristiana\b",
        )
        .raw(),
        Check::new(
            "archaism.completely_obsolescent",
            "'completely obsolescent' is archaic.",
            r"completely obsolescent",
        ),
        Check::new("archaism.cozen", "'cozen' is archaic.", r"\bcozen\b").raw(),
        Check::new(
            "archaism.divers",
            "'divers' is archaic. Use 'diverse' or 'various'.",
            r"\bdivers\b",
        )
        .raw(),
        Check::new(
            "archaism.deflexion",
            "'deflexion' is archaic. Use 'deflection'.",
            r"\bdeflexion\b",
        )
        .raw(),
        Check::new("archaism.durst", "'durst' is archaic.", r"\bdurst\b").raw(),
        Check::new("archaism.fain", "'fain' is archaic.", r"\bfain\b").raw(),
        Check::new(
            "archaism.forsooth",
            "'forsooth' is archaic.",
            r"\bforsooth\b",
        )
        .raw(),
        Check::new(
            "archaism.foreclose_from",
            "'foreclose from' is archaic. Use 'foreclose on'.",
            r"foreclose from",
        ),
        Check::new(
            "archaism.forewent",
            "'forewent' is archaic.",
            r"\bforewent\b",
        )
        .raw(),
        Check::new("archaism.haply", "'haply' is archaic.", r"\bhaply\b").raw(),
        Check::new("archaism.howbeit", "'howbeit' is archaic.", r"\bhowbeit\b").raw(),
        Check::new(
            "archaism.illumine",
            "'illumine' is archaic. Use 'illuminate'.",
            r"\billumine\b",
        )
        .raw(),
        Check::new("archaism.maugre", "'maugre' is archaic.", r"\bmaugre\b").raw(),
        Check::new("archaism.meseems", "'meseems' is archaic.", r"\bmeseems\b").raw(),
        Check::new(
            "archaism.methinks",
            "'methinks' is archaic.",
            r"\bmethinks\b",
        )
        .raw(),
        Check::new(
            "archaism.nigh",
            "'nigh' is archaic. Use 'near' or 'nearly'.",
            r"\bnigh\b",
        )
        .raw(),
        Check::new(
            "archaism.peradventure",
            "'peradventure' is archaic.",
            r"\bperadventure\b",
        )
        .raw(),
        Check::new(
            "archaism.perchance",
            "'perchance' is archaic. Use 'perhaps'.",
            r"\bperchance\b",
        )
        .raw(),
        Check::new("archaism.saith", "'saith' is archaic.", r"\bsaith\b").raw(),
        Check::new(
            "archaism.shew",
            "'shew' is archaic. Use 'show'.",
            r"\bshew\b",
        )
        .raw(),
        Check::new("archaism.sistren", "'sistren' is archaic.", r"\bsistren\b").raw(),
        Check::new("archaism.spake", "'spake' is archaic.", r"\bspake\b").raw(),
        Check::new("archaism.verily", "'verily' is archaic.", r"\bverily\b").raw(),
        Check::new("archaism.whilom", "'whilom' is archaic.", r"\bwhilom\b").raw(),
        Check::new("archaism.withal", "'withal' is archaic.", r"\bwithal\b").raw(),
        Check::new("archaism.wot", "'wot' is archaic.", r"\bwot\b").raw(),
        Check::new(
            "archaism.inforce",
            "'inforce' is archaic. Use 'enforce'.",
            r"\binforce\b",
        )
        .raw(),
        // Archaic phrases
        Check::new(
            "archaism.enclosed_please_find",
            "'Enclosed please find' is archaic business jargon.",
            r"enclosed please find",
        ),
        Check::new(
            "archaism.please_find_enclosed",
            "'Please find enclosed' is archaic business jargon.",
            r"please find enclosed",
        ),
        Check::new(
            "archaism.enclosed_herewith",
            "'Enclosed herewith' is archaic business jargon.",
            r"enclosed herewith",
        ),
        Check::new(
            "archaism.enclosed_herein",
            "'Enclosed herein' is archaic business jargon.",
            r"enclosed herein",
        ),
        Check::new("archaism.in_sooth", "'In sooth' is archaic.", r"in sooth"),
        Check::new(
            "archaism.to_wit",
            "'To wit' is archaic. Use 'namely' or 'that is'.",
            r"to wit",
        ),
        Check::new(
            "archaism.ex_postfacto",
            "'Ex postfacto' should be 'ex post facto'.",
            r"ex postfacto",
        )
        .with_replacement("ex post facto"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archaism_check() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "archaism.forsooth").unwrap();
        let results = check.run("Forsooth, this is true!");
        assert_eq!(results.len(), 1);
    }
}
