#![no_main]

use libfuzzer_sys::fuzz_target;
use proselint_wasm::{Check, Severity};

fuzz_target!(|data: &[u8]| {
    // Try to convert bytes to UTF-8 string
    if let Ok(pattern) = std::str::from_utf8(data) {
        // Don't allow patterns that are too long (DoS prevention)
        if pattern.len() > 1000 {
            return;
        }

        // Create a check with the fuzzed pattern
        let check = Check::new("fuzz.test", "Fuzz test", pattern)
            .with_severity(Severity::Warning);

        // Try to compile the regex - should never panic
        let _ = check.get_regex();

        // If regex compiles, try running it on some sample text
        if check.get_regex().is_some() {
            let sample_texts = [
                "",
                "a",
                "test",
                "This is a test.",
                "very very important",
                "Café élégant 日本語 👋",
            ];

            for text in &sample_texts {
                let _ = check.run(text);
            }
        }
    }
});
