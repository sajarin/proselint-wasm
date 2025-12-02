# Changelog

All notable changes to this project will be documented in this file.

This format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive UTF-8 support for multi-byte characters (emoji, CJK characters, accented letters).
- Support for Windows (`\r\n`), Unix (`\n`), and old Mac (`\r`) line endings.
- Input validation with configurable size limits (10MB max text, 100 max batch size).
- Binary search improvement for `QuoteTracker` (O(n) → O(log n) performance).
- Prefix-based caching for config lookups.
- Regex validation with error reporting via `validate_all_checks()`.
- Error reporting in WASM methods (returns JSON error objects instead of silent failures).
- Comprehensive test suite for UTF-8 and line ending edge cases.
- `CONTRIBUTING.md` with development setup instructions.
- `ARCHITECTURE.md` documenting design decisions.
- GitHub Actions CI/CD pipeline.
- Performance benchmarks using Criterion.
- Property-based testing with quickcheck.
- WASM integration tests.

### Changed
- **BREAKING**: `LineTracker::new()` now stores the original text for UTF-8 character counting
- **BREAKING**: Column numbers are now character-based instead of byte-based (proper UTF-8 support)
- `QuoteTracker::is_in_quote()` and `overlaps_quote()` now use binary search for better performance
- `Config::is_check_enabled()` now uses sorted prefix list for faster lookups
- Improved error handling in `lint()` and `lint_batch()` methods
- Regex compilation failures now log warnings to stderr with detailed error messages
- `offset_to_position()` now safely handles invalid UTF-8 byte boundaries

### Fixed
- UTF-8 position tracking bug where multi-byte characters caused incorrect column numbers
- Line boundary detection for edge cases (newline at exact offsets)
- Silent regex compilation failures now report errors
- Windows line ending handling (`\r\n` properly recognized as single line break)
- Mixed line ending support (documents with both `\n` and `\r\n`)

### Removed
- Unused `lazy_static` dependency (replaced with `once_cell`)

### Performance
- QuoteTracker: O(n) → O(log n) for quote position checks (binary search).
- Config lookup: Reduced iterations with sorted prefix matching.
- Position tracking: Character-based counting with UTF-8 boundary validation.

### Security
- Added input size validation to prevent DoS attacks (10MB max per text, 100 texts max per batch).
- Proper error boundaries to avoid panic on malformed input.

## [0.1.0]—2025–01–30

### Added
- Initial release of proselint-wasm
- Full Rust/WASM port of Python proselint
- Aho-Corasick pre-filtering for performance optimization
- 100+ prose linting checks across 18 categories:
  - Typography (ellipsis, em dash, curly quotes)
  - Weak modifiers and intensifiers
  - Tentative phrasing
  - Repetitive wording
  - Cliches and jargon
  - Corporate speak
  - Spelling and grammar
- Native Rust API (`Linter`)
- WASM/JavaScript API (`Proselint`)
- Batch linting support.
- Configurable check enabling/disabling.
- Quote-aware matching.
- Severity levels (error, warning, suggestion).
- `warm_all()` method for regex pre-compilation.
- Comprehensive test suite (51 tests).
- README with usage examples.
- MIT license.

[Unreleased]: https://github.com/sajarin/proselint-wasm/compare/v0.1.0…HEAD
[0.1.0]: https://github.com/sajarin/proselint-wasm/releases/tag/v0.1.0
