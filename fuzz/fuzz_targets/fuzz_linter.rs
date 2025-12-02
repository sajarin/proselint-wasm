#![no_main]

use libfuzzer_sys::fuzz_target;
use proselint_wasm::Linter;

fuzz_target!(|data: &[u8]| {
    // Try to convert bytes to UTF-8 string
    if let Ok(text) = std::str::from_utf8(data) {
        let linter = Linter::new();

        // Should never panic regardless of input
        let _results = linter.check(text);
    }
});
