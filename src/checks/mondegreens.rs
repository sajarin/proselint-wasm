//! Mondegreen checks for proselint-wasm
//!
//! Detects misheard phrases and expressions.

use crate::check::{Check, Severity};

/// Get all mondegreen checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Classic mondegreens
        Check::new(
            "mondegreens.a_posit",
            "'a posit' should be 'a priori' or rephrase.",
            r"a posit\b",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mondegreens.anchors_away",
            "'anchors away' should be 'anchors aweigh'.",
            r"anchors away",
        )
        .with_severity(Severity::Error)
        .with_replacement("anchors aweigh"),

        Check::new(
            "mondegreens.beckon_call",
            "'beckon call' should be 'beck and call'.",
            r"beckon call",
        )
        .with_severity(Severity::Error)
        .with_replacement("beck and call"),

        Check::new(
            "mondegreens.curve_your_enthusiasm",
            "'curve your enthusiasm' should be 'curb your enthusiasm'.",
            r"curve (?:your|his|her|their|my) enthusiasm",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("curb your enthusiasm"),

        Check::new(
            "mondegreens.doggy_dog_world",
            "'doggy dog world' should be 'dog-eat-dog world'.",
            r"doggy dog world",
        )
        .with_severity(Severity::Error)
        .with_replacement("dog-eat-dog world"),

        Check::new(
            "mondegreens.damp_squid",
            "'damp squid' should be 'damp squib'.",
            r"damp squid",
        )
        .with_severity(Severity::Error)
        .with_replacement("damp squib"),

        Check::new(
            "mondegreens.extract_revenge",
            "'extract revenge' should be 'exact revenge'.",
            r"extract(?:ed|ing|s)? revenge",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("exact revenge"),

        Check::new(
            "mondegreens.fall_by_the_waste_side",
            "'fall by the waste side' should be 'fall by the wayside'.",
            r"fall(?:s|ing|en)? by the waste side",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("fall by the wayside"),

        Check::new(
            "mondegreens.for_all_intensive_purposes",
            "'for all intensive purposes' should be 'for all intents and purposes'.",
            r"for all intensive purposes",
        )
        .with_severity(Severity::Error)
        .with_replacement("for all intents and purposes"),

        Check::new(
            "mondegreens.got_off_scotch_free",
            "'got off scotch free' should be 'got off scot-free'.",
            r"(?:got|get|getting) off scotch free",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("got off scot-free"),

        Check::new(
            "mondegreens.honing_in",
            "'honing in' should be 'homing in'.",
            r"hon(?:e|ed|es|ing) in on",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("homing in on"),

        Check::new(
            "mondegreens.just_deserves",
            "'just deserves' should be 'just deserts' (meaning deserved outcome).",
            r"just deserves",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mondegreens.just_desserts",
            "'just desserts' should be 'just deserts' (meaning deserved outcome).",
            r"just desserts",
        )
        .with_severity(Severity::Error)
        .with_replacement("just deserts"),

        Check::new(
            "mondegreens.mute_point",
            "'mute point' should be 'moot point'.",
            r"mute point",
        )
        .with_severity(Severity::Error)
        .with_replacement("moot point"),

        Check::new(
            "mondegreens.nip_it_in_the_butt",
            "'nip it in the butt' should be 'nip it in the bud'.",
            r"nip(?:ped|s|ping)? (?:it )?in the butt",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("nip it in the bud"),

        Check::new(
            "mondegreens.old_wise_tale",
            "'old wise tale' should be 'old wives' tale'.",
            r"old wise tale",
        )
        .with_severity(Severity::Error)
        .with_replacement("old wives' tale"),

        Check::new(
            "mondegreens.on_tender_hooks",
            "'on tender hooks' should be 'on tenterhooks'.",
            r"on tender hooks",
        )
        .with_severity(Severity::Error)
        .with_replacement("on tenterhooks"),

        Check::new(
            "mondegreens.pass_mustard",
            "'pass mustard' should be 'pass muster'.",
            r"pass(?:es|ed|ing)? mustard",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("pass muster"),

        Check::new(
            "mondegreens.peaked_interest",
            "'peaked interest' should be 'piqued interest'.",
            r"peak(?:ed|s)? (?:my|your|his|her|their|our|the) interest",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("piqued interest"),

        Check::new(
            "mondegreens.play_it_by_year",
            "'play it by year' should be 'play it by ear'.",
            r"play(?:ed|s|ing)? it by year",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("play it by ear"),

        Check::new(
            "mondegreens.shoe_in",
            "'shoe in' should be 'shoo-in'.",
            r"shoe[- ]in",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("shoo-in"),

        Check::new(
            "mondegreens.slight_of_hand",
            "'slight of hand' should be 'sleight of hand'.",
            r"slight of hand",
        )
        .with_severity(Severity::Error)
        .with_replacement("sleight of hand"),

        Check::new(
            "mondegreens.sneak_peak",
            "'sneak peak' should be 'sneak peek'.",
            r"sneak peak",
        )
        .with_severity(Severity::Error)
        .with_replacement("sneak peek"),

        Check::new(
            "mondegreens.statue_of_limitations",
            "'statue of limitations' should be 'statute of limitations'.",
            r"statue of limitations",
        )
        .with_severity(Severity::Error)
        .with_replacement("statute of limitations"),

        Check::new(
            "mondegreens.take_for_granite",
            "'take for granite' should be 'take for granted'.",
            r"tak(?:e|es|en|ing) (?:it |this |that )?for granite",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("take for granted"),

        Check::new(
            "mondegreens.tow_the_line",
            "'tow the line' should be 'toe the line'.",
            r"tow(?:ed|s|ing)? the line",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("toe the line"),

        Check::new(
            "mondegreens.wet_your_appetite",
            "'wet your appetite' should be 'whet your appetite'.",
            r"wet(?:s|ted|ting)? (?:your|my|his|her|their|our|the) appetite",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("whet your appetite"),

        Check::new(
            "mondegreens.wreck_havoc",
            "'wreck havoc' should be 'wreak havoc'.",
            r"wreck(?:ed|s|ing)? havoc",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("wreak havoc"),

        Check::new(
            "mondegreens.escape_goat",
            "'escape goat' should be 'scapegoat'.",
            r"escape goat",
        )
        .with_severity(Severity::Error)
        .with_replacement("scapegoat"),

        Check::new(
            "mondegreens.bold_faced_lie",
            "'bold faced lie' should be 'bald-faced lie'.",
            r"bold[- ]faced lie",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("bald-faced lie"),

        Check::new(
            "mondegreens.baited_breath",
            "'baited breath' should be 'bated breath'.",
            r"baited breath",
        )
        .with_severity(Severity::Error)
        .with_replacement("bated breath"),

        Check::new(
            "mondegreens.deep_seeded",
            "'deep seeded' should be 'deep-seated'.",
            r"deep seeded",
        )
        .with_severity(Severity::Error)
        .with_replacement("deep-seated"),

        Check::new(
            "mondegreens.free_reign",
            "'free reign' should be 'free rein'.",
            r"free reign",
        )
        .with_severity(Severity::Error)
        .with_replacement("free rein"),

        Check::new(
            "mondegreens.piece_of_mind",
            "'piece of mind' (when meaning tranquility) should be 'peace of mind'.",
            r"(?:give|gave|gives|giving) (?:you|me|him|her|them|us) a piece of (?:my|your|his|her|their|our) mind",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "mondegreens.reign_in",
            "'reign in' should be 'rein in'.",
            r"reign(?:ed|s|ing)? in",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("rein in"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mondegreen() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "mondegreens.doggy_dog_world")
            .unwrap();
        let results = check.run("It's a doggy dog world out there.");
        assert_eq!(results.len(), 1);
    }
}
