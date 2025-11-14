# WASM Build Setup for bevy-wasm-fsharp-ref

## Overview

This document describes the WASM build configuration completed for the bevy-wasm-fsharp-ref project.

## Build Configuration

### Cargo.toml Changes

**`crates/app/Cargo.toml`**:
- Changed package name from `bevy-wasm-fsharp-ref-app` to `app`
- Changed crate type from `[[bin]]` to `[lib]` with `crate-type = ["cdylib", "rlib"]`
- Added `wasm-bindgen` as a WASM-only dependency
- Added `getrandom` 0.3 with `wasm_js` feature for WASM target
- Configured WASM-specific Bevy features (webgl2, minimal plugins)

### Source Code Changes

**`crates/app/src/main.rs` → `crates/app/src/lib.rs`**:
- Renamed file from `main.rs` to `lib.rs` (required for cdylib crate type)
- Added `#[cfg(target_arch = "wasm32")]` import for wasm-bindgen
- Changed `fn main()` to `pub fn main()` with `#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]` attribute
- Added WASM-specific window configuration (canvas selector, fit to parent)

### Build Script

**`web/wasm-bindgen.sh`**:
- Checks for and installs wasm32-unknown-unknown target if missing
- Checks for and installs wasm-bindgen-cli if missing
- Builds the app for wasm32-unknown-unknown target in release mode
- Automatically detects cargo target directory (handles custom paths)
- Runs wasm-bindgen to generate JavaScript bindings
- Outputs WASM bundle to `web/pkg/`

### HTML Loader

**`web/index.html`**:
- Modern, polished UI with loading screen and error handling
- Dynamic WASM module loading via ES6 modules
- Proper canvas element (`#bevy`) for Bevy rendering
- Comprehensive error reporting with helpful debugging tips
- Controls documentation displayed on page
- Styled with gradient background and responsive design

### Cargo Configuration

**`.cargo/config.toml`**:
- Sets `rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]` for wasm32-unknown-unknown
- Required for getrandom 0.3 to work with WASM

### Git Configuration

**`.gitignore`**:
- Added `/web/pkg` to ignore generated WASM files
- Standard Rust/IDE ignore patterns

## Building for WASM

### Quick Build

```bash
# Build and serve in one command
just wasm

# Or separately
just build-wasm
just serve-wasm
```

### Manual Build

```bash
# Using the script directly
bash web/wasm-bindgen.sh

# Or with cargo (requires wasm-bindgen-cli installed)
cargo build -p app --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web/pkg --target web \
    ~/.cargo/target/wasm32-unknown-unknown/release/app.wasm
```

## Serving Locally

```bash
# Serve on http://localhost:8000
cd web && python3 -m http.server 8000

# Or use justfile
just serve-wasm
```

Then open http://localhost:8000 in your browser.

## Technical Details

### getrandom 0.3 WASM Support

Bevy 0.15 uses getrandom 0.3 (via ahash and const-random). For WASM:

1. **Feature**: Add `getrandom = { version = "0.3", features = ["wasm_js"] }` as dependency
2. **Config**: Set `rustflags = ["--cfg", "getrandom_backend=\"wasm_js\""]` in `.cargo/config.toml`

Without both, you'll get compile errors about missing wasm_js backend.

### cdylib vs bin

WASM requires a library crate (`cdylib`) not a binary:
- **Binary** (`[[bin]]`): Creates a `.wasm` executable (doesn't work with wasm-bindgen)
- **Library** (`cdylib`): Creates a `.wasm` library that wasm-bindgen can process

The `#[wasm_bindgen(start)]` attribute marks the entry point.

### Window Configuration

```rust
#[cfg(target_arch = "wasm32")]
canvas: Some("#bevy".to_string()),
#[cfg(target_arch = "wasm32")]
fit_canvas_to_parent: true,
```

These settings tell Bevy:
- Render into the `<canvas id="bevy">` element
- Resize canvas to fit its container

## Testing

1. **Build**: `just build-wasm` - should complete without errors
2. **Files**: Check `web/pkg/app.js` and `web/pkg/app_bg.wasm` exist
3. **Serve**: `just serve-wasm` - starts HTTP server
4. **Browser**: Open http://localhost:8000
   - Should see loading screen, then game
   - Press F12 for console
   - Press SPACE to attack
   - Combat events logged to console

## Troubleshooting

### getrandom errors

```
error: The wasm32-unknown-unknown targets are not supported by default
```

**Solution**: Check `.cargo/config.toml` has the rustflags, and `getrandom` dependency has `wasm_js` feature.

### File not found errors

```
error: failed reading 'target/wasm32-unknown-unknown/release/app.wasm'
```

**Solution**: Your cargo may use `~/.cargo/target`. The build script auto-detects this.

### Module not found in browser

```
Failed to load WASM module: Error: Cannot find module './pkg/app.js'
```

**Solution**: Run `just build-wasm` first to generate files in `web/pkg/`.

### Blank screen in browser

- Check browser console (F12) for JavaScript errors
- Ensure you're serving via HTTP (not `file://` URLs)
- Verify `<canvas id="bevy">` exists in HTML
- Check network tab to see if WASM file loaded

## File Sizes

WASM bundle sizes (release mode):
- `app_bg.wasm`: ~53MB (Bevy is large, includes rendering, audio, etc.)
- `app.js`: ~106KB (JavaScript bindings)

**Note**: WASM is loaded asynchronously and cached by browser.

## Browser Compatibility

Tested with:
- Chrome/Edge (Chromium)
- Firefox
- Safari (WebKit)

Requires:
- WebAssembly support
- WebGL2 support
- ES6 modules

## Performance

WASM builds are optimized (`--release` mode):
- Optimizations: On
- Debug symbols: Stripped
- Dead code: Eliminated

For further optimization, consider:
- `wasm-opt` tool (from binaryen)
- Compression (gzip/brotli)
- Code splitting (future work)

## Future Improvements

- [ ] Optimize bundle size with `wasm-opt`
- [ ] Add service worker for offline support
- [ ] Implement loading progress bar
- [ ] Add fullscreen toggle button
- [ ] Mobile touch controls
- [ ] Deploy to GitHub Pages / Netlify
