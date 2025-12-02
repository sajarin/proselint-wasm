#!/bin/bash
# Build script for proselint-wasm with optimization

set -e

echo "🔨 Building proselint-wasm..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Install with: cargo install wasm-pack"
    exit 1
fi

# Build target (default: web)
TARGET="${1:-web}"

echo "📦 Building for target: $TARGET"
wasm-pack build --release --target "$TARGET"

# Check if wasm-opt is available
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM binary with wasm-opt..."

    ORIGINAL_SIZE=$(wc -c < pkg/proselint_wasm_bg.wasm)

    # Optimize for size
    wasm-opt -Oz -o pkg/proselint_wasm_bg_opt.wasm pkg/proselint_wasm_bg.wasm

    # Replace original with optimized
    mv pkg/proselint_wasm_bg_opt.wasm pkg/proselint_wasm_bg.wasm

    OPTIMIZED_SIZE=$(wc -c < pkg/proselint_wasm_bg.wasm)
    REDUCTION=$(echo "scale=2; (($ORIGINAL_SIZE - $OPTIMIZED_SIZE) / $ORIGINAL_SIZE) * 100" | bc)

    echo "✨ Optimization complete:"
    echo "   Original: $(numfmt --to=iec $ORIGINAL_SIZE)"
    echo "   Optimized: $(numfmt --to=iec $OPTIMIZED_SIZE)"
    echo "   Reduction: ${REDUCTION}%"
else
    echo "⚠️  wasm-opt not found. Skipping optimization."
    echo "   Install from: https://github.com/WebAssembly/binaryen"
fi

echo "✅ Build complete! Output in pkg/"
ls -lh pkg/*.wasm
