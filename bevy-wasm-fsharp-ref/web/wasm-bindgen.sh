#!/usr/bin/env bash
# Build script for compiling the Bevy app to WASM using wasm-bindgen
set -euo pipefail

echo "🔧 Building bevy-wasm-fsharp-ref for WASM..."
echo ""

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "⚠️  WASM target not installed. Installing..."
    rustup target add wasm32-unknown-unknown
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "⚠️  wasm-bindgen-cli not found. Installing..."
    cargo install wasm-bindgen-cli
fi

# Build the app for WASM target
echo "📦 Compiling to wasm32-unknown-unknown (release mode)..."
cargo build -p app --target wasm32-unknown-unknown --release

# Run wasm-bindgen to generate JS bindings
echo "🔗 Running wasm-bindgen..."
# Get actual target directory (may be in ~/.cargo/target or project/target)
WASM_FILE=$(cargo metadata --format-version 1 | grep -o '"target_directory":"[^"]*"' | sed 's/"target_directory":"\(.*\)"/\1/')/wasm32-unknown-unknown/release/app.wasm

wasm-bindgen \
    --out-dir web/pkg \
    --target web \
    --no-typescript \
    "$WASM_FILE"

echo ""
echo "✅ WASM build complete!"
echo ""
echo "📂 Output files:"
ls -lh web/pkg/
echo ""
echo "🚀 To serve locally, run:"
echo "   just serve-wasm"
echo "   or: cd web && python3 -m http.server 8000"
