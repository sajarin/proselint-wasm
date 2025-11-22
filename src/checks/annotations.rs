//! Annotation checks for proselint-wasm
//!
//! Detects leftover editorial annotations and markup.

use crate::check::{Check, Severity};

/// Get all annotation checks
pub fn get_checks() -> Vec<Check> {
    vec![
        // Common editorial annotations
        Check::new(
            "annotations.todo",
            "Leftover TODO annotation found.",
            r"TODO:?",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.fixme",
            "Leftover FIXME annotation found.",
            r"FIXME:?",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.xxx",
            "Leftover XXX annotation found.",
            r"XXX:?",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.note_to_self",
            "Leftover 'note to self' annotation found.",
            r"note to self",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.tbd",
            "Leftover TBD (to be determined) annotation found.",
            r"TBD:?",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.tbc",
            "Leftover TBC (to be confirmed) annotation found.",
            r"TBC:?",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.insert",
            "Leftover [INSERT ...] placeholder found.",
            r"\[INSERT[^\]]*\]",
        )
        .raw()
        .with_severity(Severity::Error),

        Check::new(
            "annotations.add",
            "Leftover [ADD ...] placeholder found.",
            r"\[ADD[^\]]*\]",
        )
        .raw()
        .with_severity(Severity::Error),

        Check::new(
            "annotations.placeholder_brackets",
            "Leftover placeholder in brackets found.",
            r"\[(?:YOUR|MY|THE|THIS|COMPANY|NAME|DATE|TITLE)[^\]]*\]",
        )
        .raw()
        .with_severity(Severity::Error),

        Check::new(
            "annotations.check_this",
            "Leftover 'check this' annotation found.",
            r"check this",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.verify",
            "Leftover 'verify' or 'need to verify' annotation found.",
            r"(?:need to |please )?verify(?::|this| this)",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.citation_needed",
            "Leftover 'citation needed' annotation found.",
            r"citation needed|\[citation needed\]",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.ref_needed",
            "Leftover 'reference needed' annotation found.",
            r"(?:ref(?:erence)?|source) needed",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.editors_note",
            "Leftover editor's note found.",
            r"\[(?:editor'?s? note|ed\.?)\s*:",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.authors_note",
            "Leftover author's note found.",
            r"\[(?:author'?s? note|a\.?n\.?)\s*:",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.lorem_ipsum",
            "Placeholder 'lorem ipsum' text found.",
            r"lorem ipsum",
        )
        .with_severity(Severity::Error),

        Check::new(
            "annotations.delete_this",
            "Leftover 'delete this' annotation found.",
            r"delete this",
        )
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.rewrite",
            "Leftover 'rewrite' annotation found.",
            r"(?:needs? )?rewrite|rewrite this",
        )
        .raw()
        .with_severity(Severity::Warning),

        Check::new(
            "annotations.expand",
            "Leftover 'expand' annotation found.",
            r"(?:needs? to )?expand(?:ed| this| on this)?",
        )
        .raw()
        .with_severity(Severity::Suggestion),

        Check::new(
            "annotations.clarify",
            "Leftover 'clarify' annotation found.",
            r"(?:needs? )?clarif(?:y|ication)|clarify this",
        )
        .raw()
        .with_severity(Severity::Suggestion),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_annotation() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "annotations.todo").unwrap();
        let results = check.run("TODO: Fix this later.");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_lorem_ipsum() {
        let checks = get_checks();
        let check = checks.iter().find(|c| c.id == "annotations.lorem_ipsum").unwrap();
        let results = check.run("Lorem ipsum dolor sit amet.");
        assert_eq!(results.len(), 1);
    }
}
