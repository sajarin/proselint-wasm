//! Social awareness checks for proselint-wasm
//!
//! Detects potentially insensitive or outdated terminology.

use crate::check::{Check, Severity};

/// Get all social awareness checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Gender-inclusive language
        Check::new(
            "social_awareness.mankind",
            "'mankind' can be replaced with 'humanity', 'humankind', or 'people'.",
            r"mankind",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("humankind"),

        Check::new(
            "social_awareness.manmade",
            "'man-made' or 'manmade' can be 'artificial', 'synthetic', or 'human-made'.",
            r"man[- ]?made",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("human-made"),

        Check::new(
            "social_awareness.manpower",
            "'manpower' can be 'workforce', 'personnel', or 'staff'.",
            r"manpower",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("workforce"),

        Check::new(
            "social_awareness.chairman",
            "'chairman' can be 'chair', 'chairperson', or 'head'.",
            r"chairman",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("chairperson"),

        Check::new(
            "social_awareness.fireman",
            "'fireman' should be 'firefighter'.",
            r"fireman|firemen",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("firefighter"),

        Check::new(
            "social_awareness.policeman",
            "'policeman' should be 'police officer'.",
            r"policeman|policemen",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("police officer"),

        Check::new(
            "social_awareness.stewardess",
            "'stewardess' should be 'flight attendant'.",
            r"stewardess(?:es)?",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("flight attendant"),

        Check::new(
            "social_awareness.mailman",
            "'mailman' should be 'mail carrier' or 'postal worker'.",
            r"mailman|mailmen",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("mail carrier"),

        Check::new(
            "social_awareness.congressman",
            "'congressman' can be 'member of Congress' or 'representative'.",
            r"congressman|congressmen",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("member of Congress"),

        Check::new(
            "social_awareness.spokesman",
            "'spokesman' should be 'spokesperson'.",
            r"spokesman|spokesmen",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("spokesperson"),

        Check::new(
            "social_awareness.businessman",
            "'businessman' can be 'businessperson' or 'executive'.",
            r"businessman|businessmen",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("businessperson"),

        // Disability language
        Check::new(
            "social_awareness.handicapped",
            "'handicapped' is outdated. Use 'disabled' or 'person with a disability'.",
            r"(?:the )?handicapped",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("disabled"),

        Check::new(
            "social_awareness.crippled",
            "'crippled' is offensive. Use 'disabled' or describe the specific condition.",
            r"crippled",
        )
        .with_severity(Severity::Warning)
        .with_replacement("disabled"),

        Check::new(
            "social_awareness.wheelchair_bound",
            "'wheelchair-bound' is limiting. Use 'wheelchair user' or 'uses a wheelchair'.",
            r"wheelchair[- ]bound",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("wheelchair user"),

        Check::new(
            "social_awareness.suffers_from",
            "'suffers from' can be 'has' or 'lives with'.",
            r"suffers? from",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "social_awareness.victim_of",
            "'victim of [disease]' can be 'person with [disease]' or 'diagnosed with'.",
            r"victim of (?:cancer|diabetes|AIDS|HIV|autism|depression)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "social_awareness.mentally_retarded",
            "'mentally retarded' is offensive and outdated. Use 'intellectual disability'.",
            r"mentally retarded|retarded",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("intellectually disabled"),

        Check::new(
            "social_awareness.deaf_and_dumb",
            "'deaf and dumb' is offensive. Use 'deaf' or 'Deaf'.",
            r"deaf and dumb|deaf-mute",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("deaf"),

        Check::new(
            "social_awareness.the_blind",
            "'the blind' should be 'blind people' or 'people who are blind'.",
            r"the blind\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("blind people"),

        Check::new(
            "social_awareness.the_deaf",
            "'the deaf' should be 'deaf people' or 'Deaf community'.",
            r"the deaf\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("deaf people"),

        // Age-related
        Check::new(
            "social_awareness.elderly",
            "'elderly' can be 'older adults' or 'seniors'.",
            r"the elderly",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("older adults"),

        Check::new(
            "social_awareness.senior_citizen",
            "'senior citizen' can be 'older adult' or 'senior'.",
            r"senior citizens?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // LGBTQ+ terminology
        Check::new(
            "social_awareness.homosexual",
            "'homosexual' is clinical. Consider 'gay' or 'lesbian' for people.",
            r"homosexuals?\b",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "social_awareness.sexual_preference",
            "'sexual preference' implies choice. Use 'sexual orientation'.",
            r"sexual preference",
        )
        .with_severity(Severity::Warning)
        .with_replacement("sexual orientation"),

        Check::new(
            "social_awareness.transgendered",
            "'transgendered' is incorrect. Use 'transgender' (no -ed).",
            r"transgendered",
        )
        .with_severity(Severity::Warning)
        .with_replacement("transgender"),

        Check::new(
            "social_awareness.transsexual",
            "'transsexual' is outdated. Use 'transgender' unless specifically requested.",
            r"transsexual",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("transgender"),

        Check::new(
            "social_awareness.preferred_pronouns",
            "'preferred pronouns' implies choice. Use simply 'pronouns'.",
            r"preferred pronouns?",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("pronouns"),

        // Other inclusive language
        Check::new(
            "social_awareness.third_world",
            "'Third World' is outdated. Use 'developing countries' or specific regions.",
            r"third[- ]world",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("developing countries"),

        Check::new(
            "social_awareness.illegal_alien",
            "'illegal alien' is dehumanizing. Use 'undocumented immigrant'.",
            r"illegal aliens?",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("undocumented immigrant"),

        Check::new(
            "social_awareness.illegal_immigrant",
            "'illegal immigrant' is problematic. Consider 'undocumented immigrant'.",
            r"illegal immigrants?",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("undocumented immigrant"),

        Check::new(
            "social_awareness.eskimo",
            "'Eskimo' can be considered offensive. Use 'Inuit' or the specific group name.",
            r"eskimo",
        )
        .with_severity(Severity::Warning)
        .with_replacement("Inuit"),

        Check::new(
            "social_awareness.oriental",
            "'Oriental' for people is offensive. Use 'Asian' or the specific ethnicity.",
            r"oriental(?:\s+(?:person|people|man|woman|men|women))?",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("Asian"),

        Check::new(
            "social_awareness.colored",
            "'colored' is offensive and outdated.",
            r"colored (?:person|people|man|woman|men|women|folk)",
        )
        .raw()
        .with_severity(Severity::Error),

        Check::new(
            "social_awareness.master_slave",
            "'master/slave' terminology can be replaced with 'primary/replica' or 'leader/follower'.",
            r"master[/-]slave",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("primary/replica"),

        Check::new(
            "social_awareness.blacklist_whitelist",
            "'blacklist/whitelist' can be 'blocklist/allowlist' or 'denylist/allowlist'.",
            r"(?:black|white)list(?:ed|ing|s)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // === Additional sexism patterns (from proselint) ===
        Check::new("social_awareness.sexism.anchorman", "'anchorman' should be 'anchor'.", r"anchorman").with_severity(Severity::Suggestion).with_replacement("anchor"),
        Check::new("social_awareness.sexism.anchorwoman", "'anchorwoman' should be 'anchor'.", r"anchorwoman").with_severity(Severity::Suggestion).with_replacement("anchor"),
        Check::new("social_awareness.sexism.draftman", "'draftman' should be 'drafter'.", r"draftman").with_severity(Severity::Suggestion).with_replacement("drafter"),
        Check::new("social_awareness.sexism.ombudsman", "'ombudsman' should be 'ombuds'.", r"ombudsman").with_severity(Severity::Suggestion).with_replacement("ombuds"),
        Check::new("social_awareness.sexism.tribesman", "'tribesman' should be 'tribe member'.", r"tribes(?:man|men|woman|women)").raw().with_severity(Severity::Suggestion).with_replacement("tribe member"),
        Check::new("social_awareness.sexism.policewoman", "'policewoman' should be 'police officer'.", r"policewoman|policewomen").raw().with_severity(Severity::Suggestion).with_replacement("police officer"),
        Check::new("social_awareness.sexism.firewoman", "'firewoman' should be 'firefighter'.", r"firewoman|firewomen").raw().with_severity(Severity::Warning).with_replacement("firefighter"),
        Check::new("social_awareness.sexism.mailwoman", "'mailwoman' should be 'mail carrier'.", r"mailwoman|mailwomen").raw().with_severity(Severity::Suggestion).with_replacement("mail carrier"),
        Check::new("social_awareness.sexism.poetess", "'poetess' should be 'poet'.", r"poetess(?:es)?").raw().with_severity(Severity::Warning).with_replacement("poet"),
        Check::new("social_awareness.sexism.authoress", "'authoress' should be 'author'.", r"authoress(?:es)?").raw().with_severity(Severity::Warning).with_replacement("author"),
        Check::new("social_awareness.sexism.waitress", "'waitress' can be 'server' or 'waiter'.", r"waitress(?:es)?").raw().with_severity(Severity::Suggestion).with_replacement("server"),
        Check::new("social_awareness.sexism.comedienne", "'comedienne' should be 'comedian'.", r"comedienne").with_severity(Severity::Suggestion).with_replacement("comedian"),
        Check::new("social_awareness.sexism.executrix", "'executrix' should be 'executor'.", r"executrix").with_severity(Severity::Warning).with_replacement("executor"),
        Check::new("social_awareness.sexism.prosecutrix", "'prosecutrix' should be 'prosecutor'.", r"prosecutrix").with_severity(Severity::Warning).with_replacement("prosecutor"),
        Check::new("social_awareness.sexism.testatrix", "'testatrix' should be 'testator'.", r"testatrix").with_severity(Severity::Warning).with_replacement("testator"),
        Check::new("social_awareness.sexism.lady_lawyer", "'lady lawyer' should just be 'lawyer'.", r"lady lawyer").with_severity(Severity::Warning).with_replacement("lawyer"),
        Check::new("social_awareness.sexism.woman_doctor", "'woman doctor' should just be 'doctor'.", r"woman doctor").with_severity(Severity::Warning).with_replacement("doctor"),
        Check::new("social_awareness.sexism.woman_scientist", "'woman scientist' should just be 'scientist'.", r"women? scientists?").raw().with_severity(Severity::Suggestion).with_replacement("scientist"),
        Check::new("social_awareness.sexism.man_and_wife", "'man and wife' should be 'husband and wife'.", r"man and wife").with_severity(Severity::Warning).with_replacement("husband and wife"),
        Check::new("social_awareness.sexism.men_and_girls", "'men and girls' should be 'men and women'.", r"men and girls").with_severity(Severity::Warning).with_replacement("men and women"),
        Check::new("social_awareness.sexism.herstory", "'herstory' - use 'history' (the word is not gendered).", r"herstory").with_severity(Severity::Suggestion).with_replacement("history"),
        Check::new("social_awareness.sexism.womyn", "'womyn' - use 'women'.", r"womyn").with_severity(Severity::Suggestion).with_replacement("women"),
        Check::new("social_awareness.sexism.confidante", "'confidante' - use 'confidant' (gender-neutral).", r"confidante").with_severity(Severity::Suggestion).with_replacement("confidant"),

        // === LGBTQ patterns (from proselint) ===
        Check::new(
            "social_awareness.lgbtq.homosexual_man",
            "'homosexual man' - consider 'gay man'.",
            r"homosexual man",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("gay man"),

        Check::new(
            "social_awareness.lgbtq.homosexual_woman",
            "'homosexual woman' - consider 'lesbian'.",
            r"homosexual woman",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("lesbian"),

        Check::new(
            "social_awareness.lgbtq.admitted_homosexual",
            "'admitted homosexual' - consider 'openly gay'.",
            r"admitted homosexual",
        )
        .with_severity(Severity::Warning)
        .with_replacement("openly gay"),

        Check::new(
            "social_awareness.lgbtq.avowed_homosexual",
            "'avowed homosexual' - consider 'openly gay'.",
            r"avowed homosexual",
        )
        .with_severity(Severity::Warning)
        .with_replacement("openly gay"),

        Check::new(
            "social_awareness.lgbtq.special_rights",
            "'special rights' (for LGBTQ+) - consider 'equal rights'.",
            r"special rights",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "social_awareness.lgbtq.homosexual_lifestyle",
            "'homosexual lifestyle' / 'gay lifestyle' is problematic phrasing.",
            r"(?:homosexual|gay) lifestyle",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "social_awareness.lgbtq.transvestite",
            "'transvestite' is outdated. Consider context-appropriate alternatives.",
            r"transvestite",
        )
        .with_severity(Severity::Warning),

        // N-word handling (from proselint)
        Check::new(
            "social_awareness.nword",
            "Take responsibility for the words you want to say.",
            r"the n-?word",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mankind() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "social_awareness.mankind")
            .unwrap();
        let results = check.run("This is a great achievement for mankind.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_fireman() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "social_awareness.fireman")
            .unwrap();
        let results = check.run("The fireman saved the cat.");
        assert_eq!(results.len(), 1);
    }
}
