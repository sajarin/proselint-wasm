//! Mixed metaphor checks for proselint-wasm
//!
//! Detects inconsistent or mixed metaphors.

use crate::check::{Check, Severity};

/// Get all mixed metaphor checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // === From proselint: Bottleneck metaphors (big bottlenecks are easy to pass through!) ===
        Check::new(
            "mixed_metaphors.bottleneck.biggest",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"biggest bottleneck",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.big",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"big bottleneck",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.large",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"large bottleneck",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.largest",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"largest bottleneck",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.worldwide",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"world-?wide bottleneck",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.huge",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"huge bottleneck",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.bottleneck.massive",
            "Mixed metaphor - bottles with big necks are easy to pass through.",
            r"massive bottleneck",
        )
        .with_severity(Severity::Warning),

        // === From proselint: Other mixed metaphors ===
        Check::new(
            "mixed_metaphors.cream_crop",
            "Mixed metaphor. Try 'cream rises to the top'.",
            r"cream rises to the crop",
        )
        .with_severity(Severity::Error)
        .with_replacement("cream rises to the top"),

        Check::new(
            "mixed_metaphors.button_seatbelts",
            "Mixed metaphor. Try 'fasten your seatbelts'.",
            r"button your seatbelts",
        )
        .with_severity(Severity::Error)
        .with_replacement("fasten your seatbelts"),

        Check::new(
            "mixed_metaphors.decompose",
            "Mixed metaphor. Try 'a minute to decompress'.",
            r"a minute to decompose",
        )
        .with_severity(Severity::Error)
        .with_replacement("a minute to decompress"),

        Check::new(
            "mixed_metaphors.sharpest_marble",
            "Mixed metaphor. Try 'sharpest tool in the shed'.",
            r"sharpest marble in the (?:shed|box)",
        )
        .raw()
        .with_severity(Severity::Error)
        .with_replacement("sharpest tool in the shed"),

        Check::new(
            "mixed_metaphors.rocket_surgery",
            "Mixed metaphor. Try 'not rocket science'.",
            r"not rocket surgery",
        )
        .with_severity(Severity::Error)
        .with_replacement("not rocket science"),

        // === Additional mixed metaphors ===
        Check::new(
            "mixed_metaphors.beating_dead_horse",
            "Mixed metaphor: 'beating a dead horse' should not be mixed with other metaphors.",
            r"beat(?:ing)? a dead horse into the ground",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.burn_bridges",
            "Mixed metaphor: combining 'burn bridges' with 'water' imagery.",
            r"burn(?:ed|ing)? (?:all )?(?:your|his|her|their|our) bridges (?:and|while) (?:test|keep|stay)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.cross_that_bridge",
            "Mixed metaphor: 'burn that bridge when we come to it' mixes two phrases.",
            r"burn that bridge when (?:we|you|they|I) (?:come|get) to it",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.fish_out_of_water",
            "Mixed metaphor: 'fish out of water' mixed with flight imagery.",
            r"fish out of water (?:that|who) (?:flies|flew|flying|soar)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.hit_the_ground",
            "Mixed metaphor: 'hit the ground running' mixed with other actions.",
            r"hit the ground (?:swimming|flying|crawling)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.open_a_can_of_worms",
            "Mixed metaphor: 'open a can of worms' mixed with other containers.",
            r"open(?:ed|ing)? a (?:box|jar|bag) of worms",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "mixed_metaphors.rocket_science",
            "Mixed metaphor: combining 'rocket science' with 'brain surgery'.",
            r"(?:rocket|brain) (?:science|surgery) (?:and|or) (?:rocket|brain) (?:science|surgery)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "mixed_metaphors.tip_of_iceberg",
            "Mixed metaphor: 'tip of the iceberg' mixed with fire imagery.",
            r"tip of the iceberg (?:that's|which is) on fire",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.ball_in_court",
            "Mixed metaphor: sports metaphors should be consistent.",
            r"ball(?:'s| is)? in (?:your|their|his|her) court.+(?:home run|touchdown|goal)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "mixed_metaphors.wake_up_smell",
            "Mixed metaphor: 'wake up and smell the coffee' is often mixed incorrectly.",
            r"wake up and smell the (?:roses|bacon|music)",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_metaphor() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "mixed_metaphors.cross_that_bridge")
            .unwrap();
        let results = check.run("We'll burn that bridge when we come to it.");
        assert_eq!(results.len(), 1);
    }
}
