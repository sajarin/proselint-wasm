//! Industrial/corporate language checks for proselint-wasm
//!
//! Detects corporate jargon, buzzwords, and unnecessarily complex language.

use crate::check::{Check, Severity};

/// Get all industrial language checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Corporate jargon
        Check::new(
            "industrial_language.synergy",
            "'synergy' is corporate jargon. Consider 'cooperation' or 'collaboration'.",
            r"synerg(?:y|ies|istic|ize)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.leverage",
            "'leverage' as a verb is corporate jargon. Consider 'use' or 'take advantage of'.",
            r"leverag(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.paradigm_shift",
            "'paradigm shift' is overused jargon. Consider 'fundamental change'.",
            r"paradigm shift",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("fundamental change"),

        Check::new(
            "industrial_language.think_outside_the_box",
            "'think outside the box' is a cliche. Consider 'think creatively'.",
            r"think(?:ing)? outside the box",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("think creatively"),

        Check::new(
            "industrial_language.circle_back",
            "'circle back' is corporate jargon. Consider 'return to' or 'revisit'.",
            r"circle(?:d|s|ing)? back",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.touch_base",
            "'touch base' is corporate jargon. Consider 'contact' or 'meet'.",
            r"touch(?:ed|es|ing)? base",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.low_hanging_fruit",
            "'low-hanging fruit' is overused. Consider 'easy wins' or 'simple tasks'.",
            r"low[- ]hanging fruit",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.move_the_needle",
            "'move the needle' is corporate jargon. Consider 'make progress' or 'have impact'.",
            r"mov(?:e|ed|es|ing) the needle",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.bandwidth",
            "'bandwidth' (for capacity) is corporate jargon. Consider 'capacity' or 'time'.",
            r"(?:have|has|had|don't have|doesn't have) (?:the )?bandwidth",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.take_offline",
            "'take offline' is corporate jargon. Consider 'discuss privately' or 'discuss later'.",
            r"tak(?:e|es|ing|en) (?:this |that |it )?offline",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.actionable",
            "'actionable' is corporate jargon. Consider 'practical' or 'useful'.",
            r"actionable",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.deliverables",
            "'deliverables' is corporate jargon. Consider 'results' or 'outputs'.",
            r"deliverables?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.drill_down",
            "'drill down' is corporate jargon. Consider 'examine closely' or 'analyze'.",
            r"drill(?:ed|s|ing)? down",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.stakeholder",
            "'stakeholder' may be jargon. Consider 'participant' or 'interested party' if clearer.",
            r"stakeholders?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.best_practice",
            "'best practice' is overused. Consider 'effective method' or 'recommended approach'.",
            r"best practice",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.core_competency",
            "'core competency' is corporate jargon. Consider 'strength' or 'expertise'.",
            r"core competenc(?:y|ies)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("strength"),

        Check::new(
            "industrial_language.value_add",
            "'value-add' is corporate jargon. Consider 'benefit' or 'advantage'.",
            r"value[- ]add",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.scalable",
            "'scalable' may be jargon outside technical contexts.",
            r"scalable",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.proactive",
            "'proactive' is overused. Consider 'active' or 'anticipatory'.",
            r"proactive",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.incentivize",
            "'incentivize' is corporate jargon. Consider 'encourage' or 'motivate'.",
            r"incentiviz(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("encourage"),

        Check::new(
            "industrial_language.impactful",
            "'impactful' is jargon. Consider 'effective' or 'influential'.",
            r"impactful",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("effective"),

        Check::new(
            "industrial_language.optics",
            "'optics' (for perception) is jargon. Consider 'appearance' or 'perception'.",
            r"the optics",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("the perception"),

        Check::new(
            "industrial_language.pivot",
            "'pivot' (for change direction) is corporate jargon. Consider 'change' or 'shift'.",
            r"pivot(?:ed|s|ing)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.onboarding",
            "'onboarding' is jargon. Consider 'training' or 'orientation'.",
            r"onboarding",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.deep_dive",
            "'deep dive' is corporate jargon. Consider 'thorough analysis' or 'detailed look'.",
            r"deep dive",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("thorough analysis"),

        Check::new(
            "industrial_language.robust",
            "'robust' is overused. Consider 'strong' or 'thorough'.",
            r"robust",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.reach_out",
            "'reach out' is corporate jargon. Consider 'contact' or 'ask'.",
            r"reach(?:ed|es|ing)? out to",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.game_changer",
            "'game-changer' is overused. Consider 'significant development' or 'breakthrough'.",
            r"game[- ]changer",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.disrupt",
            "'disrupt' is overused in business contexts.",
            r"disrupt(?:ed|s|ing|ive|ion)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.ecosystem",
            "'ecosystem' (for business environment) is jargon. Consider 'environment' or 'community'.",
            r"(?:business |corporate |startup |tech )?ecosystem",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // === Airlinese (from proselint) ===
        Check::new(
            "industrial_language.airlinese.enplane",
            "'enplane' is airlinese. Use 'board' or 'get on'.",
            r"enplan(?:e|ed|ing|ement)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("board"),

        Check::new(
            "industrial_language.airlinese.deplane",
            "'deplane' is airlinese. Use 'disembark' or 'get off'.",
            r"deplan(?:e|ed|ing|ement)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("disembark"),

        Check::new(
            "industrial_language.airlinese.momentarily",
            "'taking off momentarily' is airlinese. Use 'shortly' or 'soon'.",
            r"taking off momentarily",
        )
        .with_severity(Severity::Suggestion),

        // === Bureaucratese (from proselint) ===
        Check::new(
            "industrial_language.bureaucratese.meets_approval",
            "'meets with your approval' is bureaucratese.",
            r"meets? with your approval",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // === Chatspeak (from proselint) ===
        Check::new("industrial_language.chatspeak.lol", "'lol'/'LOL' is chatspeak. Write it out.", r"\b[Ll][Oo][Ll]\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.omg", "'OMG' is chatspeak. Write it out.", r"\bOMG\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.rofl", "'rofl' is chatspeak. Write it out.", r"\brofl\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.brb", "'brb' is chatspeak. Write it out.", r"\bbrb\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.btw", "'btw' is chatspeak. Write it out.", r"\bbtw\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.ttyl", "'TTYL' is chatspeak. Write it out.", r"\bTTYL\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.afaik", "'AFAIK' is chatspeak. Write it out.", r"\bAFAIK\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.afk", "'AFK' is chatspeak. Write it out.", r"\bAFK\b").raw().with_severity(Severity::Warning),
        Check::new("industrial_language.chatspeak.asap", "'ASAP' may be too informal. Consider 'as soon as possible'.", r"\bASAP\b").raw().with_severity(Severity::Suggestion),
        Check::new("industrial_language.chatspeak.2day", "'2day' is chatspeak. Write 'today'.", r"\b2day\b").raw().with_severity(Severity::Warning).with_replacement("today"),
        Check::new("industrial_language.chatspeak.4u", "'4U' is chatspeak. Write 'for you'.", r"\b4U\b").raw().with_severity(Severity::Warning).with_replacement("for you"),
        Check::new("industrial_language.chatspeak.b4", "'B4' is chatspeak. Write 'before'.", r"\bB4\b").raw().with_severity(Severity::Warning).with_replacement("before"),
        Check::new("industrial_language.chatspeak.gr8", "'GR8' is chatspeak. Write 'great'.", r"\bGR8\b").raw().with_severity(Severity::Warning).with_replacement("great"),
        Check::new("industrial_language.chatspeak.cya", "'cya' is chatspeak. Write 'see you'.", r"\bcya\b").raw().with_severity(Severity::Warning).with_replacement("see you"),
        Check::new("industrial_language.chatspeak.thnx", "'THNX'/'THX' is chatspeak. Write 'thanks'.", r"\b(?:THNX|THX)\b").raw().with_severity(Severity::Warning).with_replacement("thanks"),
        Check::new("industrial_language.chatspeak.xoxo", "'XOXO' is chatspeak.", r"\bXOXO\b").raw().with_severity(Severity::Suggestion),

        // === Commercialese (from proselint) ===
        Check::new(
            "industrial_language.commercialese.inst",
            "'inst.' is commercialese. Use 'this month' or the actual date.",
            r"\binst\.",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.prox",
            "'prox.' is commercialese. Use 'next month' or the actual date.",
            r"\bprox\.",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.ult",
            "'ult.' is commercialese. Use 'last month' or the actual date.",
            r"\bult\.",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.beg_to_advise",
            "'beg to advise' is commercialese.",
            r"beg to advise",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.enclosed_please_find",
            "'enclosed please find' is commercialese.",
            r"enclosed please find",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.please_be_advised",
            "'please be advised that' is commercialese.",
            r"please be advised that",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "industrial_language.commercialese.thanking_in_advance",
            "'thanking you in advance' is commercialese.",
            r"thanking you in advance",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.commercialese.the_undersigned",
            "'the undersigned' is commercialese.",
            r"the undersigned",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "industrial_language.commercialese.in_the_amount_of",
            "'in the amount of' is commercialese. Use 'for'.",
            r"in the amount of",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("for"),

        // === Additional jargon (from proselint) ===
        Check::new(
            "industrial_language.jargon.in_the_affirmative",
            "'in the affirmative' is jargon. Use 'yes'.",
            r"in the affirmative",
        )
        .with_severity(Severity::Warning)
        .with_replacement("yes"),

        Check::new(
            "industrial_language.jargon.in_the_negative",
            "'in the negative' is jargon. Use 'no'.",
            r"in the negative",
        )
        .with_severity(Severity::Warning)
        .with_replacement("no"),

        Check::new(
            "industrial_language.jargon.agendize",
            "'agendize' is jargon. Use 'schedule' or 'add to the agenda'.",
            r"agendize",
        )
        .with_severity(Severity::Warning)
        .with_replacement("schedule"),

        Check::new(
            "industrial_language.jargon.disincentivize",
            "'disincentivize' is jargon. Use 'discourage'.",
            r"disincentiviz(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("discourage"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synergy() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "industrial_language.synergy").unwrap();
        let results = check.run("We need more synergy between teams.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_circle_back() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "industrial_language.circle_back").unwrap();
        let results = check.run("Let's circle back on this next week.");
        assert_eq!(results.len(), 1);
    }
}
