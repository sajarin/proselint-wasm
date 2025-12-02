# Architecture Documentation

This document describes the architecture, design decisions, and implementation details of proselint-wasm.

## Table of Contents

- [Overview](#overview)
- [Core Design Principles](#core-design-principles)
- [Module Architecture](#module-architecture)
- [Performance Optimizations](#performance-optimizations)
- [Data Structures](#data-structures)
- [Thread Safety](#thread-safety)
- [WASM Considerations](#wasm-considerations)
- [Future Improvements](#future-improvements)

## Overview

proselint-wasm is a complete Rust/WASM port of the Python proselint library. The architecture provides:
- **High performance**: Optimized for client-side web usage
- **Zero allocations in hot paths**: After warm-up
- **UTF-8 correctness**: Proper handling of multi-byte characters
- **Dual API surface**: Both native Rust and WASM/JavaScript
- **Comprehensive checking**: 100+ prose linting rules

## Core Design Principles

### 1. Performance First

- **Aho-Corasick Pre-filtering**: Reduces regex overhead by 60–80%
- **Lazy Initialization**: Regexes compiled once, cached forever
- **Binary Search**: O(log n) lookups for quotes and config
- **Minimal Allocations**: Reuse data structures where possible

### 2. Safety and Correctness

- **No `unsafe` code**: Pure safe Rust (except necessary WASM FFI)
- **UTF-8 aware**: Character-based column numbers, not byte-based
- **Input validation**: Size limits to prevent DoS
- **Proper error handling**: No silent failures

### 3. Maintainability

- **Modular check organization**: 26 separate check modules.
- **Consistent patterns**: Builder pattern for checks.
- **Comprehensive tests**: 59 tests covering edge cases.
- **Documentation**: Inline docs for all public APIs.

## Module Architecture

```
proselint-wasm/
├── src/
│   ├── lib.rs              # Public API, WASM bindings
│   ├── engine.rs           # Linting engine with AC pre-filtering
│   ├── check.rs            # Check definitions and regex caching
│   ├── config.rs           # Configuration with prefix caching
│   ├── position.rs         # UTF-8-aware position tracking
│   └── checks/
│       ├── mod.rs          # Check registry
│       ├── typography.rs   # Typography checks (ellipsis, em dash, etc.)
│       ├── weasel_words.rs # Weasel word detection
│       ├── redundancy.rs   # Redundant phrase detection
│       └── …               # 23 more check modules
```

### lib.rs - Public API Layer

Two main structs provide dual API surfaces:

```rust
// Native Rust API
pub struct Linter {
    config: Config,
}

// WASM/JavaScript API
#[wasm_bindgen]
pub struct Proselint {
    config: Config,
}
```

**Design Decision**: Separate APIs for Rust and WASM users. The Rust API returns `Vec<LintResult>` directly, while the WASM API returns JSON strings for JavaScript interop.

**Thread Safety**: Both structs are `Send + Sync`. WASM is single-threaded, so we don't mark them explicitly.

### engine.rs - Linting Orchestration

The engine implements a **two-phase linting approach**:

#### Phase 1: Aho-Corasick Pre-filtering

```rust
// Build AC automaton from check keywords
let keywords: Vec<String> = checks.iter()
    .filter_map(|check| extract_keyword(check.pattern))
    .collect();

let ac = AhoCorasick::new(&keywords)?;

// Fast scan to find potentially matching checks
let matches = ac.find_iter(&text_lower);
```

**Why Aho-Corasick?**
- Single pass through text finds ALL keywords simultaneously.
- O(n + m) where n=text length, m=pattern length.
- Reduces regex checks from 100+ to typically 10–20 per document.
- 60–80% performance improvement on average documents.

#### Phase 2: Targeted Regex Matching

```rust
for check in potentially_matching_checks {
    if let Some(regex) = check.get_regex() {
        for match in regex.find_iter(text) {
            // Process match, check quotes, apply config
        }
    }
}
```

**Design Decision**: Only run regex on checks that have potential matches based on keyword presence.

### check.rs - Check Definitions and Caching

Three types of checks:

```rust
pub struct Check {
    id: &'static str,
    pattern: &'static str,
    compiled_regex: OnceLock<Option<Regex>>,  // Lazy compilation
    // …
}

pub struct ExistenceCheck { /* … */ }  // For word lists
pub struct PairCheck { /* … */ }       // For related phrases
```

**Regex Caching Strategy**:

```rust
pub fn get_regex(&self) -> Option<&Regex> {
    self.compiled_regex.get_or_init(|| {
        Regex::new(&pattern)
            .map_err(|e| eprintln!("ERROR: {}", e))  // Log errors
            .ok()
    }).as_ref()
}
```

**Why `OnceLock`?**
- Thread-safe initialization.
- Zero-cost after first call.
- No runtime overhead compared to manual caching.
- Guarantees single compilation per Check.

### position.rs - UTF-8 Position Tracking

Converts byte offsets (from regex) to line/column positions (for display).

#### LineTracker

```rust
pub struct LineTracker {
    line_ends: Vec<usize>,  // Byte offsets after each line ending
    text: String,           // Original text for char counting
}
```

**Key Design Decisions**:

1. **Store original text**: Required for UTF-8 character counting
2. **Byte-based line_ends**: Regex match offsets are bytes
3. **Character-based columns**: Display columns count chars, not bytes

**Line Ending Support**:
- Unix: `\n`
- Windows: `\r\n` (treated as single line break)
- Old Mac: `\r`
- Mixed: All three in same document

**UTF-8 Handling**:

```rust
// Count characters (not bytes) for column number
let column = self.text[line_start..offset]
    .chars()
    .count();
```

**Why store text?**: To support multi-byte UTF-8 characters like `é` (2 bytes), `日` (3 bytes), `👋` (4 bytes). Byte offsets from regex don't correspond to display columns.

#### QuoteTracker

Tracks quoted spans. This allows optional skipping of matches inside quotes.

```rust
pub struct QuoteTracker {
    spans: Vec<(usize, usize)>,  // Sorted list of (start, end) byte offsets
}
```

**Optimization**: Binary search for O(log n) quote position checks:

```rust
pub fn is_in_quote(&self, offset: usize) -> bool {
    match self.spans.binary_search_by_key(&offset, |&(start, _)| start) {
        Ok(_) => true,  // Exact match on quote start
        Err(idx) => {
            // Check if in previous span
            idx > 0 && {
                let (start, end) = self.spans[idx - 1];
                offset >= start && offset < end
            }
        }
    }
}
```

**Before**: O(n) linear search through all quote spans
**After**: O(log n) binary search
**Impact**: Significant on documents with many quotes

### config.rs - Configuration Management

```rust
pub struct Config {
    checks: HashMap<String, bool>,
    cache: HashMap<String, bool>,      // Not serialized
    prefixes: Vec<String>,              // Not serialized, sorted by length
}
```

**Configuration Strategies**:

1. **Prefix Matching**: `"typography"` matches `"typography.symbols.ellipsis"`
2. **Sorted Prefixes**: Longer prefixes checked first (more specific)
3. **Cache**: Avoid repeated HashMap lookups

**Example**:
```rust
config.disable("typography");  // Disables all typography checks
config.enable("typography.symbols");  // Re-enables symbols only
```

## Performance Optimizations

### 1. Aho-Corasick Pre-filtering

**Problem**: Running 100+ regexes on every document is slow.

**Solution**: Extract keywords from regex patterns, build Aho-Corasick automaton, filter checks based on keyword matches.

**Benefit**: 60–80% performance improvement on average documents.

### 2. Lazy Regex Compilation

**Problem**: Compiling 100+ regexes upfront takes time.

**Solution**: Use `OnceLock` for lazy compilation on first use.

```rust
static ALL_CHECKS: Lazy<Vec<Check>> = Lazy::new(build_all_checks);

pub fn warm_all() {
    for check in &*ALL_CHECKS {
        check.get_regex();  // Force compilation
    }
}
```

**Benefit**: Fast startup, optional pre-compilation with `warm_all()`.

### 3. Binary Search Optimizations

**QuoteTracker**: O(n) → O(log n)
**Config Lookups**: Sorted prefix list reduces iterations

### 4. Memory Efficiency

- **`Arc<Regex>`**: Share compiled regexes
- **`&'static str`**: Zero-copy for const strings
- **Reuse allocations**: `Vec::clear()` instead of new `Vec`

### 5. WASM Optimizations

```toml
[profile.release]
opt-level = 3        # Maximum settings
lto = true           # Link-time work
codegen-units = 1    # Single codegen unit for better results
panic = "abort"      # Smaller binary (no unwinding)
```

**Further improvement**: `wasm-opt -Oz` post-processing reduces binary by ~30%.

## Data Structures

### Why These Choices?

| Structure | Use Case | Rationale |
|———————|——————|———————|
| `OnceLock<Option<Regex>>` | Regex caching | Thread-safe lazy init, zero overhead |
| `Vec<(usize, usize)>` | Quote spans | Sorted for binary search |
| `HashMap<String, bool>` | Config checks | O(1) lookup, adaptable keys |
| `Vec<usize>` | Line endings | Binary searchable, minimal memory |
| `AhoCorasick` | Keyword matching | Simultaneous multi-pattern matching |

## Thread Safety

### Guarantees

**All components are thread-safe:**
- `OnceLock`: Provides thread-safe initialization
- `Lazy<Vec<Check>>`: One-time initialization, shared reads
- `RwLock<HashMap>`: For dynamic regex cache

**Safe for concurrent use:**
```rust
// Multiple threads can safely share a Linter
let linter = Arc::new(Linter::new());

std::thread::scope(|s| {
    for text in texts {
        let linter = linter.clone();
        s.spawn(move || linter.check(text));
    }
});
```

**WASM Limitation**: WASM provides a single-threaded environment. We write the code to be thread-safe for native Rust usage.

## WASM Considerations

### Binary Size

Target: <2MB (currently ~1.4MB)

Strategies:
- Aggressive LTO and improvement work
- `panic = "abort"` (no unwinding code)
- Post-processing with `wasm-opt -Oz`
- Minimal dependencies

### JavaScript Interop

**Input/Output**: JSON strings for complex data structures:

```rust
#[wasm_bindgen]
pub fn lint(&self, text: &str) -> String {
    let results = engine::lint_text(text, &self.config);
    serde_json::to_string(&results).unwrap_or_else(|e| {
        format!(r#"{{"error": "{}"}}"#, e)
    })
}
```

**Error Handling**: Return JSON error objects instead of throwing exceptions:
```json
{"error": "Text too large: 15000000 bytes (max 10000000 bytes)"}
```

### Input Validation

Protect against DoS:
```rust
pub const MAX_TEXT_SIZE: usize = 10 * 1024 * 1024;  // 10MB
pub const MAX_BATCH_SIZE: usize = 100;
```

## Future Improvements

### Planned Enhancements

1. **Async Batch Linting**: Parallel processing for large batches.
2. **Streaming API**: Process large documents in chunks.
3. **Custom Check Plugins**: User-defined checks.
4. **Performance Dashboard**: Real-time profiling in browser.
5. **Smart Caching**: Cache results based on content hash.

### Potential Improvements

1. **SIMD**: Vectorized string operations.
2. **WebWorkers**: True parallel processing in browser.
3. **Incremental Linting**: Only re-lint changed portions.
4. **Trie-based Config**: Even faster prefix matching.

### Research Areas

1. **Machine Learning**: Context-aware suggestions.
2. **Language Models**: Integration with LLMs for advanced checks.
3. **Real-time Collaboration**: Shared linting state.

## Metrics and Benchmarks

### Current Performance

- **Compilation**: <2s for all checks
- **Initialization**: <10ms (`warm_all()`)
- **Linting**: ~1–5ms per 1000 words
- **Memory**: ~5MB working set
- **Binary Size**: 1.4MB (WASM)

### Targets

- **Linting**: <1ms per 1000 words
- **Binary Size**: <1MB
- **Memory**: <3MB working set

## Conclusion

proselint-wasm achieves high performance through careful architecture choices:
- Two-phase linting with Aho-Corasick pre-filtering.
- Aggressive caching and lazy initialization.
- Binary search for logarithmic lookups.
- UTF-8 correctness without sacrificing performance.
- WASM improvements for minimal binary size.

The architecture balances performance, correctness, and maintainability. This provides a solid foundation for future enhancements.
