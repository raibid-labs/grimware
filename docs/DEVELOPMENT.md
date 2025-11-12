# Development Guide

## Project Structure

This is a **library crate** with runnable examples, not a binary application. The library can be used in both:
- Terminal applications (using ratatui/crossterm)
- Web browsers (compiled to WASM)

## Quick Start

### Run Examples

The project includes several examples demonstrating the library functionality:

```bash
# Run basic example
just example basic

# Run dashboard example
just example dashboard

# Run interactive example
just example interactive

# Or directly with cargo
cargo run --example basic --features terminal
```

### Watch Mode (Auto-rebuild)

Use `bacon` for file watching with automatic rebuilds:

```bash
# Watch and build library
just watch

# Watch and run tests
just watch-test

# Watch specific example
just watch-example basic
just watch-example dashboard
just watch-example interactive

# Watch WASM build
just watch-wasm
```

### Available Bacon Jobs

Bacon is configured with several jobs (see `.bacon/bacon.toml`):

| Job | Command | Description |
|-----|---------|-------------|
| `default` | `bacon` | Build library with terminal features |
| `check` | `bacon check` | Fast syntax check |
| `test` | `bacon test` | Run tests with watch |
| `clippy` | `bacon clippy` | Linting with watch |
| `doc` | `bacon doc` | Generate documentation |
| `wasm` | `bacon wasm` | Build for WASM target |
| `wasm-check` | `bacon wasm-check` | Fast WASM syntax check |
| `example-basic` | `bacon example-basic` | Run basic example with watch |
| `example-dashboard` | `bacon example-dashboard` | Run dashboard example with watch |
| `example-interactive` | `bacon example-interactive` | Run interactive example with watch |
| `examples` | `bacon examples` | Build all examples |

### Manual Commands

```bash
# Build library (native)
cargo build --features terminal

# Build for WASM
cargo build --target wasm32-unknown-unknown --lib --no-default-features

# Run tests (native)
cargo test --features terminal

# Run tests (WASM)
wasm-pack test --node -- --lib --no-default-features

# Build documentation
cargo doc --features terminal --open

# Check code
cargo check --features terminal

# Run clippy
cargo clippy --features terminal
```

## Features

The crate uses feature flags to enable platform-specific dependencies:

### `terminal` (default on native)
Enables terminal UI support with ratatui and crossterm. Required for running examples.

```bash
cargo build --features terminal
cargo test --features terminal
```

### `web`
Enables web/WASM support with Yew and wasm-bindgen.

```bash
cargo build --features web --target wasm32-unknown-unknown
```

### No features (WASM default)
Core state management only, no platform-specific dependencies.

```bash
cargo build --no-default-features
cargo build --target wasm32-unknown-unknown --lib --no-default-features
```

## Testing

### Native Tests
```bash
# Run all tests with terminal features
just test
# or
cargo test --features terminal

# Watch tests
just watch-test
# or
bacon test
```

### WASM Tests
```bash
# Run WASM tests in browser
just wasm-test-browser

# Build WASM (verification)
just wasm-build
```

## Building for Production

### Terminal Application
```bash
# Build optimized binary
cargo build --release --features terminal

# The library will be in target/release/
```

### Web/WASM
```bash
# Build WASM for web
just wasm-build

# Build and serve
just wasm-serve

# The WASM files will be in pkg/
```

## Common Tasks

### Format Code
```bash
just fmt
# or
cargo fmt
```

### Run Linter
```bash
just lint
# or
cargo clippy --features terminal
```

### Clean Build Artifacts
```bash
just clean
# or
cargo clean
```

### Generate Documentation
```bash
cargo doc --features terminal --open
```

## Troubleshooting

### "a bin target must be available for `cargo run`"
This is a **library crate**, not a binary. Use examples instead:
```bash
cargo run --example basic --features terminal
```

### "Device not configured" when running examples
This error occurs when running terminal UI examples in non-terminal environments. Make sure you're running in an actual terminal (not through an IDE's output panel).

### WASM compilation fails
Make sure you're not including terminal features:
```bash
cargo build --target wasm32-unknown-unknown --lib --no-default-features
```

### Tests fail to compile for WASM
Ensure you're using the correct features:
```bash
wasm-pack test --node -- --lib --no-default-features
```

## IDE Setup

### VS Code
Install recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Even Better TOML

### Neovim
Use rust-analyzer with proper feature configuration:
```lua
rust_analyzer = {
  cargo = {
    features = { "terminal" }
  }
}
```

## CI/CD

The project includes GitHub Actions workflows (if configured):
- Native tests with terminal features
- WASM compilation verification
- Clippy linting
- Format checking

## Performance

### Development Builds
- Fast compilation
- Debug symbols enabled
- No optimizations

### Release Builds
```bash
cargo build --release --features terminal
```
- Optimized for performance
- LTO enabled
- Smaller binary size

### WASM Optimization
```bash
just wasm-optimize
```
Uses wasm-opt to further optimize the WASM bundle.

## Architecture

```
webatui-ref/
├── src/
│   ├── lib.rs           # Library root
│   ├── state.rs         # Core state (platform-independent)
│   ├── components/      # Terminal UI components (terminal feature)
│   └── screens/         # Terminal UI screens (terminal feature)
├── examples/
│   ├── basic.rs         # Simple example
│   ├── dashboard.rs     # Dashboard with metrics
│   └── interactive.rs   # Interactive UI
├── tests/
│   ├── integration_test.rs  # Native tests
│   └── wasm_tests.rs        # WASM-specific tests
└── .bacon/
    └── bacon.toml       # Bacon configuration
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --features terminal`
5. Run clippy: `cargo clippy --features terminal`
6. Format code: `cargo fmt`
7. Submit a pull request

## License

MIT OR Apache-2.0
