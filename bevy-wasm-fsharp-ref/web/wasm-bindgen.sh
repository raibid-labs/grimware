#!/usr/bin/env bash
# Build script for compiling the Bevy app to WASM using wasm-bindgen
# Optimized for minimal bundle size and fast loading
set -euo pipefail

echo "🔧 Building bevy-wasm-fsharp-ref for WASM (optimized)..."
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

# Check for optional optimization tools
WASM_OPT_AVAILABLE=false
BROTLI_AVAILABLE=false

if command -v wasm-opt &> /dev/null; then
    WASM_OPT_AVAILABLE=true
    echo "✓ wasm-opt detected (will apply size optimization)"
else
    echo "ℹ️  wasm-opt not found (install with: npm install -g wasm-opt or brew install binaryen)"
fi

if command -v brotli &> /dev/null; then
    BROTLI_AVAILABLE=true
    echo "✓ brotli detected (will generate .br compressed files)"
fi

echo ""

# Build the app for WASM target with optimized profile
echo "📦 Compiling to wasm32-unknown-unknown (wasm-release profile)..."
echo "   - opt-level: z (optimize for size)"
echo "   - lto: true (link-time optimization)"
echo "   - codegen-units: 1 (single unit for better optimization)"
echo ""
cargo build -p app --target wasm32-unknown-unknown --profile wasm-release

# Get target directory
TARGET_DIR=$(cargo metadata --format-version 1 | grep -o '"target_directory":"[^"]*"' | sed 's/"target_directory":"\(.*\)"/\1/')
WASM_FILE="$TARGET_DIR/wasm32-unknown-unknown/wasm-release/app.wasm"

echo ""
echo "📊 Original WASM size:"
du -h "$WASM_FILE"
ORIGINAL_SIZE=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE" 2>/dev/null)

# Run wasm-bindgen to generate JS bindings
echo ""
echo "🔗 Running wasm-bindgen..."
wasm-bindgen \
    --out-dir web/pkg \
    --target web \
    --no-typescript \
    "$WASM_FILE"

# Apply wasm-opt if available
if [ "$WASM_OPT_AVAILABLE" = true ]; then
    echo ""
    echo "⚡ Running wasm-opt for additional size reduction..."
    wasm-opt -Oz web/pkg/app_bg.wasm -o web/pkg/app_bg.wasm.opt
    mv web/pkg/app_bg.wasm.opt web/pkg/app_bg.wasm
fi

# Get optimized size
OPTIMIZED_SIZE=$(stat -f%z web/pkg/app_bg.wasm 2>/dev/null || stat -c%s web/pkg/app_bg.wasm 2>/dev/null)

# Generate compressed versions
echo ""
echo "🗜️  Generating compressed versions..."

# Gzip compression (always available on macOS/Linux)
gzip -9 -k -f web/pkg/app_bg.wasm
echo "   ✓ Created app_bg.wasm.gz"

# Brotli compression (best compression ratio)
if [ "$BROTLI_AVAILABLE" = true ]; then
    brotli -f -k -q 11 web/pkg/app_bg.wasm
    echo "   ✓ Created app_bg.wasm.br"
fi

# Calculate size reductions
echo ""
echo "📊 Bundle Size Report:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
printf "Original (pre-wasm-bindgen):  %'10d bytes  %6s\n" "$ORIGINAL_SIZE" "$(numfmt --to=iec-i --suffix=B $ORIGINAL_SIZE 2>/dev/null || echo '(size)')"
printf "Optimized (.wasm):            %'10d bytes  %6s\n" "$OPTIMIZED_SIZE" "$(numfmt --to=iec-i --suffix=B $OPTIMIZED_SIZE 2>/dev/null || echo '(size)')"

GZIP_SIZE=$(stat -f%z web/pkg/app_bg.wasm.gz 2>/dev/null || stat -c%s web/pkg/app_bg.wasm.gz 2>/dev/null)
printf "Gzip compressed (.wasm.gz):   %'10d bytes  %6s" "$GZIP_SIZE" "$(numfmt --to=iec-i --suffix=B $GZIP_SIZE 2>/dev/null || echo '(size)')"
echo "  ($(awk "BEGIN {printf \"%.1f\", ($GZIP_SIZE/$OPTIMIZED_SIZE)*100}")%)"

if [ "$BROTLI_AVAILABLE" = true ]; then
    BROTLI_SIZE=$(stat -f%z web/pkg/app_bg.wasm.br 2>/dev/null || stat -c%s web/pkg/app_bg.wasm.br 2>/dev/null)
    printf "Brotli compressed (.wasm.br): %'10d bytes  %6s" "$BROTLI_SIZE" "$(numfmt --to=iec-i --suffix=B $BROTLI_SIZE 2>/dev/null || echo '(size)')"
    echo "  ($(awk "BEGIN {printf \"%.1f\", ($BROTLI_SIZE/$OPTIMIZED_SIZE)*100}")%)"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$ORIGINAL_SIZE" -gt 0 ]; then
    REDUCTION=$(awk "BEGIN {printf \"%.1f\", (($ORIGINAL_SIZE-$OPTIMIZED_SIZE)/$ORIGINAL_SIZE)*100}")
    echo "Size reduction: $REDUCTION%"
fi

echo ""
echo "📂 Output files:"
ls -lh web/pkg/app*
echo ""
echo "✅ WASM build complete!"
echo ""
echo "🚀 To serve locally with compression support:"
echo "   just serve-wasm"
echo "   or: cd web && python3 -m http.server 8000"
echo ""
echo "💡 For best performance, configure your web server to:"
echo "   - Serve .wasm.br for browsers supporting Brotli"
echo "   - Serve .wasm.gz for browsers supporting Gzip"
echo "   - Set appropriate Cache-Control headers"
