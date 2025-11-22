//! Preferred forms checks for proselint-wasm
//!
//! Suggests preferred word forms and proper hyphenation for phrasal adjectives.

use crate::check::{Check, Severity};

/// Get all preferred forms checks
pub fn get_checks() -> Vec<Check> {
    let mut checks = Vec::new();

    // Preferred forms - non-standard vs preferred usage
    let preferred_forms: &[(&str, &str, &str)] = &[
        // a/an with silent h
        (r"\ba historic\b", "Use 'a' before words that start with a pronounced 'h'.", "a historic"),
        (r"\ban historic\b", "Use 'a' before words that start with a pronounced 'h'.", "a historic"),

        // Commonly confused/misused words
        ("firstly", "Use 'first' instead of 'firstly'.", "first"),
        ("secondly", "Use 'second' instead of 'secondly'.", "second"),
        ("thirdly", "Use 'third' instead of 'thirdly'.", "third"),
        ("utilize", "Use 'use' instead of 'utilize'.", "use"),
        ("utilization", "Use 'use' instead of 'utilization'.", "use"),
        ("methodology", "Often 'method' is sufficient.", "method"),
        ("amongst", "Use 'among' instead of 'amongst'.", "among"),
        ("whilst", "Use 'while' instead of 'whilst'.", "while"),
        ("learnt", "In American English, use 'learned'.", "learned"),
        ("burnt", "In American English, use 'burned'.", "burned"),
        ("dreamt", "In American English, use 'dreamed'.", "dreamed"),
        ("spelt", "In American English, use 'spelled'.", "spelled"),
        ("leapt", "In American English, use 'leaped'.", "leaped"),
        ("towards", "In American English, use 'toward'.", "toward"),
        ("forwards", "In American English, use 'forward'.", "forward"),
        ("backwards", "In American English, use 'backward'.", "backward"),
        ("afterwards", "In American English, use 'afterward'.", "afterward"),
        ("anyways", "Use 'anyway' instead of 'anyways'.", "anyway"),
        ("irregardless", "Use 'regardless' instead of 'irregardless'.", "regardless"),
        ("supposably", "Use 'supposedly' instead of 'supposably'.", "supposedly"),
        ("alright", "Use 'all right' instead of 'alright'.", "all right"),
        ("preventative", "Use 'preventive' instead of 'preventative'.", "preventive"),
        ("orientate", "Use 'orient' instead of 'orientate'.", "orient"),
        ("commentate", "Use 'comment' instead of 'commentate'.", "comment"),
        ("administrate", "Use 'administer' instead of 'administrate'.", "administer"),
        ("conversate", "Use 'converse' instead of 'conversate'.", "converse"),
        ("supposively", "Use 'supposedly' instead of 'supposively'.", "supposedly"),
        ("expresso", "The correct term is 'espresso'.", "espresso"),
        ("et cetera", "'Etc.' is preferred in most contexts.", "etc."),
        ("etcetera", "'Etc.' is preferred.", "etc."),
        ("per say", "The correct phrase is 'per se'.", "per se"),
        ("for all intensive purposes", "The correct phrase is 'for all intents and purposes'.", "for all intents and purposes"),
        ("all of the sudden", "The correct phrase is 'all of a sudden'.", "all of a sudden"),
        ("all of a sudden", "Consider whether this phrase is necessary.", "suddenly"),
        ("one in the same", "The correct phrase is 'one and the same'.", "one and the same"),
        ("by in large", "The correct phrase is 'by and large'.", "by and large"),
        ("case and point", "The correct phrase is 'case in point'.", "case in point"),
        ("statue of limitations", "The correct phrase is 'statute of limitations'.", "statute of limitations"),
        ("escape goat", "The correct phrase is 'scapegoat'.", "scapegoat"),
        ("biting my time", "The correct phrase is 'biding my time'.", "biding my time"),
        ("old wise tale", "The correct phrase is 'old wives' tale'.", "old wives' tale"),
        ("extract revenge", "The correct phrase is 'exact revenge'.", "exact revenge"),
        ("self-depreciating", "The correct term is 'self-deprecating'.", "self-deprecating"),
        ("deep seeded", "The correct phrase is 'deep-seated'.", "deep-seated"),
        ("peaked my interest", "The correct phrase is 'piqued my interest'.", "piqued my interest"),
        ("sneak peak", "The correct phrase is 'sneak peek'.", "sneak peek"),
        ("slight of hand", "The correct phrase is 'sleight of hand'.", "sleight of hand"),
        ("wet your appetite", "The correct phrase is 'whet your appetite'.", "whet your appetite"),
        ("shoe-in", "The correct term is 'shoo-in'.", "shoo-in"),
        ("baited breath", "The correct phrase is 'bated breath'.", "bated breath"),
        ("phase out", "The correct phrase is 'faze out' (to disturb).", "faze"),
        ("hone in", "The correct phrase is 'home in'.", "home in"),
        ("tow the line", "The correct phrase is 'toe the line'.", "toe the line"),
        ("free reign", "The correct phrase is 'free rein'.", "free rein"),
        ("reign in", "The correct phrase is 'rein in'.", "rein in"),
        ("beckon call", "The correct phrase is 'beck and call'.", "beck and call"),
        ("hunger pains", "The correct phrase is 'hunger pangs'.", "hunger pangs"),
        ("butt naked", "The original phrase is 'buck naked'.", "buck naked"),
        ("chalk full", "The correct phrase is 'chock-full'.", "chock-full"),
        ("chock full", "Consider using 'chock-full' (hyphenated).", "chock-full"),
        ("on accident", "The correct phrase is 'by accident'.", "by accident"),
        ("different than", "Use 'different from' in formal writing.", "different from"),
        ("center around", "Use 'center on' or 'revolve around'.", "center on"),
        ("try and", "Use 'try to' instead of 'try and'.", "try to"),
        ("could care less", "The correct phrase is 'couldn't care less'.", "couldn't care less"),
        ("should of", "Use 'should have' instead of 'should of'.", "should have"),
        ("would of", "Use 'would have' instead of 'would of'.", "would have"),
        ("could of", "Use 'could have' instead of 'could of'.", "could have"),
        ("must of", "Use 'must have' instead of 'must of'.", "must have"),
        ("might of", "Use 'might have' instead of 'might of'.", "might have"),
        ("a criteria", "Use 'a criterion' (criteria is plural).", "a criterion"),
        ("a phenomena", "Use 'a phenomenon' (phenomena is plural).", "a phenomenon"),
        ("a media", "Use 'a medium' (media is plural).", "a medium"),
        ("a bacteria", "Use 'a bacterium' (bacteria is plural).", "a bacterium"),
        ("a data", "While controversial, 'datum' is the traditional singular.", "a datum"),
        ("less people", "Use 'fewer people' for countable nouns.", "fewer people"),
        ("less things", "Use 'fewer things' for countable nouns.", "fewer things"),
        ("less items", "Use 'fewer items' for countable nouns.", "fewer items"),
        ("10 items or less", "Use 'fewer' for countable nouns.", "10 items or fewer"),
        ("amount of people", "Use 'number of people' for countable nouns.", "number of people"),
        ("between you and I", "Use 'between you and me'.", "between you and me"),
        ("give it to John and I", "Use 'John and me' as the object.", "John and me"),
        ("for you and I", "Use 'for you and me'.", "for you and me"),
        ("literally", "Reserve 'literally' for things that actually happened.", ""),
    ];

    for (i, (pattern, message, replacement)) in preferred_forms.iter().enumerate() {
        let mut check = Check::new(
            Box::leak(format!("preferred_forms.usage.{}", i).into_boxed_str()),
            Box::leak(message.to_string().into_boxed_str()),
            pattern,
        )
        .with_severity(Severity::Suggestion);

        if !replacement.is_empty() {
            check = check.with_replacement(replacement);
        }

        checks.push(check);
    }

    // Phrasal adjectives that need hyphenation
    let phrasal_adjectives: &[(&str, &str)] = &[
        // Common compound modifiers
        (r"\bhigh quality\b", "high-quality"),
        (r"\blow cost\b", "low-cost"),
        (r"\bfull time\b", "full-time"),
        (r"\bpart time\b", "part-time"),
        (r"\bhigh level\b", "high-level"),
        (r"\blow level\b", "low-level"),
        (r"\blong term\b", "long-term"),
        (r"\bshort term\b", "short-term"),
        (r"\breal time\b", "real-time"),
        (r"\bstate of the art\b", "state-of-the-art"),
        (r"\bup to date\b", "up-to-date"),
        (r"\bout of date\b", "out-of-date"),
        (r"\bwell known\b", "well-known"),
        (r"\bwell being\b", "well-being"),
        (r"\bself esteem\b", "self-esteem"),
        (r"\bself confidence\b", "self-confidence"),
        (r"\bself control\b", "self-control"),
        (r"\bself aware\b", "self-aware"),
        (r"\bself service\b", "self-service"),
        (r"\bself made\b", "self-made"),
        (r"\bone time\b", "one-time"),
        (r"\bfirst class\b", "first-class"),
        (r"\bsecond hand\b", "second-hand"),
        (r"\bhands on\b", "hands-on"),
        (r"\bday to day\b", "day-to-day"),
        (r"\bface to face\b", "face-to-face"),
        (r"\bone on one\b", "one-on-one"),
        (r"\bstep by step\b", "step-by-step"),
        (r"\bword of mouth\b", "word-of-mouth"),
        (r"\brun of the mill\b", "run-of-the-mill"),
        (r"\bcut and dried\b", "cut-and-dried"),
        (r"\bopen ended\b", "open-ended"),
        (r"\bclosed minded\b", "closed-minded"),
        (r"\bopen minded\b", "open-minded"),
        (r"\bnarrow minded\b", "narrow-minded"),
        (r"\blike minded\b", "like-minded"),
        (r"\bstrong willed\b", "strong-willed"),
        (r"\bweak willed\b", "weak-willed"),
        (r"\bgood looking\b", "good-looking"),
        (r"\bhigh maintenance\b", "high-maintenance"),
        (r"\blow maintenance\b", "low-maintenance"),
        (r"\bhigh risk\b", "high-risk"),
        (r"\blow risk\b", "low-risk"),
        (r"\bhigh tech\b", "high-tech"),
        (r"\blow tech\b", "low-tech"),
        (r"\bhigh end\b", "high-end"),
        (r"\blow end\b", "low-end"),
        (r"\btop notch\b", "top-notch"),
        (r"\bground breaking\b", "ground-breaking"),
        (r"\brecord breaking\b", "record-breaking"),
        (r"\bheart breaking\b", "heart-breaking"),
        (r"\bbreath taking\b", "breath-taking"),
        (r"\bmind blowing\b", "mind-blowing"),
        (r"\beye opening\b", "eye-opening"),
        (r"\blife changing\b", "life-changing"),
        (r"\bgame changing\b", "game-changing"),
        (r"\btime consuming\b", "time-consuming"),
        (r"\bcost effective\b", "cost-effective"),
        (r"\benergy efficient\b", "energy-efficient"),
        (r"\buser friendly\b", "user-friendly"),
        (r"\beco friendly\b", "eco-friendly"),
        (r"\benvironmentally friendly\b", "environmentally-friendly"),
        (r"\bchild friendly\b", "child-friendly"),
        (r"\bfamily friendly\b", "family-friendly"),
        (r"\bbudget friendly\b", "budget-friendly"),
        (r"\bpet friendly\b", "pet-friendly"),
        (r"\bbrand new\b", "brand-new"),
        (r"\bice cold\b", "ice-cold"),
        (r"\bred hot\b", "red-hot"),
        (r"\bsky high\b", "sky-high"),
        (r"\bbone dry\b", "bone-dry"),
        (r"\bcrystal clear\b", "crystal-clear"),
        (r"\brock solid\b", "rock-solid"),
        (r"\biron clad\b", "iron-clad"),
        (r"\bbattle tested\b", "battle-tested"),
        (r"\btime tested\b", "time-tested"),
        (r"\bwell established\b", "well-established"),
        (r"\bwell designed\b", "well-designed"),
        (r"\bwell documented\b", "well-documented"),
        (r"\bwell organized\b", "well-organized"),
        (r"\bwell prepared\b", "well-prepared"),
        (r"\bwell written\b", "well-written"),
        (r"\bwell read\b", "well-read"),
        (r"\bwell spoken\b", "well-spoken"),
        (r"\bwell educated\b", "well-educated"),
        (r"\bwell trained\b", "well-trained"),
        (r"\bwell behaved\b", "well-behaved"),
        (r"\bwell mannered\b", "well-mannered"),
        (r"\bwell intentioned\b", "well-intentioned"),
        (r"\bwell meaning\b", "well-meaning"),
        (r"\bwell deserved\b", "well-deserved"),
        (r"\bwell earned\b", "well-earned"),
        (r"\bwell received\b", "well-received"),
        (r"\bno brainer\b", "no-brainer"),
        (r"\bwork life balance\b", "work-life balance"),
        (r"\bwin win\b", "win-win"),
        (r"\blose lose\b", "lose-lose"),
        (r"\ball or nothing\b", "all-or-nothing"),
        (r"\bdo or die\b", "do-or-die"),
        (r"\bhit or miss\b", "hit-or-miss"),
        (r"\btake it or leave it\b", "take-it-or-leave-it"),
        (r"\bonce in a lifetime\b", "once-in-a-lifetime"),
        (r"\bnever ending\b", "never-ending"),
        (r"\bfar reaching\b", "far-reaching"),
        (r"\bwide ranging\b", "wide-ranging"),
        (r"\bfast paced\b", "fast-paced"),
        (r"\bslow moving\b", "slow-moving"),
        (r"\bhard working\b", "hard-working"),
        (r"\beasy going\b", "easy-going"),
        (r"\bfree range\b", "free-range"),
        (r"\bsugar free\b", "sugar-free"),
        (r"\bfat free\b", "fat-free"),
        (r"\bgluten free\b", "gluten-free"),
        (r"\bdairy free\b", "dairy-free"),
        (r"\bcaffeine free\b", "caffeine-free"),
        (r"\balcohol free\b", "alcohol-free"),
        (r"\bhassle free\b", "hassle-free"),
        (r"\bstress free\b", "stress-free"),
        (r"\brisk free\b", "risk-free"),
        (r"\berror free\b", "error-free"),
        (r"\bcarefree\b", "care-free"),
        (r"\bpaid for\b", "paid-for"),
        (r"\bin person\b", "in-person"),
        (r"\bin depth\b", "in-depth"),
        (r"\bin house\b", "in-house"),
        (r"\bon site\b", "on-site"),
        (r"\boff site\b", "off-site"),
        (r"\bon demand\b", "on-demand"),
        (r"\bon call\b", "on-call"),
        (r"\bon time\b", "on-time"),
        (r"\bon point\b", "on-point"),
        (r"\bon brand\b", "on-brand"),
        (r"\boff brand\b", "off-brand"),
    ];

    for (i, (pattern, replacement)) in phrasal_adjectives.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("preferred_forms.hyphenation.{}", i).into_boxed_str()),
                Box::leak(format!("Consider hyphenating as '{}'.", replacement).into_boxed_str()),
                pattern,
            )
            .raw()
            .with_severity(Severity::Suggestion)
            .with_replacement(replacement),
        );
    }

    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferred_form() {
        let checks = get_checks();
        // "utilize" is at index 5 in preferred_forms array
        let check = checks.iter().find(|c| c.id == "preferred_forms.usage.5").unwrap();
        let results = check.run("We need to utilize this tool.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_phrasal_adjective() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "preferred_forms.hyphenation.0").unwrap();
        let results = check.run("This is a high quality product.");
        assert_eq!(results.len(), 1);
    }
}
