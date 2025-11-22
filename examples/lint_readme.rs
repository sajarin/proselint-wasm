use proselint_wasm::Linter;
use std::fs;

fn main() {
    let readme = fs::read_to_string("README.md").expect("Failed to read README.md");
    let linter = Linter::new();
    let results = linter.check(&readme);

    if results.is_empty() {
        println!("No issues found in README.md");
    } else {
        println!("Found {} issues in README.md:\n", results.len());
        for result in &results {
            println!("{}", result);
        }
    }
}
