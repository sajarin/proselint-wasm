//! Miscellaneous checks for proselint-wasm
//!
//! Various writing quality checks that don't fit in other categories.

use crate::check::{Check, Severity};

/// Get all miscellaneous checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Passive voice indicators
        Check::new(
            "misc.passive_voice",
            "Consider using active voice instead of passive voice.",
            r"(?:was|were|been|being|is|are|am) (?:being )?(?:\w+ed|made|done|taken|given|shown)",
        )
        .with_severity(Severity::Suggestion),
        // Starting with "There is/are"
        Check::new(
            "misc.there_is",
            "There is/are can often be eliminated for more direct writing.",
            r"^There (?:is|are|was|were)",
        )
        .with_severity(Severity::Suggestion),
        // Double negatives
        Check::new(
            "misc.double_negative",
            "Avoid double negatives. Use a positive statement instead.",
            r"(?:not|n't) (?:un|in|im)\w+",
        )
        .with_severity(Severity::Warning),
        // Sentence starting with "But"
        Check::new(
            "misc.sentence_start_but",
            "Starting a sentence with But can be informal. Consider However in formal writing.",
            r"(?:^|[.!?]\s+)But ",
        )
        .with_severity(Severity::Suggestion),
        // Sentence starting with "And"
        Check::new(
            "misc.sentence_start_and",
            "Starting a sentence with And can be informal. Consider restructuring.",
            r"(?:^|[.!?]\s+)And ",
        )
        .with_severity(Severity::Suggestion),
        // "A lot"
        Check::new(
            "misc.a_lot",
            "A lot is informal. Consider many, much, or often.",
            r"a lot",
        )
        .with_severity(Severity::Suggestion),
        // "Alot" (misspelling)
        Check::new(
            "misc.alot",
            "Alot is not a word. Use a lot or allot.",
            r"alot",
        )
        .with_severity(Severity::Error)
        .with_replacement("a lot"),
        // "Could of" (should be "could have")
        Check::new(
            "misc.could_of",
            "Could of should be could have or could've.",
            r"could of",
        )
        .with_severity(Severity::Error)
        .with_replacement("could have"),
        // "Would of"
        Check::new(
            "misc.would_of",
            "Would of should be would have or would've.",
            r"would of",
        )
        .with_severity(Severity::Error)
        .with_replacement("would have"),
        // "Should of"
        Check::new(
            "misc.should_of",
            "Should of should be should have or should've.",
            r"should of",
        )
        .with_severity(Severity::Error)
        .with_replacement("should have"),
        // "Its" vs "It's"
        Check::new(
            "misc.its_contraction",
            "Check whether you need it's (it is) or its (possessive).",
            r"its (?:a|an|the|been|not|going|being)",
        )
        .with_severity(Severity::Warning),
        // "Your" vs "You're"
        Check::new(
            "misc.your_contraction",
            "Check whether you need you're (you are) or your (possessive).",
            r"your (?:a|an|the|not|going|being|welcome)",
        )
        .with_severity(Severity::Warning),
        // "Their" vs "They're" vs "There"
        Check::new(
            "misc.their_contraction",
            "Check whether you need they're (they are), their (possessive), or there.",
            r"their (?:is|are|was|were|going|coming)",
        )
        .with_severity(Severity::Warning),
        // "Affect" vs "Effect"
        Check::new(
            "misc.affect_effect",
            "Note: affect is usually a verb, effect is usually a noun.",
            r"the affect|an affect",
        )
        .with_severity(Severity::Warning)
        .with_replacement("the effect"),
        // Multiple exclamation marks
        Check::new(
            "misc.multiple_exclamation",
            "Multiple exclamation marks are generally unnecessary.",
            r"!{2,}",
        )
        .with_severity(Severity::Warning)
        .with_replacement("!"),
        // Multiple question marks
        Check::new(
            "misc.multiple_question",
            "Multiple question marks are generally unnecessary.",
            r"\?{2,}",
        )
        .with_severity(Severity::Warning)
        .with_replacement("?"),
        // Run-on sentence indicator (very long sentence)
        Check::new(
            "misc.run_on_sentence",
            "This sentence may be too long. Consider breaking it up.",
            r"[^.!?]{200,}[.!?]",
        )
        .with_severity(Severity::Suggestion),
        // Latin phrases - prefer English
        Check::new(
            "misc.latin.ceteris_paribus",
            "Prefer other things being equal over the Latin ceteris paribus.",
            r"ceteris paribus",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("other things being equal"),
        Check::new(
            "misc.latin.inter_alia",
            "Prefer among other things over the Latin inter alia.",
            r"inter alia",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("among other things"),
        Check::new(
            "misc.latin.simpliciter",
            "Prefer simply or in itself over the Latin simpliciter.",
            r"simpliciter",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("simply"),
        Check::new(
            "misc.latin.mutatis_mutandis",
            "Prefer with the necessary changes over mutatis mutandis.",
            r"mutatis mutandis",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("with the necessary changes"),
        Check::new(
            "misc.latin.ad_hoc",
            "Consider whether ad hoc could be replaced with improvised or for this purpose.",
            r"ad hoc",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.latin.de_facto",
            "Consider whether de facto could be replaced with in practice or actual.",
            r"de facto",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.latin.ergo",
            "Consider using therefore instead of ergo.",
            r"\bergo\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("therefore"),
        Check::new(
            "misc.latin.qua",
            "Consider whether qua could be replaced with as or in the capacity of.",
            r"\bqua\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.latin.viz",
            "Consider using namely instead of viz.",
            r"\bviz\.?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("namely"),
        // Pretension checks
        Check::new(
            "misc.pretension.reconceptualize",
            "reconceptualize is pretentious jargon. Consider rethink.",
            r"reconceptualize",
        )
        .with_severity(Severity::Warning)
        .with_replacement("rethink"),
        Check::new(
            "misc.pretension.demassification",
            "demassification is pretentious jargon.",
            r"demassification",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "misc.pretension.attitudinally",
            "attitudinally is pretentious jargon.",
            r"attitudinally",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "misc.pretension.judgmentally",
            "judgmentally is often pretentious. Consider simpler alternatives.",
            r"judgmentally",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.pretension.heretofore",
            "heretofore is pretentious. Use previously or until now.",
            r"heretofore",
        )
        .with_severity(Severity::Warning)
        .with_replacement("previously"),
        Check::new(
            "misc.pretension.hitherto",
            "hitherto is pretentious. Use until now or previously.",
            r"hitherto",
        )
        .with_severity(Severity::Warning)
        .with_replacement("until now"),
        Check::new(
            "misc.pretension.therewith",
            "therewith is pretentious. Simplify your language.",
            r"therewith",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "misc.pretension.forthwith",
            "forthwith is pretentious. Use immediately.",
            r"forthwith",
        )
        .with_severity(Severity::Warning)
        .with_replacement("immediately"),
        // Apologizing / excessive hedging in academic writing
        Check::new(
            "misc.apologizing.more_research",
            "More research is needed is an unnecessary hedge.",
            r"[Mm]ore research is needed",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.apologizing.beyond_scope",
            "Beyond the scope of this can be an unnecessary disclaimer.",
            r"beyond the scope of this",
        )
        .with_severity(Severity::Suggestion),
        // Metadiscourse
        Check::new(
            "misc.metadiscourse.this_paper",
            "Metadiscourse like this paper will can often be cut.",
            r"[Tt]his (?:paper|article|essay) will",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.metadiscourse.in_this_paper",
            "Metadiscourse like in this paper can often be cut.",
            r"[Ii]n this (?:paper|article|essay)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Back formations
        Check::new(
            "misc.back_formations.enthuse",
            "enthuse is a back-formation. Consider be enthusiastic or show enthusiasm.",
            r"enthuse(?:d|s)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.back_formations.liaise",
            "liaise is a back-formation. Consider work with or coordinate with.",
            r"liaise(?:d|s)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.back_formations.surveil",
            "surveil is a back-formation. Consider keep under surveillance or watch.",
            r"surveil(?:led|ling|s)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // False plurals
        Check::new(
            "misc.false_plurals.phenomena",
            "phenomena is plural. Use phenomenon for singular.",
            r"(?:this|a|one) phenomena",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("phenomenon"),
        Check::new(
            "misc.false_plurals.criteria",
            "criteria is plural. Use criterion for singular.",
            r"(?:this|a|one) criteria",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("criterion"),
        Check::new(
            "misc.false_plurals.data",
            "data is technically plural, though singular usage is now accepted.",
            r"(?:this|the) data is",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.false_plurals.media",
            "media is plural. Use medium for singular.",
            r"(?:this|a|one) media(?:\s+is)?",
        )
        .raw()
        .with_severity(Severity::Warning),
        Check::new(
            "misc.false_plurals.alumni",
            "alumni is plural. Use alumnus (male) or alumna (female) for singular.",
            r"(?:an?|one) alumni\b",
        )
        .raw()
        .with_severity(Severity::Error),
        // Inferior/superior usage
        Check::new(
            "misc.inferior_superior.more_superior",
            "more superior is redundant. Use just superior.",
            r"more superior",
        )
        .with_severity(Severity::Error)
        .with_replacement("superior"),
        Check::new(
            "misc.inferior_superior.more_inferior",
            "more inferior is redundant. Use just inferior.",
            r"more inferior",
        )
        .with_severity(Severity::Error)
        .with_replacement("inferior"),
        Check::new(
            "misc.inferior_superior.most_superior",
            "most superior is redundant. Use just superior or the most.",
            r"most superior",
        )
        .with_severity(Severity::Error),
        // Currency
        Check::new(
            "misc.currency.redundant_dollars",
            "Do not use both $ and dollars.",
            r"\$[\d,.]+ dollars",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.currency.redundant_pounds",
            "Do not use both pound sign and pounds.",
            r"£[\d,.]+ pounds",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.currency.redundant_euros",
            "Do not use both euro sign and euros.",
            r"€[\d,.]+ euros",
        )
        .raw()
        .with_severity(Severity::Error),
        // Scare quotes overuse
        Check::new(
            "misc.scare_quotes.so_called",
            "So-called already implies skepticism; adding quotes is redundant.",
            r#"so[- ]called ['\"]"#,
        )
        .raw()
        .with_severity(Severity::Warning),
        // Illogic
        Check::new(
            "misc.illogic.close_proximity",
            "close proximity is redundant. Use proximity or close.",
            r"close proximity",
        )
        .with_severity(Severity::Warning)
        .with_replacement("proximity"),
        Check::new(
            "misc.illogic.exact_same",
            "exact same is redundant. Use same or exactly the same.",
            r"exact same",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("same"),
        Check::new(
            "misc.illogic.reason_why",
            "reason why can often be just reason or why.",
            r"reason why",
        )
        .with_severity(Severity::Suggestion),
        // Professions
        Check::new(
            "misc.professions.attorney",
            "attorney in American usage should be attorney at law formally, or just lawyer.",
            r"\battorney(?!s? at law|\s+general)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Waxed/grew
        Check::new(
            "misc.waxed.waxed",
            "waxed meaning became is archaic. Consider became or grew.",
            r"waxed (?:eloquent|poetic|philosophical|lyrical)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Whence
        Check::new(
            "misc.whence.from_whence",
            "from whence is redundant. whence already means from where.",
            r"from whence",
        )
        .with_severity(Severity::Error)
        .with_replacement("whence"),
        // Many a
        Check::new(
            "misc.many_a.singular",
            "many a takes a singular verb, not plural.",
            r"many a \w+ (?:are|were|have)",
        )
        .raw()
        .with_severity(Severity::Error),
        // Suddenly
        Check::new(
            "misc.suddenly.overuse",
            "suddenly is often unnecessary and can weaken prose.",
            r"\bsuddenly\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // Capitalization
        Check::new(
            "misc.capitalization.internet",
            "Internet is now typically lowercase: internet.",
            r"\bInternet\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("internet"),
        Check::new(
            "misc.capitalization.web",
            "Web (meaning World Wide Web) is now typically lowercase: web.",
            r"\bthe Web\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("the web"),
        // ===== Strunk & White composition rules (omit needless words) =====
        Check::new(
            "misc.composition.the_fact_that",
            "'the fact that' is wordy. Often can be omitted entirely.",
            r"the fact that",
        )
        .with_severity(Severity::Warning),
        Check::new(
            "misc.composition.who_is",
            "'who is' / 'which was' constructions can often be shortened.",
            r"(?:who|which) (?:is|was|were|are)",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.the_question_as_to",
            "'the question as to whether' is wordy. Use 'whether'.",
            r"the question as to whether",
        )
        .with_severity(Severity::Warning)
        .with_replacement("whether"),
        Check::new(
            "misc.composition.there_is_no_doubt",
            "'there is no doubt but that' is wordy. Use 'doubtless' or 'no doubt'.",
            r"there is no doubt but that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("no doubt"),
        Check::new(
            "misc.composition.used_for_fuel",
            "'used for fuel purposes' - 'purposes' is redundant.",
            r"used for \w+ purposes",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.he_is_a_man",
            "'He is a man who' is wordy. Simplify.",
            r"[Hh]e is a man who",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.in_a_hasty_manner",
            "'in a [adjective] manner' is wordy. Use the adverb form.",
            r"in a \w+ manner",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.this_is_a_subject",
            "'this is a subject that' is wordy.",
            r"[Tt]his is a (?:subject|topic) (?:that|which)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.her_story_is",
            "'Her story is a strange one' - simplify to 'Her story is strange'.",
            r"(?:story|tale|case) is a \w+ one",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.the_reason_why",
            "'the reason why is that' is wordy. Use 'because'.",
            r"the reason why is that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("because"),
        Check::new(
            "misc.composition.owing_to",
            "'owing to the fact that' is wordy. Use 'because' or 'since'.",
            r"owing to the fact that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("because"),
        Check::new(
            "misc.composition.in_spite_of",
            "'in spite of the fact that' is wordy. Use 'although' or 'though'.",
            r"in spite of the fact that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("although"),
        Check::new(
            "misc.composition.call_attention",
            "'call your attention to the fact that' is wordy. Use 'remind you' or 'notify you'.",
            r"call (?:your|their) attention to the fact that",
        )
        .raw()
        .with_severity(Severity::Warning),
        Check::new(
            "misc.composition.i_was_unaware",
            "'I was unaware of the fact that' is wordy. Use 'I didn't know'.",
            r"[Ii] was unaware of the fact that",
        )
        .raw()
        .with_severity(Severity::Warning),
        Check::new(
            "misc.composition.the_fact_that_he",
            "'the fact that [subject] had not succeeded' is wordy. Use 'failure'.",
            r"the fact that (?:he|she|they|I|we|it) had not",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.composition.along_the_lines",
            "'along the lines of' is wordy. Use 'like'.",
            r"along the lines of",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("like"),
        Check::new(
            "misc.composition.as_to_whether",
            "'as to whether' is wordy. Use 'whether'.",
            r"as to whether",
        )
        .with_severity(Severity::Warning)
        .with_replacement("whether"),
        Check::new(
            "misc.composition.as_yet",
            "'as yet' is wordy. Use 'yet'.",
            r"as yet",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("yet"),
        Check::new(
            "misc.composition.at_the_present_time",
            "'at the present time' is wordy. Use 'now' or 'currently'.",
            r"at the present time",
        )
        .with_severity(Severity::Warning)
        .with_replacement("now"),
        Check::new(
            "misc.composition.by_means_of",
            "'by means of' is wordy. Use 'by' or 'with'.",
            r"by means of",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("by"),
        Check::new(
            "misc.composition.for_the_purpose_of",
            "'for the purpose of' is wordy. Use 'to' or 'for'.",
            r"for the purpose of",
        )
        .with_severity(Severity::Warning)
        .with_replacement("to"),
        Check::new(
            "misc.composition.in_order_to",
            "'in order to' is often wordy. Consider using just 'to'.",
            r"in order to",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("to"),
        Check::new(
            "misc.composition.in_the_event_that",
            "'in the event that' is wordy. Use 'if'.",
            r"in the event that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("if"),
        Check::new(
            "misc.composition.with_regard_to",
            "'with regard to' / 'with respect to' is wordy. Use 'about' or 'regarding'.",
            r"with (?:regard|respect) to",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        // ===== Debased/bloated phrases =====
        Check::new(
            "misc.debased.at_this_point_in_time",
            "'at this point in time' is bloated. Use 'now'.",
            r"at this point in time",
        )
        .with_severity(Severity::Warning)
        .with_replacement("now"),
        Check::new(
            "misc.debased.at_this_moment_in_time",
            "'at this moment in time' is bloated. Use 'now'.",
            r"at this moment in time",
        )
        .with_severity(Severity::Warning)
        .with_replacement("now"),
        Check::new(
            "misc.debased.going_forward",
            "'going forward' is corporate jargon. Use 'in the future' or omit.",
            r"going forward",
        )
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.debased.paradigm_shift",
            "'paradigm shift' is overused jargon. Consider a simpler alternative.",
            r"paradigm shift",
        )
        .with_severity(Severity::Suggestion),
        // ===== Greylist (commonly overused words that may be warranted) =====
        Check::new(
            "misc.greylist.obviously",
            "'obviously' may be condescending or unnecessary. If it's obvious, why say so?",
            r"\b[Oo]bviously\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.greylist.utilize",
            "'utilize' is often pretentious. Usually 'use' works just as well.",
            r"\butilize[sd]?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("use"),
        // ===== Institution names =====
        Check::new(
            "misc.institution_name.virginia_tech",
            "'Virginia Polytechnic Institute' - the university prefers 'Virginia Tech'.",
            r"Virginia Polytechnic Institute",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("Virginia Tech"),
        // ===== Narcissism (self-aggrandizing phrases) =====
        Check::new(
            "misc.narcissism.in_recent_years",
            "'In recent years' can be vague. Be more specific if possible.",
            r"[Ii]n recent years",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.narcissism.very_unique",
            "'very unique' - unique is absolute. Something is unique or it isn't.",
            r"very unique",
        )
        .with_severity(Severity::Error)
        .with_replacement("unique"),
        // ===== Legal phrasing =====
        Check::new(
            "misc.not_guilty.innocent_vs_not_guilty",
            "In legal contexts, courts find defendants 'not guilty', not 'innocent'.",
            r"(?:found|declared|pronounced)\s+innocent",
        )
        .raw()
        .with_severity(Severity::Warning),
        // ===== Tense consistency (tense_present.py patterns) =====
        Check::new(
            "misc.tense.had_had",
            "'had had' may indicate awkward tense usage. Consider rephrasing.",
            r"\bhad had\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.tense.that_that",
            "'that that' may be grammatically correct but often reads awkwardly.",
            r"\bthat that\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),
        Check::new(
            "misc.tense.is_is",
            "'is is' is likely an error.",
            r"\bis is\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.tense.the_the",
            "'the the' is likely an error.",
            r"\bthe the\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new("misc.tense.a_a", "'a a' is likely an error.", r"\ba a\b")
            .raw()
            .with_severity(Severity::Error),
        Check::new(
            "misc.tense.an_an",
            "'an an' is likely an error.",
            r"\ban an\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.tense.and_and",
            "'and and' is likely an error.",
            r"\band and\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.tense.of_of",
            "'of of' is likely an error.",
            r"\bof of\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.tense.to_to",
            "'to to' is likely an error.",
            r"\bto to\b",
        )
        .raw()
        .with_severity(Severity::Error),
        Check::new(
            "misc.tense.for_for",
            "'for for' is likely an error.",
            r"\bfor for\b",
        )
        .raw()
        .with_severity(Severity::Error),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_could_of() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "misc.could_of").unwrap();

        let results = check.run("I could of done better.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_alot() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "misc.alot").unwrap();

        let results = check.run("I have alot of work to do.");
        assert_eq!(results.len(), 1);
    }
}
