# proselint-wasm

A high-performance Rust/WASM port of [proselint](https://github.com/amperser/proselint), a linter for English prose.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **🚀 High Performance**: Aho-Corasick pre-filtering for 60–80% faster linting
- **🌐 Dual API**: Native Rust API and WASM/JavaScript bindings
- **✅ 100+ Checks**: 18 categories covering typography, hedging, redundancy, clichés, and more
- **🔧 Configurable**: Enable/disable checks by category or ID, customize severity levels
- **🌍 UTF-8 Aware**: Proper handling of multi-byte characters (emoji, CJK, accented letters)
- **📝 Line Ending Support**: Unix (`\n`), Windows (`\r\n`), and old Mac (`\r`)
- **⚡ Parallel Processing**: Optional multi-threaded batch linting (native Rust)
- **🔒 Thread-Safe**: All components are `Send + Sync`
- **💯 Zero Dependencies**: Compiled to standalone WASM (1.4MB)

## Installation

### Rust

```toml
[dependencies]
proselint-wasm = "0.1"

# Enable parallel processing
proselint-wasm = { version = "0.1", features = ["parallel"] }
```

### JavaScript/WASM

```bash
npm install proselint-wasm
```

## Quick Start

### Rust

```rust
use proselint_wasm::{Linter, check_text};

// Quick check
let results = check_text("This is important.");

// With full control
let linter = Linter::new();
let results = linter.check("The project is near completion.");

for result in &results {
    println!("{}", result); // line:col [severity] message (check_id)
}

// Filter by severity
let errors = linter.check_errors(text);
let warnings = linter.check_warnings(text);

// Filter by category
let typography_issues = linter.check_category(text, "typography");
```

### JavaScript

```javascript
import init, { Proselint } from 'proselint-wasm';

await init();
const linter = new Proselint();
const results = JSON.parse(linter.lint("This is important."));

for (const issue of results) {
    console.log(`${issue.line}:${issue.column} [${issue.severity}] ${issue.message}`);
}
```

## Building from Source

### Prerequisites

- **Rust**: 1.70 or later ([install via rustup](https://rustup.rs/))
- **wasm-pack**: For building WASM (`cargo install wasm-pack`)
- **wasm-opt** (optional): For size optimization ([download](https://github.com/WebAssembly/binaryen/releases))

### Native Rust Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build —release

# With parallel feature
cargo build —release —features parallel

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### WASM Build

```bash
# Quick build (web target)
wasm-pack build —target web

# Optimized build with wasm-opt
./build-wasm.sh

# Or manually (run these commands in sequence):
wasm-pack build —release —target web
wasm-opt -Oz -o pkg/proselint_wasm_bg.wasm pkg/proselint_wasm_bg.wasm

# Other targets
wasm-pack build —target nodejs    # For Node.js
wasm-pack build —target bundler   # For webpack/rollup
```

## Advanced Usage

### Configuration

```rust
use proselint_wasm::{Linter, Config};

let mut config = Config::default();

// Disable entire category
config.disable("typography");

// Re-enable specific check
config.enable("typography.symbols.ellipsis");

// Disable quote checking
config.check_quotes = false;

// Limit results
config.max_errors = 10;

let linter = Linter::with_config(config);
```

### Parallel Batch Processing (Rust only)

```rust
#[cfg(feature = "parallel")]
{
    use proselint_wasm::Linter;

    let linter = Linter::new();
    let texts = vec!["Text 1", "Text 2", "Text 3"];

    // Parallel processing with rayon
    let results = linter.check_parallel(&texts);

    // Custom thread pool
    use rayon::ThreadPoolBuilder;
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let results = linter.check_parallel_with_pool(&pool, &texts);
}
```

### JavaScript Batch Linting

```javascript
const linter = new Proselint();

// Pre-compile all regexes for maximum performance
linter.warm_all();

// Batch linting
const texts = JSON.stringify(["Text 1", "Text 2", "Text 3"]);
const batchResults = JSON.parse(linter.lint_batch(texts));

// Count issues quickly
const count = linter.lint_count("This is important.");
```

### Custom Configuration (JavaScript)

```javascript
const config = JSON.stringify({
    check_quotes: false,
    max_errors: 10,
    checks: {
        "typography": false,           // Disable category
        "typography.symbols": true,    // Re-enable subcategory
        "weasel_words.very": false     // <!-- proselint-ignore -->Disable specific check
    }
});

// Create linter with custom config
const linter = Proselint.with_config(config);
```

## Check Categories

| Category | Description | Example Checks |
|----------|-------------|----------------|
| **typography** | Symbols, punctuation, and formatting | Ellipsis (`…`), em dash, curly quotes |
| **weasel_words** | Vague intensifiers and qualifiers | Overuse of modifiers |
| **hedging** | Weak or uncertain language | Tentative phrasing |
| **redundancy** | Redundant phrases and words | Unnecessary repetition |
| **cliches** | Overused expressions | Stale phrases |
| **archaism** | Outdated or archaic terms | While, among, here |
| **social_awareness** | Inclusive language | Gender-neutral alternatives |
| **malapropisms** | Commonly confused words | <!-- proselint-ignore -->Could have (not could of) |
| **mondegreens** | Misheard phrases | <!-- proselint-ignore -->For all intents and purposes |
| **mixed_metaphors** | Inconsistent metaphors | Cross that bridge when we come to it |
| **oxymorons** | Contradictory terms | Self-contradicting phrases |
| **spelling** | Common misspellings | <!-- proselint-ignore -->A lot (not alot), regardless |
| **terms** | Preferred terminology | Animal adjectives, venery terms |
| **needless_variants** | Unnecessary word variations | <!-- proselint-ignore -->Toward (not towards) |
| **skunked_terms** | Disputed usage terms | Contentious language |
| **uncomparables** | Words that shouldn't be compared | Unique, perfect |
| **psychology** | Mental health terminology | Casual use of clinical terms |
| **industrial_language** | Corporate jargon | Cooperation, return to, use |

## Performance

### Benchmarks

- **Small text** (1 sentence): ~0.5ms
- **Medium text** (1 paragraph): ~2ms
- **Large text** (1 page): ~5ms
- **Batch** (100 texts): ~500ms
- **Parallel** (100 texts, 4 cores): ~150ms

### Optimization Tips

1. **Pre-compile regexes**: Call `warm_all()` once at startup.
2. **Batch processing**: Use `lint_batch()` for multiple texts.
3. **Parallel processing**: Use `check_parallel()` for large batches. (Rust only)
4. **Disable unused checks**: Configure to run only needed checks.
5. **WASM size**: Use `wasm-opt -Oz` to reduce binary by ~30%.

## Thread Safety

**All components are thread-safe:**

- `Linter` and `Proselint` are `Send + Sync`
- Regex compilation uses `OnceLock` for thread-safe lazy initialization
- Safe to share across threads or use in async contexts

```rust
use std::sync::Arc;

let linter = Arc::new(Linter::new());

// Safe to clone and use in multiple threads
std::thread::scope(|s| {
    for text in texts {
        let linter = linter.clone();
        s.spawn(move || {
            let results = linter.check(text);
            // Process results…
        });
    }
});
```

**Note**: WASM runs in a single-threaded environment, but the code is written to be thread-safe for native Rust usage.

## Testing

```bash
# Unit tests
cargo test

# Property-based tests (quickcheck)
cargo test —all-features

# Integration tests
cargo test —test '*'

# WASM tests (Node.js)
wasm-pack build —target nodejs
node tests/wasm/test.mjs

# WASM tests (Browser)
wasm-pack build —target web
# Then open tests/wasm/test.html in browser

# Benchmarks
cargo bench

# Fuzzing
cargo install cargo-fuzz
cargo fuzz run fuzz_linter
cargo fuzz run fuzz_regex
```

## Input Validation

To prevent DoS attacks, the library enforces limits:

- **Max text size**: 10MB per text
- **Max batch size**: 100 texts per batch

These can be adjusted by modifying `MAX_TEXT_SIZE` and `MAX_BATCH_SIZE` constants in `src/lib.rs`.

## Error Handling

### Rust

```rust
let results = linter.check(text); // Never panics, returns Vec<LintResult>
```

### JavaScript

```javascript
const result = linter.lint(text);
const parsed = JSON.parse(result);

// Check for errors
if (parsed.error) {
    console.error(`Error: ${parsed.error}`);
} else {
    // Process results array
    for (const issue of parsed) {
        // …
    }
}
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

### Key Optimizations

1. **Aho-Corasick Pre-filtering**: Reduces regex checks by 60–80%
2. **Lazy Regex Compilation**: Compile once, cache forever with `OnceLock`
3. **Binary Search**: O(log n) quote and config lookups
4. **UTF-8 Aware**: Character-based column numbers for proper multi-byte support
5. **Aggressive WASM Opts**: LTO, single codegen unit, `panic = "abort"`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## License

MIT License - see [LICENSE](LICENSE) for details.

This project ports [proselint](https://github.com/amperser/proselint) by Amperser Labs (BSD-3-Clause). See [NOTICE](NOTICE) for attribution.

## Acknowledgments

- Original [proselint](https://github.com/amperser/proselint) by Jordan Suchow and contributors
- Built with Rust and wasm-bindgen
- Uses Aho-Corasick algorithm for fast keyword matching
