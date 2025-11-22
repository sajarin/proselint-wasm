//! Terms checks for proselint-wasm
//!
//! Suggests more precise terminology including animal adjectives,
//! denizen labels, and collective nouns (venery).

use crate::check::{Check, Severity};

/// Get all terms checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Animal adjectives - more elegant alternatives to "-like" constructions
        Check::new(
            "terms.animal_adjectives.bear_like",
            "'bear-like' can be 'ursine'.",
            r"bear[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("ursine"),

        Check::new(
            "terms.animal_adjectives.bird_like",
            "'bird-like' can be 'avian'.",
            r"bird[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("avian"),

        Check::new(
            "terms.animal_adjectives.cat_like",
            "'cat-like' can be 'feline'.",
            r"cat[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("feline"),

        Check::new(
            "terms.animal_adjectives.cow_like",
            "'cow-like' can be 'bovine'.",
            r"cow[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("bovine"),

        Check::new(
            "terms.animal_adjectives.crow_like",
            "'crow-like' can be 'corvine'.",
            r"crow[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("corvine"),

        Check::new(
            "terms.animal_adjectives.dog_like",
            "'dog-like' can be 'canine'.",
            r"dog[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("canine"),

        Check::new(
            "terms.animal_adjectives.donkey_like",
            "'donkey-like' can be 'asinine'.",
            r"donkey[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("asinine"),

        Check::new(
            "terms.animal_adjectives.eagle_like",
            "'eagle-like' can be 'aquiline'.",
            r"eagle[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("aquiline"),

        Check::new(
            "terms.animal_adjectives.fish_like",
            "'fish-like' can be 'piscine'.",
            r"fish[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("piscine"),

        Check::new(
            "terms.animal_adjectives.fox_like",
            "'fox-like' can be 'vulpine'.",
            r"fox[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("vulpine"),

        Check::new(
            "terms.animal_adjectives.goat_like",
            "'goat-like' can be 'caprine'.",
            r"goat[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("caprine"),

        Check::new(
            "terms.animal_adjectives.hawk_like",
            "'hawk-like' can be 'accipitrine'.",
            r"hawk[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("accipitrine"),

        Check::new(
            "terms.animal_adjectives.horse_like",
            "'horse-like' can be 'equine'.",
            r"horse[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("equine"),

        Check::new(
            "terms.animal_adjectives.lion_like",
            "'lion-like' can be 'leonine'.",
            r"lion[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("leonine"),

        Check::new(
            "terms.animal_adjectives.monkey_like",
            "'monkey-like' can be 'simian'.",
            r"monkey[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("simian"),

        Check::new(
            "terms.animal_adjectives.mouse_like",
            "'mouse-like' can be 'murine'.",
            r"mouse[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("murine"),

        Check::new(
            "terms.animal_adjectives.owl_like",
            "'owl-like' can be 'strigine'.",
            r"owl[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("strigine"),

        Check::new(
            "terms.animal_adjectives.pig_like",
            "'pig-like' can be 'porcine'.",
            r"pig[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("porcine"),

        Check::new(
            "terms.animal_adjectives.rabbit_like",
            "'rabbit-like' can be 'leporine'.",
            r"rabbit[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("leporine"),

        Check::new(
            "terms.animal_adjectives.rat_like",
            "'rat-like' can be 'murine'.",
            r"rat[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("murine"),

        Check::new(
            "terms.animal_adjectives.serpent_like",
            "'serpent-like' can be 'serpentine'.",
            r"serpent[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("serpentine"),

        Check::new(
            "terms.animal_adjectives.sheep_like",
            "'sheep-like' can be 'ovine'.",
            r"sheep[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("ovine"),

        Check::new(
            "terms.animal_adjectives.snake_like",
            "'snake-like' can be 'serpentine' or 'ophidian'.",
            r"snake[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("serpentine"),

        Check::new(
            "terms.animal_adjectives.spider_like",
            "'spider-like' can be 'arachnoid'.",
            r"spider[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("arachnoid"),

        Check::new(
            "terms.animal_adjectives.swan_like",
            "'swan-like' can be 'cygnine'.",
            r"swan[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("cygnine"),

        Check::new(
            "terms.animal_adjectives.tiger_like",
            "'tiger-like' can be 'tigrine'.",
            r"tiger[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("tigrine"),

        Check::new(
            "terms.animal_adjectives.wolf_like",
            "'wolf-like' can be 'lupine'.",
            r"wolf[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("lupine"),

        Check::new(
            "terms.animal_adjectives.worm_like",
            "'worm-like' can be 'vermicular' or 'vermiform'.",
            r"worm[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("vermicular"),

        // Denizen labels - terms for inhabitants of places
        Check::new(
            "terms.denizen.person_from_paris",
            "'person from Paris' can be 'Parisian'.",
            r"person from Paris|Paris native",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Parisian"),

        Check::new(
            "terms.denizen.person_from_london",
            "'person from London' can be 'Londoner'.",
            r"person from London|London native",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Londoner"),

        Check::new(
            "terms.denizen.person_from_new_york",
            "'person from New York' can be 'New Yorker'.",
            r"person from New York",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("New Yorker"),

        Check::new(
            "terms.denizen.person_from_moscow",
            "'person from Moscow' can be 'Muscovite'.",
            r"person from Moscow|Moscow native",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Muscovite"),

        Check::new(
            "terms.denizen.person_from_naples",
            "'person from Naples' can be 'Neapolitan'.",
            r"person from Naples|Naples native",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Neapolitan"),

        Check::new(
            "terms.denizen.person_from_glasgow",
            "'person from Glasgow' can be 'Glaswegian'.",
            r"person from Glasgow|Glasgow native",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Glaswegian"),

        // Venery - collective nouns for animals
        Check::new(
            "terms.venery.group_of_crows",
            "'group of crows' can be 'murder of crows'.",
            r"(?:group|flock) of crows",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("murder of crows"),

        Check::new(
            "terms.venery.group_of_lions",
            "'group of lions' is 'pride of lions'.",
            r"group of lions",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("pride of lions"),

        Check::new(
            "terms.venery.group_of_wolves",
            "'group of wolves' is 'pack of wolves'.",
            r"group of wolves",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("pack of wolves"),

        Check::new(
            "terms.venery.group_of_fish",
            "'group of fish' can be 'school of fish' or 'shoal of fish'.",
            r"group of fish",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("school of fish"),

        Check::new(
            "terms.venery.group_of_geese",
            "'group of geese' can be 'gaggle of geese' (on ground) or 'skein' (in flight).",
            r"group of geese",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("gaggle of geese"),

        Check::new(
            "terms.venery.group_of_owls",
            "'group of owls' is 'parliament of owls'.",
            r"group of owls",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("parliament of owls"),

        Check::new(
            "terms.venery.group_of_whales",
            "'group of whales' is 'pod of whales'.",
            r"group of whales",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("pod of whales"),

        Check::new(
            "terms.venery.group_of_bees",
            "'group of bees' can be 'swarm of bees' or 'hive of bees'.",
            r"group of bees",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("swarm of bees"),

        Check::new(
            "terms.venery.group_of_ants",
            "'group of ants' can be 'colony of ants'.",
            r"group of ants",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("colony of ants"),

        Check::new(
            "terms.venery.group_of_elephants",
            "'group of elephants' is 'herd of elephants'.",
            r"group of elephants",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("herd of elephants"),

        Check::new(
            "terms.venery.group_of_ravens",
            "'group of ravens' is 'unkindness of ravens'.",
            r"group of ravens",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("unkindness of ravens"),

        Check::new(
            "terms.venery.group_of_flamingos",
            "'group of flamingos' is 'flamboyance of flamingos'.",
            r"group of flamingos",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("flamboyance of flamingos"),

        // Eponymous adjectives
        Check::new(
            "terms.eponymous.shakespeare_like",
            "'Shakespeare-like' can be 'Shakespearean'.",
            r"Shakespeare[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Shakespearean"),

        Check::new(
            "terms.eponymous.kafka_like",
            "'Kafka-like' can be 'Kafkaesque'.",
            r"Kafka[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Kafkaesque"),

        Check::new(
            "terms.eponymous.darwin_like",
            "'Darwin-like' can be 'Darwinian'.",
            r"Darwin[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Darwinian"),

        Check::new(
            "terms.eponymous.freud_like",
            "'Freud-like' can be 'Freudian'.",
            r"Freud[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Freudian"),

        Check::new(
            "terms.eponymous.orwell_like",
            "'Orwell-like' can be 'Orwellian'.",
            r"Orwell[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Orwellian"),

        Check::new(
            "terms.eponymous.dickens_like",
            "'Dickens-like' can be 'Dickensian'.",
            r"Dickens[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Dickensian"),

        Check::new(
            "terms.eponymous.machiavelli_like",
            "'Machiavelli-like' can be 'Machiavellian'.",
            r"Machiavelli[- ]like",
        )
        .raw()
        .with_severity(Severity::Suggestion)
        .with_replacement("Machiavellian"),

        // === Additional animal adjectives from proselint ===
        Check::new("terms.animal_adjectives.ape_like", "'ape-like' can be 'simian'.", r"ape[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("simian"),
        Check::new("terms.animal_adjectives.ass_like", "'ass-like' can be 'asinine'.", r"ass[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("asinine"),
        Check::new("terms.animal_adjectives.badger_like", "'badger-like' can be 'meline'.", r"badger[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("meline"),
        Check::new("terms.animal_adjectives.bee_like", "'bee-like' can be 'apian'.", r"bee[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("apian"),
        Check::new("terms.animal_adjectives.bull_like", "'bull-like' can be 'taurine'.", r"bull[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("taurine"),
        Check::new("terms.animal_adjectives.calf_like", "'calf-like' can be 'vituline'.", r"calf[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("vituline"),
        Check::new("terms.animal_adjectives.camel_like", "'camel-like' can be 'cameline'.", r"camel[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("cameline"),
        Check::new("terms.animal_adjectives.chicken_like", "'chicken-like' can be 'galline'.", r"chicken[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("galline"),
        Check::new("terms.animal_adjectives.crab_like", "'crab-like' can be 'cancrine'.", r"crab[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("cancrine"),
        Check::new("terms.animal_adjectives.deer_like", "'deer-like' can be 'cervine'.", r"deer[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("cervine"),
        Check::new("terms.animal_adjectives.dove_like", "'dove-like' can be 'columbine'.", r"dove[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("columbine"),
        Check::new("terms.animal_adjectives.duck_like", "'duck-like' can be 'anatine'.", r"duck[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("anatine"),
        Check::new("terms.animal_adjectives.elephant_like", "'elephant-like' can be 'elephantine'.", r"elephant[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("elephantine"),
        Check::new("terms.animal_adjectives.falcon_like", "'falcon-like' can be 'falconine'.", r"falcon[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("falconine"),
        Check::new("terms.animal_adjectives.finch_like", "'finch-like' can be 'fringilline'.", r"finch[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("fringilline"),
        Check::new("terms.animal_adjectives.frog_like", "'frog-like' can be 'ranine'.", r"frog[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("ranine"),
        Check::new("terms.animal_adjectives.giraffe_like", "'giraffe-like' can be 'giraffine'.", r"giraffe[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("giraffine"),
        Check::new("terms.animal_adjectives.goose_like", "'goose-like' can be 'anserine'.", r"goose[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("anserine"),
        Check::new("terms.animal_adjectives.hare_like", "'hare-like' can be 'leporine'.", r"hare[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("leporine"),
        Check::new("terms.animal_adjectives.herring_like", "'herring-like' can be 'clupeoid'.", r"herring[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("clupeoid"),
        Check::new("terms.animal_adjectives.hog_like", "'hog-like' can be 'porcine'.", r"hog[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("porcine"),
        Check::new("terms.animal_adjectives.hornet_like", "'hornet-like' can be 'vespine'.", r"hornet[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("vespine"),
        Check::new("terms.animal_adjectives.jay_like", "'jay-like' can be 'garruline'.", r"jay[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("garruline"),
        Check::new("terms.animal_adjectives.kangaroo_like", "'kangaroo-like' can be 'macropodine'.", r"kangaroo[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("macropodine"),
        Check::new("terms.animal_adjectives.lark_like", "'lark-like' can be 'alaudine'.", r"lark[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("alaudine"),
        Check::new("terms.animal_adjectives.leopard_like", "'leopard-like' can be 'pardine'.", r"leopard[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("pardine"),
        Check::new("terms.animal_adjectives.lynx_like", "'lynx-like' can be 'lyncean'.", r"lynx[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("lyncean"),
        Check::new("terms.animal_adjectives.magpie_like", "'magpie-like' can be 'picine'.", r"magpie[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("picine"),
        Check::new("terms.animal_adjectives.mole_like", "'mole-like' can be 'talpine'.", r"mole[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("talpine"),
        Check::new("terms.animal_adjectives.otter_like", "'otter-like' can be 'lutrine'.", r"otter[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("lutrine"),
        Check::new("terms.animal_adjectives.ox_like", "'ox-like' can be 'bovine'.", r"ox[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("bovine"),
        Check::new("terms.animal_adjectives.parrot_like", "'parrot-like' can be 'psittacine'.", r"parrot[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("psittacine"),
        Check::new("terms.animal_adjectives.peacock_like", "'peacock-like' can be 'pavonine'.", r"peacock[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("pavonine"),
        Check::new("terms.animal_adjectives.pigeon_like", "'pigeon-like' can be 'columbine'.", r"pigeon[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("columbine"),
        Check::new("terms.animal_adjectives.porcupine_like", "'porcupine-like' can be 'hystricine'.", r"porcupine[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("hystricine"),
        Check::new("terms.animal_adjectives.raven_like", "'raven-like' can be 'corvine'.", r"raven[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("corvine"),
        Check::new("terms.animal_adjectives.salmon_like", "'salmon-like' can be 'salmonine'.", r"salmon[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("salmonine"),
        Check::new("terms.animal_adjectives.seal_like", "'seal-like' can be 'phocine'.", r"seal[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("phocine"),
        Check::new("terms.animal_adjectives.shark_like", "'shark-like' can be 'squaline'.", r"shark[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("squaline"),
        Check::new("terms.animal_adjectives.shrimp_like", "'shrimp-like' can be 'caridean'.", r"shrimp[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("caridean"),
        Check::new("terms.animal_adjectives.slug_like", "'slug-like' can be 'limacine'.", r"slug[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("limacine"),
        Check::new("terms.animal_adjectives.snail_like", "'snail-like' can be 'cochlear'.", r"snail[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("cochlear"),
        Check::new("terms.animal_adjectives.sparrow_like", "'sparrow-like' can be 'passerine'.", r"sparrow[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("passerine"),
        Check::new("terms.animal_adjectives.squirrel_like", "'squirrel-like' can be 'sciurine'.", r"squirrel[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("sciurine"),
        Check::new("terms.animal_adjectives.stork_like", "'stork-like' can be 'ciconiine'.", r"stork[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("ciconiine"),
        Check::new("terms.animal_adjectives.swallow_like", "'swallow-like' can be 'hirundine'.", r"swallow[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("hirundine"),
        Check::new("terms.animal_adjectives.thrush_like", "'thrush-like' can be 'turdine'.", r"thrush[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("turdine"),
        Check::new("terms.animal_adjectives.toad_like", "'toad-like' can be 'batrachian'.", r"toad[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("batrachian"),
        Check::new("terms.animal_adjectives.tortoise_like", "'tortoise-like' can be 'testudinal'.", r"tortoise[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("testudinal"),
        Check::new("terms.animal_adjectives.trout_like", "'trout-like' can be 'truttaceous'.", r"trout[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("truttaceous"),
        Check::new("terms.animal_adjectives.turkey_like", "'turkey-like' can be 'meleagrine'.", r"turkey[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("meleagrine"),
        Check::new("terms.animal_adjectives.turtle_like", "'turtle-like' can be 'chelonian'.", r"turtle[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("chelonian"),
        Check::new("terms.animal_adjectives.vulture_like", "'vulture-like' can be 'vulturine'.", r"vulture[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("vulturine"),
        Check::new("terms.animal_adjectives.wasp_like", "'wasp-like' can be 'vespine'.", r"wasp[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("vespine"),
        Check::new("terms.animal_adjectives.weasel_like", "'weasel-like' can be 'musteline'.", r"weasel[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("musteline"),
        Check::new("terms.animal_adjectives.whale_like", "'whale-like' can be 'cetacean'.", r"whale[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("cetacean"),
        Check::new("terms.animal_adjectives.woodpecker_like", "'woodpecker-like' can be 'picine'.", r"woodpecker[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("picine"),
        Check::new("terms.animal_adjectives.zebra_like", "'zebra-like' can be 'zebrine'.", r"zebra[- ]like").raw().with_severity(Severity::Suggestion).with_replacement("zebrine"),

        // === Additional denizen labels from proselint ===
        Check::new("terms.denizen.person_from_beijing", "'person from Beijing' can be 'Beijinger'.", r"person from Beijing").with_severity(Severity::Suggestion).with_replacement("Beijinger"),
        Check::new("terms.denizen.person_from_berlin", "'person from Berlin' can be 'Berliner'.", r"person from Berlin").with_severity(Severity::Suggestion).with_replacement("Berliner"),
        Check::new("terms.denizen.person_from_boston", "'person from Boston' can be 'Bostonian'.", r"person from Boston").with_severity(Severity::Suggestion).with_replacement("Bostonian"),
        Check::new("terms.denizen.person_from_cairo", "'person from Cairo' can be 'Cairene'.", r"person from Cairo").with_severity(Severity::Suggestion).with_replacement("Cairene"),
        Check::new("terms.denizen.person_from_chicago", "'person from Chicago' can be 'Chicagoan'.", r"person from Chicago").with_severity(Severity::Suggestion).with_replacement("Chicagoan"),
        Check::new("terms.denizen.person_from_copenhagen", "'person from Copenhagen' can be 'Copenhagener'.", r"person from Copenhagen").with_severity(Severity::Suggestion).with_replacement("Copenhagener"),
        Check::new("terms.denizen.person_from_dallas", "'person from Dallas' can be 'Dallasite'.", r"person from Dallas").with_severity(Severity::Suggestion).with_replacement("Dallasite"),
        Check::new("terms.denizen.person_from_denver", "'person from Denver' can be 'Denverite'.", r"person from Denver").with_severity(Severity::Suggestion).with_replacement("Denverite"),
        Check::new("terms.denizen.person_from_detroit", "'person from Detroit' can be 'Detroiter'.", r"person from Detroit").with_severity(Severity::Suggestion).with_replacement("Detroiter"),
        Check::new("terms.denizen.person_from_dublin", "'person from Dublin' can be 'Dubliner'.", r"person from Dublin").with_severity(Severity::Suggestion).with_replacement("Dubliner"),
        Check::new("terms.denizen.person_from_edinburgh", "'person from Edinburgh' can be 'Edinburgher'.", r"person from Edinburgh").with_severity(Severity::Suggestion).with_replacement("Edinburgher"),
        Check::new("terms.denizen.person_from_florence", "'person from Florence' can be 'Florentine'.", r"person from Florence").with_severity(Severity::Suggestion).with_replacement("Florentine"),
        Check::new("terms.denizen.person_from_hamburg", "'person from Hamburg' can be 'Hamburger'.", r"person from Hamburg").with_severity(Severity::Suggestion).with_replacement("Hamburger"),
        Check::new("terms.denizen.person_from_hong_kong", "'person from Hong Kong' can be 'Hong Konger'.", r"person from Hong Kong").with_severity(Severity::Suggestion).with_replacement("Hong Konger"),
        Check::new("terms.denizen.person_from_houston", "'person from Houston' can be 'Houstonian'.", r"person from Houston").with_severity(Severity::Suggestion).with_replacement("Houstonian"),
        Check::new("terms.denizen.person_from_lisbon", "'person from Lisbon' can be 'Lisboner' or 'Lisbonite'.", r"person from Lisbon").with_severity(Severity::Suggestion).with_replacement("Lisboner"),
        Check::new("terms.denizen.person_from_los_angeles", "'person from Los Angeles' can be 'Angeleno'.", r"person from Los Angeles").with_severity(Severity::Suggestion).with_replacement("Angeleno"),
        Check::new("terms.denizen.person_from_madrid", "'person from Madrid' can be 'Madrileno'.", r"person from Madrid").with_severity(Severity::Suggestion).with_replacement("Madrileno"),
        Check::new("terms.denizen.person_from_manchester", "'person from Manchester' can be 'Mancunian'.", r"person from Manchester").with_severity(Severity::Suggestion).with_replacement("Mancunian"),
        Check::new("terms.denizen.person_from_melbourne", "'person from Melbourne' can be 'Melburnian'.", r"person from Melbourne").with_severity(Severity::Suggestion).with_replacement("Melburnian"),
        Check::new("terms.denizen.person_from_miami", "'person from Miami' can be 'Miamian'.", r"person from Miami").with_severity(Severity::Suggestion).with_replacement("Miamian"),
        Check::new("terms.denizen.person_from_milan", "'person from Milan' can be 'Milanese'.", r"person from Milan").with_severity(Severity::Suggestion).with_replacement("Milanese"),
        Check::new("terms.denizen.person_from_montreal", "'person from Montreal' can be 'Montrealer'.", r"person from Montreal").with_severity(Severity::Suggestion).with_replacement("Montrealer"),
        Check::new("terms.denizen.person_from_munich", "'person from Munich' can be 'Municher'.", r"person from Munich").with_severity(Severity::Suggestion).with_replacement("Municher"),
        Check::new("terms.denizen.person_from_oslo", "'person from Oslo' can be 'Oslovian'.", r"person from Oslo").with_severity(Severity::Suggestion).with_replacement("Oslovian"),
        Check::new("terms.denizen.person_from_philadelphia", "'person from Philadelphia' can be 'Philadelphian'.", r"person from Philadelphia").with_severity(Severity::Suggestion).with_replacement("Philadelphian"),
        Check::new("terms.denizen.person_from_phoenix", "'person from Phoenix' can be 'Phoenician'.", r"person from Phoenix").with_severity(Severity::Suggestion).with_replacement("Phoenician"),
        Check::new("terms.denizen.person_from_prague", "'person from Prague' can be 'Praguer'.", r"person from Prague").with_severity(Severity::Suggestion).with_replacement("Praguer"),
        Check::new("terms.denizen.person_from_rome", "'person from Rome' can be 'Roman'.", r"person from Rome").with_severity(Severity::Suggestion).with_replacement("Roman"),
        Check::new("terms.denizen.person_from_san_francisco", "'person from San Francisco' can be 'San Franciscan'.", r"person from San Francisco").with_severity(Severity::Suggestion).with_replacement("San Franciscan"),
        Check::new("terms.denizen.person_from_seattle", "'person from Seattle' can be 'Seattleite'.", r"person from Seattle").with_severity(Severity::Suggestion).with_replacement("Seattleite"),
        Check::new("terms.denizen.person_from_stockholm", "'person from Stockholm' can be 'Stockholmer'.", r"person from Stockholm").with_severity(Severity::Suggestion).with_replacement("Stockholmer"),
        Check::new("terms.denizen.person_from_sydney", "'person from Sydney' can be 'Sydneysider'.", r"person from Sydney").with_severity(Severity::Suggestion).with_replacement("Sydneysider"),
        Check::new("terms.denizen.person_from_tokyo", "'person from Tokyo' can be 'Tokyoite'.", r"person from Tokyo").with_severity(Severity::Suggestion).with_replacement("Tokyoite"),
        Check::new("terms.denizen.person_from_toronto", "'person from Toronto' can be 'Torontonian'.", r"person from Toronto").with_severity(Severity::Suggestion).with_replacement("Torontonian"),
        Check::new("terms.denizen.person_from_vancouver", "'person from Vancouver' can be 'Vancouverite'.", r"person from Vancouver").with_severity(Severity::Suggestion).with_replacement("Vancouverite"),
        Check::new("terms.denizen.person_from_venice", "'person from Venice' can be 'Venetian'.", r"person from Venice").with_severity(Severity::Suggestion).with_replacement("Venetian"),
        Check::new("terms.denizen.person_from_vienna", "'person from Vienna' can be 'Viennese'.", r"person from Vienna").with_severity(Severity::Suggestion).with_replacement("Viennese"),
        Check::new("terms.denizen.person_from_zurich", "'person from Zurich' can be 'Zuricher'.", r"person from Zurich").with_severity(Severity::Suggestion).with_replacement("Zuricher"),

        // === Additional venery (collective nouns) from proselint ===
        Check::new("terms.venery.group_of_baboons", "'group of baboons' is 'troop of baboons' or 'flange of baboons'.", r"group of baboons").with_severity(Severity::Suggestion).with_replacement("troop of baboons"),
        Check::new("terms.venery.group_of_badgers", "'group of badgers' is 'cete of badgers'.", r"group of badgers").with_severity(Severity::Suggestion).with_replacement("cete of badgers"),
        Check::new("terms.venery.group_of_bass", "'group of bass' is 'shoal of bass'.", r"group of bass").with_severity(Severity::Suggestion).with_replacement("shoal of bass"),
        Check::new("terms.venery.group_of_bats", "'group of bats' is 'colony of bats' or 'cloud of bats'.", r"group of bats").with_severity(Severity::Suggestion).with_replacement("colony of bats"),
        Check::new("terms.venery.group_of_bears", "'group of bears' is 'sleuth of bears'.", r"group of bears").with_severity(Severity::Suggestion).with_replacement("sleuth of bears"),
        Check::new("terms.venery.group_of_boars", "'group of boars' is 'sounder of boars'.", r"group of boars").with_severity(Severity::Suggestion).with_replacement("sounder of boars"),
        Check::new("terms.venery.group_of_buffalo", "'group of buffalo' is 'herd of buffalo' or 'obstinacy of buffalo'.", r"group of buffalo").with_severity(Severity::Suggestion).with_replacement("herd of buffalo"),
        Check::new("terms.venery.group_of_butterflies", "'group of butterflies' is 'flutter of butterflies' or 'kaleidoscope of butterflies'.", r"group of butterflies").with_severity(Severity::Suggestion).with_replacement("flutter of butterflies"),
        Check::new("terms.venery.group_of_camels", "'group of camels' is 'caravan of camels' or 'train of camels'.", r"group of camels").with_severity(Severity::Suggestion).with_replacement("caravan of camels"),
        Check::new("terms.venery.group_of_cats", "'group of cats' is 'clowder of cats'.", r"group of cats").with_severity(Severity::Suggestion).with_replacement("clowder of cats"),
        Check::new("terms.venery.group_of_cheetahs", "'group of cheetahs' is 'coalition of cheetahs'.", r"group of cheetahs").with_severity(Severity::Suggestion).with_replacement("coalition of cheetahs"),
        Check::new("terms.venery.group_of_cockroaches", "'group of cockroaches' is 'intrusion of cockroaches'.", r"group of cockroaches").with_severity(Severity::Suggestion).with_replacement("intrusion of cockroaches"),
        Check::new("terms.venery.group_of_crocodiles", "'group of crocodiles' is 'bask of crocodiles' or 'float of crocodiles'.", r"group of crocodiles").with_severity(Severity::Suggestion).with_replacement("bask of crocodiles"),
        Check::new("terms.venery.group_of_deer", "'group of deer' is 'herd of deer'.", r"group of deer").with_severity(Severity::Suggestion).with_replacement("herd of deer"),
        Check::new("terms.venery.group_of_dogs", "'group of dogs' is 'pack of dogs'.", r"group of dogs").with_severity(Severity::Suggestion).with_replacement("pack of dogs"),
        Check::new("terms.venery.group_of_dolphins", "'group of dolphins' is 'pod of dolphins'.", r"group of dolphins").with_severity(Severity::Suggestion).with_replacement("pod of dolphins"),
        Check::new("terms.venery.group_of_donkeys", "'group of donkeys' is 'drove of donkeys' or 'pace of donkeys'.", r"group of donkeys").with_severity(Severity::Suggestion).with_replacement("drove of donkeys"),
        Check::new("terms.venery.group_of_doves", "'group of doves' is 'dole of doves'.", r"group of doves").with_severity(Severity::Suggestion).with_replacement("dole of doves"),
        Check::new("terms.venery.group_of_dragonflies", "'group of dragonflies' is 'swarm of dragonflies'.", r"group of dragonflies").with_severity(Severity::Suggestion).with_replacement("swarm of dragonflies"),
        Check::new("terms.venery.group_of_ducks", "'group of ducks' is 'raft of ducks' (on water) or 'paddling of ducks'.", r"group of ducks").with_severity(Severity::Suggestion).with_replacement("raft of ducks"),
        Check::new("terms.venery.group_of_eagles", "'group of eagles' is 'convocation of eagles'.", r"group of eagles").with_severity(Severity::Suggestion).with_replacement("convocation of eagles"),
        Check::new("terms.venery.group_of_eels", "'group of eels' is 'swarm of eels' or 'bed of eels'.", r"group of eels").with_severity(Severity::Suggestion).with_replacement("swarm of eels"),
        Check::new("terms.venery.group_of_elk", "'group of elk' is 'gang of elk' or 'herd of elk'.", r"group of elk").with_severity(Severity::Suggestion).with_replacement("herd of elk"),
        Check::new("terms.venery.group_of_ferrets", "'group of ferrets' is 'business of ferrets'.", r"group of ferrets").with_severity(Severity::Suggestion).with_replacement("business of ferrets"),
        Check::new("terms.venery.group_of_finches", "'group of finches' is 'charm of finches'.", r"group of finches").with_severity(Severity::Suggestion).with_replacement("charm of finches"),
        Check::new("terms.venery.group_of_foxes", "'group of foxes' is 'skulk of foxes' or 'leash of foxes'.", r"group of foxes").with_severity(Severity::Suggestion).with_replacement("skulk of foxes"),
        Check::new("terms.venery.group_of_frogs", "'group of frogs' is 'army of frogs'.", r"group of frogs").with_severity(Severity::Suggestion).with_replacement("army of frogs"),
        Check::new("terms.venery.group_of_giraffes", "'group of giraffes' is 'tower of giraffes'.", r"group of giraffes").with_severity(Severity::Suggestion).with_replacement("tower of giraffes"),
        Check::new("terms.venery.group_of_gnats", "'group of gnats' is 'cloud of gnats' or 'swarm of gnats'.", r"group of gnats").with_severity(Severity::Suggestion).with_replacement("cloud of gnats"),
        Check::new("terms.venery.group_of_goats", "'group of goats' is 'tribe of goats' or 'trip of goats'.", r"group of goats").with_severity(Severity::Suggestion).with_replacement("tribe of goats"),
        Check::new("terms.venery.group_of_goldfish", "'group of goldfish' is 'troubling of goldfish'.", r"group of goldfish").with_severity(Severity::Suggestion).with_replacement("troubling of goldfish"),
        Check::new("terms.venery.group_of_gorillas", "'group of gorillas' is 'band of gorillas' or 'troop of gorillas'.", r"group of gorillas").with_severity(Severity::Suggestion).with_replacement("band of gorillas"),
        Check::new("terms.venery.group_of_grasshoppers", "'group of grasshoppers' is 'cloud of grasshoppers'.", r"group of grasshoppers").with_severity(Severity::Suggestion).with_replacement("cloud of grasshoppers"),
        Check::new("terms.venery.group_of_hawks", "'group of hawks' is 'cast of hawks' or 'kettle of hawks'.", r"group of hawks").with_severity(Severity::Suggestion).with_replacement("cast of hawks"),
        Check::new("terms.venery.group_of_hedgehogs", "'group of hedgehogs' is 'array of hedgehogs' or 'prickle of hedgehogs'.", r"group of hedgehogs").with_severity(Severity::Suggestion).with_replacement("array of hedgehogs"),
        Check::new("terms.venery.group_of_herons", "'group of herons' is 'siege of herons'.", r"group of herons").with_severity(Severity::Suggestion).with_replacement("siege of herons"),
        Check::new("terms.venery.group_of_hippopotami", "'group of hippopotami' is 'bloat of hippopotami'.", r"group of hippopotam(?:i|uses)").raw().with_severity(Severity::Suggestion).with_replacement("bloat of hippopotami"),
        Check::new("terms.venery.group_of_hornets", "'group of hornets' is 'nest of hornets' or 'swarm of hornets'.", r"group of hornets").with_severity(Severity::Suggestion).with_replacement("nest of hornets"),
        Check::new("terms.venery.group_of_horses", "'group of horses' is 'herd of horses' or 'string of horses'.", r"group of horses").with_severity(Severity::Suggestion).with_replacement("herd of horses"),
        Check::new("terms.venery.group_of_hummingbirds", "'group of hummingbirds' is 'charm of hummingbirds'.", r"group of hummingbirds").with_severity(Severity::Suggestion).with_replacement("charm of hummingbirds"),
        Check::new("terms.venery.group_of_hyenas", "'group of hyenas' is 'cackle of hyenas'.", r"group of hyenas").with_severity(Severity::Suggestion).with_replacement("cackle of hyenas"),
        Check::new("terms.venery.group_of_jaguars", "'group of jaguars' is 'shadow of jaguars'.", r"group of jaguars").with_severity(Severity::Suggestion).with_replacement("shadow of jaguars"),
        Check::new("terms.venery.group_of_jellyfish", "'group of jellyfish' is 'smack of jellyfish'.", r"group of jellyfish").with_severity(Severity::Suggestion).with_replacement("smack of jellyfish"),
        Check::new("terms.venery.group_of_kangaroos", "'group of kangaroos' is 'mob of kangaroos' or 'troop of kangaroos'.", r"group of kangaroos").with_severity(Severity::Suggestion).with_replacement("mob of kangaroos"),
        Check::new("terms.venery.group_of_kittens", "'group of kittens' is 'kindle of kittens' or 'litter of kittens'.", r"group of kittens").with_severity(Severity::Suggestion).with_replacement("kindle of kittens"),
        Check::new("terms.venery.group_of_larks", "'group of larks' is 'exaltation of larks'.", r"group of larks").with_severity(Severity::Suggestion).with_replacement("exaltation of larks"),
        Check::new("terms.venery.group_of_lemurs", "'group of lemurs' is 'conspiracy of lemurs'.", r"group of lemurs").with_severity(Severity::Suggestion).with_replacement("conspiracy of lemurs"),
        Check::new("terms.venery.group_of_leopards", "'group of leopards' is 'leap of leopards'.", r"group of leopards").with_severity(Severity::Suggestion).with_replacement("leap of leopards"),
        Check::new("terms.venery.group_of_locusts", "'group of locusts' is 'plague of locusts' or 'swarm of locusts'.", r"group of locusts").with_severity(Severity::Suggestion).with_replacement("plague of locusts"),
        Check::new("terms.venery.group_of_magpies", "'group of magpies' is 'tiding of magpies' or 'murder of magpies'.", r"group of magpies").with_severity(Severity::Suggestion).with_replacement("tiding of magpies"),
        Check::new("terms.venery.group_of_mice", "'group of mice' is 'mischief of mice'.", r"group of mice").with_severity(Severity::Suggestion).with_replacement("mischief of mice"),
        Check::new("terms.venery.group_of_moles", "'group of moles' is 'labor of moles'.", r"group of moles").with_severity(Severity::Suggestion).with_replacement("labor of moles"),
        Check::new("terms.venery.group_of_monkeys", "'group of monkeys' is 'troop of monkeys' or 'barrel of monkeys'.", r"group of monkeys").with_severity(Severity::Suggestion).with_replacement("troop of monkeys"),
        Check::new("terms.venery.group_of_moose", "'group of moose' is 'herd of moose'.", r"group of moose").with_severity(Severity::Suggestion).with_replacement("herd of moose"),
        Check::new("terms.venery.group_of_mosquitoes", "'group of mosquitoes' is 'swarm of mosquitoes'.", r"group of mosquitoes").with_severity(Severity::Suggestion).with_replacement("swarm of mosquitoes"),
        Check::new("terms.venery.group_of_mules", "'group of mules' is 'pack of mules' or 'barren of mules'.", r"group of mules").with_severity(Severity::Suggestion).with_replacement("pack of mules"),
        Check::new("terms.venery.group_of_nightingales", "'group of nightingales' is 'watch of nightingales'.", r"group of nightingales").with_severity(Severity::Suggestion).with_replacement("watch of nightingales"),
        Check::new("terms.venery.group_of_otters", "'group of otters' is 'romp of otters' or 'raft of otters'.", r"group of otters").with_severity(Severity::Suggestion).with_replacement("romp of otters"),
        Check::new("terms.venery.group_of_oxen", "'group of oxen' is 'team of oxen' or 'yoke of oxen'.", r"group of oxen").with_severity(Severity::Suggestion).with_replacement("team of oxen"),
        Check::new("terms.venery.group_of_oysters", "'group of oysters' is 'bed of oysters'.", r"group of oysters").with_severity(Severity::Suggestion).with_replacement("bed of oysters"),
        Check::new("terms.venery.group_of_pandas", "'group of pandas' is 'embarrassment of pandas'.", r"group of pandas").with_severity(Severity::Suggestion).with_replacement("embarrassment of pandas"),
        Check::new("terms.venery.group_of_parrots", "'group of parrots' is 'pandemonium of parrots' or 'company of parrots'.", r"group of parrots").with_severity(Severity::Suggestion).with_replacement("pandemonium of parrots"),
        Check::new("terms.venery.group_of_peacocks", "'group of peacocks' is 'ostentation of peacocks' or 'muster of peacocks'.", r"group of peacocks").with_severity(Severity::Suggestion).with_replacement("ostentation of peacocks"),
        Check::new("terms.venery.group_of_pelicans", "'group of pelicans' is 'pod of pelicans'.", r"group of pelicans").with_severity(Severity::Suggestion).with_replacement("pod of pelicans"),
        Check::new("terms.venery.group_of_penguins", "'group of penguins' is 'colony of penguins' or 'waddle of penguins'.", r"group of penguins").with_severity(Severity::Suggestion).with_replacement("colony of penguins"),
        Check::new("terms.venery.group_of_pheasants", "'group of pheasants' is 'bouquet of pheasants'.", r"group of pheasants").with_severity(Severity::Suggestion).with_replacement("bouquet of pheasants"),
        Check::new("terms.venery.group_of_pigs", "'group of pigs' is 'drove of pigs' or 'sounder of pigs'.", r"group of pigs").with_severity(Severity::Suggestion).with_replacement("drove of pigs"),
        Check::new("terms.venery.group_of_platypuses", "'group of platypuses' is 'paddle of platypuses'.", r"group of platypuses").with_severity(Severity::Suggestion).with_replacement("paddle of platypuses"),
        Check::new("terms.venery.group_of_porcupines", "'group of porcupines' is 'prickle of porcupines'.", r"group of porcupines").with_severity(Severity::Suggestion).with_replacement("prickle of porcupines"),
        Check::new("terms.venery.group_of_porpoises", "'group of porpoises' is 'pod of porpoises'.", r"group of porpoises").with_severity(Severity::Suggestion).with_replacement("pod of porpoises"),
        Check::new("terms.venery.group_of_prairie_dogs", "'group of prairie dogs' is 'coterie of prairie dogs'.", r"group of prairie dogs").with_severity(Severity::Suggestion).with_replacement("coterie of prairie dogs"),
        Check::new("terms.venery.group_of_quail", "'group of quail' is 'bevy of quail' or 'covey of quail'.", r"group of quail").with_severity(Severity::Suggestion).with_replacement("bevy of quail"),
        Check::new("terms.venery.group_of_rabbits", "'group of rabbits' is 'colony of rabbits' or 'warren of rabbits'.", r"group of rabbits").with_severity(Severity::Suggestion).with_replacement("colony of rabbits"),
        Check::new("terms.venery.group_of_raccoons", "'group of raccoons' is 'gaze of raccoons'.", r"group of raccoons").with_severity(Severity::Suggestion).with_replacement("gaze of raccoons"),
        Check::new("terms.venery.group_of_rats", "'group of rats' is 'mischief of rats' or 'horde of rats'.", r"group of rats").with_severity(Severity::Suggestion).with_replacement("mischief of rats"),
        Check::new("terms.venery.group_of_rhinoceroses", "'group of rhinoceroses' is 'crash of rhinoceroses'.", r"group of rhinoceroses").with_severity(Severity::Suggestion).with_replacement("crash of rhinoceroses"),
        Check::new("terms.venery.group_of_salmon", "'group of salmon' is 'run of salmon'.", r"group of salmon").with_severity(Severity::Suggestion).with_replacement("run of salmon"),
        Check::new("terms.venery.group_of_sardines", "'group of sardines' is 'family of sardines'.", r"group of sardines").with_severity(Severity::Suggestion).with_replacement("family of sardines"),
        Check::new("terms.venery.group_of_scorpions", "'group of scorpions' is 'bed of scorpions' or 'nest of scorpions'.", r"group of scorpions").with_severity(Severity::Suggestion).with_replacement("bed of scorpions"),
        Check::new("terms.venery.group_of_seagulls", "'group of seagulls' is 'flock of seagulls' or 'colony of seagulls'.", r"group of seagulls").with_severity(Severity::Suggestion).with_replacement("flock of seagulls"),
        Check::new("terms.venery.group_of_seals", "'group of seals' is 'pod of seals' or 'colony of seals'.", r"group of seals").with_severity(Severity::Suggestion).with_replacement("pod of seals"),
        Check::new("terms.venery.group_of_sharks", "'group of sharks' is 'shiver of sharks'.", r"group of sharks").with_severity(Severity::Suggestion).with_replacement("shiver of sharks"),
        Check::new("terms.venery.group_of_sheep", "'group of sheep' is 'flock of sheep'.", r"group of sheep").with_severity(Severity::Suggestion).with_replacement("flock of sheep"),
        Check::new("terms.venery.group_of_skunks", "'group of skunks' is 'stench of skunks'.", r"group of skunks").with_severity(Severity::Suggestion).with_replacement("stench of skunks"),
        Check::new("terms.venery.group_of_snakes", "'group of snakes' is 'nest of snakes' or 'pit of snakes'.", r"group of snakes").with_severity(Severity::Suggestion).with_replacement("nest of snakes"),
        Check::new("terms.venery.group_of_sparrows", "'group of sparrows' is 'host of sparrows'.", r"group of sparrows").with_severity(Severity::Suggestion).with_replacement("host of sparrows"),
        Check::new("terms.venery.group_of_spiders", "'group of spiders' is 'cluster of spiders' or 'clutter of spiders'.", r"group of spiders").with_severity(Severity::Suggestion).with_replacement("cluster of spiders"),
        Check::new("terms.venery.group_of_squids", "'group of squids' is 'squad of squids'.", r"group of squids").with_severity(Severity::Suggestion).with_replacement("squad of squids"),
        Check::new("terms.venery.group_of_squirrels", "'group of squirrels' is 'scurry of squirrels' or 'dray of squirrels'.", r"group of squirrels").with_severity(Severity::Suggestion).with_replacement("scurry of squirrels"),
        Check::new("terms.venery.group_of_starlings", "'group of starlings' is 'murmuration of starlings'.", r"group of starlings").with_severity(Severity::Suggestion).with_replacement("murmuration of starlings"),
        Check::new("terms.venery.group_of_stingrays", "'group of stingrays' is 'fever of stingrays'.", r"group of stingrays").with_severity(Severity::Suggestion).with_replacement("fever of stingrays"),
        Check::new("terms.venery.group_of_storks", "'group of storks' is 'muster of storks' or 'mustering of storks'.", r"group of storks").with_severity(Severity::Suggestion).with_replacement("muster of storks"),
        Check::new("terms.venery.group_of_swallows", "'group of swallows' is 'flight of swallows'.", r"group of swallows").with_severity(Severity::Suggestion).with_replacement("flight of swallows"),
        Check::new("terms.venery.group_of_swans", "'group of swans' is 'bevy of swans' or 'wedge of swans'.", r"group of swans").with_severity(Severity::Suggestion).with_replacement("bevy of swans"),
        Check::new("terms.venery.group_of_termites", "'group of termites' is 'colony of termites' or 'swarm of termites'.", r"group of termites").with_severity(Severity::Suggestion).with_replacement("colony of termites"),
        Check::new("terms.venery.group_of_tigers", "'group of tigers' is 'streak of tigers' or 'ambush of tigers'.", r"group of tigers").with_severity(Severity::Suggestion).with_replacement("streak of tigers"),
        Check::new("terms.venery.group_of_toads", "'group of toads' is 'knot of toads'.", r"group of toads").with_severity(Severity::Suggestion).with_replacement("knot of toads"),
        Check::new("terms.venery.group_of_trout", "'group of trout' is 'hover of trout'.", r"group of trout").with_severity(Severity::Suggestion).with_replacement("hover of trout"),
        Check::new("terms.venery.group_of_turkeys", "'group of turkeys' is 'rafter of turkeys'.", r"group of turkeys").with_severity(Severity::Suggestion).with_replacement("rafter of turkeys"),
        Check::new("terms.venery.group_of_turtles", "'group of turtles' is 'bale of turtles'.", r"group of turtles").with_severity(Severity::Suggestion).with_replacement("bale of turtles"),
        Check::new("terms.venery.group_of_vultures", "'group of vultures' is 'committee of vultures' or 'wake of vultures'.", r"group of vultures").with_severity(Severity::Suggestion).with_replacement("committee of vultures"),
        Check::new("terms.venery.group_of_walruses", "'group of walruses' is 'herd of walruses' or 'pod of walruses'.", r"group of walruses").with_severity(Severity::Suggestion).with_replacement("herd of walruses"),
        Check::new("terms.venery.group_of_wasps", "'group of wasps' is 'swarm of wasps' or 'nest of wasps'.", r"group of wasps").with_severity(Severity::Suggestion).with_replacement("swarm of wasps"),
        Check::new("terms.venery.group_of_weasels", "'group of weasels' is 'gang of weasels' or 'sneak of weasels'.", r"group of weasels").with_severity(Severity::Suggestion).with_replacement("gang of weasels"),
        Check::new("terms.venery.group_of_woodpeckers", "'group of woodpeckers' is 'descent of woodpeckers'.", r"group of woodpeckers").with_severity(Severity::Suggestion).with_replacement("descent of woodpeckers"),
        Check::new("terms.venery.group_of_worms", "'group of worms' is 'bed of worms' or 'clew of worms'.", r"group of worms").with_severity(Severity::Suggestion).with_replacement("bed of worms"),
        Check::new("terms.venery.group_of_yaks", "'group of yaks' is 'herd of yaks'.", r"group of yaks").with_severity(Severity::Suggestion).with_replacement("herd of yaks"),
        Check::new("terms.venery.group_of_zebras", "'group of zebras' is 'dazzle of zebras' or 'zeal of zebras'.", r"group of zebras").with_severity(Severity::Suggestion).with_replacement("dazzle of zebras"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_adjective() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "terms.animal_adjectives.wolf_like").unwrap();
        let results = check.run("He had a wolf-like grin.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_venery() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "terms.venery.group_of_crows").unwrap();
        let results = check.run("A group of crows flew overhead.");
        assert_eq!(results.len(), 1);
    }
}
