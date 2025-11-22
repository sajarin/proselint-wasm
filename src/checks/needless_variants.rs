//! Needless variant checks for proselint-wasm
//!
//! Detects unnecessary variant spellings and forms.

use crate::check::{Check, Severity};

/// Get all needless variant checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // British vs American variants (suggesting consistency)
        Check::new(
            "needless_variants.towards",
            "'towards' can be 'toward' in American English.",
            r"towards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("toward"),

        Check::new(
            "needless_variants.afterwards",
            "'afterwards' can be 'afterward' in American English.",
            r"afterwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("afterward"),

        Check::new(
            "needless_variants.backwards",
            "'backwards' can be 'backward' in American English.",
            r"backwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("backward"),

        Check::new(
            "needless_variants.forwards",
            "'forwards' can be 'forward' in American English.",
            r"forwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("forward"),

        Check::new(
            "needless_variants.upwards",
            "'upwards' can be 'upward' in American English.",
            r"upwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("upward"),

        Check::new(
            "needless_variants.downwards",
            "'downwards' can be 'downward' in American English.",
            r"downwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("downward"),

        Check::new(
            "needless_variants.inwards",
            "'inwards' can be 'inward' in American English.",
            r"inwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("inward"),

        Check::new(
            "needless_variants.outwards",
            "'outwards' can be 'outward' in American English.",
            r"outwards\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("outward"),

        // Needless variant forms
        Check::new(
            "needless_variants.amidst",
            "'amidst' is a needless variant. Use 'amid'.",
            r"amidst",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("amid"),

        Check::new(
            "needless_variants.amongst",
            "'amongst' is a needless variant. Use 'among'.",
            r"amongst",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("among"),

        Check::new(
            "needless_variants.whilst",
            "'whilst' is a needless variant. Use 'while'.",
            r"whilst",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("while"),

        Check::new(
            "needless_variants.unbeknownst",
            "'unbeknownst' is a needless variant. Use 'unbeknown' or 'unknown'.",
            r"unbeknownst",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("unbeknown"),

        Check::new(
            "needless_variants.preventative",
            "'preventative' is a needless variant. Use 'preventive'.",
            r"preventative",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("preventive"),

        Check::new(
            "needless_variants.orientated",
            "'orientated' is a needless variant. Use 'oriented'.",
            r"orientated",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("oriented"),

        Check::new(
            "needless_variants.transportation",
            "'transportation' can often be 'transport'.",
            r"transportation",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "needless_variants.firstly",
            "'firstly' is a needless variant. Use 'first'.",
            r"firstly",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("first"),

        Check::new(
            "needless_variants.secondly",
            "'secondly' is a needless variant. Use 'second'.",
            r"secondly",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("second"),

        Check::new(
            "needless_variants.thirdly",
            "'thirdly' is a needless variant. Use 'third'.",
            r"thirdly",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("third"),

        Check::new(
            "needless_variants.lastly",
            "'lastly' can often be 'last' or 'finally'.",
            r"lastly",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("finally"),

        Check::new(
            "needless_variants.till",
            "'till' is informal. Use 'until'.",
            r"\btill\b",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("until"),

        Check::new(
            "needless_variants.alright",
            "'alright' is nonstandard. Use 'all right'.",
            r"alright",
        )
        .with_severity(Severity::Warning)
        .with_replacement("all right"),

        Check::new(
            "needless_variants.anymore",
            "'anymore' (one word) is nonstandard in some uses. Consider 'any more'.",
            r"anymore",
        )
        .with_severity(Severity::Suggestion),

        Check::new(
            "needless_variants.anytime",
            "'anytime' as one word is informal. Consider 'any time'.",
            r"anytime",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("any time"),

        Check::new(
            "needless_variants.anyways",
            "'anyways' is nonstandard. Use 'anyway'.",
            r"anyways",
        )
        .with_severity(Severity::Warning)
        .with_replacement("anyway"),

        Check::new(
            "needless_variants.irregardless",
            "'irregardless' is nonstandard. Use 'regardless'.",
            r"irregardless",
        )
        .with_severity(Severity::Error)
        .with_replacement("regardless"),

        Check::new(
            "needless_variants.oftentimes",
            "'oftentimes' is a needless variant. Use 'often'.",
            r"oftentimes",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("often"),

        Check::new(
            "needless_variants.utilize",
            "'utilize' is often unnecessarily formal. Consider 'use'.",
            r"utiliz(?:e|ed|es|ing|ation)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "needless_variants.methodology",
            "'methodology' is often unnecessarily formal. Consider 'method'.",
            r"methodology",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("method"),

        Check::new(
            "needless_variants.functionality",
            "'functionality' is often unnecessarily formal. Consider 'function' or 'features'.",
            r"functionality",
        )
        .with_severity(Severity::Suggestion),

        // === Additional needless variants from proselint ===
        Check::new("needless_variants.abhorrment", "'abhorrment' is a needless variant. Use 'abhorrence'.", r"abhorrment").with_severity(Severity::Warning).with_replacement("abhorrence"),
        Check::new("needless_variants.absorptiveness", "'absorptiveness' is a needless variant. Use 'absorptivity'.", r"absorptiveness").with_severity(Severity::Warning).with_replacement("absorptivity"),
        Check::new("needless_variants.abstruseness", "'abstruseness' is a needless variant. Use 'abstrusity'.", r"abstruseness").with_severity(Severity::Warning).with_replacement("abstrusity"),
        Check::new("needless_variants.adequateness", "'adequateness' is a needless variant. Use 'adequacy'.", r"adequateness").with_severity(Severity::Warning).with_replacement("adequacy"),
        Check::new("needless_variants.administrant", "'administrant' is a needless variant. Use 'administrator'.", r"administrant").with_severity(Severity::Warning).with_replacement("administrator"),
        Check::new("needless_variants.affectation", "'affectation' may be confused with 'affection'. Verify usage.", r"affectation").with_severity(Severity::Suggestion),
        Check::new("needless_variants.agreeability", "'agreeability' is a needless variant. Use 'agreeableness'.", r"agreeability").with_severity(Severity::Warning).with_replacement("agreeableness"),
        Check::new("needless_variants.allurement", "'allurement' is a needless variant. Use 'allure'.", r"allurement").with_severity(Severity::Suggestion).with_replacement("allure"),
        Check::new("needless_variants.analysation", "'analysation' is a needless variant. Use 'analysis'.", r"analysation").with_severity(Severity::Error).with_replacement("analysis"),
        Check::new("needless_variants.annihilative", "'annihilative' is a needless variant. Use 'annihilating'.", r"annihilative").with_severity(Severity::Warning).with_replacement("annihilating"),
        Check::new("needless_variants.antiquated", "'antiquated' may be 'antique' in some contexts.", r"antiquated").with_severity(Severity::Suggestion),
        Check::new("needless_variants.appellate", "'appellate' is often misused. Ensure correct legal context.", r"appellate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.argufy", "'argufy' is a needless variant. Use 'argue'.", r"argufy").with_severity(Severity::Warning).with_replacement("argue"),
        Check::new("needless_variants.assertative", "'assertative' is a needless variant. Use 'assertive'.", r"assertative").with_severity(Severity::Warning).with_replacement("assertive"),
        Check::new("needless_variants.assumptive", "'assumptive' is a needless variant. Use 'assuming'.", r"assumptive").with_severity(Severity::Suggestion).with_replacement("assuming"),
        Check::new("needless_variants.authoress", "'authoress' is a needless variant. Use 'author'.", r"authoress").with_severity(Severity::Warning).with_replacement("author"),
        Check::new("needless_variants.beastliness", "'beastliness' is a needless variant. Use 'brutality' or 'bestiality'.", r"beastliness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.bemusement", "'bemusement' is a needless variant. Use 'bewilderment' or 'confusion'.", r"bemusement").with_severity(Severity::Suggestion).with_replacement("bewilderment"),
        Check::new("needless_variants.beneficent", "'beneficent' may be confused with 'beneficial'. Verify usage.", r"beneficent").with_severity(Severity::Suggestion),
        Check::new("needless_variants.brutalize", "'brutalize' can mean 'make brutal' or 'treat brutally'. Ensure clarity.", r"brutalize").with_severity(Severity::Suggestion),
        Check::new("needless_variants.burglarize", "'burglarize' is informal. Use 'burgle' (UK) or keep 'burglarize' (US).", r"burglarize").with_severity(Severity::Suggestion),
        Check::new("needless_variants.camaraderie", "'camaraderie' may be confused with 'comradeship'. Both are valid.", r"camaraderie").with_severity(Severity::Suggestion),
        Check::new("needless_variants.candidness", "'candidness' is a needless variant. Use 'candor'.", r"candidness").with_severity(Severity::Warning).with_replacement("candor"),
        Check::new("needless_variants.capacitance", "'capacitance' and 'capacity' are different. Verify technical usage.", r"capacitance").with_severity(Severity::Suggestion),
        Check::new("needless_variants.ceremonialize", "'ceremonialize' is a needless variant. Use 'ceremonize' or 'solemnize'.", r"ceremonialize").with_severity(Severity::Warning),
        Check::new("needless_variants.cognoscente", "'cognoscente' (singular) or 'cognoscenti' (plural). Verify number.", r"cognoscente").with_severity(Severity::Suggestion),
        Check::new("needless_variants.combativeness", "'combativeness' is a needless variant. Use 'combativity'.", r"combativeness").with_severity(Severity::Suggestion).with_replacement("combativity"),
        Check::new("needless_variants.commentate", "'commentate' is a needless back-formation. Use 'comment'.", r"commentate").with_severity(Severity::Warning).with_replacement("comment"),
        Check::new("needless_variants.compendious", "'compendious' is a needless variant. Use 'comprehensive' or 'concise'.", r"compendious").with_severity(Severity::Suggestion),
        Check::new("needless_variants.completive", "'completive' is a needless variant. Use 'completing'.", r"completive").with_severity(Severity::Warning).with_replacement("completing"),
        Check::new("needless_variants.concertize", "'concertize' is a needless variant. Use 'perform' or 'give concerts'.", r"concertize").with_severity(Severity::Suggestion),
        Check::new("needless_variants.conflictual", "'conflictual' is a needless variant. Use 'conflicting'.", r"conflictual").with_severity(Severity::Warning).with_replacement("conflicting"),
        Check::new("needless_variants.connexion", "'connexion' is a needless variant. Use 'connection'.", r"connexion").with_severity(Severity::Suggestion).with_replacement("connection"),
        Check::new("needless_variants.consolement", "'consolement' is a needless variant. Use 'consolation'.", r"consolement").with_severity(Severity::Warning).with_replacement("consolation"),
        Check::new("needless_variants.consultancy", "'consultancy' can be 'consulting' in some contexts.", r"consultancy").with_severity(Severity::Suggestion),
        Check::new("needless_variants.contestation", "'contestation' is a needless variant. Use 'contest' or 'dispute'.", r"contestation").with_severity(Severity::Warning),
        Check::new("needless_variants.conversate", "'conversate' is a needless variant. Use 'converse'.", r"conversate").with_severity(Severity::Error).with_replacement("converse"),
        Check::new("needless_variants.correlatable", "'correlatable' is a needless variant. Use 'correlative'.", r"correlatable").with_severity(Severity::Warning),
        Check::new("needless_variants.cowardness", "'cowardness' is a needless variant. Use 'cowardice'.", r"cowardness").with_severity(Severity::Warning).with_replacement("cowardice"),
        Check::new("needless_variants.criticalness", "'criticalness' is a needless variant. Use 'criticality'.", r"criticalness").with_severity(Severity::Warning).with_replacement("criticality"),
        Check::new("needless_variants.cruciality", "'cruciality' is a needless variant. Use 'importance' or 'criticality'.", r"cruciality").with_severity(Severity::Warning),
        Check::new("needless_variants.deceptional", "'deceptional' is a needless variant. Use 'deceptive'.", r"deceptional").with_severity(Severity::Warning).with_replacement("deceptive"),
        Check::new("needless_variants.decisional", "'decisional' is a needless variant. Use 'decision-related' or rephrase.", r"decisional").with_severity(Severity::Suggestion),
        Check::new("needless_variants.defencive", "'defencive' is a needless variant. Use 'defensive'.", r"defencive").with_severity(Severity::Error).with_replacement("defensive"),
        Check::new("needless_variants.definitional", "'definitional' is a needless variant. Use 'defining' or rephrase.", r"definitional").with_severity(Severity::Suggestion),
        Check::new("needless_variants.delegatory", "'delegatory' is a needless variant. Use 'delegating'.", r"delegatory").with_severity(Severity::Warning).with_replacement("delegating"),
        Check::new("needless_variants.delineative", "'delineative' is a needless variant. Use 'delineating'.", r"delineative").with_severity(Severity::Warning).with_replacement("delineating"),
        Check::new("needless_variants.delusionary", "'delusionary' is a needless variant. Use 'delusional'.", r"delusionary").with_severity(Severity::Warning).with_replacement("delusional"),
        Check::new("needless_variants.demonstratable", "'demonstratable' is a needless variant. Use 'demonstrable'.", r"demonstratable").with_severity(Severity::Warning).with_replacement("demonstrable"),
        Check::new("needless_variants.denouncement", "'denouncement' is a needless variant. Use 'denunciation'.", r"denouncement").with_severity(Severity::Warning).with_replacement("denunciation"),
        Check::new("needless_variants.dependant", "'dependant' (noun) vs 'dependent' (adj). Verify usage.", r"dependant").with_severity(Severity::Suggestion),
        Check::new("needless_variants.derivate", "'derivate' is a needless variant. Use 'derivative'.", r"derivate").with_severity(Severity::Warning).with_replacement("derivative"),
        Check::new("needless_variants.descriptional", "'descriptional' is a needless variant. Use 'descriptive'.", r"descriptional").with_severity(Severity::Warning).with_replacement("descriptive"),
        Check::new("needless_variants.desperateness", "'desperateness' is a needless variant. Use 'desperation'.", r"desperateness").with_severity(Severity::Warning).with_replacement("desperation"),
        Check::new("needless_variants.destructiveness", "'destructiveness' is a needless variant. Use 'destructivity'.", r"destructiveness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.detainment", "'detainment' is a needless variant. Use 'detention'.", r"detainment").with_severity(Severity::Warning).with_replacement("detention"),
        Check::new("needless_variants.determinate", "'determinate' may be confused with 'determined'. Verify usage.", r"determinate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.diagrammatic", "'diagrammatic' can be 'diagrammatical' but both are valid.", r"diagrammatical").with_severity(Severity::Suggestion).with_replacement("diagrammatic"),
        Check::new("needless_variants.differentiable", "'differentiable' (math) vs 'different'. Verify context.", r"differentiable").with_severity(Severity::Suggestion),
        Check::new("needless_variants.diminishment", "'diminishment' is a needless variant. Use 'diminution'.", r"diminishment").with_severity(Severity::Warning).with_replacement("diminution"),
        Check::new("needless_variants.directorate", "'directorate' vs 'directorship'. Both valid in different contexts.", r"directorate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.discomfiture", "'discomfiture' vs 'discomfort'. They have different meanings.", r"discomfiture").with_severity(Severity::Suggestion),
        Check::new("needless_variants.dispiritment", "'dispiritment' is a needless variant. Use 'dispiriting' or 'discouragement'.", r"dispiritment").with_severity(Severity::Warning),
        Check::new("needless_variants.doctorial", "'doctorial' is a needless variant. Use 'doctoral'.", r"doctorial").with_severity(Severity::Warning).with_replacement("doctoral"),
        Check::new("needless_variants.doubtless", "'doubtless' vs 'undoubtedly'. 'Doubtless' can imply less certainty.", r"doubtless").with_severity(Severity::Suggestion),
        Check::new("needless_variants.draughtsman", "'draughtsman' is British. Use 'draftsman' (US) or 'drafter'.", r"draughtsman").with_severity(Severity::Suggestion).with_replacement("drafter"),
        Check::new("needless_variants.effortful", "'effortful' is a needless variant. Use 'laborious' or 'difficult'.", r"effortful").with_severity(Severity::Suggestion),
        Check::new("needless_variants.embarkment", "'embarkment' is a needless variant. Use 'embarkation'.", r"embarkment").with_severity(Severity::Warning).with_replacement("embarkation"),
        Check::new("needless_variants.emolument", "'emolument' is formal. Consider 'compensation' or 'payment'.", r"emolument").with_severity(Severity::Suggestion),
        Check::new("needless_variants.empathetic", "'empathetic' vs 'empathic'. Both are valid; 'empathic' is older.", r"empathetic").with_severity(Severity::Suggestion),
        Check::new("needless_variants.enactment", "'enactment' vs 'enacting'. Both valid in different contexts.", r"enactment").with_severity(Severity::Suggestion),
        Check::new("needless_variants.endangerment", "'endangerment' vs 'endangering'. Both valid.", r"endangerment").with_severity(Severity::Suggestion),
        Check::new("needless_variants.enormity", "'enormity' (wickedness) vs 'enormousness' (size). Often confused.", r"enormity").with_severity(Severity::Warning),
        Check::new("needless_variants.enthronement", "'enthronement' is a needless variant. Use 'enthronization' or 'coronation'.", r"enthronement").with_severity(Severity::Suggestion),
        Check::new("needless_variants.envisionment", "'envisionment' is a needless variant. Use 'envisioning' or 'vision'.", r"envisionment").with_severity(Severity::Warning),
        Check::new("needless_variants.epitomise", "'epitomise' is British. Use 'epitomize' (US).", r"epitomise").with_severity(Severity::Suggestion).with_replacement("epitomize"),
        Check::new("needless_variants.equableness", "'equableness' is a needless variant. Use 'equability'.", r"equableness").with_severity(Severity::Warning).with_replacement("equability"),
        Check::new("needless_variants.eradicable", "'eradicable' vs 'eradicatable'. 'Eradicable' is preferred.", r"eradicatable").with_severity(Severity::Warning).with_replacement("eradicable"),
        Check::new("needless_variants.estimative", "'estimative' is a needless variant. Use 'estimated' or 'estimating'.", r"estimative").with_severity(Severity::Suggestion),
        Check::new("needless_variants.evaluatory", "'evaluatory' is a needless variant. Use 'evaluative'.", r"evaluatory").with_severity(Severity::Warning).with_replacement("evaluative"),
        Check::new("needless_variants.eventuality", "'eventuality' can often be 'event' or 'possibility'.", r"eventuality").with_severity(Severity::Suggestion),
        Check::new("needless_variants.evocate", "'evocate' is a needless variant. Use 'evoke'.", r"evocate").with_severity(Severity::Error).with_replacement("evoke"),
        Check::new("needless_variants.exactitude", "'exactitude' is a needless variant. Use 'exactness' or 'precision'.", r"exactitude").with_severity(Severity::Suggestion),
        Check::new("needless_variants.examinational", "'examinational' is a needless variant. Use 'examining' or rephrase.", r"examinational").with_severity(Severity::Warning),
        Check::new("needless_variants.excitative", "'excitative' is a needless variant. Use 'exciting' or 'excitatory'.", r"excitative").with_severity(Severity::Warning),
        Check::new("needless_variants.excusal", "'excusal' is a needless variant. Use 'excuse'.", r"excusal").with_severity(Severity::Warning).with_replacement("excuse"),
        Check::new("needless_variants.executorial", "'executorial' is a needless variant. Use 'executive'.", r"executorial").with_severity(Severity::Warning).with_replacement("executive"),
        Check::new("needless_variants.exhibitional", "'exhibitional' is a needless variant. Use 'exhibitory' or 'exhibition'.", r"exhibitional").with_severity(Severity::Warning),
        Check::new("needless_variants.existential", "'existential' is often overused. Consider if necessary.", r"existential").with_severity(Severity::Suggestion),
        Check::new("needless_variants.expansional", "'expansional' is a needless variant. Use 'expansive' or 'expanding'.", r"expansional").with_severity(Severity::Warning),
        Check::new("needless_variants.expediential", "'expediential' is a needless variant. Use 'expedient'.", r"expediential").with_severity(Severity::Warning).with_replacement("expedient"),
        Check::new("needless_variants.experimentative", "'experimentative' is a needless variant. Use 'experimental'.", r"experimentative").with_severity(Severity::Warning).with_replacement("experimental"),
        Check::new("needless_variants.exploitative", "'exploitative' vs 'exploitive'. Both valid; 'exploitative' more common.", r"exploitive").with_severity(Severity::Suggestion).with_replacement("exploitative"),
        Check::new("needless_variants.factional", "'factional' is valid. Just verify context.", r"factional").with_severity(Severity::Suggestion),
        Check::new("needless_variants.favouritism", "'favouritism' is British. Use 'favoritism' (US).", r"favouritism").with_severity(Severity::Suggestion).with_replacement("favoritism"),
        Check::new("needless_variants.fermentative", "'fermentative' is a needless variant. Use 'fermenting'.", r"fermentative").with_severity(Severity::Suggestion),
        Check::new("needless_variants.fervency", "'fervency' is a needless variant. Use 'fervor'.", r"fervency").with_severity(Severity::Warning).with_replacement("fervor"),
        Check::new("needless_variants.fictive", "'fictive' vs 'fictional'. 'Fictional' is more common.", r"fictive").with_severity(Severity::Suggestion),
        Check::new("needless_variants.finalize", "'finalize' can often be 'finish' or 'complete'.", r"finalize").with_severity(Severity::Suggestion),
        Check::new("needless_variants.fixative", "'fixative' (noun) is valid. As adj, consider 'fixing'.", r"fixative").with_severity(Severity::Suggestion),
        Check::new("needless_variants.flammable", "'flammable' vs 'inflammable'. Both mean 'catches fire easily'.", r"inflammable").with_severity(Severity::Warning).with_replacement("flammable"),
        Check::new("needless_variants.forbiddance", "'forbiddance' is a needless variant. Use 'prohibition'.", r"forbiddance").with_severity(Severity::Warning).with_replacement("prohibition"),
        Check::new("needless_variants.forgetfulness", "'forgetfulness' vs 'oblivion'. Different connotations.", r"forgetfulness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.formational", "'formational' is a needless variant. Use 'formative'.", r"formational").with_severity(Severity::Warning).with_replacement("formative"),
        Check::new("needless_variants.formulaic", "'formulaic' vs 'formular'. 'Formulaic' is standard.", r"formular").with_severity(Severity::Warning).with_replacement("formulaic"),
        Check::new("needless_variants.forthwith", "'forthwith' is formal. Consider 'immediately'.", r"forthwith").with_severity(Severity::Suggestion).with_replacement("immediately"),
        Check::new("needless_variants.frankness", "'frankness' vs 'candor'. Both valid.", r"frankness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.frustrater", "'frustrater' is a needless variant. Use 'frustrator'.", r"frustrater").with_severity(Severity::Warning).with_replacement("frustrator"),
        Check::new("needless_variants.funereal", "'funereal' vs 'funeral'. Different meanings.", r"funereal").with_severity(Severity::Suggestion),
        Check::new("needless_variants.gamely", "'gamely' is valid but uncommon.", r"gamely").with_severity(Severity::Suggestion),
        Check::new("needless_variants.genealogical", "'genealogical' vs 'genetic'. Different meanings.", r"genealogical").with_severity(Severity::Suggestion),
        Check::new("needless_variants.glamourise", "'glamourise' is British. Use 'glamorize' (US).", r"glamourise").with_severity(Severity::Suggestion).with_replacement("glamorize"),
        Check::new("needless_variants.globalise", "'globalise' is British. Use 'globalize' (US).", r"globalise").with_severity(Severity::Suggestion).with_replacement("globalize"),
        Check::new("needless_variants.governorate", "'governorate' is valid in specific contexts (Middle East admin).", r"governorate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.graduand", "'graduand' (about to graduate) vs 'graduate' (has graduated).", r"graduand").with_severity(Severity::Suggestion),
        Check::new("needless_variants.grammatical", "'grammatical' is valid. Just verify context.", r"grammatical").with_severity(Severity::Suggestion),
        Check::new("needless_variants.grievance", "'grievance' vs 'grief'. Different meanings.", r"grievance").with_severity(Severity::Suggestion),
        Check::new("needless_variants.growthful", "'growthful' is a needless variant. Use 'growing' or 'developing'.", r"growthful").with_severity(Severity::Warning),
        Check::new("needless_variants.guesstimate", "'guesstimate' is informal. Use 'estimate' in formal writing.", r"guesstimate").with_severity(Severity::Suggestion).with_replacement("estimate"),
        Check::new("needless_variants.habituate", "'habituate' is valid. Just verify context.", r"habituate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.haphazardness", "'haphazardness' is a needless variant. Use 'randomness' or 'disorder'.", r"haphazardness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.heartenment", "'heartenment' is a needless variant. Use 'encouragement'.", r"heartenment").with_severity(Severity::Warning).with_replacement("encouragement"),
        Check::new("needless_variants.heedfulness", "'heedfulness' is a needless variant. Use 'carefulness' or 'caution'.", r"heedfulness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.helpfulness", "'helpfulness' is valid but can often be simplified.", r"helpfulness").with_severity(Severity::Suggestion),
        Check::new("needless_variants.hereunto", "'hereunto' is archaic. Use 'to this' or rephrase.", r"hereunto").with_severity(Severity::Warning),
        Check::new("needless_variants.herewith", "'herewith' is formal/archaic. Consider 'with this' or 'enclosed'.", r"herewith").with_severity(Severity::Suggestion),
        Check::new("needless_variants.historicize", "'historicize' is a needless variant. Use 'place in historical context'.", r"historicize").with_severity(Severity::Suggestion),
        Check::new("needless_variants.homogenise", "'homogenise' is British. Use 'homogenize' (US).", r"homogenise").with_severity(Severity::Suggestion).with_replacement("homogenize"),
        Check::new("needless_variants.hospitalise", "'hospitalise' is British. Use 'hospitalize' (US).", r"hospitalise").with_severity(Severity::Suggestion).with_replacement("hospitalize"),
        Check::new("needless_variants.humanise", "'humanise' is British. Use 'humanize' (US).", r"humanise").with_severity(Severity::Suggestion).with_replacement("humanize"),
        Check::new("needless_variants.hypnotise", "'hypnotise' is British. Use 'hypnotize' (US).", r"hypnotise").with_severity(Severity::Suggestion).with_replacement("hypnotize"),
        Check::new("needless_variants.hypothesise", "'hypothesise' is British. Use 'hypothesize' (US).", r"hypothesise").with_severity(Severity::Suggestion).with_replacement("hypothesize"),
        Check::new("needless_variants.idealise", "'idealise' is British. Use 'idealize' (US).", r"idealise").with_severity(Severity::Suggestion).with_replacement("idealize"),
        Check::new("needless_variants.illegalize", "'illegalize' is a needless variant. Use 'outlaw' or 'prohibit'.", r"illegalize").with_severity(Severity::Warning),
        Check::new("needless_variants.immobilise", "'immobilise' is British. Use 'immobilize' (US).", r"immobilise").with_severity(Severity::Suggestion).with_replacement("immobilize"),
        Check::new("needless_variants.immunise", "'immunise' is British. Use 'immunize' (US).", r"immunise").with_severity(Severity::Suggestion).with_replacement("immunize"),
        Check::new("needless_variants.impactful", "'impactful' is disputed. Consider 'effective' or 'influential'.", r"impactful").with_severity(Severity::Suggestion),
        Check::new("needless_variants.imperilled", "'imperilled' is British. Use 'imperiled' (US).", r"imperilled").with_severity(Severity::Suggestion).with_replacement("imperiled"),
        Check::new("needless_variants.impermissable", "'impermissable' is a needless variant. Use 'impermissible'.", r"impermissable").with_severity(Severity::Error).with_replacement("impermissible"),
        Check::new("needless_variants.impersonalise", "'impersonalise' is British. Use 'impersonalize' (US).", r"impersonalise").with_severity(Severity::Suggestion).with_replacement("impersonalize"),
        Check::new("needless_variants.importunement", "'importunement' is a needless variant. Use 'importuning'.", r"importunement").with_severity(Severity::Warning),
        Check::new("needless_variants.improvise", "'improvise' vs 'extemporize'. Both valid.", r"improvise").with_severity(Severity::Suggestion),
        Check::new("needless_variants.inceptive", "'inceptive' is a needless variant. Use 'initial' or 'beginning'.", r"inceptive").with_severity(Severity::Suggestion),
        Check::new("needless_variants.inchoative", "'inchoative' is technical (linguistics). Verify context.", r"inchoative").with_severity(Severity::Suggestion),
        Check::new("needless_variants.indeterminant", "'indeterminant' is a needless variant. Use 'indeterminate'.", r"indeterminant").with_severity(Severity::Warning).with_replacement("indeterminate"),
        Check::new("needless_variants.individualise", "'individualise' is British. Use 'individualize' (US).", r"individualise").with_severity(Severity::Suggestion).with_replacement("individualize"),
        Check::new("needless_variants.industrialise", "'industrialise' is British. Use 'industrialize' (US).", r"industrialise").with_severity(Severity::Suggestion).with_replacement("industrialize"),
        Check::new("needless_variants.inescapeable", "'inescapeable' is a needless variant. Use 'inescapable'.", r"inescapeable").with_severity(Severity::Error).with_replacement("inescapable"),
        Check::new("needless_variants.inflectional", "'inflectional' is valid in linguistics. Verify context.", r"inflectional").with_severity(Severity::Suggestion),
        Check::new("needless_variants.informality", "'informality' is valid but can often be simplified.", r"informality").with_severity(Severity::Suggestion),
        Check::new("needless_variants.initialise", "'initialise' is British. Use 'initialize' (US).", r"initialise").with_severity(Severity::Suggestion).with_replacement("initialize"),
        Check::new("needless_variants.initiatory", "'initiatory' is a needless variant. Use 'initial' or 'introductory'.", r"initiatory").with_severity(Severity::Suggestion),
        Check::new("needless_variants.innovativeness", "'innovativeness' is a needless variant. Use 'innovation'.", r"innovativeness").with_severity(Severity::Warning).with_replacement("innovation"),
        Check::new("needless_variants.insistment", "'insistment' is a needless variant. Use 'insistence'.", r"insistment").with_severity(Severity::Warning).with_replacement("insistence"),
        Check::new("needless_variants.institutionalise", "'institutionalise' is British. Use 'institutionalize' (US).", r"institutionalise").with_severity(Severity::Suggestion).with_replacement("institutionalize"),
        Check::new("needless_variants.instrumentalise", "'instrumentalise' is British. Use 'instrumentalize' (US).", r"instrumentalise").with_severity(Severity::Suggestion).with_replacement("instrumentalize"),
        Check::new("needless_variants.intellectualise", "'intellectualise' is British. Use 'intellectualize' (US).", r"intellectualise").with_severity(Severity::Suggestion).with_replacement("intellectualize"),
        Check::new("needless_variants.intenseness", "'intenseness' is a needless variant. Use 'intensity'.", r"intenseness").with_severity(Severity::Warning).with_replacement("intensity"),
        Check::new("needless_variants.interiorize", "'interiorize' is a needless variant. Use 'internalize'.", r"interiorize").with_severity(Severity::Warning).with_replacement("internalize"),
        Check::new("needless_variants.internalise", "'internalise' is British. Use 'internalize' (US).", r"internalise").with_severity(Severity::Suggestion).with_replacement("internalize"),
        Check::new("needless_variants.internationalise", "'internationalise' is British. Use 'internationalize' (US).", r"internationalise").with_severity(Severity::Suggestion).with_replacement("internationalize"),
        Check::new("needless_variants.interpretative", "'interpretative' vs 'interpretive'. Both valid; 'interpretive' more common in US.", r"interpretative").with_severity(Severity::Suggestion),
        Check::new("needless_variants.inventional", "'inventional' is a needless variant. Use 'inventive'.", r"inventional").with_severity(Severity::Warning).with_replacement("inventive"),
        Check::new("needless_variants.invigorate", "'invigorate' is valid. Just verify context.", r"invigorate").with_severity(Severity::Suggestion),
        Check::new("needless_variants.ionise", "'ionise' is British. Use 'ionize' (US).", r"ionise").with_severity(Severity::Suggestion).with_replacement("ionize"),
        Check::new("needless_variants.irreconcileable", "'irreconcileable' is a needless variant. Use 'irreconcilable'.", r"irreconcileable").with_severity(Severity::Error).with_replacement("irreconcilable"),
        Check::new("needless_variants.itemise", "'itemise' is British. Use 'itemize' (US).", r"itemise").with_severity(Severity::Suggestion).with_replacement("itemize"),
        Check::new("needless_variants.jeopardise", "'jeopardise' is British. Use 'jeopardize' (US).", r"jeopardise").with_severity(Severity::Suggestion).with_replacement("jeopardize"),
        Check::new("needless_variants.judgement", "'judgement' is British. Use 'judgment' (US), except in legal contexts.", r"judgement").with_severity(Severity::Suggestion).with_replacement("judgment"),
        Check::new("needless_variants.knowledgable", "'knowledgable' is a needless variant. Use 'knowledgeable'.", r"knowledgable").with_severity(Severity::Error).with_replacement("knowledgeable"),
        Check::new("needless_variants.labelled", "'labelled' is British. Use 'labeled' (US).", r"labelled").with_severity(Severity::Suggestion).with_replacement("labeled"),
        Check::new("needless_variants.labelling", "'labelling' is British. Use 'labeling' (US).", r"labelling").with_severity(Severity::Suggestion).with_replacement("labeling"),
        Check::new("needless_variants.legalise", "'legalise' is British. Use 'legalize' (US).", r"legalise").with_severity(Severity::Suggestion).with_replacement("legalize"),
        Check::new("needless_variants.legitimatise", "'legitimatise' is British. Use 'legitimatize' (US) or 'legitimize'.", r"legitimatise").with_severity(Severity::Suggestion).with_replacement("legitimize"),
        Check::new("needless_variants.legitimise", "'legitimise' is British. Use 'legitimize' (US).", r"legitimise").with_severity(Severity::Suggestion).with_replacement("legitimize"),
        Check::new("needless_variants.levelled", "'levelled' is British. Use 'leveled' (US).", r"levelled").with_severity(Severity::Suggestion).with_replacement("leveled"),
        Check::new("needless_variants.levelling", "'levelling' is British. Use 'leveling' (US).", r"levelling").with_severity(Severity::Suggestion).with_replacement("leveling"),
        Check::new("needless_variants.liberalise", "'liberalise' is British. Use 'liberalize' (US).", r"liberalise").with_severity(Severity::Suggestion).with_replacement("liberalize"),
        Check::new("needless_variants.licence", "'licence' (noun, British) vs 'license'. Use 'license' (US for both).", r"licence").with_severity(Severity::Suggestion).with_replacement("license"),
        Check::new("needless_variants.likeable", "'likeable' vs 'likable'. Both valid; 'likable' more common in US.", r"likeable").with_severity(Severity::Suggestion),
        Check::new("needless_variants.localise", "'localise' is British. Use 'localize' (US).", r"localise").with_severity(Severity::Suggestion).with_replacement("localize"),
        Check::new("needless_variants.lodgement", "'lodgement' is British. Use 'lodgment' (US).", r"lodgement").with_severity(Severity::Suggestion).with_replacement("lodgment"),
        Check::new("needless_variants.magnetise", "'magnetise' is British. Use 'magnetize' (US).", r"magnetise").with_severity(Severity::Suggestion).with_replacement("magnetize"),
        Check::new("needless_variants.manoeuvrability", "'manoeuvrability' is British. Use 'maneuverability' (US).", r"manoeuvrability").with_severity(Severity::Suggestion).with_replacement("maneuverability"),
        Check::new("needless_variants.manoeuvre", "'manoeuvre' is British. Use 'maneuver' (US).", r"manoeuvre").with_severity(Severity::Suggestion).with_replacement("maneuver"),
        Check::new("needless_variants.marginalise", "'marginalise' is British. Use 'marginalize' (US).", r"marginalise").with_severity(Severity::Suggestion).with_replacement("marginalize"),
        Check::new("needless_variants.marvelled", "'marvelled' is British. Use 'marveled' (US).", r"marvelled").with_severity(Severity::Suggestion).with_replacement("marveled"),
        Check::new("needless_variants.marvelling", "'marvelling' is British. Use 'marveling' (US).", r"marvelling").with_severity(Severity::Suggestion).with_replacement("marveling"),
        Check::new("needless_variants.materialise", "'materialise' is British. Use 'materialize' (US).", r"materialise").with_severity(Severity::Suggestion).with_replacement("materialize"),
        Check::new("needless_variants.maximise", "'maximise' is British. Use 'maximize' (US).", r"maximise").with_severity(Severity::Suggestion).with_replacement("maximize"),
        Check::new("needless_variants.mechanise", "'mechanise' is British. Use 'mechanize' (US).", r"mechanise").with_severity(Severity::Suggestion).with_replacement("mechanize"),
        Check::new("needless_variants.mediaeval", "'mediaeval' is archaic. Use 'medieval'.", r"mediaeval").with_severity(Severity::Warning).with_replacement("medieval"),
        Check::new("needless_variants.memorialise", "'memorialise' is British. Use 'memorialize' (US).", r"memorialise").with_severity(Severity::Suggestion).with_replacement("memorialize"),
        Check::new("needless_variants.memorise", "'memorise' is British. Use 'memorize' (US).", r"memorise").with_severity(Severity::Suggestion).with_replacement("memorize"),
        Check::new("needless_variants.mesmerise", "'mesmerise' is British. Use 'mesmerize' (US).", r"mesmerise").with_severity(Severity::Suggestion).with_replacement("mesmerize"),
        Check::new("needless_variants.metabolise", "'metabolise' is British. Use 'metabolize' (US).", r"metabolise").with_severity(Severity::Suggestion).with_replacement("metabolize"),
        Check::new("needless_variants.militarise", "'militarise' is British. Use 'militarize' (US).", r"militarise").with_severity(Severity::Suggestion).with_replacement("militarize"),
        Check::new("needless_variants.millimetre", "'millimetre' is British. Use 'millimeter' (US).", r"millimetre").with_severity(Severity::Suggestion).with_replacement("millimeter"),
        Check::new("needless_variants.minimise", "'minimise' is British. Use 'minimize' (US).", r"minimise").with_severity(Severity::Suggestion).with_replacement("minimize"),
        Check::new("needless_variants.misbehaviour", "'misbehaviour' is British. Use 'misbehavior' (US).", r"misbehaviour").with_severity(Severity::Suggestion).with_replacement("misbehavior"),
        Check::new("needless_variants.misspelt", "'misspelt' is British. Use 'misspelled' (US).", r"misspelt").with_severity(Severity::Suggestion).with_replacement("misspelled"),
        Check::new("needless_variants.mobilise", "'mobilise' is British. Use 'mobilize' (US).", r"mobilise").with_severity(Severity::Suggestion).with_replacement("mobilize"),
        Check::new("needless_variants.modelled", "'modelled' is British. Use 'modeled' (US).", r"modelled").with_severity(Severity::Suggestion).with_replacement("modeled"),
        Check::new("needless_variants.modelling", "'modelling' is British. Use 'modeling' (US).", r"modelling").with_severity(Severity::Suggestion).with_replacement("modeling"),
        Check::new("needless_variants.modernise", "'modernise' is British. Use 'modernize' (US).", r"modernise").with_severity(Severity::Suggestion).with_replacement("modernize"),
        Check::new("needless_variants.moisturise", "'moisturise' is British. Use 'moisturize' (US).", r"moisturise").with_severity(Severity::Suggestion).with_replacement("moisturize"),
        Check::new("needless_variants.monopolise", "'monopolise' is British. Use 'monopolize' (US).", r"monopolise").with_severity(Severity::Suggestion).with_replacement("monopolize"),
        Check::new("needless_variants.moralise", "'moralise' is British. Use 'moralize' (US).", r"moralise").with_severity(Severity::Suggestion).with_replacement("moralize"),
        Check::new("needless_variants.motorise", "'motorise' is British. Use 'motorize' (US).", r"motorise").with_severity(Severity::Suggestion).with_replacement("motorize"),
        Check::new("needless_variants.nationalise", "'nationalise' is British. Use 'nationalize' (US).", r"nationalise").with_severity(Severity::Suggestion).with_replacement("nationalize"),
        Check::new("needless_variants.naturalise", "'naturalise' is British. Use 'naturalize' (US).", r"naturalise").with_severity(Severity::Suggestion).with_replacement("naturalize"),
        Check::new("needless_variants.neighbour", "'neighbour' is British. Use 'neighbor' (US).", r"neighbour").with_severity(Severity::Suggestion).with_replacement("neighbor"),
        Check::new("needless_variants.neighbourhood", "'neighbourhood' is British. Use 'neighborhood' (US).", r"neighbourhood").with_severity(Severity::Suggestion).with_replacement("neighborhood"),
        Check::new("needless_variants.neighbouring", "'neighbouring' is British. Use 'neighboring' (US).", r"neighbouring").with_severity(Severity::Suggestion).with_replacement("neighboring"),
        Check::new("needless_variants.neutralise", "'neutralise' is British. Use 'neutralize' (US).", r"neutralise").with_severity(Severity::Suggestion).with_replacement("neutralize"),
        Check::new("needless_variants.normalise", "'normalise' is British. Use 'normalize' (US).", r"normalise").with_severity(Severity::Suggestion).with_replacement("normalize"),
        Check::new("needless_variants.notarise", "'notarise' is British. Use 'notarize' (US).", r"notarise").with_severity(Severity::Suggestion).with_replacement("notarize"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_needless_variant() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "needless_variants.irregardless").unwrap();
        let results = check.run("Irregardless of the outcome...");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_towards() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "needless_variants.towards").unwrap();
        let results = check.run("Walking towards the door.");
        assert_eq!(results.len(), 1);
    }
}
