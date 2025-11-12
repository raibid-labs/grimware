# Running WebATUI Examples

Quick reference for running the example applications.

## Terminal Examples

### Basic Example
```bash
cargo run --example basic
```
Minimal "Hello World" demonstrating core webatui patterns.

### Dashboard Example
```bash
cargo run --example dashboard
```
Full-featured dashboard with multiple widgets, charts, and tabs.

### Interactive Example
```bash
cargo run --example interactive
```
Interactive demo showing buttons, lists, focus management, and state updates.

## Web Example

### Development Server
```bash
# Install trunk (one-time setup)
cargo install trunk

# Run development server with hot reload
trunk serve --features web
```
Then open http://localhost:8080 in your browser.

### Production Build
```bash
# Install wasm-pack (one-time setup)
cargo install wasm-pack

# Build optimized WASM bundle
wasm-pack build --target web --features web --release

# Serve the built files (use any static server)
python3 -m http.server 8000
```
Then open http://localhost:8000 in your browser.

## Example Features Comparison

| Feature | basic | dashboard | interactive | web_demo |
|---------|-------|-----------|-------------|----------|
| Lines of Code | ~140 | ~312 | ~385 | ~237 |
| Keyboard Input | ✓ | ✓ | ✓ | - |
| Mouse/Click | - | - | - | ✓ |
| State Management | ✓ | ✓ | ✓ | ✓ |
| Multi-Component | - | ✓ | ✓ | ✓ |
| Tab Navigation | - | ✓ | ✓ | ✓ |
| Lists | - | ✓ | ✓ | ✓ |
| Charts | - | ✓ | - | - |
| Gauges | - | ✓ | - | - |
| Buttons | - | - | ✓ | ✓ |
| Focus Management | - | - | ✓ | - |
| Web Deployment | - | - | - | ✓ |

## Quick Tips

**Terminal Examples:**
- Press `q` or `Esc` to quit any terminal example
- All examples support terminal resizing
- Minimum recommended terminal size: 80x24

**Web Example:**
- Use trunk for development (hot reload)
- Use wasm-pack for production builds
- Check browser console for errors
- Requires modern browser with WASM support

## See Also

- [examples/README.md](examples/README.md) - Detailed example documentation
- [index.html](index.html) - Web demo styling and structure
- [Cargo.toml](Cargo.toml) - Example configurations
