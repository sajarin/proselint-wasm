//! Common spelling/word confusion checks for proselint-wasm
//!
//! Detects commonly confused words and misspellings from:
//! - able_atable patterns
//! - able_ible patterns
//! - em_im_en_in patterns
//! - er_or patterns
//! - ally_ly patterns
//! - ance_ence patterns
//! - in_un patterns
//! - athlete names
//! - misc common misspellings

use crate::check::{Check, Severity};

/// Get all spelling checks
pub fn get_checks() -> Vec<Check> {
    let mut checks = vec![
        // Commonly confused words
        Check::new(
            "spelling.adverse_averse",
            "Check: 'adverse' means harmful; 'averse' means opposed to.",
            r"adverse to",
        )
        .with_severity(Severity::Warning)
        .with_replacement("averse to"),

        Check::new(
            "spelling.affect_effect",
            "Check: 'affect' is usually a verb; 'effect' is usually a noun.",
            r"the affect",
        )
        .with_severity(Severity::Warning)
        .with_replacement("the effect"),

        Check::new(
            "spelling.allude_elude",
            "Check: 'allude' means to reference; 'elude' means to escape.",
            r"elude to",
        )
        .with_severity(Severity::Warning)
        .with_replacement("allude to"),

        Check::new(
            "spelling.amoral_immoral",
            "Check: 'amoral' means without morals; 'immoral' means against morals.",
            r"ammoral",
        )
        .with_severity(Severity::Error)
        .with_replacement("amoral"),

        Check::new(
            "spelling.assure_ensure_insure",
            "'insure' is for insurance. Use 'ensure' for making certain.",
            r"insure that",
        )
        .with_severity(Severity::Warning)
        .with_replacement("ensure that"),

        Check::new(
            "spelling.complement_compliment",
            "'complement' means to complete; 'compliment' means praise.",
            r"compliment(?:s|ed|ing)? (?:each other|one another|the)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("complement"),

        Check::new(
            "spelling.discreet_discrete",
            "'discreet' means careful/prudent; 'discrete' means separate/distinct.",
            r"discrete(?:ly)? (?:ask|tell|mention|inquire|handle)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("discreet"),

        Check::new(
            "spelling.elicit_illicit",
            "'elicit' means to draw out; 'illicit' means illegal.",
            r"illicit (?:a response|information|feedback)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("elicit"),

        Check::new(
            "spelling.eminent_imminent",
            "'eminent' means distinguished; 'imminent' means about to happen.",
            r"eminent(?:ly)? (?:about to|approaching|soon|near)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("imminently"),

        Check::new(
            "spelling.flair_flare",
            "'flair' is a talent; 'flare' is a bright light or widening.",
            r"flare for",
        )
        .with_severity(Severity::Warning)
        .with_replacement("flair for"),

        Check::new(
            "spelling.flaunt_flout",
            "'flaunt' means to show off; 'flout' means to disobey.",
            r"flaunt the (?:rules|law|regulations)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("flout the"),

        Check::new(
            "spelling.gorilla_guerrilla",
            "'gorilla' is an ape; 'guerrilla' is a fighter.",
            r"gorilla (?:warfare|tactics|fighter|war)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("guerrilla"),

        Check::new(
            "spelling.lead_led",
            "'lead' (rhymes with 'bed') is a metal. Past tense of 'lead' (verb) is 'led'.",
            r"lead (?:him|her|them|us|me) (?:to|into|away)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "spelling.loose_lose",
            "'loose' means not tight; 'lose' means to misplace.",
            r"loose (?:the game|your|my|his|her|their|our)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("lose"),

        Check::new(
            "spelling.peak_peek_pique",
            "'peak' is a summit; 'peek' is a glance; 'pique' is to stimulate.",
            r"peek interest",
        )
        .with_severity(Severity::Error)
        .with_replacement("pique interest"),

        Check::new(
            "spelling.principal_principle",
            "'principal' means main/chief or school head; 'principle' is a rule/belief.",
            r"principal(?:s)? (?:of|that|behind)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("principle"),

        Check::new(
            "spelling.stationary_stationery",
            "'stationary' means not moving; 'stationery' is writing materials.",
            r"stationary (?:store|supplies|paper)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("stationery"),

        Check::new(
            "spelling.than_then",
            "Use 'than' for comparison; 'then' for time sequence.",
            r"(?:more|less|better|worse|bigger|smaller) then",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("than"),

        Check::new(
            "spelling.their_there_theyre",
            "Common confusion: 'their' (possessive), 'there' (place), 'they're' (they are).",
            r"their (?:is|are|was|were|going)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "spelling.whose_whos",
            "'whose' is possessive; 'who's' is 'who is' or 'who has'.",
            r"who's (?:book|car|house|idea|job)",
        )
        .raw()
        .with_severity(Severity::Warning)
        .with_replacement("whose"),

        // Common misspellings
        Check::new(
            "spelling.accomodate",
            "Misspelling: 'accomodate' should be 'accommodate'.",
            r"accomodat(?:e|ed|es|ing|ion)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("accommodate"),

        Check::new(
            "spelling.arguement",
            "Misspelling: 'arguement' should be 'argument'.",
            r"arguement",
        )
        .with_severity(Severity::Error)
        .with_replacement("argument"),

        Check::new(
            "spelling.beleive",
            "Misspelling: 'beleive' should be 'believe'.",
            r"beleiv(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("believe"),

        Check::new(
            "spelling.calender",
            "Misspelling: 'calender' should be 'calendar'.",
            r"calender",
        )
        .with_severity(Severity::Error)
        .with_replacement("calendar"),

        Check::new(
            "spelling.definately",
            "Misspelling: 'definately' should be 'definitely'.",
            r"definately",
        )
        .with_severity(Severity::Error)
        .with_replacement("definitely"),

        Check::new(
            "spelling.embarass",
            "Misspelling: 'embarass' should be 'embarrass'.",
            r"embarass(?:ed|es|ing|ment)?",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("embarrass"),

        Check::new(
            "spelling.occured",
            "Misspelling: 'occured' should be 'occurred'.",
            r"occured",
        )
        .with_severity(Severity::Error)
        .with_replacement("occurred"),

        Check::new(
            "spelling.recieve",
            "Misspelling: 'recieve' should be 'receive'.",
            r"reciev(?:e|ed|es|ing)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("receive"),

        Check::new(
            "spelling.seperate",
            "Misspelling: 'seperate' should be 'separate'.",
            r"seperat(?:e|ed|es|ing|ion|ly)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("separate"),

        Check::new(
            "spelling.wierd",
            "Misspelling: 'wierd' should be 'weird'.",
            r"wierd",
        )
        .with_severity(Severity::Error)
        .with_replacement("weird"),

        // More common misspellings
        Check::new(
            "spelling.neccessary",
            "Misspelling: should be 'necessary'.",
            r"necc?ess?ary",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("necessary"),

        Check::new(
            "spelling.concensus",
            "Misspelling: 'concensus' should be 'consensus'.",
            r"concensus",
        )
        .with_severity(Severity::Error)
        .with_replacement("consensus"),

        Check::new(
            "spelling.persue",
            "Misspelling: 'persue' should be 'pursue'.",
            r"persue",
        )
        .with_severity(Severity::Error)
        .with_replacement("pursue"),

        Check::new(
            "spelling.priviledge",
            "Misspelling: 'priviledge' should be 'privilege'.",
            r"priviledge",
        )
        .with_severity(Severity::Error)
        .with_replacement("privilege"),

        Check::new(
            "spelling.pronounciation",
            "Misspelling: 'pronounciation' should be 'pronunciation'.",
            r"pronounciation",
        )
        .with_severity(Severity::Error)
        .with_replacement("pronunciation"),

        Check::new(
            "spelling.publically",
            "Misspelling: 'publically' should be 'publicly'.",
            r"publically",
        )
        .with_severity(Severity::Error)
        .with_replacement("publicly"),

        Check::new(
            "spelling.questionaire",
            "Misspelling: 'questionaire' should be 'questionnaire'.",
            r"questionaire",
        )
        .with_severity(Severity::Error)
        .with_replacement("questionnaire"),

        Check::new(
            "spelling.refering",
            "Misspelling: 'refering' should be 'referring'.",
            r"refering",
        )
        .with_severity(Severity::Error)
        .with_replacement("referring"),

        Check::new(
            "spelling.succesful",
            "Misspelling: 'succesful' should be 'successful'.",
            r"succ?ess?ful",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("successful"),

        Check::new(
            "spelling.untill",
            "Misspelling: 'untill' should be 'until'.",
            r"untill",
        )
        .with_severity(Severity::Error)
        .with_replacement("until"),

        Check::new(
            "spelling.writting",
            "Misspelling: 'writting' should be 'writing'.",
            r"writting",
        )
        .with_severity(Severity::Error)
        .with_replacement("writing"),

        Check::new(
            "spelling.begining",
            "Misspelling: 'begining' should be 'beginning'.",
            r"begining",
        )
        .with_severity(Severity::Error)
        .with_replacement("beginning"),

        Check::new(
            "spelling.bizzare",
            "Misspelling: 'bizzare' should be 'bizarre'.",
            r"bizzare",
        )
        .with_severity(Severity::Error)
        .with_replacement("bizarre"),

        Check::new(
            "spelling.carribean",
            "Misspelling: 'Carribean' should be 'Caribbean'.",
            r"[Cc]arribean",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("Caribbean"),

        Check::new(
            "spelling.collegue",
            "Misspelling: 'collegue' should be 'colleague'.",
            r"collegue",
        )
        .with_severity(Severity::Error)
        .with_replacement("colleague"),

        Check::new(
            "spelling.commitee",
            "Misspelling: 'commitee' should be 'committee'.",
            r"committ?ee",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("committee"),

        Check::new(
            "spelling.concious",
            "Misspelling: 'concious' should be 'conscious'.",
            r"concious",
        )
        .with_severity(Severity::Error)
        .with_replacement("conscious"),

        Check::new(
            "spelling.enviroment",
            "Misspelling: 'enviroment' should be 'environment'.",
            r"enviroment",
        )
        .with_severity(Severity::Error)
        .with_replacement("environment"),

        Check::new(
            "spelling.existance",
            "Misspelling: 'existance' should be 'existence'.",
            r"existance",
        )
        .with_severity(Severity::Error)
        .with_replacement("existence"),

        Check::new(
            "spelling.experiance",
            "Misspelling: 'experiance' should be 'experience'.",
            r"experiance",
        )
        .with_severity(Severity::Error)
        .with_replacement("experience"),

        Check::new(
            "spelling.fiery",
            "Misspelling: 'firey' should be 'fiery'.",
            r"firey",
        )
        .with_severity(Severity::Error)
        .with_replacement("fiery"),

        Check::new(
            "spelling.foriegn",
            "Misspelling: 'foriegn' should be 'foreign'.",
            r"foriegn",
        )
        .with_severity(Severity::Error)
        .with_replacement("foreign"),

        Check::new(
            "spelling.fourty",
            "Misspelling: 'fourty' should be 'forty'.",
            r"fourty",
        )
        .with_severity(Severity::Error)
        .with_replacement("forty"),

        Check::new(
            "spelling.goverment",
            "Misspelling: 'goverment' should be 'government'.",
            r"goverment",
        )
        .with_severity(Severity::Error)
        .with_replacement("government"),

        Check::new(
            "spelling.grammer",
            "Misspelling: 'grammer' should be 'grammar'.",
            r"grammer",
        )
        .with_severity(Severity::Error)
        .with_replacement("grammar"),

        Check::new(
            "spelling.harrass",
            "Misspelling: 'harrass' should be 'harass'.",
            r"harrass",
        )
        .with_severity(Severity::Error)
        .with_replacement("harass"),

        Check::new(
            "spelling.heros",
            "Misspelling: 'heros' should be 'heroes'.",
            r"\bheros\b",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("heroes"),

        Check::new(
            "spelling.independant",
            "Misspelling: 'independant' should be 'independent'.",
            r"independant",
        )
        .with_severity(Severity::Error)
        .with_replacement("independent"),

        Check::new(
            "spelling.innoculate",
            "Misspelling: 'innoculate' should be 'inoculate'.",
            r"innoculate",
        )
        .with_severity(Severity::Error)
        .with_replacement("inoculate"),

        Check::new(
            "spelling.jewelery",
            "Misspelling: 'jewelery' should be 'jewelry' (US) or 'jewellery' (UK).",
            r"jewelery",
        )
        .with_severity(Severity::Error)
        .with_replacement("jewelry"),

        Check::new(
            "spelling.judgement_us",
            "In American English, 'judgement' is often 'judgment'.",
            r"judgement",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("judgment"),

        Check::new(
            "spelling.knowlege",
            "Misspelling: 'knowlege' should be 'knowledge'.",
            r"knowlege",
        )
        .with_severity(Severity::Error)
        .with_replacement("knowledge"),

        Check::new(
            "spelling.liason",
            "Misspelling: 'liason' should be 'liaison'.",
            r"liason",
        )
        .with_severity(Severity::Error)
        .with_replacement("liaison"),

        Check::new(
            "spelling.libary",
            "Misspelling: 'libary' should be 'library'.",
            r"libary",
        )
        .with_severity(Severity::Error)
        .with_replacement("library"),

        Check::new(
            "spelling.maintainance",
            "Misspelling: 'maintainance' should be 'maintenance'.",
            r"maintainance",
        )
        .with_severity(Severity::Error)
        .with_replacement("maintenance"),

        Check::new(
            "spelling.millenium",
            "Misspelling: 'millenium' should be 'millennium'.",
            r"millenium",
        )
        .with_severity(Severity::Error)
        .with_replacement("millennium"),

        Check::new(
            "spelling.mischievious",
            "Misspelling: 'mischievious' should be 'mischievous'.",
            r"mischievious",
        )
        .with_severity(Severity::Error)
        .with_replacement("mischievous"),

        Check::new(
            "spelling.noticable",
            "Misspelling: 'noticable' should be 'noticeable'.",
            r"noticable",
        )
        .with_severity(Severity::Error)
        .with_replacement("noticeable"),

        Check::new(
            "spelling.occassion",
            "Misspelling: 'occassion' should be 'occasion'.",
            r"occassion",
        )
        .with_severity(Severity::Error)
        .with_replacement("occasion"),

        Check::new(
            "spelling.pastime",
            "Misspelling: 'pasttime' should be 'pastime'.",
            r"pasttime",
        )
        .with_severity(Severity::Error)
        .with_replacement("pastime"),

        Check::new(
            "spelling.perseverance",
            "Misspelling: 'perseverence' should be 'perseverance'.",
            r"perseverence",
        )
        .with_severity(Severity::Error)
        .with_replacement("perseverance"),

        Check::new(
            "spelling.pharoah",
            "Misspelling: 'pharoah' should be 'pharaoh'.",
            r"pharoah",
        )
        .with_severity(Severity::Error)
        .with_replacement("pharaoh"),

        Check::new(
            "spelling.playwright",
            "Misspelling: 'playwrite' should be 'playwright'.",
            r"playwrite",
        )
        .with_severity(Severity::Error)
        .with_replacement("playwright"),

        Check::new(
            "spelling.posession",
            "Misspelling: 'posession' should be 'possession'.",
            r"posession",
        )
        .with_severity(Severity::Error)
        .with_replacement("possession"),

        Check::new(
            "spelling.potatos",
            "Misspelling: 'potatos' should be 'potatoes'.",
            r"\bpotatos\b",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("potatoes"),

        Check::new(
            "spelling.preceed",
            "Misspelling: 'preceed' should be 'precede'.",
            r"preceed",
        )
        .with_severity(Severity::Error)
        .with_replacement("precede"),

        Check::new(
            "spelling.recommend",
            "Misspelling: 'reccommend' should be 'recommend'.",
            r"reccommend",
        )
        .with_severity(Severity::Error)
        .with_replacement("recommend"),

        Check::new(
            "spelling.restaraunt",
            "Misspelling: 'restaraunt' should be 'restaurant'.",
            r"restaraunt",
        )
        .with_severity(Severity::Error)
        .with_replacement("restaurant"),

        Check::new(
            "spelling.rythm",
            "Misspelling: 'rythm' should be 'rhythm'.",
            r"rythm",
        )
        .with_severity(Severity::Error)
        .with_replacement("rhythm"),

        Check::new(
            "spelling.sieze",
            "Misspelling: 'sieze' should be 'seize'.",
            r"sieze",
        )
        .with_severity(Severity::Error)
        .with_replacement("seize"),

        Check::new(
            "spelling.suprise",
            "Misspelling: 'suprise' should be 'surprise'.",
            r"suprise",
        )
        .with_severity(Severity::Error)
        .with_replacement("surprise"),

        Check::new(
            "spelling.tomatos",
            "Misspelling: 'tomatos' should be 'tomatoes'.",
            r"\btomatos\b",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("tomatoes"),

        Check::new(
            "spelling.tommorow",
            "Misspelling: 'tommorow' should be 'tomorrow'.",
            r"tomm?orow",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("tomorrow"),

        Check::new(
            "spelling.tounge",
            "Misspelling: 'tounge' should be 'tongue'.",
            r"tounge",
        )
        .with_severity(Severity::Error)
        .with_replacement("tongue"),

        Check::new(
            "spelling.truely",
            "Misspelling: 'truely' should be 'truly'.",
            r"truely",
        )
        .with_severity(Severity::Error)
        .with_replacement("truly"),

        Check::new(
            "spelling.tyrany",
            "Misspelling: 'tyrany' should be 'tyranny'.",
            r"tyrany",
        )
        .with_severity(Severity::Error)
        .with_replacement("tyranny"),

        Check::new(
            "spelling.vaccuum",
            "Misspelling: 'vaccuum' should be 'vacuum'.",
            r"vaccuum",
        )
        .with_severity(Severity::Error)
        .with_replacement("vacuum"),

        Check::new(
            "spelling.vegeterian",
            "Misspelling: 'vegeterian' should be 'vegetarian'.",
            r"vegeterian",
        )
        .with_severity(Severity::Error)
        .with_replacement("vegetarian"),

        Check::new(
            "spelling.villian",
            "Misspelling: 'villian' should be 'villain'.",
            r"villian",
        )
        .with_severity(Severity::Error)
        .with_replacement("villain"),

        Check::new(
            "spelling.wether",
            "Misspelling or confusion: 'wether' (castrated sheep) is often 'whether' or 'weather'.",
            r"\bwether\b",
        )
        .raw()
        .with_severity(Severity::Warning),

        // ve/of confusions
        Check::new(
            "spelling.ve_of.must_of",
            "'must of' should be 'must have'.",
            r"must of",
        )
        .with_severity(Severity::Error)
        .with_replacement("must have"),

        Check::new(
            "spelling.ve_of.might_of",
            "'might of' should be 'might have'.",
            r"might of",
        )
        .with_severity(Severity::Error)
        .with_replacement("might have"),

        // Athlete names commonly misspelled
        Check::new(
            "spelling.athlete.jordan",
            "'Micheal Jordan' should be 'Michael Jordan'.",
            r"Micheal Jordan",
        )
        .with_severity(Severity::Error)
        .with_replacement("Michael Jordan"),
    ];

    // able_atable patterns - words where -atable is wrong, -able is correct
    let able_atable: &[(&str, &str)] = &[
        ("abbreviatable", "abbreviable"),
        ("abdicatable", "abdicable"),
        ("abominatable", "abominable"),
        ("accumulatable", "accumulable"),
        ("administratable", "administrable"),
        ("adulterable", "adulterable"),
        ("affiliatable", "affiliable"),
        ("alienatable", "alienable"),
        ("alleviatable", "alleviable"),
        ("allocatable", "allocable"),
        ("amelioratable", "ameliorable"),
        ("appreciatable", "appreciable"),
        ("appropriatable", "appropriable"),
        ("arbitratable", "arbitrable"),
        ("articulatable", "articulable"),
        ("calculatable", "calculable"),
        ("calibratable", "calibrable"),
        ("coagulatable", "coagulable"),
        ("communicatable", "communicable"),
        ("compensatable", "compensable"),
        ("confiscatable", "confiscable"),
        ("conglomeratable", "conglomerable"),
        ("conjugatable", "conjugable"),
        ("contaminatable", "contaminable"),
        ("cultivatable", "cultivable"),
        ("delegatable", "delegable"),
        ("demonstratable", "demonstrable"),
        ("deploratable", "deplorable"),
        ("depreciatable", "depreciable"),
        ("differentiatable", "differentiable"),
        ("discriminatable", "discriminable"),
        ("dissociatable", "dissociable"),
        ("educatable", "educable"),
        ("eliminatable", "eliminable"),
        ("enumeratable", "enumerable"),
        ("eradicatable", "eradicable"),
        ("estimatable", "estimable"),
        ("evaluatable", "evaluable"),
        ("evaporatable", "evaporable"),
        ("exaggeratable", "exaggerable"),
        ("execratable", "execrable"),
        ("expiratable", "expirable"),
        ("explicatable", "explicable"),
        ("exterminatable", "exterminable"),
        ("extradicatable", "extradicable"),
        ("fabricatable", "fabricable"),
        ("generatable", "generable"),
        ("graduatable", "graduable"),
        ("imitatable", "imitable"),
        ("inaboratable", "inelaborable"),
        ("incalculatable", "incalculable"),
        ("incommunicatable", "incommunicable"),
        ("inculcatable", "inculcable"),
        ("indeterminatable", "indeterminable"),
        ("ineducatable", "ineducable"),
        ("inestimatable", "inestimable"),
        ("inexplicatable", "inexplicable"),
        ("infatuatable", "infatuable"),
        ("innavigatable", "innavigable"),
        ("innumeratable", "innumerable"),
        ("inseparatable", "inseparable"),
        ("intimidatable", "intimidable"),
        ("investigatable", "investigable"),
        ("irrigatable", "irrigable"),
        ("iteratable", "iterable"),
        ("manipulatable", "manipulable"),
        ("medicatable", "medicable"),
        ("navigatable", "navigable"),
        ("negotiatable", "negotiable"),
        ("obligatable", "obligable"),
        ("obviatable", "obviable"),
        ("operatable", "operable"),
        ("originatable", "originable"),
        ("participatable", "participable"),
        ("penetratable", "penetrable"),
        ("permeatable", "permeable"),
        ("perpetuatable", "perpetuable"),
        ("predicatable", "predicable"),
        ("propagatable", "propagable"),
        ("regulatable", "regulable"),
        ("replicatable", "replicable"),
        ("saturatable", "saturable"),
        ("segregatable", "segregable"),
        ("separatable", "separable"),
        ("simulatable", "simulable"),
        ("speculatable", "speculable"),
        ("stimulatable", "stimulable"),
        ("subordinatable", "subordinable"),
        ("toleratable", "tolerable"),
        ("translatable", "translatable"),
        ("variatable", "variable"),
        ("vegetatable", "vegetable"),
        ("vindicatable", "vindicable"),
        ("violatable", "violable"),
        ("vitiatable", "vitiable"),
    ];

    for (i, (wrong, correct)) in able_atable.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.able_atable.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // able_ible patterns - words ending incorrectly with -able instead of -ible
    let able_ible: &[(&str, &str)] = &[
        ("accessable", "accessible"),
        ("addable", "addible"),
        ("admissable", "admissible"),
        ("audable", "audible"),
        ("collapsable", "collapsible"),
        ("collectable", "collectible"),
        ("combustable", "combustible"),
        ("compatable", "compatible"),
        ("comprehensable", "comprehensible"),
        ("compressable", "compressible"),
        ("contemptable", "contemptible"),
        ("controvertable", "controvertible"),
        ("convertable", "convertible"),
        ("corruptable", "corruptible"),
        ("credable", "credible"),
        ("deductable", "deductible"),
        ("defensable", "defensible"),
        ("destructable", "destructible"),
        ("digestable", "digestible"),
        ("divertable", "divertible"),
        ("divisable", "divisible"),
        ("edable", "edible"),
        ("eligable", "eligible"),
        ("exhaustable", "exhaustible"),
        ("expressable", "expressible"),
        ("extendable", "extendible"),
        ("fallable", "fallible"),
        ("feasable", "feasible"),
        ("flexible", "flexible"),
        ("forcable", "forcible"),
        ("gullable", "gullible"),
        ("horrble", "horrible"),
        ("illegable", "illegible"),
        ("imperceptable", "imperceptible"),
        ("implausable", "implausible"),
        ("impossable", "impossible"),
        ("impressable", "impressible"),
        ("inaccessable", "inaccessible"),
        ("inadmissable", "inadmissible"),
        ("inaudable", "inaudible"),
        ("incompatabile", "incompatible"),
        ("incomprehensable", "incomprehensible"),
        ("incorruptable", "incorruptible"),
        ("incredable", "incredible"),
        ("indefensable", "indefensible"),
        ("indestructable", "indestructible"),
        ("indigestable", "indigestible"),
        ("indivisable", "indivisible"),
        ("inedable", "inedible"),
        ("ineligable", "ineligible"),
        ("inexhaustable", "inexhaustible"),
        ("infallable", "infallible"),
        ("infeasable", "infeasible"),
        ("inflexable", "inflexible"),
        ("intangable", "intangible"),
        ("intelligable", "intelligible"),
        ("invertable", "invertible"),
        ("invisable", "invisible"),
        ("invincable", "invincible"),
        ("irascable", "irascible"),
        ("irrepressable", "irrepressible"),
        ("irresistable", "irresistible"),
        ("irresponsable", "irresponsible"),
        ("irreversable", "irreversible"),
        ("legable", "legible"),
        ("negligable", "negligible"),
        ("ostensable", "ostensible"),
        ("perceptable", "perceptible"),
        ("permissable", "permissible"),
        ("plausable", "plausible"),
        ("possable", "possible"),
        ("reducable", "reducible"),
        ("reprehensable", "reprehensible"),
        ("repressable", "repressible"),
        ("reproducable", "reproducible"),
        ("resistable", "resistible"),
        ("responsable", "responsible"),
        ("reversable", "reversible"),
        ("sensable", "sensible"),
        ("suggestable", "suggestible"),
        ("suppressable", "suppressible"),
        ("suspendable", "suspendible"),
        ("tangable", "tangible"),
        ("tensable", "tensible"),
        ("terrable", "terrible"),
        ("transmissable", "transmissible"),
        ("visable", "visible"),
    ];

    for (i, (wrong, correct)) in able_ible.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.able_ible.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // em_im_en_in patterns - prefix confusions
    let em_im_en_in: &[(&str, &str)] = &[
        ("imbalm", "embalm"),
        ("imbark", "embark"),
        ("imbattle", "embattle"),
        ("imbed", "embed"),
        ("imbitter", "embitter"),
        ("imblazon", "emblazon"),
        ("imbody", "embody"),
        ("imbolden", "embolden"),
        ("imboss", "emboss"),
        ("imbower", "embower"),
        ("imbrace", "embrace"),
        ("imbroider", "embroider"),
        ("imbroil", "embroil"),
        ("imbrown", "embrown"),
        ("imbue", "imbue"),
        ("immesh", "enmesh"),
        ("immerse", "immerse"),
        ("impassion", "impassion"),
        ("imperil", "imperil"),
        ("implant", "implant"),
        ("impower", "empower"),
        ("impress", "impress"),
        ("imprison", "imprison"),
        ("improper", "improper"),
        ("improve", "improve"),
        ("incase", "encase"),
        ("inchant", "enchant"),
        ("incircle", "encircle"),
        ("inclose", "enclose"),
        ("incompass", "encompass"),
        ("incourage", "encourage"),
        ("incroach", "encroach"),
        ("incrust", "encrust"),
        ("incumber", "encumber"),
        ("indanger", "endanger"),
        ("indear", "endear"),
        ("indeavor", "endeavor"),
        ("indorse", "endorse"),
        ("indow", "endow"),
        ("indure", "endure"),
        ("infeeble", "enfeeble"),
        ("infeoff", "enfeoff"),
        ("infold", "enfold"),
        ("inforce", "enforce"),
        ("infranchise", "enfranchise"),
        ("ingage", "engage"),
        ("ingender", "engender"),
        ("ingraft", "engraft"),
        ("ingrain", "ingrain"),
        ("ingrave", "engrave"),
        ("ingross", "engross"),
        ("ingulf", "engulf"),
        ("inhance", "enhance"),
        ("injoin", "enjoin"),
        ("inlarge", "enlarge"),
        ("inlist", "enlist"),
        ("inliven", "enliven"),
        ("inmesh", "enmesh"),
        ("innoble", "ennoble"),
        ("inrage", "enrage"),
        ("inrapture", "enrapture"),
        ("inrich", "enrich"),
        ("inrobe", "enrobe"),
        ("inroll", "enroll"),
        ("insconce", "ensconce"),
        ("inshrine", "enshrine"),
        ("inshroud", "enshroud"),
        ("inslave", "enslave"),
        ("insnare", "ensnare"),
        ("insue", "ensue"),
        ("insure", "ensure"),
        ("intail", "entail"),
        ("intangle", "entangle"),
        ("inthrall", "enthrall"),
        ("inthrone", "enthrone"),
        ("inthuse", "enthuse"),
        ("intice", "entice"),
        ("intitle", "entitle"),
        ("intomb", "entomb"),
        ("intrap", "entrap"),
        ("intreat", "entreat"),
        ("intrench", "entrench"),
        ("intrust", "entrust"),
        ("intwine", "entwine"),
        ("invelope", "envelope"),
        ("inviron", "environ"),
        ("inwrap", "enwrap"),
    ];

    for (i, (wrong, correct)) in em_im_en_in.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.em_im_en_in.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // er_or patterns - agent noun suffix confusions
    let er_or: &[(&str, &str)] = &[
        ("abducter", "abductor"),
        ("adaptator", "adapter"),
        ("collectioner", "collector"),
        ("conquerer", "conqueror"),
        ("contributer", "contributor"),
        ("conveyancer", "conveyor"),
        ("distributer", "distributor"),
        ("elevater", "elevator"),
        ("equiptment", "equipment"),
        ("executioner", "executor"),
        ("governmenter", "governor"),
        ("imposter", "impostor"),
        ("inheriter", "inheritor"),
        ("investorer", "investor"),
        ("legistlator", "legislator"),
        ("narrater", "narrator"),
        ("procrastonator", "procrastinator"),
        ("professer", "professor"),
        ("prosecuter", "prosecutor"),
        ("protester", "protestor"),
        ("providor", "provider"),
        ("sailour", "sailor"),
        ("sculpter", "sculptor"),
        ("spectater", "spectator"),
        ("supervisior", "supervisor"),
        ("surviver", "survivor"),
        ("translater", "translator"),
        ("vendour", "vendor"),
        ("visiter", "visitor"),
    ];

    for (i, (wrong, correct)) in er_or.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.er_or.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // ally_ly patterns - adverb suffix confusions
    let ally_ly: &[(&str, &str)] = &[
        ("accidentaly", "accidentally"),
        ("basicly", "basically"),
        ("dramaticly", "dramatically"),
        ("enthusiasticly", "enthusiastically"),
        ("fantasticly", "fantastically"),
        ("franticly", "frantically"),
        ("geneticly", "genetically"),
        ("historicly", "historically"),
        ("magicly", "magically"),
        ("politicly", "politically"),
        ("practicly", "practically"),
        ("realisticly", "realistically"),
        ("romanticly", "romantically"),
        ("sarcasticly", "sarcastically"),
        ("statisticly", "statistically"),
        ("symbolicly", "symbolically"),
        ("systematicly", "systematically"),
        ("tragicly", "tragically"),
    ];

    for (i, (wrong, correct)) in ally_ly.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.ally_ly.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // ance_ence patterns - suffix confusions
    let ance_ence: &[(&str, &str)] = &[
        ("absense", "absence"),
        ("acceptence", "acceptance"),
        ("acquaintence", "acquaintance"),
        ("admittence", "admittance"),
        ("allowence", "allowance"),
        ("ambulence", "ambulance"),
        ("appearence", "appearance"),
        ("assistense", "assistance"),
        ("attendence", "attendance"),
        ("balence", "balance"),
        ("brillance", "brilliance"),
        ("clearence", "clearance"),
        ("complianse", "compliance"),
        ("conferance", "conference"),
        ("confidance", "confidence"),
        ("conveniense", "convenience"),
        ("correspondance", "correspondence"),
        ("defience", "defiance"),
        ("dependance", "dependence"),
        ("differance", "difference"),
        ("distanse", "distance"),
        ("dominanse", "dominance"),
        ("eleganse", "elegance"),
        ("eloquanse", "eloquence"),
        ("eminanse", "eminence"),
        ("entrence", "entrance"),
        ("evidanse", "evidence"),
        ("excellance", "excellence"),
        ("existense", "existence"),
        ("experiense", "experience"),
        ("fragrence", "fragrance"),
        ("grievence", "grievance"),
        ("guidanse", "guidance"),
        ("ignorence", "ignorance"),
        ("importanse", "importance"),
        ("independanse", "independence"),
        ("influense", "influence"),
        ("inheritence", "inheritance"),
        ("innocense", "innocence"),
        ("instance", "instance"),
        ("insurence", "insurance"),
        ("intelligense", "intelligence"),
        ("interferance", "interference"),
        ("maintenence", "maintenance"),
        ("observence", "observance"),
        ("occurance", "occurrence"),
        ("patiance", "patience"),
        ("performanse", "performance"),
        ("permanance", "permanence"),
        ("perseverence", "perseverance"),
        ("persistanse", "persistence"),
        ("preferance", "preference"),
        ("presense", "presence"),
        ("prevelance", "prevalence"),
        ("prominanse", "prominence"),
        ("referencce", "reference"),
        ("relevense", "relevance"),
        ("reluctense", "reluctance"),
        ("remembrence", "remembrance"),
        ("remittence", "remittance"),
        ("repentense", "repentance"),
        ("residanse", "residence"),
        ("resistense", "resistance"),
        ("resonanse", "resonance"),
        ("sentense", "sentence"),
        ("sequense", "sequence"),
        ("significanse", "significance"),
        ("silense", "silence"),
        ("substanse", "substance"),
        ("surveillence", "surveillance"),
        ("temperence", "temperance"),
        ("tolerence", "tolerance"),
        ("transference", "transference"),
        ("turbulense", "turbulence"),
        ("vigilense", "vigilance"),
        ("violense", "violence"),
    ];

    for (i, (wrong, correct)) in ance_ence.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.ance_ence.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // in_un patterns - negative prefix confusions
    let in_un: &[(&str, &str)] = &[
        ("inaccustomed", "unaccustomed"),
        ("inacceptable", "unacceptable"),
        ("inachievable", "unachievable"),
        ("inaffected", "unaffected"),
        ("inalterable", "unalterable"),
        ("inanswerable", "unanswerable"),
        ("inarguable", "unarguable"),
        ("inavailable", "unavailable"),
        ("inavoidable", "unavoidable"),
        ("inbalanced", "unbalanced"),
        ("inbeatable", "unbeatable"),
        ("inbelievable", "unbelievable"),
        ("inbiased", "unbiased"),
        ("inbroken", "unbroken"),
        ("incapable", "incapable"),
        ("inchanged", "unchanged"),
        ("inclear", "unclear"),
        ("incomfortable", "uncomfortable"),
        ("incomplicated", "uncomplicated"),
        ("inconventional", "unconventional"),
        ("indecided", "undecided"),
        ("indeniable", "undeniable"),
        ("indisputed", "undisputed"),
        ("indoubted", "undoubted"),
        ("inequal", "unequal"),
        ("inessential", "unessential"),
        ("ineven", "uneven"),
        ("inexcusable", "inexcusable"),
        ("inexpected", "unexpected"),
        ("infair", "unfair"),
        ("infamiliar", "unfamiliar"),
        ("infashionable", "unfashionable"),
        ("infavorable", "unfavorable"),
        ("infit", "unfit"),
        ("inforgetable", "unforgettable"),
        ("inforgiven", "unforgiven"),
        ("infortunate", "unfortunate"),
        ("infounded", "unfounded"),
        ("infriendly", "unfriendly"),
        ("inhappy", "unhappy"),
        ("inhealthy", "unhealthy"),
        ("inhelpful", "unhelpful"),
        ("inhurt", "unhurt"),
        ("inimportant", "unimportant"),
        ("inimpressed", "unimpressed"),
        ("ininterested", "uninterested"),
        ("ininteresting", "uninteresting"),
        ("injust", "unjust"),
        ("inknown", "unknown"),
        ("inlawful", "unlawful"),
        ("inlikely", "unlikely"),
        ("inlimited", "unlimited"),
        ("inlucky", "unlucky"),
        ("innatural", "unnatural"),
        ("innecessary", "unnecessary"),
        ("innoticed", "unnoticed"),
        ("inofficial", "unofficial"),
        ("inpleasant", "unpleasant"),
        ("inpopular", "unpopular"),
        ("inpredictable", "unpredictable"),
        ("inprepared", "unprepared"),
        ("inproductive", "unproductive"),
        ("inprofessional", "unprofessional"),
        ("inqualified", "unqualified"),
        ("inreal", "unreal"),
        ("inrealistic", "unrealistic"),
        ("inreasonable", "unreasonable"),
        ("inrelated", "unrelated"),
        ("inreliable", "unreliable"),
        ("insafe", "unsafe"),
        ("insatisfactory", "unsatisfactory"),
        ("inselfish", "unselfish"),
        ("inskilled", "unskilled"),
        ("insolved", "unsolved"),
        ("instable", "unstable"),
        ("insteady", "unsteady"),
        ("insuccessful", "unsuccessful"),
        ("insure", "unsure"),
        ("intidy", "untidy"),
        ("intrue", "untrue"),
        ("inusual", "unusual"),
        ("inwanted", "unwanted"),
        ("inwelcome", "unwelcome"),
        ("inwilling", "unwilling"),
        ("inwise", "unwise"),
        ("inworthy", "unworthy"),
    ];

    for (i, (wrong, correct)) in in_un.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.in_un.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // Athlete names commonly misspelled
    let athletes: &[(&str, &str)] = &[
        ("Lebron James", "LeBron James"),
        ("Kobe Byrant", "Kobe Bryant"),
        ("Dwayne Wade", "Dwyane Wade"),
        ("Steph Curry", "Stephen Curry"),
        ("Shaquile O'Neal", "Shaquille O'Neal"),
        ("Shaquile O'Neil", "Shaquille O'Neal"),
        ("Shaq O'Neil", "Shaq O'Neal"),
        ("Kareem Abdul Jabbar", "Kareem Abdul-Jabbar"),
        ("Dikembe Mutumbo", "Dikembe Mutombo"),
        ("Hakeem Olajuwan", "Hakeem Olajuwon"),
        ("Scottie Pipin", "Scottie Pippen"),
        ("Dirk Nowinski", "Dirk Nowitzki"),
        ("Giannis Antetokoumpo", "Giannis Antetokounmpo"),
        ("Kristaps Porzingus", "Kristaps Porzingis"),
        ("Wayne Gretzsky", "Wayne Gretzky"),
        ("Sidney Crosbie", "Sidney Crosby"),
        ("Conner McDavid", "Connor McDavid"),
        ("Auston Mathews", "Auston Matthews"),
        ("Alexandar Ovechkin", "Alexander Ovechkin"),
        ("Patrick Mahommes", "Patrick Mahomes"),
        ("Tom Bradey", "Tom Brady"),
        ("Payton Manning", "Peyton Manning"),
        ("Bret Favre", "Brett Favre"),
        ("Joe Montanna", "Joe Montana"),
        ("Roger Stauback", "Roger Staubach"),
        ("Usian Bolt", "Usain Bolt"),
        ("Serena Willams", "Serena Williams"),
        ("Venus Willams", "Venus Williams"),
        ("Roger Fedrer", "Roger Federer"),
        ("Rafel Nadal", "Rafael Nadal"),
        ("Novak Djockovic", "Novak Djokovic"),
        ("Tiger Wood", "Tiger Woods"),
        ("Phil Mickelson", "Phil Mickelson"),
        ("Rory Mcilroy", "Rory McIlroy"),
        ("Mohammed Ali", "Muhammad Ali"),
        ("Muhammed Ali", "Muhammad Ali"),
        ("Floyd Mayweather Jr", "Floyd Mayweather Jr."),
        ("Manny Pacqiao", "Manny Pacquiao"),
        ("Messi Lionel", "Lionel Messi"),
        ("Christiano Ronaldo", "Cristiano Ronaldo"),
        ("Neymar Junior", "Neymar Jr."),
        ("Zlatan Ibrahimovich", "Zlatan Ibrahimovic"),
    ];

    for (i, (wrong, correct)) in athletes.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.athletes.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // Additional misc misspellings from proselint
    let misc_misspellings: &[(&str, &str)] = &[
        ("accidently", "accidentally"),
        ("accomadate", "accommodate"),
        ("acheive", "achieve"),
        ("acheiving", "achieving"),
        ("acknowlege", "acknowledge"),
        ("acquaintence", "acquaintance"),
        ("adquire", "acquire"),
        ("adress", "address"),
        ("advertisment", "advertisement"),
        ("agression", "aggression"),
        ("agressive", "aggressive"),
        ("alledge", "allege"),
        ("alledged", "alleged"),
        ("allegience", "allegiance"),
        ("allmost", "almost"),
        ("alot", "a lot"),
        ("amatuer", "amateur"),
        ("amature", "amateur"),
        ("anounce", "announce"),
        ("apparant", "apparent"),
        ("apparantly", "apparently"),
        ("aquire", "acquire"),
        ("aquit", "acquit"),
        ("artic", "arctic"),
        ("athiest", "atheist"),
        ("athelete", "athlete"),
        ("awfull", "awful"),
        ("basicly", "basically"),
        ("beatiful", "beautiful"),
        ("becuase", "because"),
        ("becomeing", "becoming"),
        ("begining", "beginning"),
        ("beleif", "belief"),
        ("beleive", "believe"),
        ("belive", "believe"),
        ("benificial", "beneficial"),
        ("benifit", "benefit"),
        ("beutiful", "beautiful"),
        ("buisness", "business"),
        ("calender", "calendar"),
        ("camoflage", "camouflage"),
        ("catagory", "category"),
        ("caugt", "caught"),
        ("cemetary", "cemetery"),
        ("changable", "changeable"),
        ("cheif", "chief"),
        ("cieling", "ceiling"),
        ("collegue", "colleague"),
        ("comming", "coming"),
        ("commited", "committed"),
        ("commitee", "committee"),
        ("committing", "committing"),
        ("comparision", "comparison"),
        ("competance", "competence"),
        ("compleatly", "completely"),
        ("concieve", "conceive"),
        ("conciousness", "consciousness"),
        ("congradulate", "congratulate"),
        ("consciencious", "conscientious"),
        ("contravercial", "controversial"),
        ("convienient", "convenient"),
        ("copywrite", "copyright"),
        ("critisize", "criticize"),
        ("criticise", "criticize"),
        ("curiousity", "curiosity"),
        ("curriculem", "curriculum"),
        ("decieve", "deceive"),
        ("definate", "definite"),
        ("definately", "definitely"),
        ("desireable", "desirable"),
        ("develope", "develop"),
        ("developement", "development"),
        ("diffrence", "difference"),
        ("dilema", "dilemma"),
        ("disapear", "disappear"),
        ("disapoint", "disappoint"),
        ("dissapoint", "disappoint"),
        ("dissappear", "disappear"),
        ("dissappoint", "disappoint"),
        ("drunkenness", "drunkenness"),
        ("dumbell", "dumbbell"),
        ("easely", "easily"),
        ("ecstacy", "ecstasy"),
        ("efficent", "efficient"),
        ("eigth", "eighth"),
        ("embarass", "embarrass"),
        ("embarassment", "embarrassment"),
        ("enviroment", "environment"),
        ("enviromental", "environmental"),
        ("equiptment", "equipment"),
        ("especialy", "especially"),
        ("exagerate", "exaggerate"),
        ("exagerrate", "exaggerate"),
        ("excede", "exceed"),
        ("excellant", "excellent"),
        ("excercise", "exercise"),
        ("exersize", "exercise"),
        ("existance", "existence"),
        ("experiance", "experience"),
        ("explaination", "explanation"),
        ("extremly", "extremely"),
        ("facinating", "fascinating"),
        ("familar", "familiar"),
        ("fatique", "fatigue"),
        ("Febuary", "February"),
        ("firey", "fiery"),
        ("flourescent", "fluorescent"),
        ("florescent", "fluorescent"),
        ("foriegn", "foreign"),
        ("forseeable", "foreseeable"),
        ("fourty", "forty"),
        ("freind", "friend"),
        ("fullfil", "fulfill"),
        ("gaurd", "guard"),
        ("generaly", "generally"),
        ("goverment", "government"),
        ("gratefull", "grateful"),
        ("greatful", "grateful"),
        ("guage", "gauge"),
        ("garantee", "guarantee"),
        ("guidence", "guidance"),
        ("happend", "happened"),
        ("harrass", "harass"),
        ("harrassment", "harassment"),
        ("heighth", "height"),
        ("heirarchy", "hierarchy"),
        ("heros", "heroes"),
        ("hopefuly", "hopefully"),
        ("humerous", "humorous"),
        ("hygene", "hygiene"),
        ("hypocrasy", "hypocrisy"),
        ("identicial", "identical"),
        ("ignorence", "ignorance"),
        ("imaginery", "imaginary"),
        ("imanent", "imminent"),
        ("imediately", "immediately"),
        ("immediatly", "immediately"),
        ("incidently", "incidentally"),
        ("independant", "independent"),
        ("indispensible", "indispensable"),
        ("innoculate", "inoculate"),
        ("inteligence", "intelligence"),
        ("intelligance", "intelligence"),
        ("intresting", "interesting"),
        ("interuption", "interruption"),
        ("irresistable", "irresistible"),
        ("irritible", "irritable"),
        ("isnt", "isn't"),
        ("jeapardy", "jeopardy"),
        ("judgement", "judgment"),
        ("knowlege", "knowledge"),
        ("lenght", "length"),
        ("liason", "liaison"),
        ("libary", "library"),
        ("liesure", "leisure"),
        ("lightening", "lightning"),
        ("likelyhood", "likelihood"),
        ("litterature", "literature"),
        ("lonliness", "loneliness"),
        ("maintainance", "maintenance"),
        ("maintenence", "maintenance"),
        ("managment", "management"),
        ("manuever", "maneuver"),
        ("marrage", "marriage"),
        ("mathmatics", "mathematics"),
        ("medeval", "medieval"),
        ("medeival", "medieval"),
        ("millenium", "millennium"),
        ("miniscule", "minuscule"),
        ("mischevious", "mischievous"),
        ("mispell", "misspell"),
        ("misterious", "mysterious"),
        ("momento", "memento"),
        ("monkies", "monkeys"),
        ("morgage", "mortgage"),
        ("naturaly", "naturally"),
        ("neccessary", "necessary"),
        ("necessery", "necessary"),
        ("neice", "niece"),
        ("nieghbor", "neighbor"),
        ("noticable", "noticeable"),
        ("occurence", "occurrence"),
        ("occured", "occurred"),
        ("occuring", "occurring"),
        ("omision", "omission"),
        ("oportunity", "opportunity"),
        ("oppurtunity", "opportunity"),
        ("orignal", "original"),
        ("outragous", "outrageous"),
        ("overrule", "overrule"),
        ("parliment", "parliament"),
        ("particulary", "particularly"),
        ("passtime", "pastime"),
        ("pavillion", "pavilion"),
        ("peacefull", "peaceful"),
        ("peice", "piece"),
        ("percieve", "perceive"),
        ("performence", "performance"),
        ("permissable", "permissible"),
        ("persistant", "persistent"),
        ("personnally", "personally"),
        ("persue", "pursue"),
        ("pharoah", "pharaoh"),
        ("phenominal", "phenomenal"),
        ("pigeon", "pigeon"),
        ("plagerism", "plagiarism"),
        ("playright", "playwright"),
        ("playwrite", "playwright"),
        ("pleasent", "pleasant"),
        ("potatoe", "potato"),
        ("potatos", "potatoes"),
        ("powerfull", "powerful"),
        ("preceed", "precede"),
        ("predjudice", "prejudice"),
        ("presance", "presence"),
        ("privelege", "privilege"),
        ("priviledge", "privilege"),
        ("probaly", "probably"),
        ("procede", "proceed"),
        ("professer", "professor"),
        ("programing", "programming"),
        ("pronounciation", "pronunciation"),
        ("propoganda", "propaganda"),
        ("protege", "protege"),
        ("protecter", "protector"),
        ("prufe", "proof"),
        ("publically", "publicly"),
        ("puchase", "purchase"),
        ("pursuade", "persuade"),
        ("questionaire", "questionnaire"),
        ("realy", "really"),
        ("reciept", "receipt"),
        ("recieve", "receive"),
        ("recieved", "received"),
        ("reccomend", "recommend"),
        ("recomend", "recommend"),
        ("reconize", "recognize"),
        ("refered", "referred"),
        ("referance", "reference"),
        ("refering", "referring"),
        ("religous", "religious"),
        ("reluctent", "reluctant"),
        ("remeber", "remember"),
        ("remembrence", "remembrance"),
        ("reoccur", "recur"),
        ("repitition", "repetition"),
        ("restarant", "restaurant"),
        ("restaraunt", "restaurant"),
        ("rythm", "rhythm"),
        ("sacrafice", "sacrifice"),
        ("saftey", "safety"),
        ("sandwhich", "sandwich"),
        ("sargeant", "sergeant"),
        ("sattelite", "satellite"),
        ("saught", "sought"),
        ("scarey", "scary"),
        ("scedule", "schedule"),
        ("sceince", "science"),
        ("seige", "siege"),
        ("sence", "sense"),
        ("sentance", "sentence"),
        ("seperate", "separate"),
        ("seperately", "separately"),
        ("sergent", "sergeant"),
        ("shineing", "shining"),
        ("sieze", "seize"),
        ("similer", "similar"),
        ("similiar", "similar"),
        ("sinceerly", "sincerely"),
        ("speach", "speech"),
        ("sponser", "sponsor"),
        ("strenght", "strength"),
        ("succede", "succeed"),
        ("succesful", "successful"),
        ("successfull", "successful"),
        ("sucess", "success"),
        ("suficient", "sufficient"),
        ("suggust", "suggest"),
        ("sumary", "summary"),
        ("superintendant", "superintendent"),
        ("supercede", "supersede"),
        ("suprise", "surprise"),
        ("surprize", "surprise"),
        ("surround", "surround"),
        ("surveilance", "surveillance"),
        ("survivied", "survived"),
        ("syncronize", "synchronize"),
        ("tecnical", "technical"),
        ("tendancy", "tendency"),
        ("therefor", "therefore"),
        ("threshhold", "threshold"),
        ("throught", "through"),
        ("tommorrow", "tomorrow"),
        ("tommorow", "tomorrow"),
        ("tomorow", "tomorrow"),
        ("tounge", "tongue"),
        ("truely", "truly"),
        ("twelth", "twelfth"),
        ("tyrany", "tyranny"),
        ("underate", "underrate"),
        ("unfortunatly", "unfortunately"),
        ("unneccessary", "unnecessary"),
        ("untill", "until"),
        ("upholstry", "upholstery"),
        ("usally", "usually"),
        ("useing", "using"),
        ("usefull", "useful"),
        ("vaccuum", "vacuum"),
        ("vegatable", "vegetable"),
        ("vehical", "vehicle"),
        ("visious", "vicious"),
        ("villian", "villain"),
        ("violance", "violence"),
        ("visable", "visible"),
        ("volumne", "volume"),
        ("volunter", "volunteer"),
        ("vulnerible", "vulnerable"),
        ("Wedensday", "Wednesday"),
        ("Wensday", "Wednesday"),
        ("whereever", "wherever"),
        ("wierd", "weird"),
        ("wieght", "weight"),
        ("wich", "which"),
        ("withdrawl", "withdrawal"),
        ("wonderfull", "wonderful"),
        ("wressel", "wrestle"),
        ("writting", "writing"),
        ("yeild", "yield"),
        ("yatch", "yacht"),
    ];

    for (i, (wrong, correct)) in misc_misspellings.iter().enumerate() {
        checks.push(
            Check::new(
                Box::leak(format!("spelling.misc.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' should be '{}'.", wrong, correct).into_boxed_str()),
                wrong,
            )
            .with_severity(Severity::Error)
            .with_replacement(correct),
        );
    }

    // US/UK spelling consistency checks - flags mixing of American and British spellings
    let us_uk_pairs: &[(&str, &str, &str)] = &[
        // (US spelling, UK spelling, word root)
        ("color", "colour", "color/colour"),
        ("favor", "favour", "favor/favour"),
        ("honor", "honour", "honor/honour"),
        ("labor", "labour", "labor/labour"),
        ("neighbor", "neighbour", "neighbor/neighbour"),
        ("behavior", "behaviour", "behavior/behaviour"),
        ("organize", "organise", "organize/organise"),
        ("realize", "realise", "realize/realise"),
        ("recognize", "recognise", "recognize/recognise"),
        ("analyze", "analyse", "analyze/analyse"),
        ("center", "centre", "center/centre"),
        ("theater", "theatre", "theater/theatre"),
        ("defense", "defence", "defense/defence"),
        ("offense", "offence", "offense/offence"),
        ("license", "licence", "license/licence"),
        ("practice", "practise", "practice/practise"),
        ("catalog", "catalogue", "catalog/catalogue"),
        ("dialog", "dialogue", "dialog/dialogue"),
        ("traveling", "travelling", "traveling/travelling"),
        ("canceled", "cancelled", "canceled/cancelled"),
        ("jewelry", "jewellery", "jewelry/jewellery"),
        ("gray", "grey", "gray/grey"),
        ("aging", "ageing", "aging/ageing"),
        ("judgment", "judgement", "judgment/judgement"),
        ("acknowledgment", "acknowledgement", "acknowledgment/acknowledgement"),
        ("fulfill", "fulfil", "fulfill/fulfil"),
        ("skillful", "skilful", "skillful/skilful"),
        ("enrollment", "enrolment", "enrollment/enrolment"),
    ];

    for (i, (us, uk, _root)) in us_uk_pairs.iter().enumerate() {
        // Add check for US spelling
        checks.push(
            Check::new(
                Box::leak(format!("spelling.consistency.us.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' is American English. Ensure consistent spelling style.", us).into_boxed_str()),
                us,
            )
            .with_severity(Severity::Suggestion),
        );
        // Add check for UK spelling
        checks.push(
            Check::new(
                Box::leak(format!("spelling.consistency.uk.{}", i).into_boxed_str()),
                Box::leak(format!("'{}' is British English. Ensure consistent spelling style.", uk).into_boxed_str()),
                uk,
            )
            .with_severity(Severity::Suggestion),
        );
    }

    checks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spelling() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "spelling.definately").unwrap();
        let results = check.run("I definately agree.");
        assert_eq!(results.len(), 1);
    }
}
