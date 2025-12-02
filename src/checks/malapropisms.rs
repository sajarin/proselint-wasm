//! Malapropism checks for proselint-wasm
//!
//! Detects commonly misused words (wrong word that sounds similar).

use crate::check::{Check, Severity};

/// Get all malapropism checks
pub fn get_checks() -> Vec<Check> {
    vec![
        Check::new(
            "malapropisms.baited_breath",
            "'baited breath' should be 'bated breath'.",
            r"baited breath",
        )
        .with_severity(Severity::Error)
        .with_replacement("bated breath"),
        Check::new(
            "malapropisms.beckon_call",
            "'beckon call' should be 'beck and call'.",
            r"beckon call",
        )
        .with_severity(Severity::Error)
        .with_replacement("beck and call"),
        Check::new(
            "malapropisms.case_and_point",
            "'case and point' should be 'case in point'.",
            r"case and point",
        )
        .with_severity(Severity::Error)
        .with_replacement("case in point"),
        Check::new(
            "malapropisms.chomping_at_bit",
            "'chomping at the bit' should be 'champing at the bit'.",
            r"chomping at the bit",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("champing at the bit"),
        Check::new(
            "malapropisms.deep_seeded",
            "'deep seeded' should be 'deep-seated'.",
            r"deep seeded",
        )
        .with_severity(Severity::Error)
        .with_replacement("deep-seated"),
        Check::new(
            "malapropisms.escape_goat",
            "'escape goat' should be 'scapegoat'.",
            r"escape goat",
        )
        .with_severity(Severity::Error)
        .with_replacement("scapegoat"),
        Check::new(
            "malapropisms.extract_revenge",
            "'extract revenge' should be 'exact revenge'.",
            r"extract(?:s|ed|ing)? revenge",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("exact revenge"),
        Check::new(
            "malapropisms.for_all_intensive_purposes",
            "'for all intensive purposes' should be 'for all intents and purposes'.",
            r"for all intensive purposes",
        )
        .with_severity(Severity::Error)
        .with_replacement("for all intents and purposes"),
        Check::new(
            "malapropisms.free_reign",
            "'free reign' should be 'free rein'.",
            r"free reign",
        )
        .with_severity(Severity::Error)
        .with_replacement("free rein"),
        Check::new(
            "malapropisms.hone_in",
            "'hone in' should be 'home in'.",
            r"hone(?:s|d|ing)? in on",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("home in on"),
        Check::new(
            "malapropisms.hunger_pains",
            "'hunger pains' should be 'hunger pangs'.",
            r"hunger pains",
        )
        .with_severity(Severity::Error)
        .with_replacement("hunger pangs"),
        Check::new(
            "malapropisms.make_due",
            "'make due' should be 'make do'.",
            r"make due",
        )
        .with_severity(Severity::Error)
        .with_replacement("make do"),
        Check::new(
            "malapropisms.mute_point",
            "'mute point' should be 'moot point'.",
            r"mute point",
        )
        .with_severity(Severity::Error)
        .with_replacement("moot point"),
        Check::new(
            "malapropisms.nip_it_in_the_butt",
            "'nip it in the butt' should be 'nip it in the bud'.",
            r"nip(?:ped|ping)? (?:it )?in the butt",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("nip it in the bud"),
        Check::new(
            "malapropisms.on_accident",
            "'on accident' should be 'by accident'.",
            r"on accident",
        )
        .with_severity(Severity::Warning)
        .with_replacement("by accident"),
        Check::new(
            "malapropisms.peaked_interest",
            "'peaked my interest' should be 'piqued my interest'.",
            r"peak(?:ed|s)? (?:my|your|his|her|their|our) interest",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("piqued interest"),
        Check::new(
            "malapropisms.phase_faze",
            "'didn't phase' should be 'didn't faze'.",
            r"(?:didn't|doesn't|won't|don't) phase",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("faze"),
        Check::new(
            "malapropisms.shoe_in",
            "'shoe in' should be 'shoo-in'.",
            r"shoe[- ]in",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("shoo-in"),
        Check::new(
            "malapropisms.sneak_peak",
            "'sneak peak' should be 'sneak peek'.",
            r"sneak peak",
        )
        .with_severity(Severity::Error)
        .with_replacement("sneak peek"),
        Check::new(
            "malapropisms.supposably",
            "'supposably' should be 'supposedly'.",
            r"supposably",
        )
        .with_severity(Severity::Error)
        .with_replacement("supposedly"),
        Check::new(
            "malapropisms.tow_the_line",
            "'tow the line' should be 'toe the line'.",
            r"tow(?:s|ed|ing)? the line",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("toe the line"),
        Check::new(
            "malapropisms.wet_your_appetite",
            "'wet your appetite' should be 'whet your appetite'.",
            r"wet(?:s|ted|ting)? (?:your|my|his|her|their|our|the) appetite",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("whet appetite"),
        Check::new(
            "malapropisms.wreck_havoc",
            "'wreck havoc' should be 'wreak havoc'.",
            r"wreck(?:s|ed|ing)? havoc",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("wreak havoc"),
        Check::new(
            "malapropisms.per_say",
            "'per say' should be 'per se'.",
            r"per say",
        )
        .with_severity(Severity::Error)
        .with_replacement("per se"),
        Check::new(
            "malapropisms.expresso",
            "'expresso' should be 'espresso'.",
            r"expresso",
        )
        .with_severity(Severity::Error)
        .with_replacement("espresso"),
        Check::new(
            "malapropisms.old_timer",
            "'old timer's disease' should be 'Alzheimer's disease'.",
            r"old timer'?s disease",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("Alzheimer's disease"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_malapropism() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "malapropisms.for_all_intensive_purposes")
            .unwrap();
        let results = check.run("For all intensive purposes, we're done.");
        assert_eq!(results.len(), 1);
    }
}
