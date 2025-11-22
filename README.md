# proselint-wasm

A Rust/WASM port of [proselint](https://github.com/amperser/proselint), a linter for English prose.

## Features

- Native Rust API via `Linter` struct
- WASM/JavaScript API via `Proselint` struct
- 18 check categories covering typography, hedging, redundancy, cliches, and more
- Configurable severity levels and check filtering
- Zero runtime dependencies on Python

## Installation

### Rust

```toml
[dependencies]
proselint-wasm = "0.1"
```

### JavaScript/WASM

```bash
npm install proselint-wasm
```

## Usage

### Rust

```rust
use proselint_wasm::{Linter, check_text};

// Quick check
let results = check_text("This is very important.");

// With full control
let linter = Linter::new();
let results = linter.check("The project is in close proximity to completion.");

for result in &results {
    println!("{}", result);
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
const results = JSON.parse(linter.lint("This is very important."));

for (const issue of results) {
    console.log(`${issue.line}:${issue.column} [${issue.severity}] ${issue.message}`);
}
```

## Check Categories

| Category | Description |
|----------|-------------|
| typography | Symbols, punctuation, and formatting |
| hedging | Weak or uncertain language |
| redundancy | Redundant phrases and words |
| cliches | Overused expressions |
| weasel_words | Vague intensifiers and qualifiers |
| archaism | Outdated or archaic terms |
| social_awareness | Inclusive language suggestions |
| malapropisms | Commonly confused words |
| mondegreens | Misheard phrases |
| mixed_metaphors | Inconsistent metaphors |
| oxymorons | Contradictory terms |
| spelling | Common misspellings |
| terms | Preferred terminology |
| needless_variants | Unnecessary word variations |
| skunked_terms | Disputed usage terms |
| uncomparables | Words that should not be compared |
| psychology | Mental health terminology |
| industrial_language | Corporate jargon |

## Configuration

```rust
use proselint_wasm::{Linter, Config};

let mut config = Config::default();
config.disabled_checks.push("hedging.very".to_string());
config.min_severity = "warning".to_string();

let linter = Linter::with_config(config);
```

## License

MIT License - see [LICENSE](LICENSE) for details.

This project ports [proselint](https://github.com/amperser/proselint) by Amperser Labs (BSD-3-Clause). See [NOTICE](NOTICE) for attribution.
