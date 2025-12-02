//! Date and time checks for proselint-wasm
//!
//! Detects incorrect date/time formatting and redundancies.

use crate::check::{Check, Severity};

/// Get all date/time checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // AM/PM redundancies
        Check::new(
            "dates_times.am_morning",
            "'AM in the morning' is redundant. Use 'AM' or 'in the morning'.",
            r"\d{1,2}\s*(?:a\.?m\.?|AM)\s+in the morning",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "dates_times.pm_evening",
            "'PM in the evening' is redundant. Use 'PM' or 'in the evening'.",
            r"\d{1,2}\s*(?:p\.?m\.?|PM)\s+in the (?:evening|afternoon)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "dates_times.pm_night",
            "'PM at night' is redundant. Use 'PM' or 'at night'.",
            r"\d{1,2}\s*(?:p\.?m\.?|PM)\s+at night",
        )
        .raw()
        .with_severity(Severity::Warning),

        // 12 AM/PM confusion
        Check::new(
            "dates_times.12am_midnight",
            "'12 AM' is ambiguous. Use 'midnight' or 'noon' for clarity.",
            r"12\s*(?:a\.?m\.?|AM)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "dates_times.12pm_noon",
            "'12 PM' is ambiguous. Use 'noon' or 'midnight' for clarity.",
            r"12\s*(?:p\.?m\.?|PM)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // Redundant date expressions
        Check::new(
            "dates_times.past_history",
            "'past history' is redundant. History is always in the past.",
            r"past history",
        )
        .with_severity(Severity::Warning)
        .with_replacement("history"),

        Check::new(
            "dates_times.future_plans",
            "'future plans' is redundant. Plans are always for the future.",
            r"future plans",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("plans"),

        Check::new(
            "dates_times.advance_planning",
            "'advance planning' is redundant. Planning is always in advance.",
            r"advance planning",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("planning"),

        Check::new(
            "dates_times.prior_planning",
            "'prior planning' is redundant. Planning is always prior.",
            r"prior planning",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("planning"),

        Check::new(
            "dates_times.advance_warning",
            "'advance warning' is redundant. Warnings are given in advance.",
            r"advance warning",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("warning"),

        // BC/AD issues
        Check::new(
            "dates_times.bc_bce",
            "'BC' and 'BCE' used together is redundant.",
            r"BC\s*/\s*BCE|BCE\s*/\s*BC",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "dates_times.ad_ce",
            "'AD' and 'CE' used together is redundant.",
            r"AD\s*/\s*CE|CE\s*/\s*AD",
        )
        .raw()
        .with_severity(Severity::Warning),

        // Decades
        Check::new(
            "dates_times.decades_apostrophe",
            "Decades don't need apostrophes. Use '1990s' not '1990's'.",
            r"\d{4}'s",
        )
        .raw()
        .with_severity(Severity::Warning),

        // Annual
        Check::new(
            "dates_times.annual_every_year",
            "'annual' already means every year. Don't add 'every year'.",
            r"annual(?:ly)? every year",
        )
        .raw()
        .with_severity(Severity::Warning),

        // Months
        Check::new(
            "dates_times.month_of",
            "'month of [Month]' is usually redundant. Just use the month name.",
            r"month of (?:January|February|March|April|May|June|July|August|September|October|November|December)",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        // Time redundancies
        Check::new(
            "dates_times.current_time",
            "'at the current time' is wordy. Use 'now' or 'currently'.",
            r"at the current time",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("now"),

        Check::new(
            "dates_times.at_this_point_in_time",
            "'at this point in time' is wordy. Use 'now' or 'currently'.",
            r"at this point in time",
        )
        .with_severity(Severity::Warning)
        .with_replacement("now"),

        Check::new(
            "dates_times.at_the_present_time",
            "'at the present time' is wordy. Use 'now' or 'currently'.",
            r"at the present time",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("now"),

        Check::new(
            "dates_times.period_of_time",
            "'period of time' is redundant. Use 'period' or 'time'.",
            r"period of time",
        )
        .with_severity(Severity::Suggestion)
        .with_replacement("period"),

        // Specific time issues
        Check::new(
            "dates_times.oclock_am",
            "'o'clock AM' is redundant. Use one or the other.",
            r"\d{1,2}\s*o'clock\s*(?:a\.?m\.?|AM|p\.?m\.?|PM)",
        )
        .raw()
        .with_severity(Severity::Warning),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_am_morning() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "dates_times.am_morning")
            .unwrap();
        let results = check.run("The meeting is at 8 AM in the morning.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_point_in_time() {
        let checks = get_checks();
        let check = checks
            .iter()
            .find(|c| c.id == "dates_times.at_this_point_in_time")
            .unwrap();
        let results = check.run("At this point in time, we need to decide.");
        assert_eq!(results.len(), 1);
    }
}
