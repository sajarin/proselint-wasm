//! Lint all documentation files with proselint-wasm

use proselint_wasm::Linter;
use std::fs;

fn is_table_divider_line(content: &str, line_num: usize) -> bool {
    if let Some(line) = content.lines().nth(line_num.saturating_sub(1)) {
        let trimmed = line.trim();
        // Check if line is a markdown table divider
        trimmed.starts_with('|') && trimmed.ends_with('|') &&
        trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace())
    } else {
        false
    }
}

fn has_ignore_directive(content: &str, line_num: usize) -> bool {
    if let Some(line) = content.lines().nth(line_num.saturating_sub(1)) {
        // Check for explicit ignore directive
        if line.contains("<!-- proselint-ignore -->") || line.contains("<!--proselint-ignore-->") {
            return true;
        }
        // Filter out markdown links (table of contents, etc.)
        if line.contains("](#") {
            return true;
        }
        // Filter out code comments (bash, etc.)
        let trimmed = line.trim();
        if trimmed.starts_with('#') && !trimmed.starts_with("##") {
            return true;
        }
        false
    } else {
        false
    }
}

fn main() {
    let linter = Linter::new();

    let docs = [
        "README.md",
        "CONTRIBUTING.md",
        "ARCHITECTURE.md",
        "CHANGELOG.md",
    ];

    for doc in &docs {
        println!("\n{}", "=".repeat(80));
        println!("Linting: {}", doc);
        println!("{}", "=".repeat(80));

        match fs::read_to_string(doc) {
            Ok(content) => {
                let results = linter.check(&content);

                // Filter out issues from markdown table dividers and ignored lines
                let filtered_results: Vec<_> = results.iter()
                    .filter(|r| !is_table_divider_line(&content, r.line))
                    .filter(|r| !has_ignore_directive(&content, r.line))
                    .collect();

                if filtered_results.is_empty() {
                    println!("✓ No issues found!");
                } else {
                    println!("Found {} issues:\n", filtered_results.len());
                    for result in &filtered_results {
                        println!("Line {}, Col {}: {}",
                            result.line,
                            result.column,
                            result.message
                        );
                        println!("  Check: {}", result.check);
                        if let Some(replacement) = &result.replacement {
                            println!("  Suggestion: {}", replacement);
                        }
                        println!();
                    }
                }
            }
            Err(e) => println!("Error reading {}: {}", doc, e),
        }
    }
}
