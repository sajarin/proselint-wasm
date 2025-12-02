# Contributing to proselint-wasm

Thank you for your interest in contributing to proselint-wasm! This document provides guidelines for setting up your development environment. It explains how to contribute to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Building the Project](#building-the-project)
- [Running Tests](#running-tests)
- [Code Style](#code-style)
- [Adding New Checks](#adding-new-checks)
- [Submitting Changes](#submitting-changes)
- [Performance Considerations](#performance-considerations)

## Getting Started

### Prerequisites

- **Rust**: Version 1.70 or later (install via [rustup](https://rustup.rs/))
- **wasm-pack**: For building WASM binaries (`cargo install wasm-pack`)
- **Node.js**: Version 16+ (for running JavaScript integration tests)
- **Bun** (optional): For faster JavaScript tooling

### Fork and Clone

1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/proselint-wasm.git
   cd proselint-wasm
   ```

## Development Setup

### Install Dependencies

```bash
# Install Rust dependencies (automatically done on first build)
cargo build

# Install wasm-pack for WASM builds
cargo install wasm-pack

# Install development tools (optional but recommended)
cargo install cargo-watch    # For auto-recompiling
cargo install cargo-expand   # For macro details
```

### VS Code Setup (Recommended)

Install these extensions:
- **rust-analyzer**: For code completion and inline errors
- **Better TOML**: For Cargo.toml editing
- **Error Lens**: For inline error display

Add this to `.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all"
}
```

## Building the Project

### Native Rust Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build —release

# Check without building
cargo check
```

### WASM Build

```bash
# Build for web with wasm-pack
wasm-pack build —target web

# Build for Node.js
wasm-pack build —target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build —target bundler
```

The WASM output will be in the `pkg/` directory.

### Optimized WASM Build

For production, use `wasm-opt` for further size reduction:

```bash
wasm-pack build —release
wasm-opt -Oz -o pkg/proselint_wasm_bg_opt.wasm pkg/proselint_wasm_bg.wasm
```

## Running Tests

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test —— —nocapture

# Run tests in release mode (faster)
cargo test —release
```

### Test Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate HTML coverage report
cargo tarpaulin —out Html
```

### Benchmarking

```bash
# Run benchmarks (requires nightly)
cargo +nightly bench

# Or use criterion (add to dev-dependencies first)
cargo bench
```

## Code Style

### Formatting

We use `rustfmt` with default settings:

```bash
# Format all code
cargo fmt

# Check formatting without changing files
cargo fmt —— —check
```

### Linting

We use `clippy` for linting:

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings as errors
cargo clippy —— -D warnings

# Run clippy with pedantic warnings
cargo clippy —— -W clippy::pedantic
```

### Code Guidelines

- **Use descriptive variable names**: Prefer `line_start_byte` over `ls`
- **Document public APIs**: All public functions must have doc comments
- **Add tests for new features**: Aim for >80% test coverage
- **Keep functions small**: Ideally under 50 lines
- **Avoid unsafe code**: Unless necessary and well-documented
- **Handle errors properly**: Use `Result` types, avoid `unwrap()` in library code
- **Profile first**: Then improve hot paths

## Adding New Checks

### 1. Create a New Check Module

Add your check to the appropriate category in `src/checks/`:

```rust
// src/checks/your_category.rs

use crate::check::{Check, Severity};

pub fn get_checks() -> Vec<Check> {
    vec![
        Check::new(
            "category.subcategory.check_name",
            "Issue description",
            r"regex_pattern",
        )
        .with_severity(Severity::Warning)
        .with_replacement("suggested replacement"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_check() {
        let checks = get_checks();
        let check = &checks[0];

        let results = check.run("text with issue");
        assert_eq!(results.len(), 1);
    }
}
```

### 2. Register the Check

Add to `src/checks/mod.rs`:

```rust
pub mod your_category;

// In build_all_checks()
checks.extend(your_category::get_checks());
```

### 3. Test the Check

```bash
cargo test your_category::tests
```

### 4. Validate Regex Patterns

Verify that your regex compiles correctly:

```bash
cargo test —— validate_all_checks
```

## Submitting Changes

### Before Submitting

1. **Run all tests**: `cargo test`
2. **Format code**: `cargo fmt`
3. **Run clippy**: `cargo clippy —— -D warnings`
4. **Build WASM**: `wasm-pack build`
5. **Update CHANGELOG.md**: Document your changes
6. **Add tests**: For new features or bug fixes

### Commit Messages

Use conventional commit format:

```
feat: add support for custom quote styles
fix: correct UTF-8 handling in position tracking
docs: update README with thread safety information
perf: <!-- proselint-ignore -->optimize QuoteTracker with binary search
test: add comprehensive UTF-8 tests
```

### Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/your-feature`.
2. **Make your changes**: Follow the code guidelines above.
3. **Commit with clear messages**: Use conventional commits.
4. **Push to your fork**: `git push origin feature/your-feature`.
5. **Open a Pull Request** including:
   - Clear description of changes
   - Link to related issues
   - Test results
   - Performance impact (if applicable)

### PR Checklist

- [ ] All tests pass (`cargo test`)
- [ ] We formatted the code (`cargo fmt`)
- [ ] Clippy passes (`cargo clippy`)
- [ ] We updated documentation
- [ ] We updated CHANGELOG.md
- [ ] We added new tests for new features
- [ ] No breaking changes (or clearly documented)

## Performance Considerations

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph (requires sudo on Linux)
cargo flamegraph —test test_name
```

### Performance Tips

1. **Use Aho-Corasick for keyword matching**: See `src/engine.rs`.
2. **Cache compiled regexes**: Use `OnceLock` for lazy initialization.
3. **Avoid allocations in hot paths**: Reuse buffers where possible.
4. **Profile first**: Then measure impact with `cargo bench`.
5. **Consider binary search**: For sorted data (see `QuoteTracker`).

### WASM Performance

- **Size matters**: Keep binary size under 2MB if possible
- **Minimize string allocations**: Reuse memory between calls
- **Use `warm_all()`**: Pre-compile regexes during initialization
- **Batch operations**: Use `lint_batch()` for multiple texts

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

### Key Components

- **`engine.rs`**: Linting orchestration with Aho-Corasick pre-filtering
- **`check.rs`**: Check definitions and regex caching
- **`position.rs`**: UTF-8-aware line/column tracking
- **`config.rs`**: Configuration management with caching
- **`checks/`**: Check implementations organized by category

## Questions?

- **Bug reports**: Open an issue on GitHub
- **Feature requests**: Open an issue with [Feature Request] prefix
- **Questions**: Open a discussion on GitHub Discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
